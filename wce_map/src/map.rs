use wce_formats::MapArchive;

use crate::{GameData, OpeningError};
use crate::camera_file::CameraFile;
use crate::custom_datas::ability::CustomAbilityFile;
use crate::custom_datas::buff::CustomBuffFile;
use crate::custom_datas::destructable::CustomDestructableFile;
use crate::custom_datas::doodad::CustomDoodadFile;
use crate::custom_datas::item::CustomItemFile;
use crate::custom_datas::unit::CustomUnitFile;
use crate::custom_datas::upgrade::CustomUpgradeFile;
use crate::doodad_map::DoodadMap;
use crate::import_file::ImportFile;
use crate::mmp_file::MMPFile;
use crate::pathmap_file::PathMapFile;
use crate::region_file::RegionFile;
use crate::shadowmap_file::ShadowMapFile;
use crate::sound_file::SoundFile;
use crate::terrain_file::TerrainFile;
use crate::trigger_jass_file::TriggerJassFile;
use crate::trigger_string_file::TriggerStringFile;
use crate::triggers::TriggersFile;
use crate::unit_map::UnitItemMap;
use crate::w3i_file::W3iFile;

pub struct Map<'a>{
    game_data: &'a GameData,
    path: String,
    infos: W3iFile,
    terrain: TerrainFile,
    cameras: Option<CameraFile>,
    regions: Option<RegionFile>,
//    minimap: MinimapFile,
    menu_minimap: MMPFile,
    shaders: ShadowMapFile,
    sounds: Option<SoundFile>,
    strings: TriggerStringFile,
    custom_scripts: TriggerJassFile,
    doodad_map: DoodadMap,
    unit_item_map: UnitItemMap,
    triggers: TriggersFile,
    import_listing: Option<ImportFile>,
    unit_datas: CustomUnitFile,
    item_datas: CustomItemFile,
    ability_datas: CustomAbilityFile,
    buff_datas: CustomBuffFile,
    doodad_datas: CustomDoodadFile,
    destructable_datas: CustomDestructableFile,
    upgrade_datas: CustomUpgradeFile
}

impl<'a> Map<'a> {
    pub fn open(path: String, game_data: &'a GameData) -> Result<Self, OpeningError>{
        let mut map = MapArchive::open(path.to_owned()).unwrap();

        let w3i = W3iFile::read_file(&mut map);
        let game_version = w3i.game_version();
       // w3i.debug();
        let mmp = MMPFile::read_file(&mut map);
//        mmp.debug();
        let regions = RegionFile::read_file(&mut map);
        // println!("{:#?}", regions);
//        regions.debug();
        let cameras = CameraFile::read_file(&mut map);
        // println!("{:#?}", cameras);
//        cameras.debug();
        let sounds = SoundFile::read_file(&mut map);
        // println!("{:#?}", sounds);
        let _pathing = PathMapFile::read_file(&mut map);
//        pathing.debug();
        let shaders = ShadowMapFile::read_file(&mut map);
//        shaders.debug();
        let environment = TerrainFile::read_file(&mut map);
//        environment.debug();
//        let mmap = MinimapFile::read_file(&mut map);
//        mmap.debug();
        let trigstrs = TriggerStringFile::read_file(&mut map);
//        trigstrs.debug();
        let triggers_ct = TriggerJassFile::read_file(&mut map);
//        triggers_ct.debug();
        let triggers = TriggersFile::read_file(&mut map, game_data.get_trigger_data()).map_err(|err| err.into())?;
        let doodad_map = DoodadMap::read_file(&mut map);
        // println!("{:#?}", doodad_map);
        let unit_item_map = UnitItemMap::read_file(&mut map);
        let import_listing = ImportFile::read_file(&mut map);
        let unit_datas = CustomUnitFile::read_file(&mut map, &game_version);
        let ability_datas = CustomAbilityFile::read_file(&mut map, &game_version);
        let item_datas = CustomItemFile::read_file(&mut map, &game_version);
        let destructable_datas = CustomDestructableFile::read_file(&mut map, &game_version);
        let doodad_datas = CustomDoodadFile::read_file(&mut map, &game_version);
        let buff_datas = CustomBuffFile::read_file(&mut map, &game_version);
        let upgrade_datas = CustomUpgradeFile::read_file(&mut map, &game_version);
        // unit_datas.debug();

        Ok(Self{
            game_data,
            path,
            infos: w3i,
            terrain: environment,
            cameras,
            regions,
//            minimap: mmap,
            menu_minimap: mmp,
            shaders,
            sounds,
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
}
