use std::collections::HashMap;
use std::mem::size_of_val;
use std::thread::sleep;
use std::time;
use std::time::Instant;

use mpq::Archive;

use war_editor::map_data::slk_datas::SLKData;
use war_editor::globals::{PROFILE_AIEDITOR_DATA, PROFILE_CAMPAIGN_ABILITY_FUNC, PROFILE_CAMPAIGN_UNIT_FUNC, PROFILE_CAMPAIGN_UPGRADE_FUNC, PROFILE_COMMON_ABILITY_FUNC, PROFILE_HUMAN_ABILITY_FUNC, PROFILE_HUMAN_UNIT_FUNC, PROFILE_HUMAN_UPGRADE_FUNC, PROFILE_ITEM_ABILITY_FUNC, PROFILE_ITEM_FUNC, PROFILE_MISC_DATA, PROFILE_NEUTRAL_ABILITY_FUNC, PROFILE_NEUTRAL_UNIT_FUNC, PROFILE_NEUTRAL_UPGRADE_FUNC, PROFILE_NIGHT_ELF_ABILITY_FUNC, PROFILE_NIGHT_ELF_UNIT_FUNC, PROFILE_NIGHT_ELF_UPGRADE_FUNC, PROFILE_ORC_ABILITY_FUNC, PROFILE_ORC_UNIT_FUNC, PROFILE_ORC_UPGRADE_FUNC, PROFILE_UNDEAD_ABILITY_FUNC, PROFILE_UNDEAD_UNIT_FUNC, PROFILE_UNDEAD_UPGRADE_FUNC, PROFILE_UNIT_EDITOR_DATA, PROFILE_WAR3SKINS, PROFILE_WORLD_EDIT_DATA, PROFILE_WORLD_EDIT_GAME_STRINGS, PROFILE_WORLD_EDIT_LAYOUT, PROFILE_WORLD_EDIT_STRINGS, SLK_ABILITY_BUFF_META_DATA, SLK_ABILITY_DATA, SLK_ABILITY_META_DATA, SLK_DESTRUCTABLE_DATA, SLK_DESTRUCTABLE_META_DATA, SLK_DOODAD_META_DATA, SLK_DOODADS, SLK_MISC_META_DATA, SLK_SKIN_META_DATA, SLK_UNIT_ABILITIES, SLK_UNIT_BALANCE, SLK_UNIT_COMBAT_SOUNDS, SLK_UNIT_DATA, SLK_UNIT_META_DATA, SLK_UNIT_UI, SLK_UNIT_WEAPONS, SLK_UPGRADE_DATA, SLK_UPGRADE_EFFECT_META_DATA, SLK_UPGRADE_META_DATA};
use war_editor::map_data::camera_file::CameraFile;
use war_editor::map_data::concat_path;
use war_editor::map_data::custom_text_trigger_file::CustomTextTriggerFile;
use war_editor::map_data::data_ini::DataIni;
use war_editor::map_data::environment_file::EnvironmentFile;
use war_editor::map_data::map::Map;
use war_editor::map_data::minimap_file::MinimapFile;
use war_editor::map_data::mmp_file::MMPFile;
use war_editor::map_data::pathmap_file::PathMapFile;
use war_editor::map_data::region_file::RegionFile;
use war_editor::map_data::shadowmap_file::ShadowMapFile;
use war_editor::map_data::sound_file::SoundFile;
use war_editor::map_data::trigger_string_file::TriggerStringFile;
use war_editor::map_data::w3i_file::W3iFile;

fn elapsed_time(instant: &Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {}:{}:{}::{}", hours, mins, seconds, millis);
}

fn format_data(path: &str) -> String{
    format!("resources/datas/{}",path)
}
fn format_slk(path: &str) -> String{
    format!("resources/slk/{}",path)
}

