use std::convert::TryFrom;
use std::ffi::CString;
use std::io;

use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion::{RoC, TFT};
use wce_formats::{BinaryConverter, GameVersion};
use wce_formats::{MapArchive, MpqError, ReadError};

use crate::globals::MAP_IMPORT_LIST;
use crate::OpeningError;

type ImportPath = Vec<(ImportPathType, CString)>;

#[derive(Debug)]
pub enum ImportError {
    MpqError(MpqError),
    InitReader(ReadError),
    Parsing(ReadError),
}

impl From<ImportError> for OpeningError {
    fn from(value: ImportError) -> Self {
        OpeningError::Import(value)
    }
}

#[derive(Debug)]
pub struct ImportFile {
    version: GameVersion,
    files: ImportPath,
}

impl ImportFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Option<Self>, OpeningError> {
        let file = map.read_file(MAP_IMPORT_LIST);
        match file {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(ImportError::InitReader)?;
                let v = reader.read::<ImportFile>().map_err(ImportError::Parsing)?;
                Ok(Some(v))
            }
            _ => Ok(None),
        }
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for ImportFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = to_game_version(reader.read_u32()?).map_err(ReadError::Reason)?;
        let count = reader.read_u32()?;
        let mut files: ImportPath = vec![];
        for _ in 0..count {
            let path_type = reader.read_u8()?;
            let path_type = match version {
                RoC => ImportPathType::RoC,
                _ => ImportPathType::from_u8(path_type).ok_or_else(|| {
                    ReadError::Reason(format!(
                        "Invalid import type '{path_type}' at {}/{}.",
                        reader.pos(),
                        reader.size()
                    ))
                })?,
            };
            let path = reader.read_c_string()?;
            files.push((path_type, path));
        }

        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_IMPORT_LIST,
            reader.size() - reader.pos() as usize
        );
        Ok(ImportFile { version, files })
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportPathType {
    STANDARD(u8),
    CUSTOM(u8),
    RoC,
}

impl ImportPathType {
    pub fn from_u8(n: u8) -> Option<ImportPathType> {
        match n {
            5 | 8 => Some(ImportPathType::STANDARD(n)),
            10 | 13 => Some(ImportPathType::CUSTOM(n)),
            _ => None,
        }
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, String> {
    match value {
        0 => Ok(RoC),
        1 => Ok(TFT),
        _ => Err(format!("Unknown or unsupported game version '{}'", value)),
    }
}
