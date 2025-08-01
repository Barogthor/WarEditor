use std::convert::TryFrom;

#[cfg(test)]
use pretty_assertions::assert_eq;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError};

use crate::globals::MAP_REGIONS;
use crate::OpeningError;

#[derive(Debug, Error)]
pub enum RegionError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse regions datas. {0}")]
    Parsing(ReadError),
}
impl From<RegionError> for OpeningError {
    fn from(value: RegionError) -> Self {
        OpeningError::Region(value)
    }
}

#[derive(Debug, Derivative)]
#[derivative(Default, PartialEq)]
pub struct Region {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    name: String,
    index: u32,
    weather_effect: String,
    weather_enabled: bool,
    ambient_sound: String,
    color: Vec<u8>,
    // skip 1 byte : end structure
}
impl BinaryConverter for Region {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let mut region = Self::default();
        region.left = reader.read_f32()?;
        region.bottom = reader.read_f32()?;
        region.right = reader.read_f32()?;
        region.top = reader.read_f32()?;
        region.name = reader.read_c_string_converted()?;
        region.index = reader.read_u32()?;
        //        let effect_id = reader.read_bytes(4);
        //        region.weather_effect = String::from_utf8(effect_id).unwrap();
        region.weather_effect = reader.read_string_utf8(4)?;
        if region.weather_effect.as_bytes() == [0u8; 4] {
            region.weather_enabled = false;
        }
        region.ambient_sound = reader.read_c_string_converted()?;
        region.color = reader.read_bytes(3)?;
        reader.skip(1);
        Ok(region)
    }

    fn write(&self, _writer: &mut BinaryWriter) -> WriteResult<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct RegionFile {
    version: u32,
    regions: Vec<Region>,
}

impl RegionFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Option<Self>, OpeningError> {
        let file = map.read_file(MAP_REGIONS);

        match file {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(RegionError::InitReader)?;
                let region = reader.read::<RegionFile>().map_err(RegionError::Parsing)?;
                Ok(Some(region))
            }
            _ => Ok(None),
        }
    }
    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for RegionFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let count_region = reader.read_u32()? as usize;
        let regions = reader.read_vec::<Region>(count_region)?;
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_REGIONS,
            reader.size() - reader.pos() as usize
        );
        Ok(RegionFile { version, regions })
    }

    fn write(&self, _writer: &mut BinaryWriter) -> WriteResult<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod w3r_test {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;

    use crate::{
        get_resources_path,
        region_file::{Region, RegionFile},
    };

    fn mock_regions() -> Vec<Region> {
        vec![
            Region {
                left: -832.0,
                right: -480.0,
                bottom: -640.0,
                top: -256.0,
                name: "Red".to_string(),
                index: 0,
                weather_effect: "RAhr".to_string(),
                weather_enabled: false,
                ambient_sound: "gg_snd_RainAmbience".to_string(),
                color: vec![0, 0, 255],
            },
            Region {
                left: 416.0,
                right: 768.0,
                bottom: -32.0,
                top: 352.0,
                name: "LightGreen".to_string(),
                index: 1,
                weather_effect: "\0\0\0\0".to_string(),
                weather_enabled: false,
                ambient_sound: "gg_snd_Avatar".to_string(),
                color: vec![128, 255, 128],
            },
            Region {
                left: 384.0,
                right: 416.0,
                bottom: -1056.0,
                top: -640.0,
                name: "White".to_string(),
                index: 2,
                weather_effect: "\0\0\0\0".to_string(),
                weather_enabled: false,
                ambient_sound: "".to_string(),
                color: vec![255, 255, 255],
            },
        ]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let mut w3r = File::open(get_path("Scenario/Sandbox_roc/war3map.w3r"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3r);
        reader.read::<RegionFile>().unwrap();
    }

    #[test]
    fn check_values() {
        let mut w3r = File::open(get_path("Scenario/Sandbox_roc/war3map.w3r"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3r);
        let region_file = reader.read::<RegionFile>().unwrap();
        let mock_regions = mock_regions();
        assert_eq!(region_file.regions, mock_regions);
    }
}
