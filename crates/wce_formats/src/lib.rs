use std::ffi::IntoStringError;
use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use thiserror::Error;

use mpq::Archive;

use crate::binary_reader::{BinaryReader, ReadResult};
use crate::binary_writer::{BinaryWriter, WriteResult};

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
    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()>;
}

pub trait BinaryConverterVersion {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self>
    where
        Self: Sized;
    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()>;
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

#[derive(Debug, Error)]
pub enum MpqError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("File is not a valid map archive")]
    NotMapArchive,
    #[error("Reason {0}")]
    Reason(String),
}

pub struct MpqFileBuffer(Vec<u8>);

impl MpqFileBuffer {
    pub fn inner(self) -> Vec<u8> {
        self.0
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

pub struct MapArchive(Archive);

impl MapArchive {
    pub fn open(path: String) -> Result<Self, MpqError> {
        let path = Path::new(&path);
        let ext = path
            .extension()
            .ok_or(format!("No extension for path '{path:?}'"))
            .map_err(MpqError::Reason)?;

        if ext == "w3m" || ext == "w3x" {
            Archive::open(path).map(Self).map_err(MpqError::IoError)
        } else {
            Err(MpqError::NotMapArchive)
        }
    }

    pub fn read_file(&mut self, path: &str) -> Result<MpqFileBuffer, MpqError> {
        let f = self.0.open_file(path).map_err(MpqError::IoError)?;
        let mut buffer: Vec<u8> = vec![0; f.size() as usize];
        f.read(&mut self.0, &mut buffer)
            .map_err(MpqError::IoError)?;
        Ok(MpqFileBuffer(buffer))
    }
}

pub struct GameMpq(Archive);
impl GameMpq {
    pub fn open(path: String) -> Result<Self, MpqError> {
        let archive = Archive::open(path).map_err(MpqError::IoError);
        archive.map(Self)
    }

    pub fn read_file(&mut self, path: &str) -> Result<MpqFileBuffer, MpqError> {
        let f = self.0.open_file(path).map_err(MpqError::IoError)?;
        let mut buffer: Vec<u8> = vec![0; f.size() as usize];
        f.read(&mut self.0, &mut buffer)
            .map_err(MpqError::IoError)?;
        Ok(MpqFileBuffer(buffer))
    }
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Unexpected end of file while parsing.")]
    EOF,
    #[error("Invalid C string: {0}")]
    InvalidCString(String),
    #[error("C string conversion failure at position {position}: {source}")]
    CStringConversionFailure {
        position: u64,
        #[source]
        source: IntoStringError,
    },
    #[error("Null C string at position {position}, length {length}")]
    NullCString { position: u64, length: usize },
    #[error("UTF-8 error at position {position}, length {length}: {source}")]
    Utf8Error {
        position: u64,
        length: usize,
        #[source]
        source: FromUtf8Error,
    },
    #[error("Read error: {0}")]
    Reason(String),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

impl From<FromUtf8Error> for ReadError {
    fn from(value: FromUtf8Error) -> Self {
        ReadError::Reason(format!("{value:?}"))
    }
}

#[derive(Debug, Error)]
pub enum WriteError {
    #[error("C string conversion failure at position {position}: {source}")]
    CStringConversionFailure {
        position: u64,
        #[source]
        source: IntoStringError,
    },
    #[error("UTF-8 error at position {position}, length {length}: {source}")]
    Utf8Error {
        position: u64,
        length: usize,
        #[source]
        source: FromUtf8Error,
    },
    #[error("Write error: {0}")]
    Reason(String),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}
