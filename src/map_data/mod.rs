pub fn is_RoC(version: i32) -> bool{
    version == 18
}
pub fn is_TFT(version: i32) -> bool{
    version == 25 || version == 28
}
pub fn is_Remastered(version: i32) -> bool {
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
pub mod environment_file;
pub mod minimap_file;
pub mod import_file;
pub mod trigger_string_file;
pub mod custom_text_trigger_file;