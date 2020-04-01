use log::{debug, error, info, trace, warn};
use mpq::Archive;

use crate::globals::{GameVersion, MAP_TRIGGERS};
use crate::globals::GameVersion::{RoC, TFT};
use crate::map_data::binary_reader::BinaryReader;
use crate::map_data::data_ini::DataIni;
use crate::map_data::triggers::config::{TriggerCategory, VariableDefinition};
use crate::map_data::triggers::trigger_data::ECADefinition;

mod enums;
mod trigger_data;
mod wtg_tests;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum WtgError {
    ParameterConversionError(String),
    SubParameterConversionError(String),
    ECAConversionError(String),
    ConditionConversionError(String),
    WtgParsingIsntCompleteError(String),
    UnknownProp(String),
}


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
    pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Result<Self, WtgError> {
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
            ecas.push(ECADefinition::from(reader, game_version, trigger_data, false)?);
        }
        Ok(Self{
            name, description, is_comment, enabled, is_script, is_on, run_init, index_category, ecas
        })
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
            triggers.push(TriggerDefinition::from(reader, &version, trigger_data).unwrap())
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