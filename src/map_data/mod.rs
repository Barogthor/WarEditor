
pub fn is_RoC(version: i32) -> bool{
    version == 18
}

pub fn is_TFT(version: i32) -> bool{
    version == 25 || version == 28
}

pub mod binary_reader;
pub mod w3i_subs;
pub mod w3i_file;
