use wce_formats::binary_reader::BinaryReader;
use wce_formats::blp::BLP;
use wce_formats::MapArchive;

use crate::globals::MAP_MINIMAP;
use crate::OpeningError;

pub struct MinimapFile {
    minimap: BLP
}


impl MinimapFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError>{
        let file = map.open_file(MAP_MINIMAP).map_err(|e| OpeningError::Minimap(format!("{}",e)))?;
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer).map_err(|e| OpeningError::Minimap(format!("{}",e)))?;
        let mut reader = BinaryReader::new(buffer);
        let minimap: BLP = BLP::from(&mut reader);
        Ok(Self{
            minimap
        })
    }

}