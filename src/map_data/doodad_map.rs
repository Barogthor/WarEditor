use std::ffi::CString;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader, BinaryConverterVersion};
use crate::map_data::binary_writer::BinaryWriter;
use mpq::Archive;
use crate::globals::{MAP_TERRAIN_DOODADS, GameVersion};
use crate::map_data::doodad_map::DestructableFlag::{InvisibleNonSolid, VisibleNonSolid, VisibleSolid, Unnamed};
use crate::globals::GameVersion::{RoC, TFT, TFT131};

pub type Radian = f32;

#[derive(PartialOrd, PartialEq, Clone)]
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

struct Item(CString, u32);

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
    flags: DestructableFlag,
    life: u8,
    entity_id: u32
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
        let flags = DestructableFlag::from(flags);
        let life = reader.read_u8();
        //TODO drop set
        match *game_version{
            TFT | TFT131 => {
                let drop_table_pointer = reader.read_i32();
                let count_drop_set = reader.read_u32();
                for i in 0..count_drop_set{
                    reader.skip(8);
                }
            },
            _ => ()
        };

        let in_editor_id = reader.read_u32();
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
            entity_id: in_editor_id
        }
    }

    fn write_version(reader: &mut BinaryWriter, game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

struct Doodad{
    model_id: CString,
    coord_x: f32,
    coord_y: f32,
    coord_z: f32
}

impl BinaryConverter for Doodad{
    fn read(reader: &mut BinaryReader) -> Self {
        let model_id = reader.read_c_string_sized(4);
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

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

pub struct EnvironnementObjectMap {
//    id: u32,
    id: CString,
    version: GameVersion,
    subversion: u32,
    destructables: Vec<Destructable>,
    doodad_version: u32,
    doodads: Vec<Doodad>,

}

impl EnvironnementObjectMap {
    pub fn open_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_TERRAIN_DOODADS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];
        
        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<EnvironnementObjectMap>()
    }
}

impl BinaryConverter for EnvironnementObjectMap {
    fn read(reader: &mut BinaryReader) -> Self {
        let id = reader.read_c_string_sized(4);
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
        EnvironnementObjectMap {
            id,
            version,
            subversion,
            destructables,
            doodad_version,
            doodads
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
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