use std::time::Instant;

use log::{debug, error, info, trace, warn};

use war_editor::init_logging;
use wce_map::{format_data, format_slk, GameData};
use wce_map::data_ini::DataIni;
use wce_map::globals::*;
use wce_map::map::Map;
use wce_map::slk_datas::SLKData;

fn elapsed_time(instant: &Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {:02}:{:02}:{:02}::{:03}", hours, mins, seconds, millis);
}

fn main() {
    init_logging();
    info!("hello world logging");
    warn!("hello world logging");
    error!("hello world logging");
    debug!("hello world logging");
    trace!("hello world logging");
//    for (key, value) in std::env::vars() {
//        println!("{}: {}", key, value);
//    }
    let now = Instant::now();

    let game_data = &GameData::new("");
    // let mut trigger_data = DataIni::new();
    // trigger_data.merge(PROFILE_TRIGGER_DATA);
    // // trigger_datas.debug();

    let mut ini = DataIni::new();

    ini.merge(&format_data("", PROFILE_ITEM_FUNC));
    ini.merge(&format_data("", PROFILE_HUMAN_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_ORC_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_UNDEAD_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_NIGHT_ELF_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_NEUTRAL_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_COMMON_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_CAMPAIGN_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_ITEM_ABILITY_FUNC));
    ini.merge(&format_data("", PROFILE_HUMAN_UNIT_FUNC));
    ini.merge(&format_data("", PROFILE_ORC_UNIT_FUNC));
    ini.merge(&format_data("", PROFILE_UNDEAD_UNIT_FUNC));
    ini.merge(&format_data("", PROFILE_NIGHT_ELF_UNIT_FUNC));
    ini.merge(&format_data("", PROFILE_NEUTRAL_UNIT_FUNC));
    ini.merge(&format_data("", PROFILE_CAMPAIGN_UNIT_FUNC));
    ini.merge(&format_data("", PROFILE_ITEM_FUNC));
    ini.merge(&format_data("", PROFILE_HUMAN_UPGRADE_FUNC));
    ini.merge(&format_data("", PROFILE_ORC_UPGRADE_FUNC));
    ini.merge(&format_data("", PROFILE_UNDEAD_UPGRADE_FUNC));
    ini.merge(&format_data("", PROFILE_NIGHT_ELF_UPGRADE_FUNC));
    ini.merge(&format_data("", PROFILE_NEUTRAL_UPGRADE_FUNC));
    ini.merge(&format_data("", PROFILE_CAMPAIGN_UPGRADE_FUNC));
    ini.merge(&format_data("", PROFILE_UNIT_EDITOR_DATA));
    ini.merge(&format_data("", PROFILE_WORLD_EDIT_STRINGS));
    ini.merge(&format_data("", PROFILE_WORLD_EDIT_LAYOUT));
    ini.merge(&format_data("", PROFILE_WORLD_EDIT_DATA));
    ini.merge(&format_data("", PROFILE_WORLD_EDIT_GAME_STRINGS));
    ini.merge(&format_data("", PROFILE_WAR3SKINS));
    ini.merge(&format_data("", PROFILE_MISC_DATA));
    ini.merge(&format_data("", PROFILE_AIEDITOR_DATA));
    ini.fit();
    // ini.debug();
    let unit_meta = SLKData::load(&format_slk("", SLK_UNIT_META_DATA));

    let doodad_meta = SLKData::load(&format_slk("", SLK_DOODAD_META_DATA));
    let destructable_meta = SLKData::load(&format_slk("", SLK_DESTRUCTABLE_META_DATA));
    let abilty_meta = SLKData::load(&format_slk("", SLK_ABILITY_META_DATA));
    let upgrade_meta = SLKData::load(&format_slk("", SLK_UPGRADE_META_DATA));
    let upgrade_effect_meta = SLKData::load(&format_slk("", SLK_UPGRADE_EFFECT_META_DATA));
    let const_meta = SLKData::load(&format_slk("", SLK_MISC_META_DATA));
    let ui_const_meta = SLKData::load(&format_slk("", SLK_SKIN_META_DATA));
    let ability_buff_meta = SLKData::load(&format_slk("", SLK_ABILITY_BUFF_META_DATA));
    let mut unit_data = SLKData::new();
    unit_data.merge(&format_slk("", SLK_UNIT_DATA));
    unit_data.merge(&format_slk("", SLK_UNIT_BALANCE));
    unit_data.merge(&format_slk("", SLK_UNIT_UI));
    unit_data.merge(&format_slk("", SLK_UNIT_ABILITIES));
    unit_data.merge(&format_slk("", SLK_UNIT_WEAPONS));
    let ability_data = SLKData::load(&format_slk("", SLK_ABILITY_DATA));
    let upgrade_data = SLKData::load(&format_slk("", SLK_UPGRADE_DATA));
    let doodad_effect_data = SLKData::load(&format_slk("", SLK_DOODADS));
    let destructable_effect_data = SLKData::load(&format_slk("", SLK_DESTRUCTABLE_DATA));
    println!("{:#?}", unit_meta);
    // let hfoo = &String::from("hfoo");
    // println!("{:#?}", unit_data.headers());
    // println!("{:#?}", unit_data.get_formatted(hfoo));
    // unit_data.debug();
    // unit_meta.debug();

    elapsed_time(&now);
    println!("Hello, world!");
//    let mut mpq = Archive::open("resources/sample_1/Test.w3x").unwrap();
    let _sample_1 = "resources/sample_1/Test.w3x".to_string();
    let _sample_2 = "resources/sample_2/Remake1 - Copie.w3x".to_string();
    let the_death_sheep = "resources/Scenario/(1)TheDeathSheep.w3m".to_string();
    let _harrow = "resources/Scenario/(2)Harrow.w3m".to_string();
    let _circumvention = "resources/Scenario/(2)Circumvention.w3x".to_string();
    let azure_tower_defense = "resources/Scenario/(8)AzureTowerDefense.w3x".to_string();
    let _sandbox_roc = "resources/Scenario/Sandbox_1.w3m".to_string();
    let _sandbox_tft = "resources/Scenario/Sandbox_1.w3x".to_string();
    let _map = Map::open(the_death_sheep, game_data);
    // let _map = Map::open(_sandbox_roc, game_data);
    // let _map = Map::open(sandbox_tft, game_data);
    let _map = Map::open(azure_tower_defense, game_data);
    // let _map = Map::open(sample_2, game_data);
//     let _map = Map::open(azure_tower_defense);
//     let _map = Map::open(circumvention);
//    let _map = Map::open(harrow);
//    println!("size rgba: {}",size_of_val(&vec![0u8,0u8,0u8,0u8][0..]));
//    println!("{:X}, {:X}", true as u8, false as u8);
//    let rgba = RGBA::by_value(0xFF5C15FF);
//    rgba.debug();
//    println!("{:X} {:X} {:X}", rgba.red(),rgba.green(), rgba.blue());

    elapsed_time(&now);
//    sleep(time::Duration::from_secs(10));

}
