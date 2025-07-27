#![allow(dead_code)]
#[macro_use]
extern crate derivative;
// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;
#[macro_use]
extern crate lazy_static;

use slkparser::SLKError;

use crate::camera_file::CameraError;
use crate::custom_datas::ability::CustomAbilityError;
use crate::custom_datas::buff::CustomBuffError;
use crate::custom_datas::destructable::CustomDestructableError;
use crate::custom_datas::doodad::CustomDoodadError;
use crate::custom_datas::item::CustomItemError;
use crate::custom_datas::unit::CustomUnitError;
use crate::custom_datas::upgrade::CustomUpgradeError;
use crate::data_ini::DataIni;
use crate::doodad_map::DoodadError;
use crate::globals::*;
use crate::import_file::ImportError;
use crate::minimap_file::MinimapError;
use crate::mmp_file::MenuMinimapError;
use crate::pathmap_file::PathmapError;
use crate::region_file::RegionError;
use crate::shadowmap_file::ShadowMapError;
use crate::slk_datas::SLKData;
use crate::sound_file::SoundError;
use crate::terrain_file::TerrainError;
use crate::trigger_jass_file::TriggerJassError;
use crate::trigger_string_file::MapStringError;
use crate::triggers::TriggersError;
use crate::unit_map::UnitMapError;
use crate::w3i_file::InfoError;

#[derive(Debug)]
pub enum OpeningError {
    Protected(String),
    Environment(TerrainError),
    CustomTextTrigger(TriggerJassError),
    Triggers(TriggersError),
    Import(ImportError),
    Minimap(MinimapError),
    MenuMinimap(MenuMinimapError),
    PathingMap(PathmapError),
    Region(RegionError),
    ShadowMap(ShadowMapError),
    Doodad(DoodadError),
    Camera(CameraError),
    UnitItem(UnitMapError),
    Sound(SoundError),
    MapStrings(MapStringError),
    Info(InfoError),
    CustomUnit(CustomUnitError),
    CustomItem(CustomItemError),
    CustomAbility(CustomAbilityError),
    CustomBuff(CustomBuffError),
    CustomUpgrade(CustomUpgradeError),
    CustomDoodad(CustomDoodadError),
    CustomDestructable(CustomDestructableError),
}

pub fn path_to_data(prefix: &str, path: &str) -> String {
    format!("{prefix}datas/{path}")
}
pub fn path_to_slk(prefix: &str, path: &str) -> String {
    format!("{prefix}slk/{path}")
}

pub struct GameData {
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
    pub fn new(prefix: &str) -> Result<Self, SLKError> {
        let mut trigger_data = DataIni::new();
        trigger_data.merge(&path_to_data(prefix, PROFILE_TRIGGER_DATA));
        let unit_meta = SLKData::load(&path_to_slk(prefix, SLK_UNIT_META_DATA))?;

        let doodad_meta = SLKData::load(&path_to_slk(prefix, SLK_DOODAD_META_DATA))?;
        let destructable_meta = SLKData::load(&path_to_slk(prefix, SLK_DESTRUCTABLE_META_DATA))?;
        let abilty_meta = SLKData::load(&path_to_slk(prefix, SLK_ABILITY_META_DATA))?;
        let upgrade_meta = SLKData::load(&path_to_slk(prefix, SLK_UPGRADE_META_DATA))?;
        let upgrade_effect_meta =
            SLKData::load(&path_to_slk(prefix, SLK_UPGRADE_EFFECT_META_DATA))?;
        let const_meta = SLKData::load(&path_to_slk(prefix, SLK_MISC_META_DATA))?;
        let ui_const_meta = SLKData::load(&path_to_slk(prefix, SLK_SKIN_META_DATA))?;
        let ability_buff_meta = SLKData::load(&path_to_slk(prefix, SLK_ABILITY_BUFF_META_DATA))?;
        let mut unit_data = SLKData::new();
        unit_data.merge(&path_to_slk(prefix, SLK_UNIT_DATA))?;
        unit_data.merge(&path_to_slk(prefix, SLK_UNIT_BALANCE))?;
        unit_data.merge(&path_to_slk(prefix, SLK_UNIT_UI))?;
        unit_data.merge(&path_to_slk(prefix, SLK_UNIT_ABILITIES))?;
        unit_data.merge(&path_to_slk(prefix, SLK_UNIT_WEAPONS))?;
        let ability_data = SLKData::load(&path_to_slk(prefix, SLK_ABILITY_DATA))?;
        let upgrade_data = SLKData::load(&path_to_slk(prefix, SLK_UPGRADE_DATA))?;
        let doodad_effect_data = SLKData::load(&path_to_slk(prefix, SLK_DOODADS))?;
        let destructable_effect_data = SLKData::load(&path_to_slk(prefix, SLK_DESTRUCTABLE_DATA))?;
        Ok(Self {
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
        })
    }

    pub fn get_trigger_data(&self) -> &DataIni {
        &self.trigger_data
    }
}

#[cfg(test)]
fn get_resources_path() -> String {
    // Utilise CARGO_MANIFEST_DIR pour obtenir le répertoire racine du workspace

    use std::path::Path;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("Should have parent directory");
    format!("{}/resources/", workspace_root.to_string_lossy())
}

pub mod camera_file;
pub mod custom_datas;
pub mod data_ini;
pub mod doodad_map;
pub mod error;
pub mod globals;
pub mod import_file;
pub mod map;
pub mod minimap_file;
pub mod mmp_file;
pub mod pathmap_file;
pub mod region_file;
pub mod shadowmap_file;
pub mod slk_datas;
pub mod sound_file;
pub mod terrain_file;
pub mod trigger_jass_file;
pub mod trigger_string_file;
pub mod triggers;
pub mod unit_map;
pub mod w3i_file;
