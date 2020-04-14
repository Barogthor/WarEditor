#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use mpq::Archive;

use crate::globals::{MAP_TERRAIN_DOODADS};
use wce_formats::{BinaryConverter, BinaryConverterVersion};
use wce_formats::binary_reader::BinaryReader;
use wce_formats::GameVersion::{self, RoC, TFT};
use wce_formats::binary_writer::BinaryWriter;
use crate::doodad_map::DestructableFlag::{InvisibleNonSolid, Unnamed, VisibleNonSolid, VisibleSolid};
use crate::unit_map::DropItem;

pub type Radian = f32;

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum DestructableFlag {
    InvisibleNonSolid,
    VisibleNonSolid,
    VisibleSolid,
    Unnamed(u8)
}

impl DestructableFlag {
    pub fn from(value: u8) -> Self{
        match value{
            0 => (InvisibleNonSolid),
            1 => (VisibleNonSolid),
            2 => (VisibleSolid),
            _ => Unnamed(value)
//            _ => panic!("Unknown destructable flag {}", value)
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct Destructable {
    model_id: String,
    variation: u32,
    coord_x: f32,
    coord_y: f32,
    coord_z: f32,
    angle: Radian,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    flags: u8,
    life: u8,
    drop_table_pointer: i32,
    drop_item_set: Vec<DropItem>,
    creation_id: u32,
}

impl BinaryConverterVersion for Destructable{
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
        let model_id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let variation = reader.read_u32();
        let coord_x = reader.read_f32();
        let coord_y = reader.read_f32();
        let coord_z = reader.read_f32();
        let angle = reader.read_f32();
        let scale_x = reader.read_f32();
        let scale_y = reader.read_f32();
        let scale_z = reader.read_f32();
        let flags = reader.read_u8();
        let life = reader.read_u8();
        let (drop_table_pointer, drop_item_set) = match *game_version{
            RoC => (-1, vec![]),
            _ => {
                let drop_table_pointer = reader.read_i32();
                let count_drop_set = reader.read_u32();
                let drop_item_set = reader.read_vec_version::<DropItem>(count_drop_set as usize, game_version);
                (drop_table_pointer, drop_item_set)
            },
        };

        let creation_id = reader.read_u32();
        Destructable{
            model_id,
            variation,
            coord_x,
            coord_y,
            coord_z,
            angle,
            scale_x,
            scale_y,
            scale_z,
            flags,
            life,
            drop_table_pointer,
            drop_item_set,
            creation_id
        }
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Doodad {
    model_id: String,
    coord_x: f32,
    coord_y: f32,
    coord_z: f32,
}

impl BinaryConverter for Doodad{
    fn read(reader: &mut BinaryReader) -> Self {
        let model_id = reader.read_string_utf8(4);
        let coord_x = reader.read_f32();
        let coord_y = reader.read_f32();
        let coord_z = reader.read_f32();
        Doodad{
            model_id,
            coord_x,
            coord_y,
            coord_z
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct DoodadMap {
    //    id: u32,
    id: String,
    version: GameVersion,
    subversion: u32,
    destructables: Vec<Destructable>,
    doodad_version: u32,
    doodads: Vec<Doodad>,

}

impl DoodadMap {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_TERRAIN_DOODADS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];
        
        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<DoodadMap>()
    }
}

impl BinaryConverter for DoodadMap {
    fn read(reader: &mut BinaryReader) -> Self {
        let id = reader.read_string_utf8(4);
//        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
//        let id = reader.read_u32();
        let version = reader.read_u32();
        let version = to_game_version(version);
        let subversion = reader.read_u32();
        let count_destructables = reader.read_u32();
        let destructables = reader.read_vec_version::<Destructable>(count_destructables as usize, &version);
        let doodad_version = reader.read_u32();
        let count_doodads = reader.read_u32();
        let doodads = reader.read_vec::<Doodad>(count_doodads as usize);
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_TERRAIN_DOODADS, reader.size() - reader.pos() as usize);
        DoodadMap {
            id,
            version,
            subversion,
            destructables,
            doodad_version,
            doodads
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

fn to_game_version(value: u32) -> GameVersion{
    match value{
        7 => RoC,
        8 => TFT,
        _ => panic!("Unknown or unsupported game version '{}'", value)
    }
}

#[cfg(test)]
mod doodads_test{
    use std::fs::File;
    use crate::doodad_map::{DoodadMap, Destructable};
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::GameVersion::{RoC};

    fn mock_destructable_roc() -> Vec<Destructable>{
        vec![
            Destructable{
                model_id: "LTlt".to_string(),
                variation: 0,
                coord_x: -1280.0,
                coord_y: 1600.0,
                coord_z: 0.0,
                angle: 4.712389,
                scale_x: 0.9766412,
                scale_y: 0.9766412,
                scale_z: 0.9766412,
                flags: 2,
                life: 100,
                drop_table_pointer: -1,
                drop_item_set: vec![],
                creation_id: 0
            },
            Destructable{
                model_id: "LRrk".to_string(),
                variation: 4,
                coord_x: 1088.0,
                coord_y: 1216.0,
                coord_z: 79.5,
                angle: 0.5061455,
                scale_x: 0.9194495,
                scale_y: 0.9194495,
                scale_z: 0.9194495,
                flags: 2,
                life: 255,
                drop_table_pointer: -1,
                drop_item_set: vec![],
                creation_id: 55
            },
            Destructable{
                model_id: "LRrk".to_string(),
                variation: 0,
                coord_x: 960.0,
                coord_y: 1280.0,
                coord_z: 46.5,
                angle: 5.969026,
                scale_x: 1.0382886,
                scale_y: 1.0382886,
                scale_z: 1.0382886,
                flags: 2,
                life: 255,
                drop_table_pointer: -1,
                drop_item_set: vec![],
                creation_id: 168
            }
        ]
    }
    
    #[test]
    fn no_failure(){
        let mut doodad_file = File::open("../resources/Scenario/Sandbox_roc/war3map.doo").unwrap();
        let mut reader = BinaryReader::from(&mut doodad_file);
        let doodad_map = reader.read::<DoodadMap>();
    }

    #[test]
    fn check_roc(){
        let mut doodad_file = File::open("../resources/Scenario/Sandbox_roc/war3map.doo").unwrap();
        let mut reader = BinaryReader::from(&mut doodad_file);
        let doodad_map = reader.read::<DoodadMap>();
        let mock_destructables = mock_destructable_roc();
        assert_eq!(doodad_map.id, "W3do".to_string());
        assert_eq!(doodad_map.version, RoC);
        let destructables: Vec<Destructable> = doodad_map.destructables.iter().filter(
            |destructable| {
                let creat_id = destructable.creation_id;
                match creat_id{
                    168 | 55 | 0 => true,
                    _ => false
                }
            }).cloned().collect();
        assert_eq!(destructables, mock_destructables);
    }
}