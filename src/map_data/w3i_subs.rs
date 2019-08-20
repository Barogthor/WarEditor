use std::ffi::CString;
use crate::map_data::binary_reader::BinaryReader;

pub mod player_data {
    use super::*;

    #[derive(Debug)]
    pub struct PlayerData{
        player_id: i32,
        player_type: i32,
        player_race: i32,
        fixed_position: i32,
        player_name: CString,
        starting_pos_x: f32,
        starting_pos_y: f32,
        ally_low_priorities: i32,
        ally_high_priorities: i32,
    }

    impl Default for PlayerData{
        fn default() -> Self {
            PlayerData{
                player_id: 0,
                player_type: 0,
                player_race: 0,
                fixed_position: 0,
                player_name: Default::default(),
                starting_pos_x: 0.0,
                starting_pos_y: 0.0,
                ally_low_priorities: 0,
                ally_high_priorities: 0
            }
        }
    }

    impl PlayerData{
        pub fn read(reader: &mut BinaryReader) -> Self{
            let mut player = Self::default();
            player.player_id = reader.read_i32();
            player.player_type = reader.read_i32();
            player.player_race = reader.read_i32();
            player.fixed_position = reader.read_i32();
            player.player_name = reader.read_c_string();
            player.starting_pos_x = reader.read_f32();
            player.starting_pos_y = reader.read_f32();
            player.ally_low_priorities = reader.read_i32();
            player.ally_high_priorities = reader.read_i32();
            player
        }
    }
}
pub mod force_data{
    use super::*;

    #[derive(Debug)]
    pub struct ForceData{
        flags: i32,
        allied: bool,
        shared_victory: bool,
        shared_vision: bool,
        shared_unit_control: bool,
        shared_advanced_unit_control: bool,
        player_mask: i32,
        name: CString,
    }

    impl Default for ForceData{
        fn default() -> Self {
            ForceData{
                flags: 0,
                allied: false,
                shared_victory: false,
                shared_vision: false,
                shared_unit_control: false,
                shared_advanced_unit_control: false,
                player_mask: 0,
                name: Default::default()
            }
        }
    }

    impl ForceData{
        pub fn read(reader: &mut BinaryReader) -> Self{
            let mut force = Self::default();
            force.flags = reader.read_i32();
            force.allied = force.flags & 0x0001 == 1;
            force.shared_victory = force.flags & 0x0002 == 1;
            force.shared_vision = force.flags & 0x0004 == 1;
            force.shared_unit_control = force.flags & 0x0010 == 1;
            force.shared_advanced_unit_control = force.flags & 0x0020 == 1;
            force.player_mask = reader.read_i32();
            force.name = reader.read_c_string();
            force
        }
    }
}
pub mod upgrade_availability{
    use super::*;

    #[derive(Debug)]
    pub struct UpgradeAvailability{
        player_availability: i32,
        upgrade_id: [char;4],
        upgrade_level: i32,
        availability: i32
    }

    impl Default for UpgradeAvailability{
        fn default() -> Self {
            UpgradeAvailability{
                player_availability: 0,
                upgrade_id: ['\0';4],
                upgrade_level: 0,
                availability: 0
            }
        }
    }

    impl UpgradeAvailability{
        pub fn read(reader: &mut BinaryReader) -> Self{
            let mut upg = Self::default();
            upg.player_availability = reader.read_i32();
            let size = upg.upgrade_id.len();
            upg.upgrade_id.copy_from_slice(&reader.read_chars(size)[0..]);
            upg.upgrade_level = reader.read_i32();
            upg.availability =  reader.read_i32();
            upg
        }
    }
}
pub mod tech_availability{
    use super::*;

    #[derive(Debug)]
    pub struct TechAvailability{
        player_availability: i32,
        tech_id: [char;4],
    }

    impl Default for TechAvailability{
        fn default() -> Self {
            TechAvailability{
                player_availability: 0,
                tech_id: ['\0';4],
            }
        }
    }

    impl TechAvailability{
        pub fn read(reader: &mut BinaryReader) -> Self{
            let mut tech = Self::default();
            tech.player_availability = reader.read_i32();
            let size = tech.upgrade_id.len();
            tech.upgrade_id.copy_from_slice(&reader.read_chars(size)[0..]);
            tech
        }
    }
}

