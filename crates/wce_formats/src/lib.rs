//! Crate root: MPQ archive access (`MapArchive`, `MapArchiveWriter`, `GameMpq`), the
//! `GameVersion` enum, the `BinaryConverter`/`BinaryConverterVersion` traits, and the
//! shared `ReadError`/`WriteError` types used across `binary_reader`, `binary_writer` and `blp`.

use std::ffi::IntoStringError;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::string::FromUtf8Error;

use thiserror::Error;

use mpq_rs::{Archive, Creator, FileOptions};

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
    use crate::ReadError;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn trailing_bytes_error_formats_file_and_counts() {
        let e = ReadError::TrailingBytes {
            file: "war3map.doo".into(),
            expected: 100,
            actual: 96,
        };
        let msg = format!("{e}");
        assert!(msg.contains("war3map.doo"));
        assert!(msg.contains("96"));
        assert!(msg.contains("100"));
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

/// Map an `mpq_rs` error into the crate's `MpqError`. The underlying library's
/// error carries its own context (bad header, missing file, unsupported
/// compression), preserved here as text so callers keep a single error surface.
fn to_mpq_err(err: mpq_rs::MpqError) -> MpqError {
    MpqError::Reason(format!("{err:?}"))
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

/// Size of the fixed Warcraft III map header that precedes the MPQ archive in
/// `.w3m`/`.w3x` files (`specs/map_formats/02_w3m_w3x_format_overview.md`).
pub const MAP_HEADER_SIZE: usize = 512;
const MAP_HEADER_MAGIC: &[u8; 4] = b"HM3W";

pub struct MapArchive {
    archive: Archive<File>,
    /// The original 512-byte `HM3W` header, preserved verbatim so it can be
    /// written back when repackaging. Empty when the file is a bare MPQ.
    header: Vec<u8>,
}

impl MapArchive {
    pub fn open(path: String) -> Result<Self, MpqError> {
        let path = Path::new(&path);
        let ext = path
            .extension()
            .ok_or(format!("No extension for path '{path:?}'"))
            .map_err(MpqError::Reason)?;

        if ext == "w3m" || ext == "w3x" {
            let header = read_map_header(path)?;
            // `mpq_rs` scans for the MPQ header, so it finds the archive behind
            // the fixed `HM3W` map header on its own.
            let file = File::open(path)?;
            let archive = Archive::open(file).map_err(to_mpq_err)?;
            Ok(Self { archive, header })
        } else {
            Err(MpqError::NotMapArchive)
        }
    }

    pub fn read_file(&mut self, path: &str) -> Result<MpqFileBuffer, MpqError> {
        let buffer = self.archive.read_file(path).map_err(to_mpq_err)?;
        Ok(MpqFileBuffer(buffer))
    }

    /// The original 512-byte map header (`HM3W`...), or an empty slice if the
    /// source was a bare MPQ without one.
    pub fn header(&self) -> &[u8] {
        &self.header
    }

    /// Every file name listed in the archive's `(listfile)`, or `None` when the
    /// archive has no readable `(listfile)`. Used to carry imported assets
    /// (models, textures, sounds) that are not modelled as typed components.
    pub fn files(&mut self) -> Option<Vec<String>> {
        Some(
            self.archive
                .files()?
                .into_iter()
                .filter(|name| !name.is_empty())
                .collect(),
        )
    }
}

/// Read the fixed 512-byte map header if the file starts with the `HM3W` magic;
/// otherwise return an empty vector (bare MPQ, no header to preserve).
fn read_map_header(path: &Path) -> Result<Vec<u8>, MpqError> {
    let mut file = File::open(path).map_err(MpqError::IoError)?;
    let mut header = vec![0u8; MAP_HEADER_SIZE];
    match file.read_exact(&mut header) {
        Ok(()) if &header[0..4] == MAP_HEADER_MAGIC => Ok(header),
        _ => Ok(Vec::new()),
    }
}

#[derive(Default)]
pub struct MapArchiveWriter(Creator);

impl MapArchiveWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file<C>(&mut self, path: &str, contents: C)
    where
        C: Into<Vec<u8>>,
    {
        self.0.add_file(
            path,
            contents,
            FileOptions {
                encrypt: false,
                compress: true,
                adjust_key: false,
            },
        );
    }

    /// Write the archive to `path`. When `header` is non-empty it is written
    /// first (the 512-byte `HM3W` map header); `mpq_rs` then places the MPQ at
    /// the next 512-byte boundary with offsets relative to it, so the header
    /// and archive compose correctly.
    pub fn save_archive(&mut self, path: &str, header: &[u8]) -> Result<(), MpqError> {
        let mut file = File::create(path)?;
        if !header.is_empty() {
            file.write_all(header)?;
        }
        self.0.write(&mut file).map_err(MpqError::IoError)?;
        Ok(())
    }
}

pub struct GameMpq(Archive<File>);
impl GameMpq {
    pub fn open(path: String) -> Result<Self, MpqError> {
        let file = File::open(path)?;
        Archive::open(file).map(Self).map_err(to_mpq_err)
    }

    pub fn read_file(&mut self, path: &str) -> Result<MpqFileBuffer, MpqError> {
        let buffer = self.0.read_file(path).map_err(to_mpq_err)?;
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
    #[error("reader for {file} hasn't reached EOF: read {actual} of {expected} bytes")]
    TrailingBytes {
        file: String,
        expected: usize,
        actual: usize,
    },
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
