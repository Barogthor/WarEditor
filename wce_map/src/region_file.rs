#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use mpq::Archive;

use crate::globals::MAP_REGIONS;
use crate::binary_reader::{BinaryConverter, BinaryReader};
use crate::binary_writer::BinaryWriter;

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
impl BinaryConverter for Region{
    fn read(reader: &mut BinaryReader) -> Self {
        let mut region = Self::default();
        region.left = reader.read_f32();
        region.bottom = reader.read_f32();
        region.right = reader.read_f32();
        region.top = reader.read_f32();
        region.name = reader.read_c_string().into_string().unwrap();
        region.index = reader.read_u32();
//        let effect_id = reader.read_bytes(4);
//        region.weather_effect = String::from_utf8(effect_id).unwrap();
        region.weather_effect = reader.read_string_utf8(4);
        if region.weather_effect.as_bytes() == [0u8;4] {
            region.weather_enabled = false;
        }
        region.ambient_sound = reader.read_c_string().into_string().unwrap();
        region.color = reader.read_bytes(3);
        reader.skip(1);
        region
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct RegionFile {
    version: u32,
    regions: Vec<Region>,
}

impl RegionFile{
    pub fn read_file(mpq: &mut Archive) -> Option<Self>{
        let file = mpq.open_file(MAP_REGIONS);

        match file{
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(mpq, &mut buffer).unwrap();
                let mut reader = BinaryReader::new(buffer);
                Some(reader.read::<RegionFile>())
            },
            _ => None
        }
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for RegionFile{
    fn read(reader: &mut BinaryReader) -> Self {
        let version = reader.read_u32();
        let count_region = reader.read_u32() as usize;
        let regions = reader.read_vec::<Region>(count_region);
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_REGIONS, reader.size() - reader.pos() as usize);
        RegionFile{
            version,
            regions
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[cfg(test)]
mod w3r_test{
    use std::fs::File;
    use crate::binary_reader::BinaryReader;
    use crate::region_file::{RegionFile, Region};

    fn mock_regions() -> Vec<Region>{
        vec![
            Region{
                left: -832.0,
                right: -480.0,
                bottom: -640.0,
                top: -256.0,
                name: "Red".to_string(),
                index: 0,
                weather_effect: "RAhr".to_string(),
                weather_enabled: false,
                ambient_sound: "gg_snd_RainAmbience".to_string(),
                color: vec![0, 0, 255]
            },
            Region{
                left: 416.0,
                right: 768.0,
                bottom: -32.0,
                top: 352.0,
                name: "LightGreen".to_string(),
                index: 1,
                weather_effect: "\0\0\0\0".to_string(),
                weather_enabled: false,
                ambient_sound: "gg_snd_Avatar".to_string(),
                color: vec![128, 255, 128]
            },
            Region{
                left: 384.0,
                right: 416.0,
                bottom: -1056.0,
                top: -640.0,
                name: "White".to_string(),
                index: 2,
                weather_effect: "\0\0\0\0".to_string(),
                weather_enabled: false,
                ambient_sound: "".to_string(),
                color: vec![255, 255, 255]
            }
        ]
    }

    #[test]
    fn no_failure(){
        let mut w3r = File::open("../resources/Scenario/Sandbox_roc/war3map.w3r").unwrap();
        let mut reader = BinaryReader::from(&mut w3r);
        reader.read::<RegionFile>();
    }

    #[test]
    fn check_values(){
        let mut w3r = File::open("../resources/Scenario/Sandbox_roc/war3map.w3r").unwrap();
        let mut reader = BinaryReader::from(&mut w3r);
        let region_file = reader.read::<RegionFile>();
        let mock_regions = mock_regions();
        assert_eq!(region_file.regions, mock_regions);
    }
}