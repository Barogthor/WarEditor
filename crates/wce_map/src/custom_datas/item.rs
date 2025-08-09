use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{GameVersion, MpqError, ReadError, WriteError};

use crate::custom_datas::ObjectDefinition;
use crate::globals::MAP_CUSTOM_ITEMS;
use crate::MapError;

use super::ObjectId;

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
impl From<CustomItemError> for MapError {
    fn from(value: CustomItemError) -> Self {
        MapError::CustomItem(value)
    }
}

#[derive(Debug)]
pub struct CustomItemFile {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>,
}

impl CustomItemFile {
    pub const FILE_NAME: &str = MAP_CUSTOM_ITEMS;
    
    pub fn read_file(
        map: &mut MapArchive,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        let file = map.read_file(MAP_CUSTOM_ITEMS);
        match file {
            Ok(buffer) => {
                let mut reader =
                    BinaryReader::try_from(buffer).map_err(CustomItemError::InitReader)?;
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
            let custom_item =
                Self::parse(reader, game_version).map_err(CustomItemError::Parsing)?;
            Ok(Some(custom_item))
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
            MAP_CUSTOM_ITEMS,
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
        self.write(&mut writer, game_version).map_err(CustomItemError::SaveError)?;
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
        Ok(ObjectDefinition::read_without_optional(
            reader,
            id,
            game_version,
        )?)
    } else {
        let custom_id = [custom_id[0], custom_id[1], custom_id[2], custom_id[3]];
        let id = ObjectId::for_custom(original_id, custom_id);
        Ok(ObjectDefinition::read_without_optional(
            reader,
            id,
            game_version,
        )?)
    }
}

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
