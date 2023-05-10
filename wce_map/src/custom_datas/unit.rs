use mpq::Archive;

use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::BinaryConverter;

use crate::custom_datas::ObjectDefinition;
use crate::GameData;
use crate::globals::MAP_CUSTOM_UNITS;

use super::{CustomIdCode, ObjectId, OriginalIdCode};

#[derive(Debug)]
pub struct CustomUnitFile {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>
}

impl CustomUnitFile {
    pub fn read_file(mpq: &mut Archive, data: &GameData) -> Self{
        let file = mpq.open_file(MAP_CUSTOM_UNITS).expect(&format!("Custom unit file should be present"));
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        Self::from(&mut reader, data)
    }

    fn from(reader: &mut BinaryReader, data: &GameData) -> Self {
        let version = reader.read_u32();
        let original_unit_modified = reader.read_u32();
        let mut original_objects = vec![];
        let mut custom_objects = vec![];
        for i in 0..original_unit_modified {
            let object = read_object(reader, data);
            original_objects.push(object);
        }
        let custom_table_count = reader.read_u32();
        for i in 0..custom_table_count {
            let object = read_object(reader, data);
            custom_objects.push(object);
        }

        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_CUSTOM_UNITS, reader.size() - reader.pos() as usize);
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

fn read_object(reader: &mut BinaryReader, data: &GameData) -> ObjectDefinition {
    let original_id = reader.read_bytes(4);
    let original_id = [original_id[0],original_id[1], original_id[2], original_id[3]];
    let custom_id = reader.read_bytes(4);
    if custom_id.iter().all(|c| *c == 0) {
        let id = ObjectId::for_original(original_id);
        ObjectDefinition::without_optional(reader, data, id)
        // ObjectDefinition::without_optional(reader, id)
    } else {
        let custom_id = [custom_id[0],custom_id[1], custom_id[2], custom_id[3]];
        let id = ObjectId::for_custom(original_id, custom_id);
        ObjectDefinition::without_optional(reader, data, id)
    }
}
