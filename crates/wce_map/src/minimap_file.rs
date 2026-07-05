use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::blp::{BLPError, BLP};
use wce_formats::{MapArchive, MpqError, ReadError, WriteError};

use crate::globals::MAP_MINIMAP;
use crate::MapError;

#[derive(Debug, Error)]
pub enum MinimapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse minimap image. {0}")]
    Blp(BLPError),
    #[error("Failed to save minimap data. {0}")]
    SaveError(WriteError),
}

impl From<MinimapError> for MapError {
    fn from(value: MinimapError) -> Self {
        MapError::Minimap(value)
    }
}

pub struct MinimapFile {
    minimap: BLP,
}

impl MinimapFile {
    pub const FILE_NAME: &str = MAP_MINIMAP;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map.read_file(MAP_MINIMAP).map_err(MinimapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(MinimapError::InitReader)?;
        let minimap: BLP = BLP::from(&mut reader).map_err(MinimapError::Blp)?;
        Ok(Self { minimap })
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        self.minimap.write(&mut writer).map_err(MinimapError::Blp)?;
        Ok(writer)
    }
}

#[cfg(test)]
mod minimap_test {
    use wce_formats::MapArchive;

    use crate::{get_resources_path, minimap_file::MinimapFile};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        MinimapFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));
    }
}
