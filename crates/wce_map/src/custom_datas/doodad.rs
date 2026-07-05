//! Custom doodads table (`war3map.w3d`) — thin kind definition over
//! [`CustomObjectsFile`].

use thiserror::Error;
use wce_formats::{MpqError, ReadError, WriteError};

use crate::custom_datas::{CustomObjectKind, CustomObjectsFile};
use crate::globals::MAP_CUSTOM_DOODADS;
use crate::MapError;

#[derive(Debug, Error)]
pub enum CustomDoodadError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom doodads datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save custom doodad data. {0}")]
    SaveError(WriteError),
}
impl From<CustomDoodadError> for MapError {
    fn from(value: CustomDoodadError) -> Self {
        MapError::CustomDoodad(value)
    }
}

/// Kind marker for the custom doodads table.
#[derive(Debug)]
pub enum DoodadKind {}

impl CustomObjectKind for DoodadKind {
    const FILE_NAME: &'static str = MAP_CUSTOM_DOODADS;
    const HAS_LEVEL_DATA: bool = true;
    fn init_error(e: ReadError) -> MapError {
        CustomDoodadError::InitReader(e).into()
    }
    fn parsing_error(e: ReadError) -> MapError {
        CustomDoodadError::Parsing(e).into()
    }
    fn save_error(e: WriteError) -> MapError {
        CustomDoodadError::SaveError(e).into()
    }
}

pub type CustomDoodadFile = CustomObjectsFile<DoodadKind>;

#[cfg(test)]
mod custom_doodad_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{GameVersion, MapArchive};

    use crate::{custom_datas::doodad::CustomDoodadFile, get_resources_path};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;
        let cdoodad = CustomDoodadFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(cdoodad.is_some());
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_doodad_file = CustomDoodadFile {
            version: 2,
            original_objects: vec![],
            custom_objects: vec![],
            _kind: std::marker::PhantomData,
        };

        let mut writer = BinaryWriter::new();
        let game_version = GameVersion::TFT;
        empty_doodad_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(buffer.len(), 0, "Empty doodads should produce empty buffer");

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomDoodadFile::read_opt(&mut reader, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(
            reconstructed.is_none(),
            "Reading empty buffer should return nothing"
        );
    }

    #[test]
    fn test_doodad_file_round_trip_tft() {
        let mut map = MapArchive::open(get_path("Scenario/Sandbox_1.w3x"))
            .unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;

        let original_file = CustomDoodadFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e))
            .unwrap();

        let mut writer = BinaryWriter::new();
        original_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed =
            CustomDoodadFile::parse(&mut reader, &game_version).unwrap_or_else(|e| panic!("{}", e));

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
