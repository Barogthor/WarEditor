use mpq::Archive;

use wce_formats::binary_reader::BinaryReader;
use wce_formats::blp::BLP;

use crate::globals::MAP_MINIMAP;

pub struct MinimapFile {
    minimap: BLP
}


impl MinimapFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_MINIMAP).expect(&format!("Couldn't open minimap in map file '{}'", MAP_MINIMAP));
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).expect(&format!("Couldn't read minimap into buffer"));
        let mut reader = BinaryReader::new(buffer);
        let minimap: BLP = BLP::from(&mut reader);
        Self{
            minimap
        }
    }

}