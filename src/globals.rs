pub const REG_PROPERTIES: &str = r"^([^=\r\n]+)=(.+)\r?\n?$";

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum GameVersion {
    RoC,
    TFT,
    TFT131,
}
impl Default for GameVersion{
    fn default() -> Self {
        GameVersion::TFT
    }
}

pub const MPQ_LISTFILES:                        &str = "(listfile)";
pub const MPQ_ATTRIBUTES:                       &str = "(attributes)";

pub const MAP_TERRAIN:                          &str = "war3map.w3e";
pub const MAP_INFOS:                            &str = "war3map.w3i";
pub const MAP_TRIGGERS:                         &str = "war3map.wtg";
pub const MAP_TRIGGERS_SCRIPT:                  &str = "war3map.wct";
pub const MAP_STRINGS:                          &str = "war3map.wts";
pub const MAP_SCRIPT:                           &str = "war3map.j";
pub const MAP_SHADERS:                          &str = "war3map.shd";
pub const MAP_MINIMAP:                          &str = "war3mapMap.blp";
pub const MAP_OLD_MINIMAP:                      &str = "war3mapMap.blp";
//pub const a:                                      &str = "war3mapMap.b00";
pub const MAP_OLD_PATH_MAP:                     &str = "war3mapPath.tga";
//pub const a:                                      &str = "war3mapPreview.tga";
pub const MAP_MENU_MINIMAP:                     &str = "war3map.mmp";
pub const MAP_PATH_MAP:                         &str = "war3map.wpm";
pub const MAP_TERRAIN_DOODADS:                  &str = "war3map.doo";
pub const MAP_TERRAIN_UNITS:                    &str = "war3mapUnits.doo";
pub const MAP_REGIONS:                          &str = "war3map.w3r";
pub const MAP_CAMERAS:                          &str = "war3map.w3c";
pub const MAP_SOUNDS:                           &str = "war3map.w3s";
pub const MAP_CUSTOM_UNITS:                     &str = "war3map.w3u";
pub const MAP_CUSTOM_ITEMS:                     &str = "war3map.w3t";
pub const MAP_CUSTOM_ABILITIES:                 &str = "war3map.w3a";
pub const MAP_CUSTOM_DESTRUCTABLES:             &str = "war3map.w3b";
pub const MAP_CUSTOM_DOODADS:                   &str = "war3map.w3d";
pub const MAP_CUSTOM_UPGRADES:                  &str = "war3map.w3q";
pub const MAP_CUSTOM_BUFFS:                     &str = "war3map.w3h";
pub const MAP_CUSTOM_OBJECT:                    &str = "war3mapMap.w3o";
pub const MAP_AI:                               &str = "war3mapMap.wai";
pub const MAP_GAME_CONSTANTS:                   &str = "war3mapMisc.txt";
pub const MAP_GAME_INTERFACE:                   &str = "war3mapSkin.txt";
pub const MAP_GAME_MISC:                        &str = "war3mapExtra.txt";
pub const MAP_IMPORT_LIST:                      &str = "war3map.imp";
pub const MAP_ROOT_IMPORTED_FILES:              &str = "war3mapImported";

pub const CAMPAIGN_CUSTOM_UNITS:                &str = "war3campaign.w3u";
pub const CAMPAIGN_CUSTOM_ITEMS:                &str = "war3campaign.w3t";
pub const CAMPAIGN_CUSTOM_ABILITIES:            &str = "war3campaign.w3a";
pub const CAMPAIGN_CUSTOM_DESTRUCTABLES:        &str = "war3campaign.w3b";
pub const CAMPAIGN_CUSTOM_DOODADS:              &str = "war3campaign.w3d";
pub const CAMPAIGN_CUSTOM_UPGRADES:             &str = "war3campaign.w3q";
pub const CAMPAIGN_CUSTOM_BUFFS:                &str = "war3campaign.w3h";
pub const CAMPAIGN_INFOS:                       &str = "war3campaign.w3f";
pub const CAMPAIGN_ROOT_IMPORTED_FILES:         &str = "war3campaignImported";

