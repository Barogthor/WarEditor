use std::ffi::{CString, CStr};
use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::map_data::{is_TFT, is_RoC};
use std::fmt::{Debug, Formatter, Error, Pointer};
use crate::map_data::binary_reader::BinaryReader;
use std::io::Read;

pub union GlobalWeather{
    value: i32,
    id: [char;4]
}

impl Debug for GlobalWeather{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"")
    }
}

#[derive(Debug)]
pub struct W3iFile{

    version: i32,
    count_saves: i32,
    editor_version: i32,
    map_name: CString,
    map_author: CString,
    map_description: CString,
    recommended_players: CString,
    camera_bounds : [f32;8],
    camera_bounds_complements: [i32;4],
    map_playable_width: i32,
    map_playable_height: i32,

    flags: i32,
    hide_minimap_preview: bool,
    modifiy_ally_priorities: bool,
    is_melee: bool,
    unknown: bool,
    mask_partial_vision: bool,
    fixed_custom_player_force: bool,
    use_custom_force: bool,
    use_custom_tree: bool,
    use_custom_abilities: bool,
    use_custom_upgrades: bool,
    unkwown_2: bool,
    show_waves_cliff_shores: bool,
    show_waves_rolling_shores: bool,
    unkwown_3: bool, // TFT
    unkwown_4: bool, // TFT
    unkwown_5: bool, // TFT

    ground_type: char,
    campaign_background: i32, // RoC
    custom_loading_screen_model_path: CString, // TFT
    loading_screen_index: i32,
    loading_screen_text: CString,
    loading_screen_title: CString,
    loading_screen_subtitle: CString,
    user_game_dataset: i32, // TFT
    prologue_screen_path: CString, // TFT
    prologue_screen_text: CString,
    prologue_screen_title: CString,
    prologue_screen_subtitle: CString,
    // TFT
    fog_style: i32,
    fog_z_height_start: f32,
    fog_z_height_end: f32,
    fog_density: f32,
    fog_red_tint: u8,
    fog_green_tint: u8,
    fog_blue_tint: u8,
    fog_alpha_value: u8,
    global_weather: i32,
    custom_sound_environment: CString,
    custom_light_environment_id: char,
    custom_water_red_tint: u8,
    custom_water_green_tint: u8,
    custom_water_blue_tint: u8,
    custom_water_alpha_tint: u8,

    max_players: i32,

}

impl Default for W3iFile{
    fn default() -> Self {
        W3iFile{
            version: 0,
            count_saves: 0,
            editor_version: 0,
            map_name: Default::default(),
            map_author: Default::default(),
            map_description: Default::default(),
            recommended_players: Default::default(),
            camera_bounds: [-1234f32;8],
            camera_bounds_complements: [-1234i32;4],
            map_playable_width: 0,
            map_playable_height: 0,
            flags: 0,
            hide_minimap_preview: false,
            modifiy_ally_priorities: false,
            is_melee: false,
            unknown: false,
            mask_partial_vision: false,
            fixed_custom_player_force: false,
            use_custom_force: false,
            use_custom_tree: false,
            use_custom_abilities: false,
            use_custom_upgrades: false,
            unkwown_2: false,
            show_waves_cliff_shores: false,
            show_waves_rolling_shores: false,
            unkwown_3: false,
            unkwown_4: false,
            unkwown_5: false,
            ground_type: '0',
            campaign_background: 0,
            custom_loading_screen_model_path: Default::default(),
            loading_screen_index: 0,
            loading_screen_text: Default::default(),
            loading_screen_title: Default::default(),
            loading_screen_subtitle: Default::default(),
            user_game_dataset: 0,
            prologue_screen_path: Default::default(),
            prologue_screen_text: Default::default(),
            prologue_screen_title: Default::default(),
            prologue_screen_subtitle: Default::default(),
            fog_style: 0,
            fog_z_height_start: 0.0,
            fog_z_height_end: 0.0,
            fog_density: 0.0,
            fog_red_tint: 0,
            fog_green_tint: 0,
            fog_blue_tint: 0,
            fog_alpha_value: 0,
            global_weather: 0,
            custom_sound_environment: Default::default(),
            custom_light_environment_id: '0',
            custom_water_red_tint: 0,
            custom_water_green_tint: 0,
            custom_water_blue_tint: 0,
            custom_water_alpha_tint: 0,
            max_players: 1
        }
    }
}

impl W3iFile{

    pub fn read_file(){
        let mut f = File::open("resources/war3map.w3i").unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer.to_owned());
        let mut w3i_file = W3iFile::default();

