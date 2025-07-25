use std::ffi::IntoStringError;
use std::fmt::Debug;
use std::io::Error;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::string::FromUtf8Error;

use mpq::Archive;

use crate::binary_reader::{BinaryReader, ReadResult};
use crate::binary_writer::BinaryWriter;
use crate::MpqError::IoError;

#[cfg(test)]
fn get_resources_path() -> String {
    // Utilise CARGO_MANIFEST_DIR pour obtenir le répertoire racine du workspace
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("Should have parent directory");
    format!("{}/resources/", workspace_root.to_string_lossy())
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Default)]
pub enum GameVersion {
    RoC,
    #[default]
    TFT,
    Reforged,
}

impl GameVersion {
    pub fn is_tft(&self) -> bool {
        match self {
            GameVersion::RoC => false,
            GameVersion::TFT | GameVersion::Reforged => true,
        }
    }
    pub fn is_roc(&self) -> bool {
        match self {
            GameVersion::RoC => true,
            GameVersion::TFT | GameVersion::Reforged => false,
        }
    }
    pub fn is_remaster(&self) -> bool {
        matches!(self, GameVersion::Reforged)
    }
}

pub mod binary_reader;
pub mod binary_writer;
pub mod blp;

pub trait BinaryConverter {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self>
    where
        Self: Sized;
    fn write(&self, writer: &mut BinaryWriter);
}

pub trait BinaryConverterVersion {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self>
    where
        Self: Sized;
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
    Reason(String),
}
// #[derive(Deref, DerefMut)]
pub struct MapArchive(Archive);

impl MapArchive {
    pub fn open(path: String) -> Result<Self, MpqError> {
        let path = Path::new(&path);
        let ext = path
            .extension()
            .ok_or(format!("No extension for path '{path:?}'"))
            .map_err(MpqError::Reason)?;

        if ext == "w3m" || ext == "w3x" {
            let archive = Archive::open(path);
            archive.map(Self).map_err(IoError)
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
        archive.map(Self)
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

#[derive(Debug)]
pub enum ReadError {
    EOF(u64, usize),
    InvalidCString(String),
    CStringConversionFailure(u64, IntoStringError),
    NullCString(u64, usize),
    Utf8Error(u64, usize, FromUtf8Error),
    Other(Error),
    Reason(String),
}

impl From<FromUtf8Error> for ReadError {
    fn from(value: FromUtf8Error) -> Self {
        ReadError::Reason(format!("{value:?}"))
    }
}
