use std::io;

use wce_formats::binary_reader::BinaryReader;
use wce_formats::blp::{BLPError, BLP};
use wce_formats::MapArchive;

use crate::globals::MAP_MINIMAP;
use crate::OpeningError;

#[derive(Debug)]
pub enum MinimapError {
    IoError(io::Error),
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
        let file = map.open_file(MAP_MINIMAP).map_err(MinimapError::IoError)?;
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer).map_err(MinimapError::IoError)?;
        let mut reader = BinaryReader::new(buffer);
        let minimap: BLP = BLP::from(&mut reader).map_err(MinimapError::Blp)?;
        Ok(Self { minimap })
    }
}
