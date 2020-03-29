use std::ffi::CString;
use std::fs::File;
use std::io::Read;

use log::{debug, error, info, trace, warn};
use mpq::Archive;

use crate::GameData;
use crate::globals::GameVersion::{self, RoC, TFT};
use crate::globals::MAP_TRIGGERS;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;
use crate::map_data::concat_path;
use crate::map_data::data_ini::DataIni;
use crate::map_data::trigger_file::config::{TriggerCategory, VariableDefinition};
use crate::map_data::trigger_file::trigger::TriggerDefinition;

mod config{
    use crate::globals::GameVersion;
    use crate::map_data::binary_reader::BinaryReader;

    #[derive(Debug, Default)]
    pub struct VariableDefinition {
        name: String,
        var_type: String,
        unknown: i32,
        is_array: bool,
        array_size: u32,
        initialized: bool,
        init_value: String,
    }

    impl VariableDefinition {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
            let mut def = Self::default();
            def.name = reader.read_c_string().into_string().unwrap();
            def.var_type = reader.read_c_string().into_string().unwrap();
            def.unknown = reader.read_i32();
            def.is_array = reader.read_u32() == 1;
            if game_version.is_tft() {
                def.array_size = reader.read_u32();
            }
            def.initialized = reader.read_u32() == 1;
            def.init_value = reader.read_c_string().into_string().unwrap();
            def
        }
    }


    #[derive(Debug, Default)]
    pub struct TriggerCategory {
        id: u32,
        name: String,
        is_comment: bool,
    }

    impl TriggerCategory {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion) -> Self{
            let mut def = Self::default();

            def.id = reader.read_u32();
            def.name = reader.read_c_string().into_string().unwrap();
            if game_version.is_tft() { def.is_comment = reader.read_u32() == 1; }
            def
        }
    }


}



mod trigger {
    use std::fmt::{Debug, Error, Formatter};

    use crate::map_data::trigger_file::trigger::ParameterType::{FUNCTION, INVALID, PRESET, STRING, VARIABLE};

    use super::*;

    #[derive(Debug)]
    pub struct TriggerDefinition {
        name: String,
        description: String,
        is_comment: Option<bool>,
        enabled: bool,
        is_script: bool,
        is_on: bool,
        run_init: bool,
        index_category: u32,
        ecas: Vec<ECADefinition>,
    }

    impl TriggerDefinition {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Self {
            debug!("======= Trigger Definition =======");
            let name = reader.read_c_string().into_string().unwrap();
            debug!("======= [TRIGGER] name: {}", name);
            let description = reader.read_c_string().into_string().unwrap();
            let is_comment = match game_version {
                RoC => None,
                _ => Some(reader.read_u32() != 0)
            };
            let enabled = reader.read_u32() == 1;
            let is_script = reader.read_u32() == 1;
            let is_on = reader.read_u32() == 0;
            let run_init = reader.read_i32() == 0;
            let index_category = reader.read_u32();
            let count_ecas = reader.read_u32();
            let mut ecas = vec![];
            for _ in 0..count_ecas{
                ecas.push(ECADefinition::from(reader, game_version, trigger_data, false));
            }
            Self{
                name, description, is_comment, enabled, is_script, is_on, run_init, index_category, ecas
            }
        }
    }


    #[derive(Debug)]
    struct ECADefinition {
        ftype: ECAType,
        condition_group: Option<ConditionType>,
        name: String,
        enabled: bool,
        parameters: Vec<Parameter>,
        childs_eca: Option<Vec<ECADefinition>>
    }

