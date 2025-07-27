use std::convert::TryFrom;
use std::io::{self, BufReader};

use wce_formats::binary_reader::BinaryReader;
use wce_formats::blp::{BLPError, BLP};
use wce_formats::{MapArchive, MpqError, ReadError};

use crate::globals::MAP_MINIMAP;
use crate::OpeningError;

#[derive(Debug)]
pub enum MinimapError {
    MpqError(MpqError),
    InitReader(ReadError),
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
