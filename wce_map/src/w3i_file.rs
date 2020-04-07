#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use std::fmt::Debug;
use mpq::Archive;
use crate::globals::{GameVersion, MAP_INFOS};
use crate::globals::GameVersion::{RoC, TFT, TFT131};
use crate::binary_reader::{BinaryConverter, BinaryReader};
use crate::binary_writer::BinaryWriter;

#[derive(Debug, PartialEq)]
struct PlayerData{
    player_id: i32,
    player_type: i32,
    player_race: i32,
    fixed_position: i32,
    player_name: String,
    starting_pos_x: f32,
    starting_pos_y: f32,
    ally_low_priorities: i32,
    ally_high_priorities: i32,
}

impl BinaryConverter for PlayerData{
    fn read(reader: &mut BinaryReader) -> Self{
        let player_id = reader.read_i32();
        let player_type = reader.read_i32();
        let player_race = reader.read_i32();
        let fixed_position = reader.read_i32();
        let player_name = reader.read_c_string().into_string().unwrap();
        let starting_pos_x = reader.read_f32();
        let starting_pos_y = reader.read_f32();
        let ally_low_priorities = reader.read_i32();
        let ally_high_priorities = reader.read_i32();
        PlayerData{
            player_id,
            player_type,
            player_race,
            fixed_position,
            player_name,
            starting_pos_x,
            starting_pos_y,
            ally_low_priorities,
            ally_high_priorities
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
struct ForceData{
    flags: i32,
    allied: bool,
    shared_victory: bool,
    shared_vision: bool,
    shared_unit_control: bool,
    shared_advanced_unit_control: bool,
    player_mask: i32,
    name: String,
}

impl BinaryConverter for ForceData{
    fn read(reader: &mut BinaryReader) -> Self{
        let flags = reader.read_i32();
        let allied = flags & 0x0001 == 1;
        let shared_victory = flags & 0x0002 == 1;
        let shared_vision = flags & 0x0004 == 1;
        let shared_unit_control = flags & 0x0010 == 1;
        let shared_advanced_unit_control = flags & 0x0020 == 1;
        let player_mask = reader.read_i32();
        let name = reader.read_c_string().into_string().unwrap();
        ForceData{
            flags,
            allied,
            shared_victory,
            shared_vision,
            shared_unit_control,
            shared_advanced_unit_control,
            player_mask,
            name
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
struct UpgradeAvailability{
    player_availability: i32,
    upgrade_id: String,
    upgrade_level: i32,
    availability: i32
}

impl BinaryConverter for UpgradeAvailability{
    fn read(reader: &mut BinaryReader) -> Self{
        let player_availability = reader.read_i32();
        let upgrade_id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let upgrade_level = reader.read_i32();
        let availability =  reader.read_i32();
        UpgradeAvailability{
            player_availability,
            upgrade_id,
            upgrade_level,
            availability
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
struct TechAvailability{
    player_availability: i32,
    tech_id: String,
}

impl BinaryConverter for TechAvailability{
    fn read(reader: &mut BinaryReader) -> Self{
        let player_availability = reader.read_i32();
        let tech_id = String::from_utf8(reader.read_bytes(4)).unwrap();
        TechAvailability{
            player_availability,
            tech_id
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
struct RandomUnitSet{
    chance: u32,
    ids: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum RandomTablePositionType {
    Unit,
    Building,
    Item,
}

impl RandomTablePositionType {
    pub fn from(n : u32) -> Result<Self, String>{
        match n {
            0 => Ok(RandomTablePositionType::Unit),
            1 => Ok(RandomTablePositionType::Building),
            2 => Ok(RandomTablePositionType::Item),
            _ => Err(format!("Unknown position type: {}",n))
        }
    }
}

#[derive(Debug, PartialEq)]
struct RandomUnitTable{
    id: i32,
    name: String,
    position_types: Vec<RandomTablePositionType>,
    sets: Vec<RandomUnitSet>,
}

impl BinaryConverter for RandomUnitTable{
    fn read(reader: &mut BinaryReader) -> Self {
        let id = reader.read_i32();
        let name = reader.read_c_string().into_string().unwrap();
        let count_pos = reader.read_i32() as usize;
        let mut position_types = vec![];
        for _ in 0..count_pos{
            position_types.push(RandomTablePositionType::from(reader.read_u32()).unwrap())
        }
        let mut sets = vec![];
        let count_lines = reader.read_u32();
        for _ in 0..count_lines{
            let chance = reader.read_u32();
            let mut ids = vec![];
            for _ in 0..count_pos{
                ids.push(reader.read_string_utf8(4));
            }
            sets.push(RandomUnitSet{
                chance,
                ids
            });
        }
        RandomUnitTable{
            id,
            name,
            position_types,
            sets
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
struct RandomItemSet{
    items: Vec<(u32,String)>
}


impl BinaryConverter for RandomItemSet{
    fn read(reader: &mut BinaryReader) -> Self {
        let count_items = reader.read_i32();
        let mut items = vec![];
        for _ in 0..count_items{
            let chance = reader.read_u32();
            let id = String::from_utf8(reader.read_bytes(4)).unwrap();
            items.push((chance, id));
        }
        RandomItemSet{
            items
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
pub struct RandomItemTable{
    id: i32,
    name: String,
    sets: Vec<RandomItemSet>,
}

impl BinaryConverter for RandomItemTable{
    fn read(reader: &mut BinaryReader) -> Self {
        let id = reader.read_i32();
        let name = reader.read_c_string().into_string().unwrap();
        let count_sets = reader.read_i32() as usize;
        let sets = reader.read_vec::<RandomItemSet>(count_sets);
        RandomItemTable{
            id,
            name,
            sets
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


#[derive(Derivative)]
#[derivative(Debug, Default, PartialEq)]
pub struct W3iFile{

    version: GameVersion,
    #[derivative(PartialEq="ignore")]
    count_saves: i32,
    #[derivative(PartialEq="ignore")]
    editor_version: i32,
    map_name: String,
    map_author: String,
    map_description: String,
    recommended_players: String,
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
    custom_loading_screen_model_path: String, // TFT
    loading_screen_index: i32,
    loading_screen_text: String,
    loading_screen_title: String,
    loading_screen_subtitle: String,
    user_game_dataset: i32, // TFT
    prologue_screen_path: String, // TFT
    prologue_screen_text: String,
    prologue_screen_title: String,
    prologue_screen_subtitle: String,
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
    custom_sound_environment: String,
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

impl W3iFile{

    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_INFOS).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<W3iFile>()
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for W3iFile{
    fn read(reader: &mut BinaryReader) -> Self {
        let mut w3i = W3iFile::default();

        let version = reader.read_u32();
        w3i.version = to_game_version(version);
        w3i.count_saves = reader.read_i32();
        w3i.editor_version = reader.read_i32();
        w3i.map_name = reader.read_c_string().into_string().unwrap();
        w3i.map_author = reader.read_c_string().into_string().unwrap();
        w3i.map_description = reader.read_c_string().into_string().unwrap();
        w3i.recommended_players = reader.read_c_string().into_string().unwrap();
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

        match w3i.version{
            RoC => {
                w3i.campaign_background = reader.read_i32();
                w3i.loading_screen_text = reader.read_c_string().into_string().unwrap();
                w3i.loading_screen_title = reader.read_c_string().into_string().unwrap();
                w3i.loading_screen_subtitle = reader.read_c_string().into_string().unwrap();
                w3i.loading_screen_index = reader.read_i32();
                w3i.prologue_screen_text = reader.read_c_string().into_string().unwrap();
                w3i.prologue_screen_title = reader.read_c_string().into_string().unwrap();
                w3i.prologue_screen_subtitle = reader.read_c_string().into_string().unwrap();
            },
            _ => {
                w3i.loading_screen_index = reader.read_i32();
                w3i.custom_loading_screen_model_path = reader.read_c_string().into_string().unwrap();
                w3i.loading_screen_text = reader.read_c_string().into_string().unwrap();
                w3i.loading_screen_title = reader.read_c_string().into_string().unwrap();
                w3i.loading_screen_subtitle = reader.read_c_string().into_string().unwrap();
                w3i.user_game_dataset = reader.read_i32();
                w3i.prologue_screen_path = reader.read_c_string().into_string().unwrap();
                w3i.prologue_screen_text = reader.read_c_string().into_string().unwrap();
                w3i.prologue_screen_title = reader.read_c_string().into_string().unwrap();
                w3i.prologue_screen_subtitle = reader.read_c_string().into_string().unwrap();
                w3i.fog_style = reader.read_i32();
                w3i.fog_z_height_start = reader.read_f32();
                w3i.fog_z_height_end = reader.read_f32();
                w3i.fog_density = reader.read_f32();
                w3i.fog_red_tint = reader.read_u8();
                w3i.fog_green_tint = reader.read_u8();
                w3i.fog_blue_tint = reader.read_u8();
                w3i.fog_alpha_value = reader.read_u8();
                w3i.global_weather = reader.read_i32();
                w3i.custom_sound_environment= reader.read_c_string().into_string().unwrap();
                w3i.custom_light_environment_id = reader.read_char();
                w3i.custom_water_red_tint = reader.read_u8();
                w3i.custom_water_green_tint = reader.read_u8();
                w3i.custom_water_blue_tint = reader.read_u8();
                w3i.custom_water_alpha_tint = reader.read_u8();
            }
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
        if w3i.version.is_tft(){
            let random_item_table_count = reader.read_u32() as usize;
            w3i.random_item_tables = reader.read_vec::<RandomItemTable>(random_item_table_count);
        }
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_INFOS, reader.size() - reader.pos() as usize);
        w3i
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

fn to_game_version(value: u32) -> GameVersion{
    match value{
        18 => RoC,
        25 => TFT,
        28 => TFT131,
        _ => panic!("Unknown or unsupported game version '{}'", value)
    }
}

#[cfg(test)]
mod w3i_tests{
    use std::fs::File;
    use crate::globals::GameVersion::{RoC, TFT};
    use crate::binary_reader::BinaryReader;
    use crate::w3i_file::{W3iFile, PlayerData, ForceData, RandomUnitTable, RandomTablePositionType, RandomUnitSet, RandomItemTable, RandomItemSet};

    fn get_roc_mock() -> W3iFile{
        W3iFile{
            version: RoC,
            count_saves: 0,
            editor_version: 0,
            map_name: "TRIGSTR_001".to_string(),
            map_author: "TRIGSTR_004".to_string(),
            map_description: "TRIGSTR_003".to_string(),
            recommended_players: "TRIGSTR_002".to_string(),
            camera_bounds: vec![ -1152.0, -1408.0, 1152.0, 1408.0, -1152.0, 1408.0, 1152.0, -1408.0],
            camera_bounds_complements: vec![3,3,3,3],
            map_playable_width: 26,
            map_playable_height: 26,
            flags: 39952,
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
            ground_type: 'L',
            campaign_background: -1,
            custom_loading_screen_model_path: "".to_string(),
            loading_screen_index: 0,
            loading_screen_text: "".to_string(),
            loading_screen_title: "".to_string(),
            loading_screen_subtitle: "".to_string(),
            user_game_dataset: 0,
            prologue_screen_path: "".to_string(),
            prologue_screen_text: "".to_string(),
            prologue_screen_title: "".to_string(),
            prologue_screen_subtitle: "".to_string(),
            fog_style: 0,
            fog_z_height_start: 0.0,
            fog_z_height_end: 0.0,
            fog_density: 0.0,
            fog_red_tint: 0,
            fog_green_tint: 0,
            fog_blue_tint: 0,
            fog_alpha_value: 0,
            global_weather: 0,
            custom_sound_environment: "".to_string(),
            custom_light_environment_id: '\0',
            custom_water_red_tint: 0,
            custom_water_green_tint: 0,
            custom_water_blue_tint: 0,
            custom_water_alpha_tint: 0,
            players: vec![PlayerData{
                player_id: 0,
                player_type: 1,
                player_race: 1,
                fixed_position: 0,
                player_name: "TRIGSTR_005".to_string(),
                starting_pos_x: 0.0,
                starting_pos_y: 256.0,
                ally_low_priorities: 0,
                ally_high_priorities: 2
            }, PlayerData{
                player_id: 1,
                player_type: 1,
                player_race: 2,
                fixed_position: 0,
                player_name: "TRIGSTR_006".to_string(),
                starting_pos_x: -1280.0,
                starting_pos_y: -1280.0,
                ally_low_priorities: 0,
                ally_high_priorities: 1
            }],
            forces: vec![ForceData{
                flags: 0,
                allied: false,
                shared_victory: false,
                shared_vision: false,
                shared_unit_control: false,
                shared_advanced_unit_control: false,
                player_mask: -1,
                name: "TRIGSTR_007".to_string()
            }],
            upgrades: vec![],
            techs: vec![],
            random_unit_tables: vec![RandomUnitTable{
                id: 0,
                name: "TestRandGroupUnit".to_string(),
                position_types: vec![RandomTablePositionType::Unit, RandomTablePositionType::Unit],
                sets: vec![RandomUnitSet{ chance: 95, ids: vec!["YYU6".to_string(), "YYU:".to_string()] }, RandomUnitSet{ chance: 5, ids: vec!["nrwm".to_string(), "\0\0\0\0".to_string()] }]
            },
                                     RandomUnitTable{
                                         id: 1,
                                         name: "TestRandGroupItem".to_string(),
                                         position_types: vec![RandomTablePositionType::Item, RandomTablePositionType::Item],
                                         sets: vec![RandomUnitSet{ chance: 50, ids: vec!["YjI2".to_string(), "desc".to_string()] }, RandomUnitSet{ chance: 50, ids: vec!["ofro".to_string(), "desc".to_string()] }]
                                     }],
            random_item_tables: vec![]
        }
    }

    fn get_tft_mock() -> W3iFile{
        W3iFile{
            version: TFT,
            count_saves: 0,
            editor_version: 0,
            map_name: "TRIGSTR_001".to_string(),
            map_author: "TRIGSTR_004".to_string(),
            map_description: "TRIGSTR_003".to_string(),
            recommended_players: "TRIGSTR_002".to_string(),
            camera_bounds: vec![-1152.0, -1408.0, 1152.0, 1408.0, -1152.0, 1408.0, 1152.0, -1408.0],
            camera_bounds_complements: vec![3, 3, 3, 3],
            map_playable_width: 26,
            map_playable_height: 26,
            flags: 56336,
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
            ground_type: 'L',
            campaign_background: 0,
            custom_loading_screen_model_path: "".to_string(),
            loading_screen_index: -1,
            loading_screen_text: "".to_string(),
            loading_screen_title: "".to_string(),
            loading_screen_subtitle: "".to_string(),
            user_game_dataset: 0,
            prologue_screen_path: "".to_string(),
            prologue_screen_text: "".to_string(),
            prologue_screen_title: "".to_string(),
            prologue_screen_subtitle: "".to_string(),
            fog_style: 0,
            fog_z_height_start: 3000.0,
            fog_z_height_end: 5000.0,
            fog_density: 0.5,
            fog_red_tint: 0,
            fog_green_tint: 0,
            fog_blue_tint: 0,
            fog_alpha_value: 255,
            global_weather: 0,
            custom_sound_environment: "".to_string(),
            custom_light_environment_id: '\0',
            custom_water_red_tint: 255,
            custom_water_green_tint: 255,
            custom_water_blue_tint: 255,
            custom_water_alpha_tint: 255,
            players: vec![PlayerData{
                player_id: 0,
                player_type: 1,
                player_race: 1,
                fixed_position: 0,
                player_name: "TRIGSTR_005".to_string(),
                starting_pos_x: 0.0,
                starting_pos_y: 256.0,
                ally_low_priorities: 0,
                ally_high_priorities: 2
            }, PlayerData{
                player_id: 1,
                player_type: 1,
                player_race: 2,
                fixed_position: 0,
                player_name: "TRIGSTR_006".to_string(),
                starting_pos_x: -1280.0,
                starting_pos_y: -1280.0,
                ally_low_priorities: 0,
                ally_high_priorities: 1
            }],
            forces: vec![ForceData{
                flags: 0,
                allied: false,
                shared_victory: false,
                shared_vision: false,
                shared_unit_control: false,
                shared_advanced_unit_control: false,
                player_mask: -1,
                name: "TRIGSTR_007".to_string()
            }],
            upgrades: vec![],
            techs: vec![],
            random_unit_tables: vec![],
            random_item_tables: vec![RandomItemTable{
                id: 0,
                name: "TestItemTable".to_string(),
                sets: vec![RandomItemSet{ items: vec![(100, "modt".to_string())] }, RandomItemSet{ items: vec![(50, "YkI2".to_string()), (50, "YjI2".to_string())] }]
            }]
        }
    }

    #[test]
    fn w3i_roc_test(){
        let mut w3i = File::open("../resources/Scenario/Sandbox_roc/war3map.w3i").unwrap();
        let mut reader = BinaryReader::from(&mut w3i);
        let w3i = reader.read::<W3iFile>();
        let mock_w3i = get_roc_mock();
        assert_eq!(w3i, mock_w3i);
    }
    #[test]
    fn w3i_tft_test(){
        let mut w3i = File::open("../resources/Scenario/Sandbox_tft/war3map.w3i").unwrap();
        let mut reader = BinaryReader::from(&mut w3i);
        let w3i = reader.read::<W3iFile>();
        let mock_w3i = get_tft_mock();
        assert_eq!(w3i, mock_w3i);
    }
}