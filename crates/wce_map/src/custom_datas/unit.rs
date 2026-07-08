//! Custom units table (`war3map.w3u`) — thin kind definition over
//! [`CustomObjectsFile`].

use thiserror::Error;
use wce_formats::{MpqError, ReadError, WriteError};

use crate::custom_datas::{CustomObjectKind, CustomObjectsFile};
use crate::globals::MAP_CUSTOM_UNITS;
use crate::MapError;

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

/// Kind marker for the custom units table.
#[derive(Debug)]
pub enum UnitKind {}

impl CustomObjectKind for UnitKind {
    const FILE_NAME: &'static str = MAP_CUSTOM_UNITS;
    const HAS_LEVEL_DATA: bool = false;
    fn init_error(e: ReadError) -> MapError {
        CustomUnitError::InitReader(e).into()
    }
    fn parsing_error(e: ReadError) -> MapError {
        CustomUnitError::Parsing(e).into()
    }
    fn save_error(e: WriteError) -> MapError {
        CustomUnitError::SaveError(e).into()
    }
}

pub type CustomUnitFile = CustomObjectsFile<UnitKind>;

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
            _kind: std::marker::PhantomData,
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
