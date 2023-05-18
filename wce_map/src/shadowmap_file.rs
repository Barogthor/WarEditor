use wce_formats::MapArchive;

use crate::globals::MAP_SHADERS;
use crate::OpeningError;

#[derive(Debug)]
pub struct ShadowMapFile {
    shaders: Vec<u8>
}

impl ShadowMapFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError>{
        let file = map.open_file(MAP_SHADERS).map_err(|e| OpeningError::ShadowMap(format!("{}",e)))?;
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer).map_err(|e| OpeningError::ShadowMap(format!("{}",e)))?;
        Ok(Self{
            shaders: buffer
        })

    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}
