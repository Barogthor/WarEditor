use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::BinaryReader;
use wce_formats::blp::{BLPError, BLP};
use wce_formats::{MapArchive, MpqError, ReadError};

use crate::globals::MAP_MINIMAP;
use crate::OpeningError;

#[derive(Debug, Error)]
pub enum MinimapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse minimap image. {0}")]
    Blp(BLPError),
}

impl From<MinimapError> for OpeningError {
    fn from(value: MinimapError) -> Self {
        OpeningError::Minimap(value)
    }
}

pub struct MinimapFile {
    minimap: BLP,
}

impl MinimapFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let buffer = map.read_file(MAP_MINIMAP).map_err(MinimapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(MinimapError::InitReader)?;
        let minimap: BLP = BLP::from(&mut reader).map_err(MinimapError::Blp)?;
        Ok(Self { minimap })
    }
}
