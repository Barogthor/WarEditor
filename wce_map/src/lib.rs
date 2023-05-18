#![allow(dead_code)]
#[macro_use] extern crate derivative;
// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;
#[macro_use] extern crate lazy_static;

use crate::data_ini::DataIni;
use crate::globals::*;
use crate::slk_datas::SLKData;

pub const PREFIX_SAMPLE_PATH: &str = "resources/sample_1";
pub const PREFIX_MDL_PATH: &str = "resources/blp";
pub const PREFIX_BLP_PATH: &str = "resources/mdl";

pub fn concat_path(path: &str) -> String{
    format!("{}/{}",PREFIX_SAMPLE_PATH, path)
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpeningError {
    Protected(String),
    Environment(String),
    CustomTextTrigger(String),
    Triggers(String),
    Import(String),
    Minimap(String),
    MenuMinimap(String),
    PathingMap(String),
    Region(String),
    ShadowMap(String),
    Doodad(String),
    Camera(String),
    UnitItem(String),
    Sound(String),
    MapStrings(String),
    Info(String),
    CustomUnit(String),
    CustomItem(String),
    CustomAbility(String),
    CustomBuff(String),
    CustomUpgrade(String),
    CustomDoodad(String),
    CustomDestructable(String),
}


pub fn format_data(prefix: &str, path: &str) -> String{
    format!("{}resources/datas/{}", prefix, path)
}
pub fn format_slk(prefix: &str, path: &str) -> String{
    format!("{}resources/slk/{}", prefix, path)
}

pub struct GameData{
    trigger_data: DataIni,
    unit_data: SLKData,
    unit_meta: SLKData,
    doodad_meta: SLKData,
    destructable_meta: SLKData,
    abilty_meta: SLKData,
    upgrade_meta: SLKData,
    upgrade_effect_meta: SLKData,
    const_meta: SLKData,
    ui_const_meta: SLKData,
    ability_buff_meta: SLKData,
    ability_data: SLKData,
    upgrade_data: SLKData,
    doodad_effect_data: SLKData,
    destructable_effect_data: SLKData,
}

impl GameData {
    pub fn new(prefix: &str) -> Self{
        let mut trigger_data = DataIni::new();
        trigger_data.merge(&format_data( prefix,PROFILE_TRIGGER_DATA));
        let unit_meta = SLKData::load(&format_slk(prefix, SLK_UNIT_META_DATA));

        let doodad_meta = SLKData::load(&format_slk(prefix, SLK_DOODAD_META_DATA));
        let destructable_meta = SLKData::load(&format_slk(prefix, SLK_DESTRUCTABLE_META_DATA));
        let abilty_meta = SLKData::load(&format_slk(prefix, SLK_ABILITY_META_DATA));
        let upgrade_meta = SLKData::load(&format_slk(prefix, SLK_UPGRADE_META_DATA));
        let upgrade_effect_meta = SLKData::load(&format_slk(prefix, SLK_UPGRADE_EFFECT_META_DATA));
        let const_meta = SLKData::load(&format_slk(prefix, SLK_MISC_META_DATA));
        let ui_const_meta = SLKData::load(&format_slk(prefix, SLK_SKIN_META_DATA));
        let ability_buff_meta = SLKData::load(&format_slk(prefix, SLK_ABILITY_BUFF_META_DATA));
        let mut unit_data = SLKData::new();
        unit_data.merge(&format_slk(prefix, SLK_UNIT_DATA));
        unit_data.merge(&format_slk(prefix, SLK_UNIT_BALANCE));
        unit_data.merge(&format_slk(prefix, SLK_UNIT_UI));
        unit_data.merge(&format_slk(prefix, SLK_UNIT_ABILITIES));
        unit_data.merge(&format_slk(prefix, SLK_UNIT_WEAPONS));
        let ability_data = SLKData::load(&format_slk(prefix, SLK_ABILITY_DATA));
        let upgrade_data = SLKData::load(&format_slk(prefix, SLK_UPGRADE_DATA));
        let doodad_effect_data = SLKData::load(&format_slk(prefix, SLK_DOODADS));
        let destructable_effect_data = SLKData::load(&format_slk(prefix, SLK_DESTRUCTABLE_DATA));
        Self{
            trigger_data,
            unit_data,
            unit_meta,
            doodad_meta,
            destructable_meta,
            abilty_meta,
            upgrade_meta,
            upgrade_effect_meta,
            const_meta,
            ui_const_meta,
            ability_buff_meta,
            ability_data,
            upgrade_data,
            doodad_effect_data,
            destructable_effect_data,
        }
    }

    pub fn get_trigger_data(&self) -> &DataIni{ &self.trigger_data }
}

pub mod error;
pub mod globals;
pub mod w3i_file;
pub mod mmp_file;
pub mod region_file;
pub mod camera_file;
pub mod sound_file;
pub mod pathmap_file;
pub mod shadowmap_file;
pub mod terrain_file;
pub mod minimap_file;
pub mod import_file;
pub mod trigger_string_file;
pub mod trigger_jass_file;
pub mod triggers;
pub mod map;
pub mod slk_datas;
pub mod data_ini;
pub mod doodad_map;
pub mod unit_map;
pub mod custom_datas;
