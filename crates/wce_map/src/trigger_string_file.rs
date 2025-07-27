use std::{collections::HashMap, convert::TryFrom};

use regex::Regex;
use thiserror::Error;

use wce_formats::{binary_reader::BinaryReader, MapArchive, MpqError, ReadError};

use crate::{globals::MAP_STRINGS, OpeningError};

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+";
type TRIGSTR = String;

#[derive(Debug, Error)]
pub enum MapStringError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse trigger strings. {0}")]
    Parsing(ReadError),
    #[error("Regex compilation error. {0}")]
    Regex(#[from] regex::Error),
    #[error("Failed to capture string ID from trigger string")]
    CaptureId,
    #[error("Failed to capture content from trigger string")]
    CaptureContent,
}
impl From<MapStringError> for OpeningError {
    fn from(value: MapStringError) -> Self {
        OpeningError::MapStrings(value)
    }
}

#[derive(Debug)]
pub struct MapStringFile {
    trigger_strings: HashMap<String, TRIGSTR>,
}

impl MapStringFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let buffer = map
            .read_file(MAP_STRINGS)
            .map_err(MapStringError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(MapStringError::InitReader)?;
        let buffer = reader
            .read_string_utf8(reader.size())
            .map_err(MapStringError::Parsing)?;
        // let buffer = unsafe { String::from_utf8_unchecked(buf) };
        let reg: Regex = Regex::new(EXTRACT_DATA).unwrap();

        let mut trigger_strings = HashMap::new();
        for caps in reg.captures_iter(buffer.as_str()) {
            let id = caps
                .get(1)
                .ok_or(MapStringError::CaptureId)?
                .as_str()
                .to_string();
            let content = String::from(caps.get(2).ok_or(MapStringError::CaptureContent)?.as_str());
            trigger_strings.insert(id, content);
        }
        Ok(MapStringFile { trigger_strings })
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportPathType {
    STANDARD(u8),
    CUSTOM(u8),
}
