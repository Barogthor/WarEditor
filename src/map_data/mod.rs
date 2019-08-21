pub fn is_RoC(version: i32) -> bool{
    version == 18
}
pub fn is_TFT(version: i32) -> bool{
    version == 25 || version == 28
}
pub fn is_Remastered(version: i32) -> bool {
    version == 28
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