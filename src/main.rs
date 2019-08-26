
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate getset;

use war_editor::map_data::w3i_file::W3iFile;
use war_editor::map_data::mmp_file::MMPFile;
use war_editor::map_data::region_file::RegionFile;
use war_editor::map_data::camera_file::CameraFile;
use war_editor::map_data::pathmap_file::PathMapFile;
use war_editor::map_data::shadowmap_file::ShadowMapFile;
use war_editor::map_data::environment_file::EnvironmentFile;
use war_editor::slk::document::SLKDocument;
use war_editor::slk::merge_slk;
use war_editor::map_data::minimap_file::MinimapFile;
use war_editor::map_data::trigger_string_file::TriggerStringFile;
use std::time::Instant;
use war_editor::map_data::custom_text_trigger_file::CustomTextTriggerFile;


fn elapsed_time(instant: &Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {}:{}:{}::{}", hours, mins, seconds, millis);
}

fn main() {

    println!("Hello, world!");
    let mut slk_reader = SLKReader::open_file("resources/slk/test.slk".to_string());
    let document = slk_reader.parse().unwrap();
//    document.debug();
    let mut lines = document.get_cells_value_sorted_by_line();
    let mut slk_reader = SLKReader::open_file("resources/slk/test_2.slk".to_string());
    let document = slk_reader.parse().unwrap();
    merge_slk(&mut lines, &document);
    println!("{:#?}", lines);


    let now = Instant::now();
    println!("Hello, world!");
//    println!("size rgba: {}",size_of_val(&vec![0u8,0u8,0u8,0u8][0..]));
//    println!("{:X}, {:X}", true as u8, false as u8);
//    let rgba = RGBA::by_value(0xFF5C15FF);
//    rgba.debug();
//    println!("{:X} {:X} {:X}", rgba.red(),rgba.green(), rgba.blue());
    let _w3i = W3iFile::read_file();
//    w3i.debug();
//    println!("size w3i: {}",size_of_val(&w3i));
//    let mut slk_reader = SLKReader::open_file("resources/slk/Cliffs.slk".to_string());
//    let record = slk_reader.parse();
//    println!("{:#?}",record);
    let _mmp = MMPFile::read_file();
//    mmp.debug();
    let _regions = RegionFile::read_file();
//    regions.debug();
    let _cameras = CameraFile::read_file();
//    cameras.debug();
//    let sounds = SoundFile::read_file();
//    sounds.debug();
    let _pathing = PathMapFile::read_file();
//    pathing.debug();
    let _shaders = ShadowMapFile::read_file();
//    shaders.debug();
    let _environment = EnvironmentFile::read_file();
//    environment.debug();
    let _mmap = MinimapFile::read_file();
//    mmap.debug();
    let _trigstrs = TriggerStringFile::read_file();
//    trigstrs.debug();
    let _triggers_ct = CustomTextTriggerFile::read_file();
//    triggers_ct.debug();

    elapsed_time(&now);
}
