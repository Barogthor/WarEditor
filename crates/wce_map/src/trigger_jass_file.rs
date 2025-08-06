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
                text_triggers.push(String::new()); // Add empty string instead of skipping
            } else {
                text_triggers.push(reader.read_string_utf8(length)?);
            }
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

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_u32(from_game_version(&self.version))?;

        match self.version {
            RoC => (),
            _ => {
                writer.write_c_string_converted(&self.global_comment)?;
                writer.write_u32(self.global_script.len() as u32)?;
                writer.write_string_utf8(&self.global_script)?;
            }
        }

        writer.write_u32(self.triggers_script.len() as u32)?;
        for script in &self.triggers_script {
            writer.write_u32(script.len() as u32)?;
            if !script.is_empty() {
                writer.write_string_utf8(script)?;
            }
        }
        Ok(())
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, String> {
    match value {
        0 => Ok(RoC),
        1 => Ok(TFT),
        _ => Err(format!("Unknown or unsupported game version '{value}'")),
    }
}

fn from_game_version(game_version: &GameVersion) -> u32 {
    match game_version {
        RoC => 0,
        TFT => 1,
        wce_formats::GameVersion::Reforged => unimplemented!(),
    }
}

#[cfg(test)]
mod trigger_jass_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::BinaryConverter;
    use wce_formats::GameVersion::{RoC, TFT};
    use wce_formats::MapArchive;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

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

    #[test]
    fn trigger_jass_roc_test() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let jass = TriggerJassFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));

        // Basic validation for RoC file
        assert_eq!(jass.version, RoC);
        assert!(jass.global_comment.is_empty()); // RoC doesn't have global comment
        assert!(jass.global_script.is_empty()); // RoC doesn't have global script
    }

    #[test]
    fn trigger_jass_tft_test() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let jass = TriggerJassFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));

        // Basic validation for TFT file
        assert_eq!(jass.version, TFT);
        // TFT can have global comment and script (may be empty)
    }

    #[test]
    fn write_read_roundtrip_roc() {
        // Read original data from map archive
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let original_jass =
            TriggerJassFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_jass
            .write(&mut writer)
            .expect("Failed to write TriggerJassFile");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_jass = reader
            .read::<TriggerJassFile>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare
        assert_eq!(original_jass.version, written_jass.version);
        assert_eq!(original_jass.global_comment, written_jass.global_comment);
        assert_eq!(original_jass.global_script, written_jass.global_script);
        assert_eq!(
            original_jass.triggers_script.len(),
            written_jass.triggers_script.len()
        );

        // Compare each trigger script
        for (original, written) in original_jass
            .triggers_script
            .iter()
            .zip(written_jass.triggers_script.iter())
        {
            assert_eq!(original, written);
        }
    }

    #[test]
    fn write_read_roundtrip_tft() {
        // Read original data from map archive
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let original_jass =
            TriggerJassFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_jass
            .write(&mut writer)
            .expect("Failed to write TriggerJassFile");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_jass = reader
            .read::<TriggerJassFile>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare
        assert_eq!(original_jass.version, written_jass.version);
        assert_eq!(original_jass.global_comment, written_jass.global_comment);
        assert_eq!(original_jass.global_script, written_jass.global_script);
        assert_eq!(
            original_jass.triggers_script.len(),
            written_jass.triggers_script.len()
        );

        // Compare each trigger script
        for (original, written) in original_jass
            .triggers_script
            .iter()
            .zip(written_jass.triggers_script.iter())
        {
            assert_eq!(original, written);
        }
    }

    #[test]
    fn test_empty_trigger_script_handling() {
        // Create a test TriggerJassFile with empty trigger scripts
        let jass = TriggerJassFile {
            version: TFT,
            global_comment: "Test comment".to_string(),
            global_script: "// Test global script".to_string(),
            triggers_script: vec![
                "function test1() {}\n".to_string(),
                "".to_string(), // Empty script
                "function test2() {}\n".to_string(),
            ],
        };

        // Write to buffer
        let mut writer = BinaryWriter::new();
        jass.write(&mut writer)
            .expect("Failed to write TriggerJassFile");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_jass = reader
            .read::<TriggerJassFile>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare
        assert_eq!(jass.version, written_jass.version);
        assert_eq!(jass.global_comment, written_jass.global_comment);
        assert_eq!(jass.global_script, written_jass.global_script);
        assert_eq!(
            jass.triggers_script.len(),
            written_jass.triggers_script.len()
        );
        assert_eq!(jass.triggers_script[0], written_jass.triggers_script[0]);
        assert_eq!(jass.triggers_script[1], ""); // Empty script should remain empty
        assert_eq!(jass.triggers_script[2], written_jass.triggers_script[2]);
    }
}
