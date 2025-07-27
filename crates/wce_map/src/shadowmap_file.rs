use std::convert::TryFrom;
use std::io;

use wce_formats::{MapArchive, ReadError};

use crate::globals::MAP_SHADERS;
use crate::OpeningError;

#[derive(Debug)]
pub enum ShadowMapError {
    IoError(io::Error),
    Parsing(ReadError),
}
impl From<ShadowMapError> for OpeningError {
    fn from(value: ShadowMapError) -> Self {
        OpeningError::ShadowMap(value)
    }
}

#[derive(Debug)]
pub enum ShadowType {
    Shadow = 0x00,
    NoShadow = 0xff,
}

impl TryFrom<u8> for ShadowType {
    type Error = ReadError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Shadow),
            255 => Ok(Self::NoShadow),
            v => Err(ReadError::Reason(format!("Invalid shadow value {v}"))),
        }
    }
}

#[derive(Debug)]
pub struct ShadowMapFile {
    shaders: Vec<ShadowType>,
}

impl ShadowMapFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let file = map
            .open_file(MAP_SHADERS)
            .map_err(ShadowMapError::IoError)?;
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer)
            .map_err(ShadowMapError::IoError)?;
        let shaders = buffer
            .into_iter()
            .map(|byte| ShadowType::try_from(byte).map_err(ShadowMapError::Parsing))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { shaders })
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}
