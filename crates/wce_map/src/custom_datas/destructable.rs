//! Custom destructables table (`war3map.w3b`) — thin kind definition over
//! [`CustomObjectsFile`].

use thiserror::Error;
use wce_formats::{MpqError, ReadError, WriteError};

use crate::custom_datas::{CustomObjectKind, CustomObjectsFile};
use crate::globals::MAP_CUSTOM_DESTRUCTABLES;
use crate::MapError;

#[derive(Debug, Error)]
pub enum CustomDestructableError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom destructables datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save custom destructable data. {0}")]
    SaveError(WriteError),
}
impl From<CustomDestructableError> for MapError {
    fn from(value: CustomDestructableError) -> Self {
        MapError::CustomDestructable(value)
    }
}

/// Kind marker for the custom destructables table.
#[derive(Debug)]
pub enum DestructableKind {}

impl CustomObjectKind for DestructableKind {
    const FILE_NAME: &'static str = MAP_CUSTOM_DESTRUCTABLES;
    const HAS_LEVEL_DATA: bool = false;
    fn init_error(e: ReadError) -> MapError {
        CustomDestructableError::InitReader(e).into()
    }
    fn parsing_error(e: ReadError) -> MapError {
        CustomDestructableError::Parsing(e).into()
    }
    fn save_error(e: WriteError) -> MapError {
        CustomDestructableError::SaveError(e).into()
    }
}

pub type CustomDestructableFile = CustomObjectsFile<DestructableKind>;

#[cfg(test)]
mod custom_destructable_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{GameVersion, MapArchive};

    use crate::{custom_datas::destructable::CustomDestructableFile, get_resources_path};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;
        let cdestruct = CustomDestructableFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(cdestruct.is_some());
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_destructable_file = CustomDestructableFile {
            version: 2,
            original_objects: vec![],
            custom_objects: vec![],
            _kind: std::marker::PhantomData,
        };

        let mut writer = BinaryWriter::new();
        let game_version = GameVersion::TFT;
        empty_destructable_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(
            buffer.len(),
            0,
            "Empty destructables should produce empty buffer"
        );

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomDestructableFile::read_opt(&mut reader, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(
            reconstructed.is_none(),
            "Reading empty buffer should return nothing"
        );
    }

    #[test]
    fn test_destructable_file_round_trip_tft() {
        let mut map = MapArchive::open(get_path("Scenario/Sandbox_1.w3x"))
            .unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;

        let original_file = CustomDestructableFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e))
            .unwrap();

        let mut writer = BinaryWriter::new();
        original_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomDestructableFile::parse(&mut reader, &game_version)
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
