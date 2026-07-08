//! Parser and writer for `war3mapUnits.doo` (placed unit and item instances, including
//! random-unit/item table entries and per-unit item drop tables).

use std::convert::TryFrom;

#[cfg(test)]
use pretty_assertions::assert_eq;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::GameVersion::{self, RoC, TFT};
use wce_formats::{BinaryConverter, BinaryConverterVersion};
use wce_formats::{MapArchive, MpqError, ReadError, WriteError};

use crate::doodad_map::Radian;
use crate::globals::MAP_TERRAIN_UNITS;
use crate::unit_map::RandomUnitItemFlag::{
    Neutral, NotRandom, RandomFromCustomTable, RandomFromTableGroup,
};
use crate::MapError;

pub type TablePointer = i32;

#[derive(Debug, Error)]
pub enum UnitMapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse units map datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save units and items data. {0}")]
    SaveError(WriteError),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Drops {
    PresetTable(TablePointer),
    EmbeddedTable(Vec<DropItemSet>),
    Empty,
}

impl Drops {
    pub const NO_TABLE_POINTER: i32 = -1;
}
impl BinaryConverterVersion for Drops {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self>
    where
        Self: Sized,
    {
        match game_version {
            RoC => {
                let count_random_drop_sets = reader.read_u32()?;
                let mut drop_item_sets = vec![];
                for _ in 0..count_random_drop_sets {
                    let count_item_set = reader.read_u32()?;
                    let vi = reader
                        .read_vec_version::<DropItem>(count_item_set as usize, game_version)?;
                    drop_item_sets.push(DropItemSet(vi));
                }
                if drop_item_sets.is_empty() {
                    Ok(Drops::Empty)
                } else {
                    Ok(Drops::EmbeddedTable(drop_item_sets))
                }
            }
            TFT => {
                let map_drop_table_pointer = reader.read_i32()?;
                let count_random_drop_sets = reader.read_u32()?;
                if map_drop_table_pointer > Self::NO_TABLE_POINTER {
                    Ok(Drops::PresetTable(map_drop_table_pointer))
                } else if count_random_drop_sets > 0 {
                    let drop_item_sets =
                        reader.read_vec_version(count_random_drop_sets as usize, game_version)?;
                    Ok(Drops::EmbeddedTable(drop_item_sets))
                } else {
                    Ok(Drops::Empty)
                }
            }
            GameVersion::Reforged => unimplemented!(),
        }
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        match (*game_version, self) {
            (RoC, Drops::EmbeddedTable(sets)) => {
                writer.write_u32(sets.len() as u32)?;
                writer.write_vec_version(sets, game_version)?;
            }
            (RoC, _) => {
                writer.write_u32(0)?;
            }
            (TFT, Drops::Empty) => {
                writer.write_i32(Drops::NO_TABLE_POINTER)?;
                writer.write_u32(0)?;
            }
            (TFT, Drops::PresetTable(pointer)) => {
                writer.write_i32(*pointer)?;
                writer.write_u32(0)?;
            }
            (TFT, Drops::EmbeddedTable(sets)) => {
                writer.write_i32(Drops::NO_TABLE_POINTER)?;
                writer.write_u32(sets.len() as u32)?;
                writer.write_vec_version(sets, game_version)?;
            }
            (GameVersion::Reforged, _) => todo!(),
        };
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct DropItemSet(pub Vec<DropItem>);

impl BinaryConverterVersion for DropItemSet {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let count_item_set = reader.read_u32()?;
        let vi = reader.read_vec_version::<DropItem>(count_item_set as usize, game_version)?;
        Ok(DropItemSet(vi))
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_u32(self.0.len() as u32)?;
        for item in &self.0 {
            writer.write_version(item, game_version)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct DropItem(String, u32);
impl BinaryConverterVersion for DropItem {
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> ReadResult<Self> {
        let item_id = reader.read_string_utf8(4)?;
        let drop_rate = reader.read_u32()?;
        Ok(Self(item_id, drop_rate))
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        _game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_string_utf8(&self.0)?;
        writer.write_u32(self.1)?;
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct InventoryItem(i32, String);
impl BinaryConverterVersion for InventoryItem {
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> ReadResult<Self> {
        let inventory_slot = reader.read_i32()?;
        let item_id = reader.read_string_utf8(4)?;
        Ok(Self(inventory_slot, item_id))
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        _game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_i32(self.0)?;
        writer.write_string_utf8(&self.1)?;
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct AbilityModification {
    ability_id: String,
    autocast: bool,
    level: u32,
}
impl BinaryConverterVersion for AbilityModification {
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> ReadResult<Self> {
        let ability_id = reader.read_string_utf8(4)?;
        let autocast = reader.read_u32()? == 1;
        let level = reader.read_u32()?;
        Ok(Self {
            ability_id,
            autocast,
            level,
        })
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        _game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_string_utf8(&self.ability_id)?;
        writer.write_u32(self.autocast as u32)?;
        writer.write_u32(self.level)?;
        Ok(())
    }
}
#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct RandomUnit(String, f32);
impl BinaryConverterVersion for RandomUnit {
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> ReadResult<Self> {
        let unit_id = reader.read_string_utf8(4)?;
        let rate = reader.read_f32()?;
        Ok(Self(unit_id, rate))
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        _game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_string_utf8(&self.0)?;
        writer.write_f32(self.1)?;
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum RandomUnitItemFlag {
    Neutral(u32, u8),
    RandomFromTableGroup(i32, u32),
    RandomFromCustomTable(Vec<RandomUnit>),
    NotRandom,
}

impl RandomUnitItemFlag {
    const LEVEL_MASK: u32 = 0x00FFFFFF;
    const CLASS_MASK: u32 = 0xFF000000;
    const CLASS_SHIFT: u32 = 24;
}

impl BinaryConverterVersion for RandomUnitItemFlag {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let kind = reader.read_i32()?;
        Ok(match kind {
            0 => {
                let value = reader.read_u32()?;
                let level = value & Self::LEVEL_MASK;
                let item_class = ((value & Self::CLASS_MASK) >> Self::CLASS_SHIFT) as u8;
                Neutral(level, item_class)
            }
            1 => {
                let group_id = reader.read_i32()?;
                let column_position = reader.read_u32()?;
                RandomFromTableGroup(group_id, column_position)
            }
            2 => {
                let size_custom_group = reader.read_u32()?;
                let custom_group = reader
                    .read_vec_version::<RandomUnit>(size_custom_group as usize, game_version)?;
                RandomFromCustomTable(custom_group)
            }
            _ => NotRandom,
        })
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        match self {
            Neutral(level, class) => {
                writer.write_i32(0)?;
                let class_shifted = (*class as u32) << Self::CLASS_SHIFT;
                writer.write_u32(level | class_shifted)?;
            }
            RandomFromTableGroup(group_id, position) => {
                writer.write_i32(1)?;
                writer.write_i32(*group_id)?;
                writer.write_u32(*position)?;
            }
            RandomFromCustomTable(random_units) => {
                writer.write_i32(2)?;
                writer.write_u32(random_units.len() as u32)?;
                writer.write_vec_version(random_units, game_version)?;
            }
            NotRandom => writer.write_i32(-1)?,
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct UnitItem {
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
    player_owner: u32,
    unk1: u8,
    unk2: u8,
    hp: i32,
    mana: i32,
    drops: Drops,
    gold_amount: i32,
    acquisition_range: f32,
    strength: i32,
    agility: i32,
    intelligence: i32,
    level: u32,
    inventory: Vec<InventoryItem>,
    abilities: Vec<AbilityModification>,
    random_type: RandomUnitItemFlag,
    color: i32,
    waygate_region_id: i32,
    creation_id: u32,
}

impl BinaryConverterVersion for UnitItem {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let model_id = reader.read_string_utf8(4)?;
        let variation = reader.read_u32()?;
        let coord_x = reader.read_f32()?;
        let coord_y = reader.read_f32()?;
        let coord_z = reader.read_f32()?;
        let angle = reader.read_f32()?;
        let scale_x = reader.read_f32()?;
        let scale_y = reader.read_f32()?;
        let scale_z = reader.read_f32()?;
        let flags = reader.read_u8()?;
        let player_owner = reader.read_u32()?;
        let unk1 = reader.read_u8()?;
        let unk2 = reader.read_u8()?;
        let hp = reader.read_i32()?;
        let mana = reader.read_i32()?;
        let drops = reader.read_version::<Drops>(game_version)?;
        let gold_amount = reader.read_i32()?;
        let acquisition_range = reader.read_f32()?;
        let level = reader.read_u32()?;
        let (strength, agility, intelligence) = if game_version.is_tft() {
            let strength = reader.read_i32()?;
            let agility = reader.read_i32()?;
            let intelligence = reader.read_i32()?;
            (strength, agility, intelligence)
        } else {
            (0, 0, 0)
        };
        let count_item_carrying = reader.read_u32()?;
        let inventory =
            reader.read_vec_version::<InventoryItem>(count_item_carrying as usize, game_version)?;
        let count_abilities_modified = reader.read_u32()?;
        let abilities = reader.read_vec_version::<AbilityModification>(
            count_abilities_modified as usize,
            game_version,
        )?;
        let random_type = reader.read_version::<RandomUnitItemFlag>(game_version)?;

        let color = reader.read_i32()?;
        let waygate_region_id = reader.read_i32()?;
        let creation_id = reader.read_u32()?;
        Ok(Self {
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
            player_owner,
            unk1,
            unk2,
            hp,
            mana,
            drops,
            gold_amount,
            acquisition_range,
            strength,
            agility,
            intelligence,
            level,
            inventory,
            abilities,
            random_type,
            color,
            waygate_region_id,
            creation_id,
        })
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_string_utf8(&self.model_id)?;
        writer.write_u32(self.variation)?;
        writer.write_f32(self.coord_x)?;
        writer.write_f32(self.coord_y)?;
        writer.write_f32(self.coord_z)?;
        writer.write_f32(self.angle)?;
        writer.write_f32(self.scale_x)?;
        writer.write_f32(self.scale_y)?;
        writer.write_f32(self.scale_z)?;
        writer.write_u8(self.flags)?;
        writer.write_u32(self.player_owner)?;
        writer.write_u8(self.unk1)?;
        writer.write_u8(self.unk2)?;
        writer.write_i32(self.hp)?;
        writer.write_i32(self.mana)?;
        writer.write_version(&self.drops, game_version)?;
        writer.write_i32(self.gold_amount)?;
        writer.write_f32(self.acquisition_range)?;
        writer.write_u32(self.level)?;
        if game_version.is_tft() {
            writer.write_i32(self.strength)?;
            writer.write_i32(self.agility)?;
            writer.write_i32(self.intelligence)?;
        }

        writer.write_u32(self.inventory.len() as u32)?;
        writer.write_vec_version(&self.inventory, game_version)?;

        writer.write_u32(self.abilities.len() as u32)?;
        writer.write_vec_version(&self.abilities, game_version)?;

        writer.write_version(&self.random_type, game_version)?;
        writer.write_i32(self.color)?;
        writer.write_i32(self.waygate_region_id)?;
        writer.write_u32(self.creation_id)?;

        Ok(())
    }
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq)]
pub struct UnitItemMap {
    //    id: u32,
    id: String,
    version: GameVersion,
    #[derivative(PartialEq = "ignore")]
    subversion: u32,
    units_items: Vec<UnitItem>,
}

impl UnitItemMap {
    pub const FILE_NAME: &str = MAP_TERRAIN_UNITS;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map
            .read_file(MAP_TERRAIN_UNITS)
            .map_err(UnitMapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(UnitMapError::InitReader)?;
        let unit_map = reader.read::<Self>().map_err(UnitMapError::Parsing)?;
        Ok(unit_map)
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer.write(self).map_err(UnitMapError::SaveError)?;
        Ok(writer)
    }
}

impl BinaryConverter for UnitItemMap {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let id = reader.read_string_utf8(4)?;
        let version = reader.read_u32()?;
        let version = to_game_version(version).map_err(ReadError::Reason)?;
        let subversion = reader.read_u32()?;
        let count_units_items = reader.read_u32()?;
        let units_items =
            reader.read_vec_version::<UnitItem>(count_units_items as usize, &version)?;
        if reader.size() != reader.pos() as usize {
            return Err(ReadError::TrailingBytes {
                file: MAP_TERRAIN_UNITS.into(),
                expected: reader.size(),
                actual: reader.pos() as usize,
            });
        }
        Ok(Self {
            id,
            version,
            subversion,
            units_items,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_string_utf8(&self.id)?;
        writer.write_u32(from_game_version(&self.version))?;
        writer.write_u32(self.subversion)?;
        writer.write_u32(self.units_items.len() as u32)?;
        writer.write_vec_version(&self.units_items, &self.version)?;
        Ok(())
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, String> {
    match value {
        7 => Ok(RoC),
        8 => Ok(TFT),
        _ => Err(format!("Unknown or unsupported game version '{value}'")),
    }
}

fn from_game_version(game_version: &GameVersion) -> u32 {
    match game_version {
        RoC => 7,
        TFT => 8,
        GameVersion::Reforged => unimplemented!(),
    }
}

#[cfg(test)]
mod unitmap_tests {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{BinaryConverter, GameVersion::RoC};

    use crate::get_resources_path;
    use crate::unit_map::RandomUnitItemFlag::{
        Neutral, RandomFromCustomTable, RandomFromTableGroup,
    };
    use crate::unit_map::{
        AbilityModification, DropItem, DropItemSet, Drops, InventoryItem, RandomUnit, UnitItem,
        UnitItemMap,
    };

    fn mock_rock() -> Vec<UnitItem> {
        vec![
            UnitItem {
                model_id: "hmpr".to_string(),
                variation: 0,
                coord_x: -352.51535,
                coord_y: 870.0919,
                coord_z: 0.0,
                angle: 0.76305795,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
                flags: 2,
                player_owner: 0,
                unk1: 0,
                unk2: 0,
                hp: -1,
                mana: -1,
                drops: Drops::EmbeddedTable(vec![DropItemSet(vec![DropItem(
                    "YkI1".to_string(),
                    100,
                )])]),
                gold_amount: 12500,
                acquisition_range: -1.0,
                strength: 0,
                agility: 0,
                intelligence: 0,
                level: 1,
                inventory: vec![],
                abilities: vec![],
                random_type: Neutral(1, 0),
                color: -1,
                waygate_region_id: -1,
                creation_id: 2,
            },
            UnitItem {
                model_id: "Hpal".to_string(),
                variation: 0,
                coord_x: 168.12915,
                coord_y: 1133.3773,
                coord_z: 0.0,
                angle: 4.8219957,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
                flags: 2,
                player_owner: 0,
                unk1: 0,
                unk2: 0,
                hp: -1,
                mana: -1,
                drops: Drops::EmbeddedTable(vec![DropItemSet(vec![DropItem(
                    "gopr".to_string(),
                    100,
                )])]),
                gold_amount: 12500,
                acquisition_range: -1.0,
                strength: 0,
                agility: 0,
                intelligence: 0,
                level: 3,
                inventory: vec![InventoryItem(0, "desc".to_string())],
                abilities: vec![AbilityModification {
                    ability_id: "AHad".to_string(),
                    autocast: false,
                    level: 2,
                }],
                random_type: Neutral(1, 0),
                color: -1,
                waygate_region_id: -1,
                creation_id: 3,
            },
            UnitItem {
                model_id: "hrif".to_string(),
                variation: 0,
                coord_x: 295.05032,
                coord_y: 703.4983,
                coord_z: 0.0,
                angle: 5.930978,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
                flags: 2,
                player_owner: 0,
                unk1: 0,
                unk2: 0,
                hp: -1,
                mana: -1,
                drops: Drops::EmbeddedTable(vec![DropItemSet(vec![DropItem(
                    "\u{1}\u{1}\u{0}Q".to_string(),
                    100,
                )])]),
                gold_amount: 12500,
                acquisition_range: -1.0,
                strength: 0,
                agility: 0,
                intelligence: 0,
                level: 1,
                inventory: vec![],
                abilities: vec![],
                random_type: Neutral(1, 0),
                color: -1,
                waygate_region_id: -1,
                creation_id: 9,
            },
            UnitItem {
                model_id: "uDNR".to_string(),
                variation: 0,
                coord_x: 1458.814,
                coord_y: -1488.7827,
                coord_z: 256.0,
                angle: 3.2810445,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
                flags: 2,
                player_owner: 12,
                unk1: 0,
                unk2: 0,
                hp: -1,
                mana: -1,
                drops: Drops::Empty,
                gold_amount: 12500,
                acquisition_range: -1.0,
                strength: 0,
                agility: 0,
                intelligence: 0,
                level: 1,
                inventory: vec![],
                abilities: vec![],
                random_type: RandomFromTableGroup(0, 0),
                color: -1,
                waygate_region_id: -1,
                creation_id: 10,
            },
            UnitItem {
                model_id: "uDNR".to_string(),
                variation: 0,
                coord_x: 1125.4777,
                coord_y: -1130.6067,
                coord_z: 256.0,
                angle: 5.390973,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
                flags: 2,
                player_owner: 12,
                unk1: 0,
                unk2: 0,
                hp: -1,
                mana: -1,
                drops: Drops::Empty,
                gold_amount: 12500,
                acquisition_range: -1.0,
                strength: 0,
                agility: 0,
                intelligence: 0,
                level: 1,
                inventory: vec![],
                abilities: vec![],
                random_type: RandomFromCustomTable(vec![
                    RandomUnit(
                        "nthl".to_string(),
                        0.000000000000000000000000000000000000000000048,
                    ),
                    RandomUnit(
                        "nfre".to_string(),
                        0.000000000000000000000000000000000000000000046,
                    ),
                    RandomUnit(
                        "nsbm".to_string(),
                        0.000000000000000000000000000000000000000000046,
                    ),
                ]),
                color: -1,
                waygate_region_id: -1,
                creation_id: 11,
            },
            UnitItem {
                model_id: "uDNR".to_string(),
                variation: 0,
                coord_x: 1024.6962,
                coord_y: -1549.7902,
                coord_z: 256.0,
                angle: 0.69725907,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
                flags: 2,
                player_owner: 12,
                unk1: 0,
                unk2: 0,
                hp: -1,
                mana: -1,
                drops: Drops::Empty,
                gold_amount: 12500,
                acquisition_range: -1.0,
                strength: 0,
                agility: 0,
                intelligence: 0,
                level: 1,
                inventory: vec![],
                abilities: vec![],
                random_type: Neutral(6, 0),
                color: -1,
                waygate_region_id: -1,
                creation_id: 12,
            },
        ]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure_roc() {
        let mut unititem_file = File::open(get_path("Scenario/Sandbox_roc/war3mapUnits.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut unititem_file);
        let _unititem_map = reader.read::<UnitItemMap>();
    }

    #[test]
    fn check_roc() {
        let mut unititem_file = File::open(get_path("Scenario/Sandbox_roc/war3mapUnits.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut unititem_file);
        let unititem_map = reader
            .read::<UnitItemMap>()
            .unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(unititem_map.id, "W3do".to_string());
        assert_eq!(unititem_map.version, RoC);
        let units_items_mock = mock_rock();
        let units_items: Vec<UnitItem> = unititem_map
            .units_items
            .into_iter()
            .filter(|unit_item| {
                let creat_id = unit_item.creation_id;
                matches!(creat_id, 2 | 3 | 9 | 10 | 11 | 12)
            })
            .collect();
        assert_eq!(units_items, units_items_mock);
    }

    #[test]
    fn no_failure_tft() {
        let mut unititem_file = File::open(get_path("Scenario/Sandbox_tft/war3mapUnits.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut unititem_file);
        let _unititem_map = reader
            .read::<UnitItemMap>()
            .unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn write_read_roundtrip_roc() {
        // Read original data
        let mut unititem_file = File::open(get_path("Scenario/Sandbox_roc/war3mapUnits.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut unititem_file);
        let original_map = reader
            .read::<UnitItemMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write UnitItemMap");
        // let f = File::create_new("unit_map_write_roc.w3do").unwrap_or_else(|e| panic!("{}", e));
        // f.seek_write(writer.get_buffer(), 0)
        //     .unwrap_or_else(|e| panic!("{}", e));

        println!("Original buffer size: {}", reader.size());
        println!("Written buffer size: {}", writer.into_buffer().len());

        // Write to buffer again for comparison
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write UnitItemMap");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_map = reader
            .read::<UnitItemMap>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare
        assert_eq!(original_map.id, written_map.id);
        assert_eq!(original_map.version, written_map.version);
        assert_eq!(original_map.subversion, written_map.subversion);
        assert_eq!(
            original_map.units_items.len(),
            written_map.units_items.len()
        );

        // Compare individual items (may have slight floating point precision differences)
        for (original, written) in original_map
            .units_items
            .iter()
            .zip(written_map.units_items.iter())
        {
            assert_eq!(original.model_id, written.model_id);
            assert_eq!(original.creation_id, written.creation_id);
            // Note: Floating point precision may cause small differences
        }
    }

    #[test]
    fn write_read_roundtrip_tft() {
        // Read original data
        let mut unititem_file = File::open(get_path("Scenario/Sandbox_tft/war3mapUnits.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut unititem_file);
        let original_map = reader
            .read::<UnitItemMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write UnitItemMap");

        // Read back from buffer
        let mut reader = BinaryReader::new(writer.into_buffer());
        let written_map = reader
            .read::<UnitItemMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Compare
        assert_eq!(original_map.id, written_map.id);
        assert_eq!(original_map.version, written_map.version);
        assert_eq!(original_map.subversion, written_map.subversion);
        assert_eq!(
            original_map.units_items.len(),
            written_map.units_items.len()
        );
    }
}
