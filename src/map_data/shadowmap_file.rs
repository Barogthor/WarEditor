use std::ffi::CString;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use std::fs::File;
use std::io::Read;
use crate::map_data::binary_writer::BinaryWriter;

#[derive(Debug)]
pub struct ShadowMapFile {
    shaders: Vec<u8>
}

impl ShadowMapFile {
    pub fn read_file() -> Self{
        let mut f = File::open("resources/war3map.shd").unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<ShadowMapFile>()
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for ShadowMapFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let count_shader = reader.read_u32() as usize;
        let shaders = reader.read_bytes(count_shader);
        ShadowMapFile {
            shaders
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}