fn main() {
//    for (key, value) in std::env::vars() {
//        println!("{}: {}", key, value);
//    }
    let now = Instant::now();
    let mut ini = DataIni::new();
    ini.merge(&format_data(PROFILE_ITEM_FUNC));
    ini.merge(&format_data(PROFILE_HUMAN_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_ORC_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_UNDEAD_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_NIGHT_ELF_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_NEUTRAL_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_COMMON_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_CAMPAIGN_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_ITEM_ABILITY_FUNC));
    ini.merge(&format_data(PROFILE_HUMAN_UNIT_FUNC));
    ini.merge(&format_data(PROFILE_ORC_UNIT_FUNC));
    ini.merge(&format_data(PROFILE_UNDEAD_UNIT_FUNC));
    ini.merge(&format_data(PROFILE_NIGHT_ELF_UNIT_FUNC));
    ini.merge(&format_data(PROFILE_NEUTRAL_UNIT_FUNC));
    ini.merge(&format_data(PROFILE_CAMPAIGN_UNIT_FUNC));
    ini.merge(&format_data(PROFILE_ITEM_FUNC));
    ini.merge(&format_data(PROFILE_HUMAN_UPGRADE_FUNC));
    ini.merge(&format_data(PROFILE_ORC_UPGRADE_FUNC));
    ini.merge(&format_data(PROFILE_UNDEAD_UPGRADE_FUNC));
    ini.merge(&format_data(PROFILE_NIGHT_ELF_UPGRADE_FUNC));
    ini.merge(&format_data(PROFILE_NEUTRAL_UPGRADE_FUNC));
    ini.merge(&format_data(PROFILE_CAMPAIGN_UPGRADE_FUNC));
    ini.merge(&format_data(PROFILE_UNIT_EDITOR_DATA));
    ini.merge(&format_data(PROFILE_WORLD_EDIT_STRINGS));
    ini.merge(&format_data(PROFILE_WORLD_EDIT_LAYOUT));
    ini.merge(&format_data(PROFILE_WORLD_EDIT_DATA));
    ini.merge(&format_data(PROFILE_WORLD_EDIT_GAME_STRINGS));
    ini.merge(&format_data(PROFILE_WAR3SKINS));
    ini.merge(&format_data(PROFILE_MISC_DATA));
    ini.merge(&format_data(PROFILE_AIEDITOR_DATA));
    ini.fit();
//    ini.debug();
    let unit_meta = SLKData::load(&format_slk(SLK_UNIT_META_DATA));
    let doodad_meta = SLKData::load(&format_slk(SLK_DOODAD_META_DATA));
    let destructable_meta = SLKData::load(&format_slk(SLK_DESTRUCTABLE_META_DATA));
    let abilty_meta = SLKData::load(&format_slk(SLK_ABILITY_META_DATA));
    let upgrade_meta = SLKData::load(&format_slk(SLK_UPGRADE_META_DATA));
    let upgrade_effect_meta = SLKData::load(&format_slk(SLK_UPGRADE_EFFECT_META_DATA));
    let const_meta = SLKData::load(&format_slk(SLK_MISC_META_DATA));
    let ui_const_meta = SLKData::load(&format_slk(SLK_SKIN_META_DATA));
    let ability_buff_meta = SLKData::load(&format_slk(SLK_ABILITY_BUFF_META_DATA));
    let mut unit_data = SLKData::new();
    unit_data.merge(&format_slk(SLK_UNIT_DATA));
    unit_data.merge(&format_slk(SLK_UNIT_BALANCE));
    unit_data.merge(&format_slk(SLK_UNIT_UI));
    unit_data.merge(&format_slk(SLK_UNIT_ABILITIES));
    unit_data.merge(&format_slk(SLK_UNIT_WEAPONS));
    let ability_data = SLKData::load(&format_slk(SLK_ABILITY_DATA));
    let upgrade_data = SLKData::load(&format_slk(SLK_UPGRADE_DATA));
    let doodad_effect_data = SLKData::load(&format_slk(SLK_DOODADS));
    let destructable_effect_data = SLKData::load(&format_slk(SLK_DESTRUCTABLE_DATA));

    let hfoo = &String::from("hfoo");
    println!("{:#?}", unit_data.headers());
    println!("{:#?}", unit_data.get_formatted(hfoo));
//    unit_data.debug();
//    unit_meta.debug();

    elapsed_time(&now);
    println!("Hello, world!");
//    let mut mpq = Archive::open("resources/sample_1/Test.w3x").unwrap();
    let  sample_1 = "resources/sample_1/Test.w3x".to_string();
    let  sample_2 = "resources/sample_2/Remake1 - Copie.w3x".to_string();
//    let map = Map::open(sample_2);
//    println!("size rgba: {}",size_of_val(&vec![0u8,0u8,0u8,0u8][0..]));
//    println!("{:X}, {:X}", true as u8, false as u8);
//    let rgba = RGBA::by_value(0xFF5C15FF);
//    rgba.debug();
//    println!("{:X} {:X} {:X}", rgba.red(),rgba.green(), rgba.blue());

    elapsed_time(&now);
//    sleep(time::Duration::from_secs(10));

}
