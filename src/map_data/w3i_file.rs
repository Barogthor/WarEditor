use std::ffi::{CString};
use std::fs::File;
use crate::map_data::{is_TFT, is_RoC};
use std::fmt::{Debug, Formatter, Error};
use crate::map_data::binary_reader::BinaryReader;
use std::io::Read;
use crate::map_data::w3i_subs::player_data::PlayerData;
use crate::map_data::w3i_subs::force_data::ForceData;
use crate::map_data::w3i_subs::upgrade_availability::UpgradeAvailability;
use crate::map_data::w3i_subs::tech_availability::TechAvailability;
use crate::map_data::w3i_subs::random_unit_table::RandomUnitTable;
use crate::map_data::w3i_subs::random_item_table::RandomItemTable;

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
    camera_bounds : Vec<f32>, // 8
    camera_bounds_complements: Vec<i32>, // 4
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

    players: Vec<PlayerData>,
    forces: Vec<ForceData>,
    upgrades: Vec<UpgradeAvailability>,
    techs: Vec<TechAvailability>,
    random_unit_tables: Vec<RandomUnitTable>,
    random_item_tables: Vec<RandomItemTable>

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
            camera_bounds: vec![],
            camera_bounds_complements: vec![],
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
            players: vec![],
            forces: vec![],
            upgrades: vec![],
            techs: vec![],
            random_unit_tables: vec![],
            random_item_tables: vec![]
        }
    }
}

impl W3iFile{

    pub fn read_file() -> Self{
        let mut f = File::open("resources/war3map.w3i").unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        let mut w3i = W3iFile::default();

        w3i.version = reader.read_i32();
        w3i.count_saves = reader.read_i32();
        w3i.editor_version = reader.read_i32();
        w3i.map_name = reader.read_c_string();
        w3i.map_author = reader.read_c_string();
        w3i.map_description = reader.read_c_string();
        w3i.recommended_players = reader.read_c_string();
        w3i.camera_bounds = reader.read_vec_f32(8);
        w3i.camera_bounds_complements = reader.read_vec_i32(4);
        w3i.map_playable_width = reader.read_i32();
        w3i.map_playable_height = reader.read_i32();
        w3i.flags = reader.read_i32();
        w3i.hide_minimap_preview = w3i.flags & 0x0001 == 1;
        w3i.modifiy_ally_priorities = w3i.flags & 0x0002 == 1;
        w3i.is_melee = w3i.flags & 0x0004 == 1;
        w3i.unknown = w3i.flags & 0x0008 == 1;
        w3i.mask_partial_vision = w3i.flags & 0x0010 == 1;
        w3i.fixed_custom_player_force = w3i.flags & 0x0020 == 1;
        w3i.use_custom_force = w3i.flags & 0x0040 == 1;
        w3i.use_custom_tree = w3i.flags & 0x0080 == 1;
        w3i.use_custom_abilities = w3i.flags & 0x0100 == 1;
        w3i.use_custom_upgrades = w3i.flags & 0x0200 == 1;
        w3i.unkwown_2 = w3i.flags & 0x0400 == 1;
        w3i.show_waves_cliff_shores = w3i.flags & 0x0800 == 1;
        w3i.show_waves_rolling_shores = w3i.flags & 0x1000 == 1;
        w3i.unkwown_3 = w3i.flags & 0x2000 == 1;
        w3i.unkwown_4 = w3i.flags & 0x4000 == 1;
        w3i.unkwown_5 = w3i.flags & 0x8000 == 1;
        w3i.ground_type = reader.read_char();

        if is_RoC(w3i.version) {
            w3i.campaign_background = reader.read_i32();
            w3i.loading_screen_text = reader.read_c_string();
            w3i.loading_screen_title = reader.read_c_string();
            w3i.loading_screen_subtitle = reader.read_c_string();
            w3i.loading_screen_index = reader.read_i32();
            w3i.prologue_screen_text = reader.read_c_string();
            w3i.prologue_screen_title = reader.read_c_string();
            w3i.prologue_screen_subtitle = reader.read_c_string();
        }
        else if is_TFT(w3i.version) {
            w3i.loading_screen_index = reader.read_i32();
            w3i.custom_loading_screen_model_path = reader.read_c_string();
            w3i.loading_screen_text = reader.read_c_string();
            w3i.loading_screen_title = reader.read_c_string();
            w3i.loading_screen_subtitle = reader.read_c_string();
            w3i.user_game_dataset = reader.read_i32();
            w3i.prologue_screen_path = reader.read_c_string();
            w3i.prologue_screen_text = reader.read_c_string();
            w3i.prologue_screen_title = reader.read_c_string();
            w3i.prologue_screen_subtitle = reader.read_c_string();
            w3i.fog_style = reader.read_i32();
            w3i.fog_z_height_start = reader.read_f32();
            w3i.fog_z_height_end = reader.read_f32();
            w3i.fog_density = reader.read_f32();
            w3i.fog_red_tint = reader.read_u8();
            w3i.fog_green_tint = reader.read_u8();
            w3i.fog_blue_tint = reader.read_u8();
            w3i.fog_alpha_value = reader.read_u8();
//        let mut gw = GlobalWeather{value: reader.read_i32(;
            w3i.global_weather = reader.read_i32();
            w3i.custom_sound_environment= reader.read_c_string();
            w3i.custom_light_environment_id = reader.read_char();
            w3i.custom_water_red_tint = reader.read_u8();
            w3i.custom_water_green_tint = reader.read_u8();
            w3i.custom_water_blue_tint = reader.read_u8();
            w3i.custom_water_alpha_tint = reader.read_u8();

        }
        let max_players = reader.read_u32() as usize;
        w3i.players = reader.read_vec::<PlayerData>(max_players);
        let max_forces = reader.read_u32() as usize;
        w3i.forces = reader.read_vec::<ForceData>(max_forces);
        let upgrade_count = reader.read_u32() as usize;
        w3i.upgrades = reader.read_vec::<UpgradeAvailability>(upgrade_count);
        let tech_count = reader.read_u32()  as usize;
        w3i.techs = reader.read_vec::<TechAvailability>(tech_count);
        let random_unit_table_count = reader.read_u32() as usize;
        w3i.random_unit_tables = reader.read_vec::<RandomUnitTable>(random_unit_table_count);
        let random_item_table_count = reader.read_u32() as usize;
        w3i.random_item_tables = reader.read_vec::<RandomItemTable>(random_item_table_count);

        println!("{:#?}",w3i);
        println!("pos: {}, size: {}", reader.pos(), buffer_size);
        w3i
    }
}

