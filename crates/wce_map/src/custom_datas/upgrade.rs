//! Custom upgrades table (`war3map.w3q`) — thin kind definition over
//! [`CustomObjectsFile`].

use thiserror::Error;
use wce_formats::{MpqError, ReadError, WriteError};

use crate::custom_datas::{CustomObjectKind, CustomObjectsFile};
use crate::globals::MAP_CUSTOM_UPGRADES;
use crate::MapError;

#[derive(Debug, Error)]
pub enum CustomUpgradeError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom upgrades datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save custom upgrade data. {0}")]
    SaveError(WriteError),
}
impl From<CustomUpgradeError> for MapError {
    fn from(value: CustomUpgradeError) -> Self {
        MapError::CustomUpgrade(value)
    }
}

/// Kind marker for the custom upgrades table.
#[derive(Debug)]
pub enum UpgradeKind {}

impl CustomObjectKind for UpgradeKind {
    const FILE_NAME: &'static str = MAP_CUSTOM_UPGRADES;
    const HAS_LEVEL_DATA: bool = true;
    fn init_error(e: ReadError) -> MapError {
        CustomUpgradeError::InitReader(e).into()
    }
    fn parsing_error(e: ReadError) -> MapError {
        CustomUpgradeError::Parsing(e).into()
    }
    fn save_error(e: WriteError) -> MapError {
        CustomUpgradeError::SaveError(e).into()
    }
}

pub type CustomUpgradeFile = CustomObjectsFile<UpgradeKind>;

#[cfg(test)]
mod custom_upgrade_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{GameVersion, MapArchive};

    use crate::{custom_datas::upgrade::CustomUpgradeFile, get_resources_path};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;
        let cupgrade = CustomUpgradeFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(cupgrade.is_some());
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_upgrade_file = CustomUpgradeFile {
            version: 2,
            original_objects: vec![],
            custom_objects: vec![],
            _kind: std::marker::PhantomData,
        };

        let mut writer = BinaryWriter::new();
        let game_version = GameVersion::TFT;
        empty_upgrade_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(
            buffer.len(),
            0,
            "Empty upgrades should produce empty buffer"
        );

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomUpgradeFile::read_opt(&mut reader, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(
            reconstructed.is_none(),
            "Reading empty buffer should return nothing"
        );
    }

    #[test]
    fn test_upgrade_file_round_trip_tft() {
        let mut map = MapArchive::open(get_path("Scenario/Sandbox_1.w3x"))
            .unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;

        let original_file = CustomUpgradeFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e))
            .unwrap();

        let mut writer = BinaryWriter::new();
        original_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomUpgradeFile::parse(&mut reader, &game_version)
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
