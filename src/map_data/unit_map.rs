use std::ffi::CString;
use crate::globals::{GameVersion, MAP_TERRAIN_UNITS};
use crate::map_data::doodad_map::Radian;
use mpq::Archive;
use crate::map_data::binary_reader::{BinaryReader, BinaryConverter, BinaryConverterVersion};
use crate::map_data::binary_writer::BinaryWriter;
use crate::globals::GameVersion::{RoC, TFT};
use crate::map_data::unit_map::RandomUnitItemFlag::{Neutral, RandomFromTableGroup, RandomFromCustomTable};

struct DropItem(String,f32);
impl BinaryConverterVersion for DropItem{
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let item_id = reader.read_string_utf8(4);
        let drop_rate = reader.read_f32();
        Self(item_id, drop_rate)
    }

    fn write_version(writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

struct InventoryItem(i32, String);
impl BinaryConverterVersion for InventoryItem{
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let inventory_slot = reader.read_i32();
        let item_id = reader.read_string_utf8(4);
        Self(inventory_slot, item_id)
    }

    fn write_version(writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

struct AbilityModification{
    ability_id: String,
    autocast: bool,
    level: u32
}
impl BinaryConverterVersion for AbilityModification{
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let ability_id = reader.read_string_utf8(4);
        let autocast = reader.read_u32() == 1;
        let level = reader.read_u32();
        Self{ability_id, autocast, level}
    }

    fn write_version(writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}
struct RandomUnit(String, f32);
impl BinaryConverterVersion for RandomUnit {
    fn read_version(reader: &mut BinaryReader, _game_version: &GameVersion) -> Self {
        let unit_id = reader.read_string_utf8(4);
        let rate = reader.read_f32();
        Self(unit_id, rate)
    }

    fn write_version(writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

enum RandomUnitItemFlag {
    Neutral(u32, u8),
    RandomFromTableGroup(i32, u32),
    RandomFromCustomTable(Vec<RandomUnit>),
}
impl BinaryConverterVersion for RandomUnitItemFlag {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
        let kind = reader.read_u32();
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
            _ => panic!("Unknown RandomUnitFlag type {}", kind)
        }
    }

    fn write_version(writer: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}

struct UnitItem{
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
    drop_item_set: Vec<DropItem>,
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
    entity_id: u32,
}

impl BinaryConverterVersion for UnitItem{
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> Self {
        let model_id = reader.read_string_utf8(4);
        let is_random = model_id.eq(&"iDNR".to_string()) || model_id.eq(&"uDNR".to_string());
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
        let map_drop_table_pointer = if *game_version == TFT {
            reader.read_i32()
        } else { -1 };
        let count_random_drop = reader.read_u32();
        let drop_item_set = reader.read_vec_version(count_random_drop as usize, &game_version);
        let gold_amount = reader.read_i32();
        let acquisition_range = reader.read_f32();
        let level = reader.read_u32();
        let (strength, agility, intelligence) = if *game_version == TFT{
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
        let entity_id = reader.read_u32();
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
            drop_item_set,
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
            entity_id
        }

    }

    fn write_version(reader: &mut BinaryWriter, _game_version: &GameVersion) -> Self {
        unimplemented!()
    }
}


pub struct UnitItemMap{
    //    id: u32,
    id: String,
    version: GameVersion,
    subversion: u32,
    units_items: Vec<UnitItem>
}

impl UnitItemMap {
    pub fn open_file(mpq: &mut Archive) -> Self{
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
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF missing {} bytes", MAP_TERRAIN_UNITS, reader.size() - reader.pos() as usize);
        Self{
            id,
            version,
            subversion,
            units_items
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
