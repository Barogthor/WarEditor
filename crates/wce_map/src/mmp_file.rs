use std::convert::TryFrom;
use std::io;

use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError};

use crate::globals::MAP_MENU_MINIMAP;
use crate::OpeningError;

type RGBA = Vec<u8>;
#[derive(Debug)]
pub enum MenuMinimapError {
    MpqError(MpqError),
    InitReader(ReadError),
    Parsing(ReadError),
}
impl From<MenuMinimapError> for OpeningError {
    fn from(value: MenuMinimapError) -> Self {
        OpeningError::MenuMinimap(value)
    }
}

#[derive(Debug)]
pub struct MMPDataset {
    icon_type: u32,
    x: i32,
    y: i32,
    color: RGBA,
}
impl BinaryConverter for MMPDataset {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let icon_type = reader.read_u32()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let color = reader.read_bytes(4)?;
        Ok(MMPDataset {
            icon_type,
            x,
            y,
            color,
        })
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct MMPFile {
    unknown: i32,
    datasets: Vec<MMPDataset>,
}

impl BinaryConverter for MMPFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let unknown = reader.read_i32()?;
        let count_dataset = reader.read_i32()? as usize;
        let datasets = reader.read_vec::<MMPDataset>(count_dataset)?;
        Ok(MMPFile { unknown, datasets })
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

impl MMPFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let buffer = map
            .read_file(MAP_MENU_MINIMAP)
            .map_err(MenuMinimapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(MenuMinimapError::InitReader)?;
        let mmp = reader
            .read::<MMPFile>()
            .map_err(MenuMinimapError::Parsing)?;
        Ok(mmp)
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}
