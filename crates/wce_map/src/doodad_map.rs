use std::convert::TryFrom;
use std::io;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion::{self, RoC, TFT};
use wce_formats::{BinaryConverter, BinaryConverterVersion};
use wce_formats::{MapArchive, MpqError, ReadError};

use crate::doodad_map::DestructableFlag::{InvisibleNonSolid, VisibleNonSolid, VisibleSolid};
use crate::globals::MAP_TERRAIN_DOODADS;
use crate::unit_map::{DropItem, DropItemSet, Drops};
use crate::OpeningError;

pub type Radian = f32;

#[derive(Debug, Error)]
pub enum DoodadError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse doodads datas. {0}")]
    Parsing(ReadError),
}
impl From<DoodadError> for OpeningError {
    fn from(value: DoodadError) -> Self {
        OpeningError::Doodad(value)
    }
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum DestructableFlag {
    InvisibleNonSolid = 0,
    VisibleNonSolid = 1,
    VisibleSolid = 2,
}

impl DestructableFlag {
    pub fn from(value: u8) -> Result<Self, String> {
        match value {
            0 => Ok(InvisibleNonSolid),
            1 => Ok(VisibleNonSolid),
            2 => Ok(VisibleSolid),
            _ => Err(format!("Unknown destructable flag {value}")),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
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
    drops: Drops,
    creation_id: u32,
}

impl BinaryConverterVersion for Destructable {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let model_id = reader.read_string_utf8_safe(4)?;
        let variation = reader.read_u32()?;
        let coord_x = reader.read_f32()?;
        let coord_y = reader.read_f32()?;
        let coord_z = reader.read_f32()?;
        let angle = reader.read_f32()?;
        let scale_x = reader.read_f32()?;
        let scale_y = reader.read_f32()?;
        let scale_z = reader.read_f32()?;
        let flags = reader.read_u8()?;
        // let flags = DestructableFlag::from(flags).map_err(ReadError::Reason)?;
        let life = reader.read_u8()?;
        let drops = Self::load_drops(reader, game_version)?;

        let creation_id = reader.read_u32()?;
        Ok(Destructable {
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
            drops,
            creation_id,
        })
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

impl Destructable {
    fn load_drops(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Drops> {
        let drops = match *game_version {
            RoC => Drops::Empty,
            _ => {
                let drop_table_pointer = reader.read_i32()?;
                let count_drop_set = reader.read_u32()?;
                if drop_table_pointer >= 0 {
                    Drops::PresetTable(drop_table_pointer)
                } else if count_drop_set == 0 {
                    Drops::Empty
                } else {
                    let mut drop_sets = vec![];
                    for _ in 0..count_drop_set {
                        let count_drop_item = reader.read_u32()?;
                        let drop_item_set = reader
                            .read_vec_version::<DropItem>(count_drop_item as usize, game_version)?;
                        drop_sets.push(DropItemSet(drop_item_set));
                    }
                    Drops::EmbeddedTable(drop_sets)
                }
            }
        };
        Ok(drops)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct SpecialDoodad {
    model_id: String,
    coord_x: f32,
    coord_y: f32,
    coord_z: f32,
}

impl BinaryConverter for SpecialDoodad {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let model_id = reader.read_string_utf8(4)?;
        let coord_x = reader.read_f32()?;
        let coord_y = reader.read_f32()?;
        let coord_z = reader.read_f32()?;
        Ok(SpecialDoodad {
            model_id,
            coord_x,
            coord_y,
            coord_z,
        })
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq)]
pub struct DoodadMap {
    id: String,
    version: GameVersion,
    #[derivative(PartialEq = "ignore")]
    subversion: u32,
    destructables: Vec<Destructable>,
    special_doodad_version: u32,
    special_doodads: Vec<SpecialDoodad>,
}

impl DoodadMap {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError> {
        let buffer = map
            .read_file(MAP_TERRAIN_DOODADS)
            .map_err(DoodadError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(DoodadError::InitReader)?;
        let doodads = reader.read::<DoodadMap>().map_err(DoodadError::Parsing)?;
        Ok(doodads)
    }
}

impl BinaryConverter for DoodadMap {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let id = reader.read_string_utf8(4)?;
        let version = reader.read_u32()?;
        let version = to_game_version(version).map_err(ReadError::Reason)?;
        let subversion = reader.read_u32()?;
        let count_destructables = reader.read_u32()?;
        let destructables =
            reader.read_vec_version::<Destructable>(count_destructables as usize, &version)?;
        let special_doodad_version = reader.read_u32()?;
        let count_special_doodads = reader.read_u32()?;
        let special_doodads = reader.read_vec::<SpecialDoodad>(count_special_doodads as usize)?;
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_TERRAIN_DOODADS,
            reader.size() - reader.pos() as usize
        );
        Ok(DoodadMap {
            id,
            version,
            subversion,
            destructables,
            special_doodad_version,
            special_doodads,
        })
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, String> {
    match value {
        7 => Ok(RoC),
        8 => Ok(TFT),
        _ => Err(format!("Unknown or unsupported game version '{value}'")),
    }
}

#[cfg(test)]
mod doodads_test {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::GameVersion::RoC;

    use crate::{
        doodad_map::{Destructable, DoodadMap, Drops},
        get_resources_path,
    };

    fn mock_destructable_roc() -> Vec<Destructable> {
        vec![
            Destructable {
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
                drops: Drops::Empty,
                creation_id: 0,
            },
            Destructable {
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
                drops: Drops::Empty,
                creation_id: 55,
            },
            Destructable {
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
                drops: Drops::Empty,
                creation_id: 168,
            },
        ]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure_roc() {
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_roc/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file);
        let _doodad_map = reader.read::<DoodadMap>();
    }

    #[test]
    fn check_roc() {
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_roc/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file);
        let doodad_map = reader.read::<DoodadMap>().unwrap();
        let mock_destructables = mock_destructable_roc();
        assert_eq!(doodad_map.id, "W3do".to_string());
        assert_eq!(doodad_map.version, RoC);
        let destructables: Vec<Destructable> = doodad_map
            .destructables
            .into_iter()
            .filter(|destructable| {
                let creat_id = destructable.creation_id;
                matches!(creat_id, 168 | 55 | 0)
            })
            .collect();
        assert_eq!(destructables, mock_destructables);
    }

    #[test]
    fn no_failure_tft() {
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_tft/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file);
        let _doodad_map = reader.read::<DoodadMap>();
    }
}
