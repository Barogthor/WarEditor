use mpq::Archive;

use crate::globals::MAP_MINIMAP;
use crate::binary_reader::{BinaryConverter, BinaryReader};
use crate::binary_writer::BinaryWriter;

#[derive(Debug)]
pub struct MinimapFile {
    // minimap: BLP
}


impl MinimapFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_MINIMAP).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        // let minimap: BLP = reader.read();
        // Self{
        //     minimap
        // }
        Self{}
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}