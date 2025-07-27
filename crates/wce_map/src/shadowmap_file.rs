use std::convert::TryFrom;
use std::io::{self, Read};

use thiserror::Error;
use wce_formats::{MapArchive, MpqError, ReadError};

use crate::globals::MAP_SHADERS;
use crate::OpeningError;

#[derive(Debug, Error)]
pub enum ShadowMapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("I/O error while reading shadow data. {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse shadow value: {0}")]
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
        let buffer = map
            .read_file(MAP_SHADERS)
            .map_err(ShadowMapError::MpqError)?;
        let shaders = buffer
            .inner()
            .bytes()
            .map(|byte| {
                ShadowType::try_from(byte.map_err(ShadowMapError::IoError)?)
                    .map_err(ShadowMapError::Parsing)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { shaders })
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}