pub const SLK_ABILITY_DATA:                     &str = "AbilityData.slk";
pub const SLK_ENVIRONMENT_SOUNDS:               &str = "EnvironmentSounds.slk";
pub const SLK_UBER_SPLAT_DATA:                  &str = "UberSplatData.slk";
pub const SLK_ABILITY_META_DATA:                &str = "AbilityMetaData.slk";
pub const SLK_ITEM_DATA:                        &str = "ItemData.slk";
pub const SLK_UISOUNDS:                         &str = "UISounds.slk";
pub const SLK_ABILITY_SOUNDS:                   &str = "AbilitySounds.slk";
pub const SLK_LIGHTNING_DATA:                   &str = "LightningData.slk";
pub const SLK_UNIT_ABILITIES:                   &str = "UnitAbilities.slk";
pub const SLK_AMBIENCE_SOUNDS:                  &str = "AmbienceSounds.slk";
pub const SLK_MIDISOUNDS:                       &str = "MIDISounds.slk";
pub const SLK_UNIT_ACK_SOUNDS:                  &str = "UnitAckSounds.slk";
pub const SLK_ANIM_LOOKUPS:                     &str = "AnimLookups.slk";
pub const SLK_MISC_META_DATA:                   &str = "MiscMetaData.slk";
pub const SLK_UNIT_BALANCE:                     &str = "UnitBalance.slk";
pub const SLK_ANIM_SOUNDS:                      &str = "AnimSounds.slk";
pub const SLK_NOT_USED_UNIT_DATA:               &str = "NotUsed_UnitData.slk";
pub const SLK_UNIT_COMBAT_SOUNDS:               &str = "UnitCombatSounds.slk";
pub const SLK_CITY_CLIFFS:                      &str = "CityCliffs.slk";
pub const SLK_NOT_USED_UNIT_UI:                 &str = "NotUsed_unitUI.slk";
pub const SLK_UNIT_DATA:                        &str = "UnitData.slk";
pub const SLK_CLIFFS:                           &str = "Cliffs.slk";
pub const SLK_PORTRAIT_ANIMS:                   &str = "PortraitAnims.slk";
pub const SLK_UNIT_META_DATA:                   &str = "UnitMetaData.slk";
pub const SLK_CLIFF_TYPES:                      &str = "CliffTypes.slk";
pub const SLK_AMPLE_1:                          &str = "sample_1.slk";
pub const SLK_NIT_UI:                           &str = "unitUI.slk";
pub const SLK_DESTRUCTABLE_DATA:                &str = "DestructableData.slk";
pub const SLK_SKIN_META_DATA:                   &str = "SkinMetaData.slk";
pub const SLK_UNIT_WEAPONS:                     &str = "UnitWeapons.slk";
pub const SLK_DESTRUCTABLE_META_DATA:           &str = "DestructableMetaData.slk";
pub const SLK_SPAWN_DATA:                       &str = "SpawnData.slk";
pub const SLK_UPGRADE_DATA:                     &str = "UpgradeData.slk";
pub const SLK_DIALOG_SOUNDS:                    &str = "DialogSounds.slk";
pub const SLK_SPLAT_DATA:                       &str = "SplatData.slk";
pub const SLK_UPGRADE_EFFECT_META_DATA:         &str = "UpgradeEffectMetaData.slk";
pub const SLK_DOODAD_META_DATA:                 &str = "DoodadMetaData.slk";
pub const SLK_TERRAIN:                          &str = "Terrain.slk";
pub const SLK_UPGRADE_META_DATA:                &str = "UpgradeMetaData.slk";
pub const SLK_DOODADS:                          &str = "Doodads.slk";
pub const SLK_WATER:                            &str = "Water.slk";
pub const SLK_EAXDEFS:                          &str = "EAXDefs.slk";
pub const SLK_WEATHER:                          &str = "Weather.slk";

