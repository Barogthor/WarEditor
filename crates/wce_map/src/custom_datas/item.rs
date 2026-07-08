//! Custom items table (`war3map.w3t`) — thin kind definition over
//! [`CustomObjectsFile`].

use thiserror::Error;
use wce_formats::{MpqError, ReadError, WriteError};

use crate::custom_datas::{CustomObjectKind, CustomObjectsFile};
use crate::globals::MAP_CUSTOM_ITEMS;
use crate::MapError;

#[derive(Debug, Error)]
pub enum CustomItemError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom items datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save custom item data. {0}")]
    SaveError(WriteError),
}

/// Kind marker for the custom items table.
#[derive(Debug)]
pub enum ItemKind {}

impl CustomObjectKind for ItemKind {
    const FILE_NAME: &'static str = MAP_CUSTOM_ITEMS;
    const HAS_LEVEL_DATA: bool = false;
    fn init_error(e: ReadError) -> MapError {
        CustomItemError::InitReader(e).into()
    }
    fn parsing_error(e: ReadError) -> MapError {
        CustomItemError::Parsing(e).into()
    }
    fn save_error(e: WriteError) -> MapError {
        CustomItemError::SaveError(e).into()
    }
}

pub type CustomItemFile = CustomObjectsFile<ItemKind>;

#[cfg(test)]
mod custom_item_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{GameVersion, MapArchive};

    use crate::{custom_datas::item::CustomItemFile, get_resources_path};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;
        let citem =
            CustomItemFile::read_file(&mut map, &game_version).unwrap_or_else(|e| panic!("{}", e));
        assert!(citem.is_some());
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_item_file = CustomItemFile {
            version: 2,
            original_objects: vec![],
            custom_objects: vec![],
            _kind: std::marker::PhantomData,
        };

        let mut writer = BinaryWriter::new();
        let game_version = GameVersion::TFT;
        empty_item_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(buffer.len(), 0, "Empty items should produce empty buffer");

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CustomItemFile::read_opt(&mut reader, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        assert!(
            reconstructed.is_none(),
            "Reading empty buffer should return nothing"
        );
    }

    #[test]
    fn test_item_file_round_trip_tft() {
        let mut map = MapArchive::open(get_path("Scenario/Sandbox_1.w3x"))
            .unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;

        let original_file = CustomItemFile::read_file(&mut map, &game_version)
            .unwrap_or_else(|e| panic!("{}", e))
            .unwrap();

        let mut writer = BinaryWriter::new();
        original_file
            .write(&mut writer, &game_version)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed =
            CustomItemFile::parse(&mut reader, &game_version).unwrap_or_else(|e| panic!("{}", e));

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
