#![allow(dead_code)]
#[macro_use]
extern crate derivative;
// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;
#[macro_use]
extern crate lazy_static;

use slkparser::SLKError;
use thiserror::Error;
use wce_formats::MpqError;

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
use crate::map_string_file::MapStringError;
use crate::minimap_file::MinimapError;
use crate::mmp_file::MenuMinimapError;
use crate::pathmap_file::PathmapError;
use crate::region_file::RegionError;
use crate::shadowmap_file::ShadowMapError;
use crate::slk_datas::SLKData;
use crate::sound_file::SoundError;
use crate::terrain_file::TerrainError;
use crate::trigger_jass_file::TriggerJassError;
use crate::triggers::TriggersError;
use crate::unit_map::UnitMapError;
use crate::w3i_file::InfoError;

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Map is likely protecte. {0}")]
    Protected(MpqError),
    #[error("Failed to write map archive. {0}")]
    Archive(MpqError),
    #[error("Refusing to save: these archive entries could not be read on open (unsupported MPQ compression) and would be lost: {0:?}")]
    UnreadableArchiveEntries(Vec<String>),
    #[error("Failed to write file '{path}'. {source}")]
    SaveFileIo {
        path: String,
        source: std::io::Error,
    },
    #[error("Failed on terrain environment. {0}")]
    Environment(TerrainError),
    #[error("Failed on custom text triggers. {0}")]
    CustomTextTrigger(TriggerJassError),
    #[error("Failed on trigger data. {0}")]
    Triggers(TriggersError),
    #[error("Failed on import file list. {0}")]
    Import(ImportError),
    #[error("Failed on minimap. {0}")]
    Minimap(MinimapError),
    #[error("Failed on menu minimap. {0}")]
    MenuMinimap(MenuMinimapError),
    #[error("Failed on pathing map. {0}")]
    PathingMap(PathmapError),
    #[error("Failed on regions. {0}")]
    Region(RegionError),
    #[error("Failed on shadow map. {0}")]
    ShadowMap(ShadowMapError),
    #[error("Failed on doodads. {0}")]
    Doodad(DoodadError),
    #[error("Failed on cameras. {0}")]
    Camera(CameraError),
    #[error("Failed on units and items. {0}")]
    UnitItem(UnitMapError),
    #[error("Failed on sounds. {0}")]
    Sound(SoundError),
    #[error("Failed on map strings. {0}")]
    MapStrings(MapStringError),
    #[error("Failed on map info. {0}")]
    Info(InfoError),
    #[error("Failed on custom units. {0}")]
    CustomUnit(CustomUnitError),
    #[error("Failed on custom items. {0}")]
    CustomItem(CustomItemError),
    #[error("Failed on custom abilities. {0}")]
    CustomAbility(CustomAbilityError),
    #[error("Failed on custom buffs. {0}")]
    CustomBuff(CustomBuffError),
    #[error("Failed on custom upgrades. {0}")]
    CustomUpgrade(CustomUpgradeError),
    #[error("Failed on custom doodads. {0}")]
    CustomDoodad(CustomDoodadError),
    #[error("Failed on custom destructables. {0}")]
    CustomDestructable(CustomDestructableError),
}

/// Game database loading errors (see [`GameData::new`]).
#[derive(Debug, Error)]
pub enum GameDataError {
    /// Failed to load or parse an SLK table.
    #[error("Failed to load SLK game data. {0}")]
    Slk(#[from] SLKError),
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
    pub fn new(prefix: &str) -> Result<Self, GameDataError> {
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
    // Uses CARGO_MANIFEST_DIR to get the workspace root directory

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
pub mod map_string_file;
pub mod minimap_file;
pub mod mmp_file;
pub mod pathmap_file;
pub mod region_file;
pub mod shadowmap_file;
pub(crate) mod slk_datas;
pub mod sound_file;
pub mod terrain_file;
pub mod trigger_jass_file;
pub mod triggers;
pub mod unit_map;
pub mod w3i_file;

#[cfg(test)]
mod gamedata_snapshot;
