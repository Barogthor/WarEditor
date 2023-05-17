use wce_formats::MapArchive;

use crate::globals::MAP_SHADERS;

#[derive(Debug)]
pub struct ShadowMapFile {
    shaders: Vec<u8>
}

impl ShadowMapFile {
    pub fn read_file(map: &mut MapArchive) -> Self{
        let file = map.open_file(MAP_SHADERS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer).unwrap();
        Self{
            shaders: buffer
        }

    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}
