use std::ffi::CString;

use mpq::Archive;

use crate::globals::MAP_REGIONS;
use crate::binary_reader::{BinaryConverter, BinaryReader};
use crate::binary_writer::BinaryWriter;

#[derive(Debug)]
pub struct Region {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    name: CString,
    index: u32,
    weather_effect: String,
    weather_enabled: bool,
    ambient_sound: CString,
    color: Vec<u8>,
    // skip 1 byte : end structure
}
impl Default for Region{
    fn default() -> Self {
        Region{
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
            top: 0.0,
            name: Default::default(),
            index: 0,
            weather_effect: "".to_string(),
            weather_enabled: false,
            ambient_sound: Default::default(),
            color: vec![]
        }
    }
}
impl BinaryConverter for Region{
    fn read(reader: &mut BinaryReader) -> Self {
        let mut region = Self::default();
        region.left = reader.read_f32();
        region.right = reader.read_f32();
        region.bottom = reader.read_f32();
        region.top = reader.read_f32();
        region.name = reader.read_c_string();
        region.index = reader.read_u32();
//        let effect_id = reader.read_bytes(4);
//        region.weather_effect = String::from_utf8(effect_id).unwrap();
        region.weather_effect = reader.read_string_utf8(4);
        if region.weather_effect.as_bytes() == [0u8;4] {
            region.weather_enabled = false;
        }
        region.ambient_sound = reader.read_c_string();
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