    impl ECADefinition {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni, is_child_eca: bool) -> Self{
            debug!("============= ECA Definition (child: {}) =============", is_child_eca);
            let ftype = reader.read_u32();
            let ftype = ECAType::from(ftype);
            debug!("========== [ECA] (byte: {}) - type: {:?}", reader.pos(), ftype);
            let condition_group = match (game_version, is_child_eca){
                (RoC, _) | (_, false)  => None,
                (_, true) => {
                    let condition = ConditionType::from(reader.read_u32());
                    Some(condition)
                }
            };
            debug!("========== [ECA] (byte: {}) - cond group: {:?}", reader.pos(), condition_group);
            let name = reader.read_c_string().into_string().unwrap();
            debug!("========== [ECA] (byte: {}) - name: {}", reader.pos(), name);
            let eca_info = trigger_data.get_prop(ftype.get_sector(), &name);
            debug!("========== [ECA] (byte: {}) - eca info: {:?}", reader.pos(), eca_info);
            match eca_info{
                Some(info) => {
                    let info_split = info.split(",").collect::<Vec<&str>>();
                    let count_parameters = match info_split[1]{
                        "nothing" => 0,
                        _ => info_split.len() - 1
                    };
                    let mut parameters = vec![];
                    let enabled = reader.read_u32() == 1;
                    debug!("========== [ECA] (byte: {}) - enabled: {}", reader.pos(), enabled);
                    for _ in 0..count_parameters{
                        parameters.push(Parameter::from(reader, game_version, trigger_data));
                    }
                    let childs_eca = match (game_version, is_child_eca) {
                        (RoC, _) | (_, false) => None,
                        (_, true) => {
                            let count_childs = reader.read_u32();
                            let mut v = vec![];
                            for _ in 0..count_childs{
                                v.push(ECADefinition::from(reader, game_version, trigger_data, true));
                            };
                            Some(v)
                        }
                    };
                    return Self{
                        ftype, condition_group, name, enabled, parameters, childs_eca
                    };
                },
                None => {
                    error!("sector {} or function {} doesn't seem to exist", ftype.get_sector(), name);
                }
            }
            panic!("couldn't finish ECA definition");
        }
    }

    #[derive(Debug)]
    pub struct Parameter {
        ptype: ParameterType,
        value: String,
        sub_parameters: Option<SubParameters>,
        unknown: Option<i32>,
        array_parameter: Option<Box<Self>>,
    }

    impl Parameter {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Self{
            debug!("============= Parameter Definition =============");
            let ptype = reader.read_i32();
            let ptype = ParameterType::from(ptype);
            debug!("============ [Param] (byte: {}) - type: {:?}", reader.pos(), ptype);
            let value = reader.read_c_string().into_string().unwrap();
            debug!("============ [Param] (byte: {}) - value: {}", reader.pos(), value);
            let has_sub_parameters = reader.read_u32() == 1;
            debug!("============ [Param] (byte: {}) - has sub: {}", reader.pos(), has_sub_parameters);
            let sub_parameters = if has_sub_parameters {
                Some(SubParameters::from(reader, game_version, trigger_data))
            } else {None};
            let unknown = match (game_version, ptype){
                (RoC, ParameterType::FUNCTION) => Some(reader.read_i32()),
                (RoC, _) => None,
                (_, _) => Some(reader.read_i32())
            };
            debug!("============ [Param] (byte: {}) - unknown: {:?}", reader.pos(), unknown);
            let array_parameter = match (game_version, ptype) {
                (RoC, FUNCTION) => None,
                (_, _) => {
                    let is_array = reader.read_u32() == 1;
                    debug!("============ [Param] (byte: {}) - is array: {}", reader.pos(), is_array);
                    if is_array {
                        let p = Parameter::from(reader, game_version, trigger_data);
                        Some(Box::new(p))
                    } else { None }
                }
            };
            debug!("============ [Param] (byte: {}) - array_parameter: {:?}", reader.pos(), array_parameter);
            Self{
                ptype, value, sub_parameters, unknown, array_parameter
            }
        }
    }

    #[derive(Debug)]
    pub struct SubParameters {
        ptype: SubParameterType,
        name: String,
        parameters: Vec<Parameter>,
    }

    impl SubParameters {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Self{
            debug!("================ SubParameter Definition ================");
            let ptype = reader.read_u32();
            let ptype = SubParameterType::from(ptype);
            debug!("============== [SubParam] (byte: {}) - type: {:?}", reader.pos(), ptype);
            let name = reader.read_c_string().into_string().unwrap();
            debug!("============== [SubParam] (byte: {}) - name: {}", reader.pos(), name);
            let info_parameters = trigger_data.get_prop(ptype.get_sector(), &name);
            debug!("============== [SubParam] (byte: {}) - infos: {:?}", reader.pos(), info_parameters);
            match info_parameters{
                Some(info) => {
                    let substract = match ptype{
                        SubParameterType::Call => 3,
                        _ => 1
                    };
                    let info_split = info.split(",").collect::<Vec<&str>>();
                    let count_parameters = if info_split.len() <=substract || info_split[substract] == "nothing"{
                         0
                    } else {
                        info_split.len() - substract
                    };
                    let mut parameters = vec![];
                    // if count_parameters > 0 {
                        let begin_parameters = reader.read_u32() != 0;
                        if begin_parameters {
                            for _ in 0..count_parameters {
                                parameters.push(Parameter::from(reader, game_version, trigger_data));
                            }
                        }
                    // }
                    return Self{
                        ptype, name, parameters
                    }
                },
                None => {
                    error!("sector {} or function {} doesn't seem to exist", ptype.get_sector(), name);
                }
            }
            panic!("couldn't finish Sub parameter definition");
        }
    }


    #[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
    pub enum ParameterType {
        PRESET,
        VARIABLE,
        FUNCTION,
        STRING,
        INVALID,
    }

    impl ParameterType {
        pub fn from(n: i32) -> Self {
            match n{
                0 => PRESET,
                1 => VARIABLE,
                2 => FUNCTION,
                3 => STRING,
                -1 => INVALID,
                _ => {
                    info!("Unknown Parameter type {} was found", n);
                    INVALID
                }
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub enum ECAType {
        Event,
        Condition,
        Action
    }
    impl ECAType {
        pub fn from(n: u32) -> ECAType {
            match n{
                0 => (ECAType::Event),
                1 => (ECAType::Condition),
                2 => (ECAType::Action),
                _ => panic!("Unknown function type {}",n)
            }
        }
        pub fn get_sector(&self) -> &str {
            match self{
                ECAType::Event => "TriggerEvents",
                ECAType::Condition => "TriggerConditions",
                ECAType::Action => "TriggerActions",
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub enum SubParameterType {
        Event,
        Condition,
        Action,
        Call
    }
    impl SubParameterType {
        pub fn from(n: u32) -> Self {
            match n{
                0 => (SubParameterType::Event),
                1 => (SubParameterType::Condition),
                2 => (SubParameterType::Action),
                3 => (SubParameterType::Call),
                _ => panic!("Unknown sub parameter type {}",n)
            }
        }
        pub fn get_sector(&self) -> &str {
            match self{
                SubParameterType::Event => "TriggerEvents",
                SubParameterType::Condition => "TriggerConditions",
                SubParameterType::Action => "TriggerActions",
                SubParameterType::Call => "TriggerCalls",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub enum ConditionType{
        Condition,
        Then,
        Else
    }
    impl ConditionType{
        pub fn from(n: u32) -> ConditionType {
            match n{
                0 => ConditionType::Condition,
                1 => ConditionType::Then,
                2 => ConditionType::Else,
                _ => panic!("Unknown condition type {}",n)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct TriggersFile {
    id: String,
    version: GameVersion,
    categories: Vec<TriggerCategory>,
    unknown: i32,
    vars: Vec<VariableDefinition>,
    triggers: Vec<TriggerDefinition>,
}

impl TriggersFile {
    pub fn read_file(mpq: &mut Archive, trigger_data: &DataIni) -> Self{
        let file = mpq.open_file(MAP_TRIGGERS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];
        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        Self::from(&mut reader, trigger_data)
    }

    fn from(reader: &mut BinaryReader, trigger_data: &DataIni) -> Self{
        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let version = reader.read_u32();
        let version = to_game_version(version);
        let count_categories = reader.read_u32();
        let mut categories = vec![];
        for _ in 0..count_categories {
            categories.push(TriggerCategory::from(reader, &version));
        }
        let unknown = reader.read_i32();
        let count_vars = reader.read_u32();
        let mut vars = vec![];
        for _ in 0..count_vars {
            vars.push(VariableDefinition::from(reader, &version));
        }
        let count_triggers = reader.read_u32();
        let mut triggers = vec![];
        for _ in 0..count_triggers{
            triggers.push(TriggerDefinition::from(reader, &version, trigger_data))
        }
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_TRIGGERS, reader.size() - reader.pos() as usize);
        Self{
            id, version, categories, unknown, vars, triggers
        }
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

fn to_game_version(value: u32) -> GameVersion{
    match value{
        4 => RoC,
        7 => TFT,
        _ => panic!("Unknown or unsupported game version '{}'", value)
    }
}