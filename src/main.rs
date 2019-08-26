#[macro_use] extern crate lazy_static;
#[macro_use] extern crate getset;

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
use regex::Regex;
use war_editor::slk::document::SLKDocument;
use war_editor::slk::merge_slk;


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
}
