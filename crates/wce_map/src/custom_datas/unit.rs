use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{GameVersion, MpqError, ReadError, WriteError};

use crate::custom_datas::ObjectDefinition;
use crate::globals::MAP_CUSTOM_UNITS;
use crate::MapError;

use super::ObjectId;

#[derive(Debug, Error)]
pub enum CustomUnitError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom units datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save custom unit data. {0}")]
    SaveError(WriteError),
}
impl From<CustomUnitError> for MapError {
    fn from(value: CustomUnitError) -> Self {
        MapError::CustomUnit(value)
    }
}

#[derive(Debug)]
pub struct CustomUnitFile {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>,
}

impl CustomUnitFile {
    pub const FILE_NAME: &str = MAP_CUSTOM_UNITS;

    pub fn read_file(
        map: &mut MapArchive,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        let file = map.read_file(MAP_CUSTOM_UNITS);
        match file {
            Ok(buffer) => {
                let mut reader =
                    BinaryReader::try_from(buffer).map_err(CustomUnitError::InitReader)?;
                Self::read_opt(&mut reader, game_version)
            }
            _ => Ok(None),
        }
    }

    fn read_opt(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        if reader.size() > 0 {
            let custom_unit =
                Self::parse(reader, game_version).map_err(CustomUnitError::Parsing)?;
            Ok(Some(custom_unit))
        } else {
            Ok(None)
        }
    }

    fn parse(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let original_unit_modified = reader.read_u32()?;
        let mut original_objects = vec![];
        let mut custom_objects = vec![];
        for _i in 0..original_unit_modified {
            let object = read_object(reader, game_version)?;
            original_objects.push(object);
        }
        let custom_table_count = reader.read_u32()?;
        for _i in 0..custom_table_count {
            let object = read_object(reader, game_version)?;
            custom_objects.push(object);
        }

        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_CUSTOM_UNITS,
            reader.size() - reader.pos() as usize
        );
        Ok(Self {
            version,
            original_objects,
            custom_objects,
        })
    }

    pub fn prepare_write(&self, game_version: &GameVersion) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        self.write(&mut writer, game_version).map_err(CustomUnitError::SaveError)?;
        Ok(writer)
    }

    fn write(&self, writer: &mut BinaryWriter, game_version: &GameVersion) -> WriteResult<()> {
        if !self.original_objects.is_empty() || !self.custom_objects.is_empty() {
            writer.write_u32(self.version)?;
            writer.write_u32(self.original_objects.len() as u32)?;
            for obj in &self.original_objects {
                obj.write_without_optional(writer, game_version)?;
            }
            writer.write_u32(self.custom_objects.len() as u32)?;
            for obj in &self.custom_objects {
                obj.write_without_optional(writer, game_version)?;
            }
        }
        Ok(())
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

fn read_object(
    reader: &mut BinaryReader,
    game_version: &GameVersion,
) -> ReadResult<ObjectDefinition> {
    let original_id = reader.read_bytes(4)?;
    let original_id = [
        original_id[0],
        original_id[1],
        original_id[2],
        original_id[3],
    ];
    let custom_id = reader.read_bytes(4)?;
    if custom_id.iter().all(|c| *c == 0) {
        let id = ObjectId::for_original(original_id);
        ObjectDefinition::read_without_optional(reader, id, game_version)
    } else {
        let custom_id = [custom_id[0], custom_id[1], custom_id[2], custom_id[3]];
        let id = ObjectId::for_custom(original_id, custom_id);
        ObjectDefinition::read_without_optional(reader, id, game_version)
    }
}

#[cfg(test)]
mod custom_unit_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{GameVersion, MapArchive};

    use crate::{custom_datas::unit::CustomUnitFile, get_resources_path};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure_tft() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;
        let cunit =
            CustomUnitFile::read_file(&mut map, &game_version).unwrap_or_else(|e| panic!("{}", e));
        assert!(cunit.is_some());
    }

    #[test]
    fn no_failure_roc() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::RoC;
        let cunit =
            CustomUnitFile::read_file(&mut map, &game_version).unwrap_or_else(|e| panic!("{}", e));
        assert!(cunit.is_some());
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_unit_file = CustomUnitFile {
            version: 2,
            original_objects: vec![],
            custom_objects: vec![],
        };

        let mut writer = BinaryWriter::new();
        let game_version = GameVersion::TFT;
        empty_unit_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(buffer.len(), 0, "Empty units should produce empty buffer");

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomUnitFile::read_opt(&mut reader, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(
            reconstructed.is_none(),
            "Reading empty buffer should return nothing"
        );
    }

    #[test]
    fn test_unit_file_round_trip_tft() {
        let mut map = MapArchive::open(get_path("Scenario/Sandbox_1.w3x"))
            .unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;

        let original =
            CustomUnitFile::read_file(&mut map, &game_version).unwrap_or_else(|e| panic!("{}", e));

        if let Some(original_file) = original {
            let mut writer = BinaryWriter::new();
            original_file
                .write(&mut writer, &game_version)
                .unwrap_or_else(|e| panic!("{}", e));
            let buffer = writer.into_buffer();

            let mut reader = BinaryReader::new(buffer);
            let reconstructed = CustomUnitFile::parse(&mut reader, &game_version)
                .unwrap_or_else(|e| panic!("{}", e));

            assert_eq!(original_file.version, reconstructed.version);
            assert_eq!(
                original_file.original_objects.len(),
                reconstructed.original_objects.len()
            );
            assert_eq!(
                original_file.custom_objects.len(),
                reconstructed.custom_objects.len()
            );
        }
    }

    #[test]
    fn test_unit_file_round_trip_roc() {
        let mut map = MapArchive::open(get_path("Scenario/Sandbox_1.w3m"))
            .unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::RoC;

        let original_file = CustomUnitFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e))
            .unwrap();

        let mut writer = BinaryWriter::new();
        original_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed =
            CustomUnitFile::parse(&mut reader, &game_version).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original_file.version, reconstructed.version);
        assert_eq!(
            original_file.original_objects.len(),
            reconstructed.original_objects.len()
        );
        assert_eq!(
            original_file.custom_objects.len(),
            reconstructed.custom_objects.len()
        );
    }
}
