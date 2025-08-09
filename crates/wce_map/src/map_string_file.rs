use std::{collections::HashMap, convert::TryFrom};

use regex::Regex;
use thiserror::Error;

use wce_formats::{
    binary_reader::BinaryReader, binary_writer::BinaryWriter, MapArchive, MpqError, ReadError,
    WriteError,
};

use crate::{globals::MAP_STRINGS, MapError};

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s*.*\s+\{\s*([^\}]*)\s*\}";
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
    #[error("Failed to save map strings data. {0}")]
    SaveError(WriteError),
}
impl From<MapStringError> for MapError {
    fn from(value: MapStringError) -> Self {
        MapError::MapStrings(value)
    }
}

#[derive(Debug)]
pub struct MapStringFile {
    trigger_strings: HashMap<String, TRIGSTR>,
}

impl MapStringFile {
    pub const FILE_NAME: &str = MAP_STRINGS;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map
            .read_file(MAP_STRINGS)
            .map_err(MapStringError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(MapStringError::InitReader)?;
        let buffer = reader
            .read_string_utf8(reader.size())
            .map_err(MapStringError::Parsing)?;
        // let buffer = unsafe { String::from_utf8_unchecked(buf) };
        Self::extract(&buffer).map_err(From::from)
    }

    fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        for (id, content) in self.trigger_strings.iter() {
            let mstr = format!("STRING {id} {{\n{content}\n}}\n");
            writer
                .write_string_utf8(&mstr)
                .map_err(MapStringError::SaveError)?;
        }
        Ok(writer)
    }

    fn extract(buffer: &str) -> Result<MapStringFile, MapStringError> {
        let reg: Regex = Regex::new(EXTRACT_DATA).map_err(MapStringError::Regex)?;

        let mut trigger_strings = HashMap::new();
        for caps in reg.captures_iter(buffer) {
            let id = caps
                .get(1)
                .ok_or(MapStringError::CaptureId)?
                .as_str()
                .to_string();
            let content = String::from(
                caps.get(2)
                    .ok_or(MapStringError::CaptureContent)?
                    .as_str()
                    .trim(),
            );
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

#[cfg(test)]
mod map_strings_tests {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;

    use crate::{get_resources_path, map_string_file::MapStringFile};

    const SAMPLE: &str = r###"
STRING 1
{
Sandbox Roc
}

STRING 2
{
Tous
}

STRING 3
{
Sans description
}

STRING 4
{
Inconnu
}

STRING 5
{
Joueur 1
}

STRING 6
{
Joueur 2
}

STRING 7
{
Force 1
}
        "###;

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn extraction_strings() {
        let msf = MapStringFile::extract(SAMPLE).unwrap_or_else(|e| panic!("{}", e));
        let strings = msf.trigger_strings;
        assert_eq!(strings.keys().len(), 7);
        assert_eq!(strings.get("1"), Some(&"Sandbox Roc".to_string()));
        assert_eq!(strings.get("3"), Some(&"Sans description".to_string()));
    }

    #[test]
    fn test_parsing_file_roc() {
        let mut f = File::open(get_path("Scenario/Sandbox_roc/war3map.wts"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut f);
        let buffer = reader
            .read_string_utf8_safe(reader.size())
            .unwrap_or_else(|e| panic!("{}", e));
        let map_str = MapStringFile::extract(&buffer)
            .unwrap_or_else(|e| panic!("{}", e))
            .trigger_strings;

        assert_eq!(map_str.keys().len(), 8);
        assert_eq!(map_str.get("1"), Some(&"Sandbox Roc".to_string()));
        assert_eq!(map_str.get("3"), Some(&"Map pour mocker".to_string()));
        assert_eq!(map_str.get("8"), Some(&"Fantassin Test".to_string()));
    }

    #[test]
    fn test_parsing_file_tft() {
        let mut f = File::open(get_path("Scenario/Sandbox_tft/war3map.wts"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut f);
        let buffer = reader
            .read_string_utf8_safe(reader.size())
            .unwrap_or_else(|e| panic!("{}", e));
        let map_str = MapStringFile::extract(&buffer)
            .unwrap_or_else(|e| panic!("{}", e))
            .trigger_strings;

        assert_eq!(map_str.keys().len(), 15);
        assert_eq!(map_str.get("1"), Some(&"Sandbox Roc".to_string()));
        assert_eq!(map_str.get("3"), Some(&"Sans description".to_string()));
        assert_eq!(map_str.get("15"), Some(&"Elevage Ex".to_string()));
    }
}
