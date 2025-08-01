use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::GameVersion::{RoC, TFT};
use wce_formats::{BinaryConverter, GameVersion};
use wce_formats::{MapArchive, MpqError, ReadError};

use crate::globals::MAP_TRIGGERS_SCRIPT;
use crate::OpeningError;

type TextScript = String;

#[derive(Debug, Error)]
pub enum TriggerJassError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom triggers content. {0}")]
    Parsing(ReadError),
}
impl From<TriggerJassError> for OpeningError {
    fn from(value: TriggerJassError) -> Self {
        OpeningError::CustomTextTrigger(value)
    }
}

#[derive(Debug)]
pub struct TriggerJassFile {
    version: GameVersion,
    global_comment: String,
    global_script: TextScript,
    triggers_script: Vec<TextScript>,
}

impl TriggerJassFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let buffer = map
            .read_file(MAP_TRIGGERS_SCRIPT)
            .map_err(TriggerJassError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(TriggerJassError::InitReader)?;
        let jass = reader
            .read::<TriggerJassFile>()
            .map_err(TriggerJassError::Parsing)?;
        Ok(jass)
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for TriggerJassFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let version = to_game_version(version).map_err(ReadError::Reason)?;
        let mut global_comment: String = Default::default();
        let mut global_script: String = Default::default();
        let mut text_triggers: Vec<TextScript> = Vec::new();
        match version {
            RoC => (),
            _ => {
                global_comment = reader.read_c_string_converted()?;
                let s = reader.read_u32()? as usize;
                global_script = reader.read_string_utf8(s)?;
            }
        }
        let count_triggers = reader.read_u32()? as usize;
        for _ in 0..count_triggers {
            let length = reader.read_u32()? as usize;
            if length == 0 {
                continue;
            }
            text_triggers.push(reader.read_string_utf8(length)?);
        }
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_TRIGGERS_SCRIPT,
            reader.size() - reader.pos() as usize
        );

        Ok(TriggerJassFile {
            version,
            global_comment,
            global_script,
            triggers_script: text_triggers,
        })
    }

    fn write(&self, _writer: &mut BinaryWriter) -> WriteResult<()> {
        unimplemented!()
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, String> {
    match value {
        0 => Ok(RoC),
        1 => Ok(TFT),
        _ => Err(format!("Unknown or unsupported game version '{value}'")),
    }
}

#[cfg(test)]
mod trigger_jass_test {
    use wce_formats::MapArchive;

    use crate::{get_resources_path, trigger_jass_file::TriggerJassFile};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure_sandbox_roc() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        TriggerJassFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn no_failure_sandbox_tft() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        TriggerJassFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));
    }
}
