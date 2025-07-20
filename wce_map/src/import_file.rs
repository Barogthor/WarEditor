use std::ffi::CString;

use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion::{RoC, TFT};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, GameVersion};

use crate::globals::MAP_IMPORT_LIST;
use crate::OpeningError;

type ImportPath = Vec<(ImportPathType, CString)>;

#[derive(Debug)]
pub struct ImportFile {
    version: GameVersion,
    files: ImportPath,
}

impl ImportFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Option<Self>, OpeningError> {
        let file = map.open_file(MAP_IMPORT_LIST);
        match file {
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(map, &mut buffer)
                    .map_err(|e| OpeningError::Import(format!("{e}")))?;
                let mut reader = BinaryReader::new(buffer);
                let v = reader
                    .read::<ImportFile>()
                    .map_err(|e| OpeningError::Import(format!("{e:?}")))?;
                Ok(Some(v))
            }
            _ => Ok(None),
        }
    }
    pub fn debug(&self) {
        println!("{:#?}", self);
    }
}

impl BinaryConverter for ImportFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = to_game_version(reader.read_u32()?);
        let count = reader.read_u32()?;
        let mut files: ImportPath = vec![];
        for _ in 0..count {
            let path_type = reader.read_u8()?;
            let path_type = match version {
                RoC => ImportPathType::RoC,
                _ => ImportPathType::from_u8(path_type)
                    .unwrap_or_else(|| panic!("Path type : '{path_type}'")),
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

fn to_game_version(value: u32) -> GameVersion {
    match value {
        0 => RoC,
        1 => TFT,
        _ => panic!("Unknown or unsupported game version '{}'", value),
    }
}
