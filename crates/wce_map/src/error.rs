//! Aggregated error type for map loading and saving.

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
use crate::doodad_map::DoodadError;
use crate::import_file::ImportError;
use crate::map_string_file::MapStringError;
use crate::minimap_file::MinimapError;
use crate::mmp_file::MenuMinimapError;
use crate::pathmap_file::PathmapError;
use crate::region_file::RegionError;
use crate::shadowmap_file::ShadowMapError;
use crate::sound_file::SoundError;
use crate::terrain_file::TerrainError;
use crate::trigger_jass_file::TriggerJassError;
use crate::triggers::TriggersError;
use crate::unit_map::UnitMapError;
use crate::w3i_file::InfoError;

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Map is likely protected. {0}")]
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
    Environment(#[from] TerrainError),
    #[error("Failed on custom text triggers. {0}")]
    CustomTextTrigger(#[from] TriggerJassError),
    #[error("Failed on trigger data. {0}")]
    Triggers(#[from] TriggersError),
    #[error("Failed on import file list. {0}")]
    Import(#[from] ImportError),
    #[error("Failed on minimap. {0}")]
    Minimap(#[from] MinimapError),
    #[error("Failed on menu minimap. {0}")]
    MenuMinimap(#[from] MenuMinimapError),
    #[error("Failed on pathing map. {0}")]
    PathingMap(#[from] PathmapError),
    #[error("Failed on regions. {0}")]
    Region(#[from] RegionError),
    #[error("Failed on shadow map. {0}")]
    ShadowMap(#[from] ShadowMapError),
    #[error("Failed on doodads. {0}")]
    Doodad(#[from] DoodadError),
    #[error("Failed on cameras. {0}")]
    Camera(#[from] CameraError),
    #[error("Failed on units and items. {0}")]
    UnitItem(#[from] UnitMapError),
    #[error("Failed on sounds. {0}")]
    Sound(#[from] SoundError),
    #[error("Failed on map strings. {0}")]
    MapStrings(#[from] MapStringError),
    #[error("Failed on map info. {0}")]
    Info(#[from] InfoError),
    #[error("Failed on custom units. {0}")]
    CustomUnit(#[from] CustomUnitError),
    #[error("Failed on custom items. {0}")]
    CustomItem(#[from] CustomItemError),
    #[error("Failed on custom abilities. {0}")]
    CustomAbility(#[from] CustomAbilityError),
    #[error("Failed on custom buffs. {0}")]
    CustomBuff(#[from] CustomBuffError),
    #[error("Failed on custom upgrades. {0}")]
    CustomUpgrade(#[from] CustomUpgradeError),
    #[error("Failed on custom doodads. {0}")]
    CustomDoodad(#[from] CustomDoodadError),
    #[error("Failed on custom destructables. {0}")]
    CustomDestructable(#[from] CustomDestructableError),
}
