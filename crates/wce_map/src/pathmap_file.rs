use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::{BinaryConverter, ReadError};
use wce_formats::{MapArchive, MpqError};

use crate::globals::MAP_PATH_MAP;
use crate::OpeningError;

#[derive(Debug, Error)]
pub enum PathmapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse pathmap datas. {0}")]
    Parsing(ReadError),
}
impl From<PathmapError> for OpeningError {
    fn from(value: PathmapError) -> Self {
        OpeningError::PathingMap(value)
    }
}

type Flag = u8;
#[derive(Debug)]
pub struct PathCell {
    flags: Flag,
}
impl PathCell {
    pub fn walkable(&self) -> bool {
        self.flags & 0x02 == 0
    }
    pub fn flyable(&self) -> bool {
        self.flags & 0x04 == 0
    }
    pub fn buildable(&self) -> bool {
        self.flags & 0x08 == 0
    }
    pub fn blight(&self) -> bool {
        self.flags & 0x20 == 0
    }
    pub fn water(&self) -> bool {
        self.flags & 0x40 == 0
    }
    pub fn normal(&self) -> bool {
        self.flags & 0x80 == 0 || !self.blight()
    }

    pub fn update_flags(&mut self, value: Flag) {
        self.flags = value;
    }
}

#[derive(Debug)]
pub struct PathMapFile {
    id: String,
    version: u32,
    pathmap_width: u32,
    pathmap_height: u32,
    pathing: Vec<PathCell>,
}

impl PathMapFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let buffer = map
            .read_file(MAP_PATH_MAP)
            .map_err(PathmapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(PathmapError::InitReader)?;
        let pathmaps = reader
            .read::<PathMapFile>()
            .map_err(PathmapError::Parsing)?;
        Ok(pathmaps)
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for PathMapFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let id = reader.read_string_utf8_safe(4)?;
        let version = reader.read_u32()?;
        let pathmap_width = reader.read_u32()?;
        let pathmap_height = reader.read_u32()?;
        let mut pathing: Vec<PathCell> = Vec::new();
        for _i in 0..pathmap_width * pathmap_height {
            let flags = reader.read_u8()?;

            //            println!("{:x}",flags);
            pathing.push(PathCell { flags });
        }
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_PATH_MAP,
            reader.size() - reader.pos() as usize
        );
        Ok(PathMapFile {
            id,
            version,
            pathmap_width,
            pathmap_height,
            pathing,
        })
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}
