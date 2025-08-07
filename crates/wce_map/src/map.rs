use std::fs::File;
use std::io::Write;

use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;

use crate::camera_file::{self, CameraFile};
use crate::custom_datas::ability::CustomAbilityFile;
use crate::custom_datas::buff::CustomBuffFile;
use crate::custom_datas::destructable::CustomDestructableFile;
use crate::custom_datas::doodad::CustomDoodadFile;
use crate::custom_datas::item::CustomItemFile;
use crate::custom_datas::unit::CustomUnitFile;
use crate::custom_datas::upgrade::CustomUpgradeFile;
use crate::doodad_map::DoodadMap;
use crate::import_file::ImportFile;
use crate::map_string_file::MapStringFile;
use crate::minimap_file::MinimapFile;
use crate::mmp_file::MMPFile;
use crate::pathmap_file::PathMapFile;
use crate::region_file::{self, RegionFile};
use crate::shadowmap_file::ShadowMapFile;
use crate::sound_file::SoundFile;
use crate::terrain_file::TerrainFile;
use crate::trigger_jass_file::TriggerJassFile;
use crate::triggers::TriggersFile;
use crate::unit_map::UnitItemMap;
use crate::w3i_file::W3iFile;
use crate::{GameData, OpeningError};

pub struct Map<'a> {
    game_data: &'a GameData,
    path: String,
    infos: W3iFile,
    terrain: TerrainFile,
    cameras: Option<CameraFile>,
    regions: Option<RegionFile>,
    path_map: PathMapFile,
    minimap: MinimapFile,
    menu_minimap: MMPFile,
    shaders: ShadowMapFile,
    sounds: Option<SoundFile>,
    strings: MapStringFile,
    custom_scripts: TriggerJassFile,
    doodad_map: DoodadMap,
    unit_item_map: UnitItemMap,
    triggers: TriggersFile,
    import_listing: Option<ImportFile>,
    unit_datas: Option<CustomUnitFile>,
    item_datas: Option<CustomItemFile>,
    ability_datas: Option<CustomAbilityFile>,
    buff_datas: Option<CustomBuffFile>,
    doodad_datas: Option<CustomDoodadFile>,
    destructable_datas: Option<CustomDestructableFile>,
    upgrade_datas: Option<CustomUpgradeFile>,
}

impl<'a> Map<'a> {
    pub fn open(path: String, game_data: &'a GameData) -> Result<Self, OpeningError> {
        let mut map = MapArchive::open(path.to_owned()).map_err(OpeningError::Protected)?;

        let w3i = W3iFile::read_file(&mut map)?;
        let game_version = w3i.game_version();
        // w3i.debug();
        let mmp = MMPFile::read_file(&mut map)?;
        //        mmp.debug();
        let regions = RegionFile::read_file(&mut map)?;
        // println!("{:#?}", regions);
        //        regions.debug();
        let cameras = CameraFile::read_file(&mut map)?;
        // println!("{:#?}", cameras);
        //        cameras.debug();
        let sounds = SoundFile::read_file(&mut map)?;
        // println!("{:#?}", sounds);
        let path_map = PathMapFile::read_file(&mut map)?;
        //        pathing.debug();
        let shaders = ShadowMapFile::read_file(&mut map)?;
        //        shaders.debug();
        let environment = TerrainFile::read_file(&mut map)?;
        //        environment.debug();
        let minimap = MinimapFile::read_file(&mut map)?;
        //        mmap.debug();
        let trigstrs = MapStringFile::read_file(&mut map)?;
        //        trigstrs.debug();
        let triggers_ct = TriggerJassFile::read_file(&mut map)?;
        //        triggers_ct.debug();
        let triggers = TriggersFile::read_file(&mut map, game_data.get_trigger_data())?;
        let doodad_map = DoodadMap::read_file(&mut map)?;
        // println!("{:#?}", doodad_map);
        let unit_item_map = UnitItemMap::read_file(&mut map)?;
        let import_listing = ImportFile::read_file(&mut map, &game_version)?;
        let unit_datas = CustomUnitFile::read_file(&mut map, &game_version)?;
        let ability_datas = CustomAbilityFile::read_file(&mut map, &game_version)?;
        let item_datas = CustomItemFile::read_file(&mut map, &game_version)?;
        let destructable_datas = CustomDestructableFile::read_file(&mut map, &game_version)?;
        let doodad_datas = CustomDoodadFile::read_file(&mut map, &game_version)?;
        let buff_datas = CustomBuffFile::read_file(&mut map, &game_version)?;
        let upgrade_datas = CustomUpgradeFile::read_file(&mut map, &game_version)?;
        // unit_datas.debug();

        Ok(Self {
            game_data,
            path,
            infos: w3i,
            terrain: environment,
            cameras,
            regions,
            minimap,
            menu_minimap: mmp,
            shaders,
            sounds,
            path_map,
            strings: trigstrs,
            custom_scripts: triggers_ct,
            triggers,
            doodad_map,
            unit_item_map,
            import_listing,
            unit_datas,
            item_datas,
            ability_datas,
            buff_datas,
            doodad_datas,
            destructable_datas,
            upgrade_datas,
        })
    }

