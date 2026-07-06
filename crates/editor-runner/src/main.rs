use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use dotenv::dotenv;
use log::{debug, error, info, trace, warn};

use editor_runner::init_logging;
use wce_map::data_ini::DataIni;
use wce_map::globals::*;
use wce_map::map::Map;
use wce_map::{path_to_data, GameData};

fn elapsed_time(instant: &Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {hours:02}:{mins:02}:{seconds:02}::{millis:03}");
}

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

fn main() {
    dotenv().unwrap();
    init_logging();
    info!("hello world logging");
    warn!("hello world logging");
    error!("hello world logging");
    debug!("hello world logging");
    trace!("hello world logging");
    // for (key, value) in std::env::vars() {
    //     println!("{}: {}", key, value);
    // }
    let now = Instant::now();

    let prefix: &str = &get_resources_path();
    let local_game_data = &GameData::new(prefix).unwrap_or_else(|e| panic!("{e:?}"));
    // let mut trigger_data = DataIni::new();
    // trigger_data.merge(PROFILE_TRIGGER_DATA);
    // // trigger_datas.debug();

    let mut ini = DataIni::new();

    ini.merge(&path_to_data(prefix, PROFILE_ITEM_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_HUMAN_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_ORC_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_UNDEAD_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_NIGHT_ELF_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_NEUTRAL_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_COMMON_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_CAMPAIGN_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_ITEM_ABILITY_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_HUMAN_UNIT_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_ORC_UNIT_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_UNDEAD_UNIT_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_NIGHT_ELF_UNIT_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_NEUTRAL_UNIT_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_CAMPAIGN_UNIT_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_ITEM_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_HUMAN_UPGRADE_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_ORC_UPGRADE_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_UNDEAD_UPGRADE_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_NIGHT_ELF_UPGRADE_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_NEUTRAL_UPGRADE_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_CAMPAIGN_UPGRADE_FUNC));
    ini.merge(&path_to_data(prefix, PROFILE_UNIT_EDITOR_DATA));
    ini.merge(&path_to_data(prefix, PROFILE_WORLD_EDIT_STRINGS));
    ini.merge(&path_to_data(prefix, PROFILE_WORLD_EDIT_LAYOUT));
    ini.merge(&path_to_data(prefix, PROFILE_WORLD_EDIT_DATA));
    ini.merge(&path_to_data(prefix, PROFILE_WORLD_EDIT_GAME_STRINGS));
    ini.merge(&path_to_data(prefix, PROFILE_WAR3SKINS));
    ini.merge(&path_to_data(prefix, PROFILE_MISC_DATA));
    ini.merge(&path_to_data(prefix, PROFILE_AIEDITOR_DATA));
    ini.fit();
    // ini.debug();

    elapsed_time(&now);
    println!("Hello, world!");
    //    let mut mpq = Archive::open("resources/sample_1/Test.w3x").unwrap();
    let _sample_1 = "resources/sample_1/Test.w3x".to_string();
    let _sample_2 = "resources/sample_2/Remake1 - Copie.w3x".to_string();
    let the_death_sheep = "resources/Scenario/(1)TheDeathSheep.w3m".to_string();
    let harrow = "resources/Scenario/(2)Harrow.w3m".to_string();
    let circumvention = "resources/Scenario/(2)Circumvention.w3x".to_string();
    let azure_tower_defense = "resources/Scenario/(8)AzureTowerDefense.w3x".to_string();
    let sandbox_roc = "resources/Scenario/Sandbox_1.w3m".to_string();
    let sandbox_tft = "resources/Scenario/Sandbox_1.w3x".to_string();
    let _map = Map::open(the_death_sheep, local_game_data);
    let _map = Map::open(sandbox_roc, local_game_data);
    let _map = Map::open(sandbox_tft, local_game_data);
    let _map = Map::open(azure_tower_defense, local_game_data);
    // let _map = Map::open(sample_2, game_data);
    //     let _map = Map::open(azure_tower_defense);
    let _map = Map::open(circumvention, local_game_data);
    let _map = Map::open(harrow, local_game_data);
    let old_dir_w3 = std::env::var("OLD_WARCRAFT_DIRECTORY").unwrap();
    // let maps = test_melee_maps(&Path::new(&old_dir_w3).join("Maps"));
    let maps = paths_custom_maps(&Path::new(&old_dir_w3).join("Maps"));
    for map in maps {
        let path = map.into_os_string().into_string().unwrap();
        println!("{path:?}");
        let map_res = Map::open(path.clone(), local_game_data);
        if let Err(err) = map_res {
            error!("Error on map '{path}' : {err}");
        }
    }
    elapsed_time(&now);
}

pub fn is_blizzard_maps(path: &PathBuf) -> bool {
    path.is_dir() && (path.ends_with("Scenario") || path.ends_with("FrozenThrone"))
}

pub fn is_custom_maps(path: &PathBuf) -> bool {
    path.is_dir() && (path.ends_with("Download"))
}

pub fn paths_blizzard_maps(path: &Path) -> Vec<PathBuf> {
    paths_maps(path, vec![], is_blizzard_maps)
}

pub fn paths_custom_maps(path: &Path) -> Vec<PathBuf> {
    paths_maps(path, vec![], is_custom_maps)
}

pub fn paths_maps(
    path: &Path,
    mut acc: Vec<PathBuf>,
    predicate: fn(path: &PathBuf) -> bool,
) -> Vec<PathBuf> {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let epath = entry.path();
        let ext = epath.extension();
        if predicate(&epath) {
            let mut child_acc = paths_maps(&epath, vec![], predicate);
            acc.append(&mut child_acc);
            // println!("dir{:?}", acc);
        } else if let Some(ext) = ext {
            if ext == "w3m" || ext == "w3x" {
                acc.push(epath);
            }
            // println!("file{:?}", acc);
        }
        // println!("{:?}",entry);
    }
    acc
}

#[cfg(test)]
mod tests_maps {
    use std::path::Path;

    use dotenv::dotenv;

    use wce_map::map::Map;
    use wce_map::GameData;

    use crate::{get_resources_path, paths_blizzard_maps, paths_custom_maps};

    #[test]
    fn test_blizzard_maps() {
        dotenv().unwrap();
        let old_dir_w3 = std::env::var("OLD_WARCRAFT_DIRECTORY").unwrap();
        let mut on_error = false;

        let game_data = &GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{e:?}"));
        let maps = paths_blizzard_maps(&Path::new(&old_dir_w3).join("Maps"));
        for map in maps {
            let path = map.into_os_string().into_string().unwrap();
            let map_res = Map::open(path.clone(), game_data);
            if let Err(err) = map_res {
                println!("Error on map '{path}' : {err:?}");
                on_error = true;
            }
            if on_error {
                panic!("Check this test logs");
            }
        }
    }

    #[test]
    fn test_custom_maps_opening_without_panic() {
        dotenv().unwrap();
        let old_dir_w3 = std::env::var("OLD_WARCRAFT_DIRECTORY").unwrap();

        let game_data = &GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{e:?}"));
        let maps = paths_custom_maps(&Path::new(&old_dir_w3).join("Maps"));
        for map in maps {
            let path = map.into_os_string().into_string().unwrap();
            println!("Opening map '{path}'");
            let _map_res = Map::open(path.clone(), game_data);
        }
    }
}
