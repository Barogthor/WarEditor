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
