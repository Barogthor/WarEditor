#[macro_use]
extern crate derivative;
// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;
#[macro_use]
extern crate lazy_static;

use slkparser::SLKError;
use thiserror::Error;

use crate::data_ini::DataIni;
use crate::globals::*;
use crate::slk_datas::SLKData;

pub use error::MapError;

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

// The SLK table fields below are read only by the cfg(test) gamedata_snapshot
// regression test; no production accessor consumes them yet (only
// `get_trigger_data` is exposed so far). Kept as loaded state for future
// GameData query APIs rather than deleted.
pub struct GameData {
    trigger_data: DataIni,
    #[allow(dead_code)]
    unit_data: SLKData,
    #[allow(dead_code)]
    unit_meta: SLKData,
    #[allow(dead_code)]
    doodad_meta: SLKData,
    #[allow(dead_code)]
    destructable_meta: SLKData,
    #[allow(dead_code)]
    abilty_meta: SLKData,
    #[allow(dead_code)]
    upgrade_meta: SLKData,
    #[allow(dead_code)]
    upgrade_effect_meta: SLKData,
    #[allow(dead_code)]
    const_meta: SLKData,
    #[allow(dead_code)]
    ui_const_meta: SLKData,
    #[allow(dead_code)]
    ability_buff_meta: SLKData,
    #[allow(dead_code)]
    ability_data: SLKData,
    #[allow(dead_code)]
    upgrade_data: SLKData,
    #[allow(dead_code)]
    doodad_effect_data: SLKData,
    #[allow(dead_code)]
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
