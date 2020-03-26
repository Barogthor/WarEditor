use std::ffi::CString;
use std::fs::File;
use std::io::Read;

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
    use std::ffi::CString;

    use crate::globals::GameVersion;
    use crate::map_data::binary_reader::BinaryReader;

    #[derive(Debug, Default)]
    pub struct VariableDefinition {
        name: CString,
        var_type: CString,
        unknown: i32,
        is_array: bool,
        array_size: u32,
        initialized: bool,
        init_value: CString,
    }

    impl VariableDefinition {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
            let mut def = Self::default();
            def.name = reader.read_c_string();
            def.var_type = reader.read_c_string();
            def.unknown = reader.read_i32();
            def.is_array = reader.read_u32() == 1;
            def.initialized = reader.read_u32() == 1;
            if game_version.is_tft() {
                def.array_size = reader.read_u32();
            }
            def.init_value = reader.read_c_string();
            def
        }
    }


    #[derive(Debug, Default)]
    pub struct TriggerCategory {
        id: u32,
        name: CString,
        is_comment: bool,
    }

    impl TriggerCategory {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion) -> Self{
            let mut def = Self::default();

            def.id = reader.read_u32();
            def.name = reader.read_c_string();
            if game_version.is_tft() { def.is_comment = reader.read_u32() == 1; }
            def
        }
    }


}



mod trigger {
    use crate::map_data::trigger_file::trigger::FunctionType::{Action, Condition, Event};

    use super::*;

    #[derive(Debug, Default)]
    pub struct TriggerDefinition {
        name: CString,
        description: CString,
        is_comment: bool,
        enabled: bool,
        is_script: bool,
        init_on: bool,
        run_init: bool,
        index_category: u32,
        ecas: Vec<FunctionDefinition>,
    }

    impl TriggerDefinition {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Self {
            let mut def = Self::default();
            def.name = reader.read_c_string();
            def.description = reader.read_c_string();
            if game_version.is_tft() {
                def.is_comment = reader.read_u32() != 0;
            }
            def.enabled = reader.read_u32() == 1;
            def.is_script = reader.read_u32() == 1;
            def.init_on = reader.read_u32() == 1;
            def.run_init = reader.read_i32() == 1;
            def.index_category = reader.read_u32();
            let count_ecas = reader.read_u32();
            def
        }
    }


    #[derive(Debug, Default)]
    struct FunctionDefinition {
        ftype: FunctionType,
        condition_group: Option<ConditionType>,
        name: CString,
        enabled: bool,
    }

    impl FunctionDefinition {
        pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni, is_child_eca: bool) -> Self{
            let mut def = Self::default();
            def.ftype = FunctionType::from(reader.read_u32());
            def.condition_group = if is_child_eca{
                let condition = ConditionType::from(reader.read_u32());
                Some(condition)
            } else {None};
            def.name = reader.read_c_string();
            def.enabled = reader.read_u32() == 1;
            def
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum FunctionType{
        Event,
        Condition,
        Action
    }
    impl Default for FunctionType{
        fn default() -> Self {
            Action
        }
    }
    impl FunctionType{
        pub fn from(n: u32) -> FunctionType {
            match n{
                0 => (Event),
                1 => (Condition),
                2 => (Action),
                _ => panic!("Unknown function type {}",n)
            }
        }
        pub fn get_sector(&self) -> &str {
            match self{
                Event => "TriggerEvents",
                Condition => "TriggerConditions",
                Action => "TriggerActions",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ConditionType{
        Condition,
        Then,
        Else
    }
    impl Default for ConditionType{
        fn default() -> Self {
            ConditionType::Condition
        }
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
        let mut def = Self::default();
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
        match version {
            RoC => {

            },
            _ => {

            }
        }
        def
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