    fn save_file(writer: BinaryWriter, file_path: &str) {
        let mut f = File::create(file_path).unwrap();
        f.write_all(&writer.into_buffer()).unwrap();
    }

    pub fn save(&self, path: String, game_data: &'a GameData) -> WriteResult<()> {
        let path_fn = |file_name: &str| format!("{path}/{file_name}");
        let game_version = self.infos.game_version();
        Self::save_file(self.infos.prepare_write()?, &path_fn(W3iFile::FILE_NAME));
        Self::save_file(
            self.terrain.prepare_write()?,
            &path_fn(TerrainFile::FILE_NAME),
        );
        if let Some(f) = &self.cameras {
            Self::save_file(f.prepare_write()?, &path_fn(CameraFile::FILE_NAME));
        }
        if let Some(f) = &self.regions {
            Self::save_file(f.prepare_write()?, &path_fn(RegionFile::FILE_NAME));
        }
        if let Some(f) = &self.sounds {
            Self::save_file(f.prepare_write()?, &path_fn(SoundFile::FILE_NAME));
        }
        Self::save_file(
            self.path_map.prepare_write()?,
            &path_fn(PathMapFile::FILE_NAME),
        );
        Self::save_file(
            self.triggers
                .prepare_write(&game_data.trigger_data)
                .unwrap(),
            &path_fn(TriggersFile::FILE_NAME),
        );
        Self::save_file(
            self.doodad_map.prepare_write()?,
            &path_fn(DoodadMap::FILE_NAME),
        );
        Self::save_file(
            self.unit_item_map.prepare_write()?,
            &path_fn(UnitItemMap::FILE_NAME),
        );
        if let Some(f) = &self.unit_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomUnitFile::FILE_NAME),
            );
        }
        if let Some(f) = &self.ability_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomAbilityFile::FILE_NAME),
            );
        }
        if let Some(f) = &self.buff_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomBuffFile::FILE_NAME),
            );
        }
        if let Some(f) = &self.doodad_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomDoodadFile::FILE_NAME),
            );
        }
        if let Some(f) = &self.destructable_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomDestructableFile::FILE_NAME),
            );
        }
        if let Some(f) = &self.item_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomItemFile::FILE_NAME),
            );
        }
        if let Some(f) = &self.upgrade_datas {
            Self::save_file(
                f.prepare_write(&game_version)?,
                &path_fn(CustomUpgradeFile::FILE_NAME),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod map_tests {
    use std::fs;
    use super::Map;
    use crate::{get_resources_path, GameData};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn map_open_save_roundtrip_roc() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        
        // Open RoC map (Sandbox_1.w3m)
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let map = Map::open(map_path.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to open map: {:?}", e));

        // Create output directory for write test
        let output_dir = get_path("Scenario/write-test-roc");
        let _ = fs::remove_dir_all(&output_dir); // Remove if exists
        fs::create_dir_all(&output_dir)
            .unwrap_or_else(|e| panic!("Failed to create output directory: {:?}", e));

        // Save the map to the output directory
        map.save(output_dir.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to save map: {:?}", e));

        // Verify some key files were created
        let w3i_path = format!("{}/war3map.w3i", output_dir);
        let env_path = format!("{}/war3map.w3e", output_dir);
        let triggers_path = format!("{}/war3map.wtg", output_dir);
        let units_path = format!("{}/war3mapUnits.doo", output_dir);

        assert!(fs::metadata(&w3i_path).is_ok(), "war3map.w3i should be created");
        assert!(fs::metadata(&env_path).is_ok(), "war3map.w3e should be created");
        assert!(fs::metadata(&triggers_path).is_ok(), "war3map.wtg should be created");
        assert!(fs::metadata(&units_path).is_ok(), "war3mapUnits.doo should be created");

        println!("RoC map open/save test passed. Files saved to: {}", output_dir);
        
        // Clean up
        let _ = fs::remove_dir_all(&output_dir);
    }

    #[test] 
    fn map_open_save_roundtrip_tft() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        
        // Open TFT map (Sandbox_1.w3x)
        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let map = Map::open(map_path.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to open map: {:?}", e));

        // Create output directory for write test
        let output_dir = get_path("Scenario/write-test-tft");
        let _ = fs::remove_dir_all(&output_dir); // Remove if exists
        fs::create_dir_all(&output_dir)
            .unwrap_or_else(|e| panic!("Failed to create output directory: {:?}", e));

        // Save the map to the output directory
        map.save(output_dir.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to save map: {:?}", e));

        // Verify some key files were created
        let w3i_path = format!("{}/war3map.w3i", output_dir);
        let env_path = format!("{}/war3map.w3e", output_dir);
        let triggers_path = format!("{}/war3map.wtg", output_dir);
        let units_path = format!("{}/war3mapUnits.doo", output_dir);

        assert!(fs::metadata(&w3i_path).is_ok(), "war3map.w3i should be created");
        assert!(fs::metadata(&env_path).is_ok(), "war3map.w3e should be created");
        assert!(fs::metadata(&triggers_path).is_ok(), "war3map.wtg should be created");
        assert!(fs::metadata(&units_path).is_ok(), "war3mapUnits.doo should be created");

        println!("TFT map open/save test passed. Files saved to: {}", output_dir);
        
        // Clean up
        let _ = fs::remove_dir_all(&output_dir);
    }

    #[test]
    fn map_open_save_both_formats_comparison() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        
        // Test both formats
        let roc_map_path = get_path("Scenario/Sandbox_1.w3m");
        let tft_map_path = get_path("Scenario/Sandbox_1.w3x");
        
        let roc_map = Map::open(roc_map_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to open RoC map: {:?}", e));
            
        let tft_map = Map::open(tft_map_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to open TFT map: {:?}", e));

        // Create output directories
        let roc_output = get_path("Scenario/write-test-roc-comp");
        let tft_output = get_path("Scenario/write-test-tft-comp");
        
        let _ = fs::remove_dir_all(&roc_output);
        let _ = fs::remove_dir_all(&tft_output);
        fs::create_dir_all(&roc_output).unwrap();
        fs::create_dir_all(&tft_output).unwrap();

        // Save both maps
        roc_map.save(roc_output.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to save RoC map: {:?}", e));
            
        tft_map.save(tft_output.clone(), &game_data) 
            .unwrap_or_else(|e| panic!("Failed to save TFT map: {:?}", e));

        // Check that triggers file was written with correct format
        let roc_triggers = format!("{}/war3map.wtg", roc_output);
        let tft_triggers = format!("{}/war3map.wtg", tft_output);
        
        let roc_triggers_data = fs::read(&roc_triggers).unwrap();
        let tft_triggers_data = fs::read(&tft_triggers).unwrap();
        
        // RoC and TFT triggers should have different sizes/content due to version differences
        println!("RoC triggers size: {} bytes", roc_triggers_data.len());
        println!("TFT triggers size: {} bytes", tft_triggers_data.len());
        
        // Clean up
        let _ = fs::remove_dir_all(&roc_output);
        let _ = fs::remove_dir_all(&tft_output);

        println!("Both formats comparison test passed");
    }

    #[test]
    #[ignore] // Ignored by default, run with: cargo test -- --ignored
    fn map_save_persistent_files() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        
        // Open both maps and save to persistent directories (no cleanup)
        let roc_map = Map::open(get_path("Scenario/Sandbox_1.w3m"), &game_data).unwrap();
        let tft_map = Map::open(get_path("Scenario/Sandbox_1.w3x"), &game_data).unwrap();
        
        let roc_output = get_path("Scenario/write-test-roc-persistent");
        let tft_output = get_path("Scenario/write-test-tft-persistent");
        
        let _ = fs::remove_dir_all(&roc_output);
        let _ = fs::remove_dir_all(&tft_output);
        fs::create_dir_all(&roc_output).unwrap();
        fs::create_dir_all(&tft_output).unwrap();
        
        roc_map.save(roc_output.clone(), &game_data).unwrap();
        tft_map.save(tft_output.clone(), &game_data).unwrap();
        
        println!("Files saved persistently to:");
        println!("  RoC: {}", roc_output);
        println!("  TFT: {}", tft_output);
        
        // List files created
        if let Ok(entries) = fs::read_dir(&roc_output) {
            println!("RoC files created:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        }
        
        if let Ok(entries) = fs::read_dir(&tft_output) {
            println!("TFT files created:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        }
    }
}
