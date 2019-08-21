#[macro_use] extern crate lazy_static;
extern crate regex;

use war_editor::map_data::w3i_file::W3iFile;
use std::mem::size_of_val;
use war_editor::slk::slk::SLKReader;
use war_editor::map_data::mmp_file::MMPFile;
use war_editor::map_data::region_file::RegionFile;
use war_editor::map_data::camera_file::{CameraFile};
use war_editor::map_data::sound_file::SoundFile;
use war_editor::map_data::pathmap_file::PathMapFile;
use war_editor::map_data::shadowmap_file::ShadowMapFile;
use war_editor::map_data::environment_file::EnvironmentFile;


fn main() {
    println!("Hello, world!");
    let w3i = W3iFile::read_file();
    w3i.debug();
//    println!("size w3i: {}",size_of_val(&w3i));
//    let mut slk_reader = SLKReader::open_file("resources/slk/Cliffs.slk".to_string());
//    let record = slk_reader.parse();
//    println!("{:#?}",record);
    let mmp = MMPFile::read_file();
//    mmp.debug();
    let regions = RegionFile::read_file();
//    regions.debug();
    let cameras = CameraFile::read_file();
//    cameras.debug();
//    let sounds = SoundFile::read_file();
//    sounds.debug();
    let pathing = PathMapFile::read_file();
//    pathing.debug();
    let shaders = ShadowMapFile::read_file();
//    shaders.debug();
    let environment = EnvironmentFile::read_file();
    environment.debug();
}