        w3i_file.version = reader.read_i32();
        w3i_file.count_saves = reader.read_i32();
        w3i_file.editor_version = reader.read_i32();
        w3i_file.map_name = reader.read_c_string();
        w3i_file.map_author = reader.read_c_string();
        w3i_file.map_description = reader.read_c_string();
        w3i_file.recommended_players = reader.read_c_string();

//        w3i_file.camera_bounds = [-1234f32;8];
//        w3i_file.camera_bounds_complements = [-1234i32;4];
        for x in 0..w3i_file.camera_bounds.len(){
            w3i_file.camera_bounds[x] = reader.read_f32();
        }
        for x in 0..w3i_file.camera_bounds_complements.len() {
            w3i_file.camera_bounds_complements[x] = reader.read_i32();
        }

        w3i_file.map_playable_width = reader.read_i32();
        w3i_file.map_playable_height = reader.read_i32();
        w3i_file.flags = reader.read_i32();
        w3i_file.hide_minimap_preview = w3i_file.flags & 0x0001 == 1;
        w3i_file.modifiy_ally_priorities = w3i_file.flags & 0x0002 == 1;
        w3i_file.is_melee = w3i_file.flags & 0x0004 == 1;
        w3i_file.unknown = w3i_file.flags & 0x0008 == 1;
        w3i_file.mask_partial_vision = w3i_file.flags & 0x0010 == 1;
        w3i_file.fixed_custom_player_force = w3i_file.flags & 0x0020 == 1;
        w3i_file.use_custom_force = w3i_file.flags & 0x0040 == 1;
        w3i_file.use_custom_tree = w3i_file.flags & 0x0080 == 1;
        w3i_file.use_custom_abilities = w3i_file.flags & 0x0100 == 1;
        w3i_file.use_custom_upgrades = w3i_file.flags & 0x0200 == 1;
        w3i_file.unkwown_2 = w3i_file.flags & 0x0400 == 1;
        w3i_file.show_waves_cliff_shores = w3i_file.flags & 0x0800 == 1;
        w3i_file.show_waves_rolling_shores = w3i_file.flags & 0x1000 == 1;
        w3i_file.unkwown_3 = w3i_file.flags & 0x2000 == 1;
        w3i_file.unkwown_4 = w3i_file.flags & 0x4000 == 1;
        w3i_file.unkwown_5 = w3i_file.flags & 0x8000 == 1;
        w3i_file.ground_type = reader.read_char();

        if is_RoC(w3i_file.version) {
            w3i_file.campaign_background = reader.read_i32();
            w3i_file.loading_screen_text = reader.read_c_string();
            w3i_file.loading_screen_title = reader.read_c_string();
            w3i_file.loading_screen_subtitle = reader.read_c_string();
            w3i_file.loading_screen_index = reader.read_i32();
            w3i_file.prologue_screen_text = reader.read_c_string();
            w3i_file.prologue_screen_title = reader.read_c_string();
            w3i_file.prologue_screen_subtitle = reader.read_c_string();
        }
        else if is_TFT(w3i_file.version) {
            w3i_file.loading_screen_index = reader.read_i32();
            w3i_file.custom_loading_screen_model_path = reader.read_c_string();
            w3i_file.loading_screen_text = reader.read_c_string();
            w3i_file.loading_screen_title = reader.read_c_string();
            w3i_file.loading_screen_subtitle = reader.read_c_string();
            w3i_file.user_game_dataset = reader.read_i32();
            w3i_file.prologue_screen_path = reader.read_c_string();
            w3i_file.prologue_screen_text = reader.read_c_string();
            w3i_file.prologue_screen_title = reader.read_c_string();
            w3i_file.prologue_screen_subtitle = reader.read_c_string();
            w3i_file.fog_style = reader.read_i32();
            w3i_file.fog_z_height_start = reader.read_f32();
            w3i_file.fog_z_height_end = reader.read_f32();
            w3i_file.fog_density = reader.read_f32();
            w3i_file.fog_red_tint = reader.read_u8();
            w3i_file.fog_green_tint = reader.read_u8();
            w3i_file.fog_blue_tint = reader.read_u8();
            w3i_file.fog_alpha_value = reader.read_u8();
//        let mut gw = GlobalWeather{value: reader.read_i32(;
            w3i_file.global_weather = reader.read_i32();
            w3i_file.custom_sound_environment= reader.read_c_string();
            w3i_file.custom_light_environment_id = reader.read_char();
            w3i_file.custom_water_red_tint = reader.read_u8();
            w3i_file.custom_water_green_tint = reader.read_u8();
            w3i_file.custom_water_blue_tint = reader.read_u8();
            w3i_file.custom_water_alpha_tint = reader.read_u8();

        }
        w3i_file.max_players = reader.read_i32();

        println!("{:#?}",w3i_file);

    }
}

