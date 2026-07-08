use std::convert::TryFrom;
use std::io::{self, Read};

use thiserror::Error;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::{MapArchive, MpqError, ReadError, WriteError};

use crate::globals::MAP_SHADERS;
use crate::MapError;

#[derive(Debug, Error)]
pub enum ShadowMapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("I/O error while reading shadow data. {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse shadow value: {0}")]
    Parsing(ReadError),
    #[error("Failed to save shadowmap data. {0}")]
    SaveError(WriteError),
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

impl ShadowType {
    fn to_byte(&self) -> u8 {
        match self {
            ShadowType::Shadow => 0x00,
            ShadowType::NoShadow => 0xff,
        }
    }
}

#[derive(Debug)]
pub struct ShadowMapFile {
    shaders: Vec<ShadowType>,
}

impl ShadowMapFile {
    pub const FILE_NAME: &str = MAP_SHADERS;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
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

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        for shadow in &self.shaders {
            writer
                .write_u8(shadow.to_byte())
                .map_err(ShadowMapError::SaveError)?;
        }
        Ok(writer)
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

#[cfg(test)]
mod shadowmap_test {
    use wce_formats::MapArchive;

    use crate::{get_resources_path, shadowmap_file::ShadowMapFile};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        ShadowMapFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));
    }
}
