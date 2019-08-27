use war_editor::map_data::w3i_file::W3iFile;
use war_editor::map_data::mmp_file::MMPFile;
use war_editor::map_data::region_file::RegionFile;
use war_editor::map_data::camera_file::CameraFile;
use war_editor::map_data::pathmap_file::PathMapFile;
use war_editor::map_data::shadowmap_file::ShadowMapFile;
use war_editor::map_data::environment_file::EnvironmentFile;
use war_editor::map_data::minimap_file::MinimapFile;
use war_editor::map_data::trigger_string_file::TriggerStringFile;
use std::time::Instant;
use war_editor::map_data::custom_text_trigger_file::CustomTextTriggerFile;
use mpq::Archive;
use war_editor::map_data::concat_path;
use war_editor::map_data::sound_file::SoundFile;
use war_editor::slk::slk::SLKReader;
use war_editor::map_data::map::Map;

fn elapsed_time(instant: &Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {}:{}:{}::{}", hours, mins, seconds, millis);
}


fn main() {
    let now = Instant::now();


    println!("Hello, world!");
//    let mut mpq = Archive::open("resources/sample_1/Test.w3x").unwrap();
    let mut sample_1 = "resources/sample_1/Test.w3x".to_string();
    let mut sample_2 = "resources/sample_2/Remake1 - Copie.w3x".to_string();
    let mut Map = Map::open(sample_2);
//    println!("size rgba: {}",size_of_val(&vec![0u8,0u8,0u8,0u8][0..]));
//    println!("{:X}, {:X}", true as u8, false as u8);
//    let rgba = RGBA::by_value(0xFF5C15FF);
//    rgba.debug();
//    println!("{:X} {:X} {:X}", rgba.red(),rgba.green(), rgba.blue());




    elapsed_time(&now);
}
