use wce_formats::binary_reader::BinaryReader;
use wce_formats::GameVersion;
use wce_formats::MapArchive;

use crate::custom_datas::ObjectDefinition;
use crate::globals::MAP_CUSTOM_DOODADS;

use super::ObjectId;

#[derive(Debug)]
pub struct CustomDoodadFile {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>
}

impl CustomDoodadFile {
    pub fn read_file(map: &mut MapArchive, game_version: &GameVersion) -> Self{
        let file = map.open_file(MAP_CUSTOM_DOODADS);
        match file {
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(map, &mut buffer).unwrap();
                let mut reader = BinaryReader::new(buffer);
                Self::from(&mut reader, game_version)
            }
            _ => {
                Self {
                    version: 0,
                    original_objects: vec![],
                    custom_objects: vec![],
                }
            }
        }
    }

    fn from(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
        let version = reader.read_u32();
        let original_unit_modified = reader.read_u32();
        let mut original_objects = vec![];
        let mut custom_objects = vec![];
        for _i in 0..original_unit_modified {
            let object = read_object(reader, game_version);
            original_objects.push(object);
        }
        let custom_table_count = reader.read_u32();
        for _i in 0..custom_table_count {
            let object = read_object(reader, game_version);
            custom_objects.push(object);
        }

        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_CUSTOM_DOODADS, reader.size() - reader.pos() as usize);
        Self {
            version,
            original_objects,
            custom_objects
        }
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

fn read_object(reader: &mut BinaryReader, game_version: &GameVersion) -> ObjectDefinition {
    let original_id = reader.read_bytes(4);
    let original_id = [original_id[0],original_id[1], original_id[2], original_id[3]];
    let custom_id = reader.read_bytes(4);
    if custom_id.iter().all(|c| *c == 0) {
        let id = ObjectId::for_original(original_id);
        ObjectDefinition::with_optional(reader, id, game_version)
    } else {
        let custom_id = [custom_id[0],custom_id[1], custom_id[2], custom_id[3]];
        let id = ObjectId::for_custom(original_id, custom_id);
        ObjectDefinition::with_optional(reader, id, game_version)
    }
}
