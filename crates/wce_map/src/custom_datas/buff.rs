use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::MapArchive;
use wce_formats::{GameVersion, MpqError, ReadError};

use crate::custom_datas::ObjectDefinition;
use crate::globals::MAP_CUSTOM_BUFFS;
use crate::OpeningError;

use super::ObjectId;

#[derive(Debug, Error)]
pub enum CustomBuffError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse custom buffs datas. {0}")]
    Parsing(ReadError),
}
impl From<CustomBuffError> for OpeningError {
    fn from(value: CustomBuffError) -> Self {
        OpeningError::CustomBuff(value)
    }
}

#[derive(Debug)]
pub struct CustomBuffFile {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>,
}

impl CustomBuffFile {
    pub fn read_file(
        map: &mut MapArchive,
        game_version: &GameVersion,
    ) -> Result<Option<CustomBuffFile>, OpeningError> {
        let file = map.read_file(MAP_CUSTOM_BUFFS);
        match file {
            Ok(buffer) => {
                let mut reader =
                    BinaryReader::try_from(buffer).map_err(CustomBuffError::InitReader)?;
                let custom_buff =
                    Self::from(&mut reader, game_version).map_err(CustomBuffError::Parsing)?;
                Ok(Some(custom_buff))
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
            MAP_CUSTOM_BUFFS,
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
        Ok(ObjectDefinition::without_optional(
            reader,
            id,
            game_version,
        )?)
    } else {
        let custom_id = [custom_id[0], custom_id[1], custom_id[2], custom_id[3]];
        let id = ObjectId::for_custom(original_id, custom_id);
        Ok(ObjectDefinition::without_optional(
            reader,
            id,
            game_version,
        )?)
    }
}

#[cfg(test)]
mod custom_buff_test {
    use wce_formats::{GameVersion, MapArchive};

    use crate::{custom_datas::buff::CustomBuffFile, get_resources_path};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("sample_2/Remake1 - Copie.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let game_version = GameVersion::TFT;
        let cbuff =
            CustomBuffFile::read_file(&mut map, &game_version).unwrap_or_else(|e| panic!("{}", e));
        assert!(cbuff.is_some());
    }
}
