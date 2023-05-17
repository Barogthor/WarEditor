use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use mpq::Archive;

use crate::binary_reader::BinaryReader;
use crate::binary_writer::BinaryWriter;
use crate::MpqError::IoError;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum GameVersion {
    RoC,
    TFT,
    Reforged,
}

impl GameVersion {
    pub fn is_tft(&self) -> bool{
        match self{
            GameVersion::RoC => false,
            GameVersion::TFT | GameVersion::Reforged => true
        }
    }
    pub fn is_roc(&self) -> bool{
        match self{
            GameVersion::RoC => true,
            GameVersion::TFT | GameVersion::Reforged => false
        }
    }
    pub fn is_remaster(&self) -> bool{
        match self{
            GameVersion::Reforged => true,
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

pub trait GameDataVersionDescriptorT: Debug {}
#[derive(Debug)]
pub struct GameDataRocDescriptor;
impl GameDataVersionDescriptorT for GameDataRocDescriptor {}
#[derive(Debug)]
pub struct GameDataTftDescriptor;
impl GameDataVersionDescriptorT for GameDataTftDescriptor {}
#[derive(Debug)]
pub struct GameDataReforgedDescriptor;
impl GameDataVersionDescriptorT for GameDataReforgedDescriptor {}


#[derive(Debug)]
pub enum MpqError {
    IoError(std::io::Error),
    NotMapArchive,
}
// #[derive(Deref, DerefMut)]
pub struct MapArchive(Archive);

impl MapArchive {
    pub fn open(path: String) -> Result<Self, MpqError> {
        let path = Path::new(&path);
        let ext = path.extension().expect(&format!("No extension for path '{:?}'",path));

        if ext == "w3m" || ext == "w3x" {
            let archive = Archive::open(path);
            archive.map(|a| Self(a)).map_err(IoError)
        } else {
            Err(MpqError::NotMapArchive)
        }
    }
}

impl Deref for MapArchive {
    type Target = Archive;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MapArchive {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct GameMpq(Archive);
impl GameMpq {
    pub fn open(path: String) -> Result<Self, std::io::Error> {
        let archive = Archive::open(path);
        archive.map(|a| Self(a))
    }
}
impl Deref for GameMpq {
    type Target = Archive;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for GameMpq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}