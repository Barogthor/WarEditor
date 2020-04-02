#[macro_use] extern crate lazy_static;
use crate::data_ini::DataIni;
use crate::globals::PROFILE_TRIGGER_DATA;

pub fn is_roc(version: i32) -> bool{
    version == 18
}
pub fn is_tft(version: i32) -> bool{
    version == 25 || version == 28
}
pub fn is_remastered(version: i32) -> bool {
    version == 28
}

pub const PREFIX_SAMPLE_PATH: &str = "resources/sample_1";
pub const PREFIX_MDL_PATH: &str = "resources/blp";
pub const PREFIX_BLP_PATH: &str = "resources/mdl";

pub fn concat_path(path: &str) -> String{
    format!("{}/{}",PREFIX_SAMPLE_PATH, path)
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpeningError {
    Protected(String),
    Environment(String),
    CustomTextTrigger(String),
    Import(String),
    Minimap(String),
    MenuMinimap(String),
    PathingMap(String),
    Region(String),
    ShadowMap(String),
    Sound(String),
    TriggerString(String),
    Info(String),
}


pub fn format_data(path: &str) -> String{
    format!("resources/datas/{}",path)
}
pub fn format_slk(path: &str) -> String{
    format!("resources/slk/{}",path)
}

pub struct GameData{
    trigger_data: DataIni
}

impl GameData {
    pub fn new() -> Self{
        let mut trigger_data = DataIni::new();
        trigger_data.merge(&format_data(PROFILE_TRIGGER_DATA));
        Self{
            trigger_data
        }
    }

    pub fn get_trigger_data(&self) -> &DataIni{ &self.trigger_data }
}

pub mod globals;
pub mod binary_reader;
pub mod binary_writer;
pub mod w3i_subs;
pub mod w3i_file;
pub mod mmp_file;
pub mod region_file;
pub mod camera_file;
pub mod sound_file;
pub mod pathmap_file;
pub mod shadowmap_file;
pub mod terrain_file;
pub mod minimap_file;
pub mod import_file;
pub mod trigger_string_file;
pub mod trigger_jass_file;
pub mod triggers;
pub mod map;
pub mod slk_datas;
pub mod data_ini;
pub mod doodad_map;
pub mod unit_map;