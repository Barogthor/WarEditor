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
            let ftype = reader.read_u32();
            let ftype = ECAType::from(ftype);
            let condition_group = match (game_version, is_child_eca){
                (RoC, _) | (_, false)  => None,
                (_, true) => {
                    let condition = ConditionType::from(reader.read_u32());
                    Some(condition)
                }
            };
            let name = reader.read_c_string().into_string().unwrap();
            let eca_info = trigger_data.get_prop(ftype.get_sector(), &name);
            match eca_info{
                Some(info) => {
                    let info_split = info.split(",").collect::<Vec<&str>>();
                    let count_parameters = match info_split[1]{
                        "nothing" => 0,
                        _ => info_split.len() - 1
                    };
                    let mut parameters = vec![];
                    let enabled = reader.read_u32() == 1;
                    for _ in 0..count_parameters{
                        parameters.push(Parameter::from(reader, game_version, trigger_data));
                    }
                    let childs_eca = match game_version {
                        (RoC) => None,
                        (_) => {
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
        fn for_roc(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni, ptype: &ParameterType) -> (Option<i32>, Option<Box<Parameter>>){
            let unknown = match ptype{
                ParameterType::FUNCTION => Some(reader.read_i32()),
                _ => None
            };
            let array_parameter = match ptype{
                ParameterType::FUNCTION => None,
                _ => {
                    let is_array = reader.read_u32() == 1;
                    if is_array {
                        let p = Parameter::from(reader, game_version, trigger_data);
                        Some(Box::new(p))
                    } else { None }
                }
            };
            (unknown, array_parameter)
        }

        fn for_tft(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni, ptype: &ParameterType, has_sub_parameter: bool) -> (Option<i32>, Option<Box<Parameter>>) {
            let unknown = match has_sub_parameter{
                false => None,
                true => Some(reader.read_i32())
            };
            let is_array = reader.read_u32() == 1;
            let array_parameter = match is_array {
                false => None,
                true => {
                    let p = Parameter::from(reader, game_version, trigger_data);
                    Some(Box::new(p))
                }
            };
            (unknown, array_parameter)
        }

        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Self{
            let ptype = reader.read_i32();
            let ptype = ParameterType::from(ptype);
            let value = reader.read_c_string().into_string().unwrap();
            let has_sub_parameters = reader.read_u32() == 1;
            let sub_parameters = match has_sub_parameters {
                false => None,
                true => Some(SubParameters::from(reader, game_version, trigger_data))
            };

            let (unknown, array_parameter) = match game_version{
                RoC => Self::for_roc(reader, game_version, trigger_data, &ptype),
                _ => Self::for_tft(reader, game_version, trigger_data, &ptype, has_sub_parameters),
            };

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
            let ptype = reader.read_u32();
            let ptype = SubParameterType::from(ptype);
            let name = reader.read_c_string().into_string().unwrap();
            let info_parameters = trigger_data.get_prop(ptype.get_sector(), &name);
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
        // for _ in 0..3{
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

#[cfg(test)]
mod trigger_file_tests{
    use std::io::Cursor;

    use crate::GameData;
    use crate::globals::GameVersion::TFT;
    use crate::map_data::binary_reader::BinaryReader;
    use crate::map_data::trigger_file::trigger::TriggerDefinition;

    const TFT_TRIGGER_EVENT: [u8;116] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x54, 0x69, 0x6D, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x53, 0x69, 0x6E, 0x67, 0x6C, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x35, 0x2E, 0x30, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_CONDITION: [u8;266] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x43, 0x6F, 0x6E, 0x64, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x43, 0x6F, 0x6E, 0x64, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x43, 0x6F, 0x6D, 0x70, 0x61, 0x72, 0x65, 0x4F, 0x72, 0x64, 0x65, 0x72, 0x43, 0x6F, 0x64, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x47, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x64, 0x4F, 0x72, 0x64, 0x65, 0x72, 0x49, 0x64, 0x42, 0x4A, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x47, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x64, 0x4F, 0x72, 0x64, 0x65, 0x72, 0x49, 0x64, 0x42, 0x4A, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x45, 0x71, 0x75, 0x61, 0x6C, 0x45, 0x4E, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x32, 0x4F, 0x72, 0x64, 0x65, 0x72, 0x49, 0x64, 0x42, 0x4A, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x32, 0x4F, 0x72, 0x64, 0x65, 0x72, 0x49, 0x64, 0x42, 0x4A, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x73, 0x74, 0x6F, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_ACTION: [u8;102] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x53, 0x6C, 0x65, 0x65, 0x70, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_DISABLED: [u8;119] = [54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6C, 0x65, 0x64, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6C, 0x65, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x53, 0x6C, 0x65, 0x65, 0x70, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_DISABLED_BUT_ON: [u8;118] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6C, 0x65, 0x64, 0x42, 0x75, 0x74, 0x4F, 0x6E, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6C, 0x65, 0x64, 0x20, 0x42, 0x75, 0x74, 0x20, 0x4F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x53, 0x6C, 0x65, 0x65, 0x70, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_DISABLED_NOT_ON: [u8;118] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6C, 0x65, 0x64, 0x4E, 0x6F, 0x74, 0x4F, 0x6E, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6C, 0x65, 0x64, 0x20, 0x4E, 0x6F, 0x74, 0x20, 0x4F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x53, 0x6C, 0x65, 0x65, 0x70, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_IF_THEN_ELSE: [u8;312] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x49, 0x66, 0x54, 0x68, 0x65, 0x6E, 0x45, 0x6C, 0x73, 0x65, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x49, 0x66, 0x20, 0x54, 0x68, 0x65, 0x6E, 0x20, 0x45, 0x6C, 0x73, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x49, 0x66, 0x54, 0x68, 0x65, 0x6E, 0x45, 0x6C, 0x73, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x43, 0x6F, 0x6D, 0x70, 0x61, 0x72, 0x65, 0x42, 0x6F, 0x6F, 0x6C, 0x65, 0x61, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x54, 0x52, 0x55, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x45, 0x71, 0x75, 0x61, 0x6C, 0x45, 0x4E, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x74, 0x72, 0x75, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x44, 0x6F, 0x4E, 0x6F, 0x74, 0x68, 0x69, 0x6E, 0x67, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x53, 0x6C, 0x65, 0x65, 0x70, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x44, 0x6F, 0x4E, 0x6F, 0x74, 0x68, 0x69, 0x6E, 0x67, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x44, 0x6F, 0x4E, 0x6F, 0x74, 0x68, 0x69, 0x6E, 0x67, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const TFT_TRIGGER_FOR_LOOP_A: [u8;160] = [0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x4C, 0x6F, 0x6F, 0x70, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x4C, 0x6F, 0x6F, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x46, 0x6F, 0x72, 0x4C, 0x6F, 0x6F, 0x70, 0x41, 0x4D, 0x75, 0x6C, 0x74, 0x69, 0x70, 0x6C, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x31, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x53, 0x6C, 0x65, 0x65, 0x70, 0x41, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    #[test]
    fn parse_event_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_EVENT.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos()as usize, TFT_TRIGGER_EVENT.len());
    }

    #[test]
    fn parse_condition_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_CONDITION.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos()as usize, TFT_TRIGGER_CONDITION.len());
    }

    #[test]
    fn parse_action_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_ACTION.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos()as usize, TFT_TRIGGER_ACTION.len());
    }

    #[test]
    fn parse_disabled_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_DISABLED.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos()as usize, TFT_TRIGGER_DISABLED.len());
    }

    #[test]
    fn parse_disabled_not_on_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_DISABLED_BUT_ON.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos()as usize, TFT_TRIGGER_DISABLED_BUT_ON.len());
    }

    #[test]
    fn parse_if_then_else_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_IF_THEN_ELSE.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos() as usize, TFT_TRIGGER_IF_THEN_ELSE.len());
    }

    #[test]
    fn parse_loop_a_trigger(){
        let game_data = GameData::new();
        let buffer = TFT_TRIGGER_FOR_LOOP_A.to_vec();
        let mut reader = BinaryReader::new(buffer);
        let trigger = TriggerDefinition::from(&mut reader, &TFT, game_data.get_trigger_data());
        assert_eq!(reader.pos() as usize, TFT_TRIGGER_FOR_LOOP_A.len());
    }
}