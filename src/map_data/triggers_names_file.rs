use std::ffi::CString;

use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;
use crate::map_data::triggers_names_file::category::TriggerCategory;
use std::fs::File;
use crate::map_data::concat_path;
use std::io::Read;

mod function{
    use super::*;

    #[derive(Debug, Default)]
    pub struct FunctionDefinition {
        version: Version,
        ftype: FunctionType,
        name: CString,
        enabled: bool,
    }

    impl FunctionDefinition {
        pub fn read_definition(reader: &mut BinaryReader, version: Version) -> Self{
            let mut def = Self::default();
            def.version = version;
            def.ftype = FunctionType::from(reader.read_u32()).unwrap();
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
            FunctionType::Action
        }
    }

    impl FunctionType{
        pub fn from(n: u32) -> Option<FunctionType> {
            match n{
                0 => Some(FunctionType::Event),
                1 => Some(FunctionType::Condition),
                2 => Some(FunctionType::Action),
                _ => None
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
        version: Version,
        name: CString,
        var_type: CString,
        unknown: i32,
        is_array: bool,
        array_size: u32,
        initialized: bool,
        init_value: CString,
    }

    impl VariableDefinition{
        pub fn read_definition(reader: &mut BinaryReader, version: Version) -> Self{
            let mut def = Self::default();
            def.version = version;
            def.name = reader.read_c_string();
            def.var_type = reader.read_c_string();
            def.unknown = reader.read_i32();
            def.is_array = reader.read_u32() == 1;
            if version == Version::TFT {
                def.array_size = reader.read_u32();
            }
            def.init_value = reader.read_c_string();
            def
        }
    }
}
mod trigger {
    use super::*;
    use super::function::FunctionDefinition;

    #[derive(Debug, Default)]
    pub struct TriggerDefinition {
        version: Version,
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

    impl TriggerDefinition{
        pub fn read_definition(reader: &mut BinaryReader, version: Version) -> Self{
            let mut def = Self::default();
            def.version = version;
            def.name = reader.read_c_string();
            def.description = reader.read_c_string();
            if version == Version::TFT {
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

    impl BinaryConverter for TriggerDefinition{
        fn read(reader: &mut BinaryReader) -> Self {
            unimplemented!()
        }

        fn write(&self, writer: &mut BinaryWriter) {
            unimplemented!()
        }
    }

}
mod category {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TriggerCategory {
        version: Version,
        id: u32,
        name: CString,
        is_comment: bool,
    }
    impl TriggerCategory{
        pub fn read_definition(reader: &mut BinaryReader, version: Version) -> Self{
            let mut def = Self::default();
            def.version = version;
            def.id = reader.read_u32();
            def.name = reader.read_c_string();
            if version == Version::TFT{ def.is_comment = reader.read_u32() == 1; }
            def
        }
    }

}

#[derive(Debug, Default)]
pub struct TriggersNameFile {
    id: String,
    version: Version,
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
        let version = Version::from(reader.read_u32()).unwrap();
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Version{
    RoC,
    TFT
}

impl Default for Version{
    fn default() -> Self {
        Version::TFT
    }
}
impl Version{
    pub fn from(n: u32) -> Option<Version> {
        match n{
            4 => Some(Version::RoC),
            7 => Some(Version::TFT),
            _ => None
        }
    }
}
