
use mpq::Archive;

use crate::globals::MAP_SHADERS;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

#[derive(Debug)]
pub struct ShadowMapFile {
    shaders: Vec<u8>
}

impl ShadowMapFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_SHADERS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        Self{
            shaders: buffer
        }

    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}
