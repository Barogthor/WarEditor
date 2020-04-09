#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use mpq::Archive;

use crate::globals::{GameVersion, MAP_TERRAIN_UNITS};
use crate::globals::GameVersion::{RoC, TFT};
use crate::binary_reader::{BinaryConverter, BinaryConverterVersion, BinaryReader};
use crate::binary_writer::BinaryWriter;
use crate::doodad_map::Radian;
use crate::unit_map::RandomUnitItemFlag::{Neutral, NotRandom, RandomFromCustomTable, RandomFromTableGroup};

const RANDOM_ITEM_ID: &str = "iDNR";
const RANDOM_UNIT_ID: &str = "uDNR";

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct DropItem(String, u32);
impl BinaryConverterVersion for DropItem{
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let item_id = reader.read_string_utf8(4);
        let drop_rate = reader.read_u32();
        Self(item_id, drop_rate)
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct InventoryItem(i32, String);
impl BinaryConverterVersion for InventoryItem{
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let inventory_slot = reader.read_i32();
        let item_id = reader.read_string_utf8(4);
        Self(inventory_slot, item_id)
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct AbilityModification {
    ability_id: String,
    autocast: bool,
    level: u32,
}
impl BinaryConverterVersion for AbilityModification{
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let ability_id = reader.read_string_utf8(4);
        let autocast = reader.read_u32() == 1;
        let level = reader.read_u32();
        Self{ability_id, autocast, level}
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}
#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct RandomUnit(String, f32);
impl BinaryConverterVersion for RandomUnit {
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let unit_id = reader.read_string_utf8(4);
        let rate = reader.read_f32();
        Self(unit_id, rate)
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
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
    fn is_none(&self) -> bool {
        match self{
            NotRandom => true,
            _ => false
        }
    }
}

impl BinaryConverterVersion for RandomUnitItemFlag {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
        let kind = reader.read_i32();
        match kind{
            0 => {
                let value = reader.read_u32();
                let level = value & 0x00FFFFFF;
                let item_class = (value & 0xFF000000) as u8;
                Neutral(level, item_class)
            },
            1 => {
                let group_id = reader.read_i32();
                let column_position = reader.read_u32();
                RandomFromTableGroup(group_id, column_position)
            },
            2 => {
                let size_custom_group = reader.read_u32();
                let custom_group = reader.read_vec_version::<RandomUnit>(size_custom_group as usize, game_version);
                RandomFromCustomTable(custom_group)
            },
            _ => {
                NotRandom
            }
        }
    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
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
    map_drop_table_pointer: i32,
    drop_item_sets: Vec<Vec<DropItem>>,
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

impl BinaryConverterVersion for UnitItem{
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
        let model_id = reader.read_string_utf8(4);
        let variation = reader.read_u32();
        let coord_x = reader.read_f32();
        let coord_y = reader.read_f32();
        let coord_z = reader.read_f32();
        let angle = reader.read_f32();
        let scale_x = reader.read_f32();
        let scale_y = reader.read_f32();
        let scale_z = reader.read_f32();
        let flags = reader.read_u8();
        let player_owner =  reader.read_u32();
        let unk1 =  reader.read_u8();
        let unk2 =  reader.read_u8();
        let hp =  reader.read_i32();
        let mana =  reader.read_i32();
        let map_drop_table_pointer = if game_version.is_tft() {
            reader.read_i32()
        } else { -1 };
        let count_random_drop_sets = reader.read_u32();
        let mut drop_item_sets = vec![];
        if count_random_drop_sets > 0{
            for _ in 0..count_random_drop_sets {
                let count_item_set = reader.read_u32();
                let vi =reader.read_vec_version::<DropItem>(count_item_set as usize, &game_version);
                drop_item_sets.push(vi);
            }
        }
        let gold_amount = reader.read_i32();
        let acquisition_range = reader.read_f32();
        let level = reader.read_u32();
        let (strength, agility, intelligence) = if game_version.is_tft(){
            let strength = reader.read_i32();
            let agility = reader.read_i32();
            let intelligence = reader.read_i32();
            (strength, agility, intelligence)
        } else { (0,0,0) };
        let count_item_carrying = reader.read_u32();
        let inventory = reader.read_vec_version::<InventoryItem>(count_item_carrying as usize, game_version);
        let count_abilities_modified = reader.read_u32();
        let abilities = reader.read_vec_version::<AbilityModification>(count_abilities_modified as usize, game_version);
        let random_type = reader.read_version::<RandomUnitItemFlag>(game_version);

        let color = reader.read_i32();
        let waygate_region_id = reader.read_i32();
        let creation_id = reader.read_u32();
        Self{
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
            map_drop_table_pointer,
            drop_item_sets,
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
            creation_id
        }

    }

    fn write_version(&self, _writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}


#[derive(Debug, PartialOrd, PartialEq)]
pub struct UnitItemMap {
    //    id: u32,
    id: String,
    version: GameVersion,
    subversion: u32,
    units_items: Vec<UnitItem>,
}

impl UnitItemMap {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_TERRAIN_UNITS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<Self>()
    }
}

impl BinaryConverter for UnitItemMap{
    fn read(reader: &mut BinaryReader) -> Self {
        let id = reader.read_string_utf8(4);
//        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
//        let id = reader.read_u32();
        let version = reader.read_u32();
        let version = to_game_version(version);
        let subversion = reader.read_u32();
        let count_units_items = reader.read_u32();
        let units_items = reader.read_vec_version::<UnitItem>(count_units_items as usize, &version);
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_TERRAIN_UNITS, reader.size() - reader.pos() as usize);
        Self{
            id,
            version,
            subversion,
            units_items
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
mod unitmap_tests{
    use std::fs::File;
    use crate::binary_reader::BinaryReader;
    use crate::unit_map::UnitItemMap;

    #[test]
    fn no_failure_roc(){
        let mut unititem_file = File::open("../resources/Scenario/Sandbox_roc/war3mapUnits.doo").unwrap();
        let mut reader = BinaryReader::from(&mut unititem_file);
        let unititem_map = reader.read::<UnitItemMap>();
    }

    #[test]
    fn no_failure_tft(){
        let mut unititem_file = File::open("../resources/Scenario/Sandbox_tft/war3mapUnits.doo").unwrap();
        let mut reader = BinaryReader::from(&mut unititem_file);
        let unititem_map = reader.read::<UnitItemMap>();
    }
}