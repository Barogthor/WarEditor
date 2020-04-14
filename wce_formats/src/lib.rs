use crate::binary_reader::BinaryReader;
use crate::binary_writer::BinaryWriter;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum GameVersion {
    RoC,
    TFT,
    TFT131,
}

impl GameVersion {
    pub fn is_tft(&self) -> bool{
        match self{
            GameVersion::RoC => false,
            GameVersion::TFT | GameVersion::TFT131 => true
        }
    }
    pub fn is_roc(&self) -> bool{
        match self{
            GameVersion::RoC => true,
            GameVersion::TFT | GameVersion::TFT131 => false
        }
    }
    pub fn is_remaster(&self) -> bool{
        match self{
            GameVersion::TFT131 => true,
            _ => false
        }
    }

}


impl Default for GameVersion{
    fn default() -> Self {
        GameVersion::TFT
    }
}

pub mod binary_reader;
pub mod binary_writer;
pub mod blp;

pub trait BinaryConverter{
    fn read(reader: &mut BinaryReader) -> Self;
    fn write(&self, writer: &mut BinaryWriter);
}

pub trait BinaryConverterVersion{
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self;
    fn write_version(&self, writer: &mut BinaryWriter, game_version: &GameVersion) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
