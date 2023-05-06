#![allow(dead_code)]
// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate derivative;
use crate::data_ini::DataIni;
use crate::globals::PROFILE_TRIGGER_DATA;

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


pub fn format_data(prefix: &str, path: &str) -> String{
    format!("{}resources/datas/{}", prefix, path)
}
pub fn format_slk(prefix: &str, path: &str) -> String{
    format!("{}resources/slk/{}", prefix, path)
}

pub struct GameData{
    trigger_data: DataIni
}

impl GameData {
    pub fn new(prefix: &str) -> Self{
        let mut trigger_data = DataIni::new();
        trigger_data.merge(&format_data( prefix,PROFILE_TRIGGER_DATA));
        Self{
            trigger_data
        }
    }

    pub fn get_trigger_data(&self) -> &DataIni{ &self.trigger_data }
}

pub mod error;
pub mod globals;
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