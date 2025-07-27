use std::{collections::HashMap, io};

use regex::Regex;

use wce_formats::{MapArchive, ReadError};

use crate::{globals::MAP_STRINGS, OpeningError};

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+";
type TRIGSTR = String;

#[derive(Debug)]
pub enum MapStringError {
    IoError(io::Error),
    Parsing(ReadError),
    Regex(regex::Error),
    CaptureId,
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
        let file = map
            .open_file(MAP_STRINGS)
            .map_err(MapStringError::IoError)?;
        let mut buf: Vec<u8> = vec![0; file.size() as usize];
        file.read(map, &mut buf).map_err(MapStringError::IoError)?;
        let buffer = String::from_utf8_lossy(&buf).to_string();
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
