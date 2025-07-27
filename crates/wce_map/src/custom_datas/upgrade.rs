use std::convert::TryFrom;
use std::io;

use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::MapArchive;
use wce_formats::{GameVersion, MpqError, ReadError};

use crate::custom_datas::ObjectDefinition;
use crate::globals::MAP_CUSTOM_UPGRADES;
use crate::OpeningError;

use super::ObjectId;

#[derive(Debug)]
pub enum CustomUpgradeError {
    MpqError(MpqError),
    InitReader(ReadError),
    Parsing(ReadError),
}
impl From<CustomUpgradeError> for OpeningError {
    fn from(value: CustomUpgradeError) -> Self {
        OpeningError::CustomUpgrade(value)
    }
}

#[derive(Debug)]
pub struct CustomUpgradeFile {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>,
}

impl CustomUpgradeFile {
    pub fn read_file(
        map: &mut MapArchive,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, OpeningError> {
        let file = map.read_file(MAP_CUSTOM_UPGRADES);
        match file {
            Ok(buffer) => {
                let mut reader =
                    BinaryReader::try_from(buffer).map_err(CustomUpgradeError::InitReader)?;
                let custom_upgrade =
                    Self::from(&mut reader, game_version).map_err(CustomUpgradeError::Parsing)?;
                Ok(Some(custom_upgrade))
            }
            _ => Ok(None),
        }
    }

    fn from(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
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
            MAP_CUSTOM_UPGRADES,
            reader.size() - reader.pos() as usize
        );
        Ok(Self {
            version,
            original_objects,
            custom_objects,
        })
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
        Ok(ObjectDefinition::with_optional(reader, id)?)
    } else {
        let custom_id = [custom_id[0], custom_id[1], custom_id[2], custom_id[3]];
        let id = ObjectId::for_custom(original_id, custom_id);
        Ok(ObjectDefinition::with_optional(reader, id)?)
    }
}
