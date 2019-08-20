use std::ffi::CString;
use crate::map_data::binary_reader::BinaryReader;
use crate::map_data::binary_reader::BinaryConverter;

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

    impl BinaryConverter for PlayerData{
        fn read(reader: &mut BinaryReader) -> Self{
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

    impl BinaryConverter for ForceData{
        fn read(reader: &mut BinaryReader) -> Self{
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
        upgrade_id: String,
        upgrade_level: i32,
        availability: i32
    }

    impl Default for UpgradeAvailability{
        fn default() -> Self {
            UpgradeAvailability{
                player_availability: 0,
                upgrade_id: Default::default(),
                upgrade_level: 0,
                availability: 0
            }
        }
    }

    impl BinaryConverter for UpgradeAvailability{
        fn read(reader: &mut BinaryReader) -> Self{
            let mut upg = Self::default();
            upg.player_availability = reader.read_i32();
            upg.upgrade_id = String::from_utf8(reader.read_bytes(4)).unwrap();
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
        tech_id: String,
    }

    impl Default for TechAvailability{
        fn default() -> Self {
            TechAvailability{
                player_availability: 0,
                tech_id: Default::default(),
            }
        }
    }

    impl BinaryConverter for TechAvailability{
        fn read(reader: &mut BinaryReader) -> Self{
            let mut tech = Self::default();
            tech.player_availability = reader.read_i32();
            tech.tech_id = String::from_utf8(reader.read_bytes(4)).unwrap();
            tech
        }
    }
}
pub mod random_unit_table {
    use super::*;
    use crate::map_data::binary_reader::BinaryConverter;

    #[derive(Debug)]
    pub struct RandomUnitSet{
        chance: i32,
        ids: Vec<u8>,
    }

    impl Default for RandomUnitSet{
        fn default() -> Self {
            RandomUnitSet{
                chance: 0,
                ids: vec![]
            }
        }
    }

    #[derive(Debug)]
    pub struct RandomUnitTable{
        id: i32,
        name: CString,
        position_types: Vec<i32>,
        sets: Vec<RandomUnitSet>,
    }

    impl Default for RandomUnitTable{
        fn default() -> Self {
            RandomUnitTable{
                id: 0,
                name: Default::default(),
                position_types: vec![],
                sets: vec![]
            }
        }
    }
    impl BinaryConverter for RandomUnitTable{
        fn read(reader: &mut BinaryReader) -> Self {
            let mut table = Self::default();
            table.id = reader.read_i32();
            table.name = reader.read_c_string();
            let count_pos = reader.read_i32() as usize;
            table.position_types = reader.read_vec_i32(count_pos);
            for _i in 0..count_pos{
                let mut set = RandomUnitSet::default();
                set.chance = reader.read_i32();
                set.ids = reader.read_bytes( count_pos*4);
                table.sets.push(set);
            }
            table
        }
    }
}
pub mod random_item_table{
    use super::*;

    #[derive(Debug)]
    pub struct RandomItemSet{
        items: Vec<(i32,String)>
    }

    impl Default for RandomItemSet{
        fn default() -> Self {
            RandomItemSet{
                items: vec![]
            }
        }
    }
    impl BinaryConverter for RandomItemSet{
        fn read(reader: &mut BinaryReader) -> Self {
            let mut set = Self::default();
            let count_items = reader.read_i32();
            for _i in 0..count_items{
                let chance = reader.read_i32();
                let id = String::from_utf8(reader.read_bytes(4)).unwrap();
                set.items.push((chance, id));
            }
            set
        }
    }

    #[derive(Debug)]
    pub struct RandomItemTable{
        id: i32,
        name: CString,
        sets: Vec<RandomItemSet>,
    }

    impl Default for RandomItemTable{
        fn default() -> Self {
            RandomItemTable{
                id: 0,
                name: Default::default(),
                sets: vec![]
            }
        }
    }

    impl BinaryConverter for RandomItemTable{
        fn read(reader: &mut BinaryReader) -> Self {
            let mut table = Self::default();
            table.id = reader.read_i32();
            table.name = reader.read_c_string();
            let count_sets = reader.read_i32() as usize;
            table.sets = reader.read_vec::<RandomItemSet>(count_sets);
            table
        }
    }
}