pub const PROFILE_AIEDITOR_DATA:                &str = "AIEditorData.txt";
pub const PROFILE_MAC_WORLD_EDIT_STRINGS:       &str = "MacWorldEditStrings.txt";
pub const PROFILE_TRIGGER_DATA:                 &str = "TriggerData.txt";
pub const PROFILE_CAMPAIGN_ABILITY_FUNC:        &str = "CampaignAbilityFunc.txt";
pub const PROFILE_MISC_DATA:                    &str = "MiscData.txt";
pub const PROFILE_TRIGGER_STRINGS:              &str = "TriggerStrings.txt";
pub const PROFILE_CAMPAIGN_STRINGS:             &str = "CampaignStrings.txt";
pub const PROFILE_MISC_UI:                      &str = "MiscUI.txt";
pub const PROFILE_UNDEAD_ABILITY_FUNC:          &str = "UndeadAbilityFunc.txt";
pub const PROFILE_CAMPAIGN_STRINGS_EXP:         &str = "CampaignStrings_exp.txt";
pub const PROFILE_NEUTRAL_ABILITY_FUNC:         &str = "NeutralAbilityFunc.txt";
pub const PROFILE_UNDEAD_UNIT_FUNC:             &str = "UndeadUnitFunc.txt";
pub const PROFILE_CAMPAIGN_UNIT_FUNC:           &str = "CampaignUnitFunc.txt";
pub const PROFILE_NEUTRAL_UNIT_FUNC:            &str = "NeutralUnitFunc.txt";
pub const PROFILE_UNDEAD_UPGRADE_FUNC:          &str = "UndeadUpgradeFunc.txt";
pub const PROFILE_CAMPAIGN_UPGRADE_FUNC:        &str = "CampaignUpgradeFunc.txt";
pub const PROFILE_NEUTRAL_UPGRADE_FUNC:         &str = "NeutralUpgradeFunc.txt";
pub const PROFILE_UNIT_EDITOR_DATA:             &str = "UnitEditorData.txt";
pub const PROFILE_COMMON_ABILITY_FUNC:          &str = "CommonAbilityFunc.txt";
pub const PROFILE_NIGHT_ELF_ABILITY_FUNC:       &str = "NightElfAbilityFunc.txt";
pub const PROFILE_WAR3SKINS:                    &str = "war3skins.txt";
pub const PROFILE_HELP_STRINGS:                 &str = "HelpStrings.txt";
pub const PROFILE_NIGHT_ELF_UNIT_FUNC:          &str = "NightElfUnitFunc.txt";
pub const PROFILE_WORLD_EDIT_DATA:              &str = "WorldEditData.txt";
pub const PROFILE_HUMAN_ABILITY_FUNC:           &str = "HumanAbilityFunc.txt";
pub const PROFILE_NIGHT_ELF_UPGRADE_FUNC:       &str = "NightElfUpgradeFunc.txt";
pub const PROFILE_WORLD_EDIT_GAME_STRINGS:      &str = "WorldEditGameStrings.txt";
pub const PROFILE_HUMAN_UNIT_FUNC:              &str = "HumanUnitFunc.txt";
pub const PROFILE_ORC_ABILITY_FUNC:             &str = "OrcAbilityFunc.txt";
pub const PROFILE_WORLD_EDIT_LAYOUT:            &str = "WorldEditLayout.txt";
pub const PROFILE_HUMAN_UPGRADE_FUNC:           &str = "HumanUpgradeFunc.txt";
pub const PROFILE_ORC_UNIT_FUNC:                &str = "OrcUnitFunc.txt";
pub const PROFILE_WORLD_EDIT_LICENSE:           &str = "WorldEditLicense.txt";
pub const PROFILE_ITEM_ABILITY_FUNC:            &str = "ItemAbilityFunc.txt";
pub const PROFILE_ORC_UPGRADE_FUNC:             &str = "OrcUpgradeFunc.txt";
pub const PROFILE_WORLD_EDIT_STARTUP_STRINGS:   &str = "WorldEditStartupStrings.txt";
pub const PROFILE_ITEM_FUNC:                    &str = "ItemFunc.txt";
pub const PROFILE_STARTUP_STRINGS:              &str = "StartupStrings.txt";
pub const PROFILE_WORLD_EDIT_STRINGS:           &str = "WorldEditStrings.txt";
pub const PROFILE_MAC_HELP_STRINGS:             &str = "MacHelpStrings.txt";
pub const PROFILE_TELEMETRY:                    &str = "Telemetry.txt";
pub const PROFILE_MAC_STRINGS:                  &str = "MacStrings.txt";
pub const PROFILE_TIP_STRINGS:                  &str = "TipStrings.txt";