use std::ffi::CString;

use crate::map_data::binary_reader::{BinaryConverter, BinaryConverterVersion, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;
use crate::map_data::triggers_names_file::category::TriggerCategory;
use std::fs::File;
use crate::map_data::concat_path;
use std::io::Read;
use crate::globals::GameVersion;
use crate::globals::GameVersion::{RoC, TFT};

mod function{
    use super::*;

    #[derive(Debug, Default)]
    pub struct FunctionDefinition {
        ftype: FunctionType,
        name: CString,
        enabled: bool,
    }

    impl BinaryConverterVersion for FunctionDefinition {
        fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
            let mut def = Self::default();
            def.ftype = FunctionType::from(reader.read_u32());
            def.name = reader.read_c_string();
            def.enabled = reader.read_u32() == 1;
            def
        }

        fn write_version(writer: &mut BinaryWriter, game_version: &GameVersion) -> Self {
            unimplemented!()
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
            FunctionType::Action
        }
    }
    impl FunctionType{
        pub fn from(n: u32) -> FunctionType {
            match n{
                0 => (FunctionType::Event),
                1 => (FunctionType::Condition),
                2 => (FunctionType::Action),
                _ => panic!("Unknown function type {}",n)
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
        pub fn from(n: u32) -> Option<ConditionType> {
            match n{
                0 => Some(ConditionType::Condition),
                1 => Some(ConditionType::Then),
                2 => Some(ConditionType::Else),
                _ => None
            }
        }
    }
}
mod variable {
    use super::*;

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

    impl BinaryConverterVersion for VariableDefinition{
        fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
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

        fn write_version(writer: &mut BinaryWriter, game_version: &GameVersion) -> Self {
            unimplemented!()
        }
    }
}
mod trigger {
    use super::*;
    use super::function::FunctionDefinition;

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

    impl BinaryConverterVersion for TriggerDefinition{
        fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
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

        fn write_version(writer: &mut BinaryWriter, game_version: &GameVersion) -> Self {
            unimplemented!()
        }
    }

}
mod category {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TriggerCategory {
        id: u32,
        name: CString,
        is_comment: bool,
    }
    impl BinaryConverterVersion for TriggerCategory{
        fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
            let mut def = Self::default();

            def.id = reader.read_u32();
            def.name = reader.read_c_string();
            if game_version.is_tft() { def.is_comment = reader.read_u32() == 1; }
            def
        }

        fn write_version(writer: &mut BinaryWriter, game_version: &GameVersion) -> Self {
            unimplemented!()
        }
    }

}

#[derive(Debug, Default)]
pub struct TriggersNameFile {
    id: String,
    version: GameVersion,
    categories: Vec<TriggerCategory>,
    unknown: i32,
}

impl TriggersNameFile {
    pub fn read_file() -> Self{
        let mut f = File::open(concat_path("war3map.wtg")).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<TriggersNameFile>()
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for TriggersNameFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let mut def = Self::default();
        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let version = reader.read_u32();
        let version = to_game_version(version);
        let count_categories = reader.read_u32();
        let unknown = reader.read_i32();
        let count_vars = reader.read_u32();
        let count_triggers = reader.read_u32();
        match version {
            Version::TFT => {

            }
            Version::RoC => {

            },
        }
        def
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


fn to_game_version(value: u32) -> GameVersion{
    match value{
        4 => RoC,
        7 => TFT,
        _ => panic!("Unknown or unsupported game version '{}'", value)
    }
}