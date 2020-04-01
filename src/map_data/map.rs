use mpq::Archive;

use crate::{format_data, GameData};
use crate::globals::PROFILE_TRIGGER_DATA;
use crate::map_data::camera_file::CameraFile;
use crate::map_data::custom_text_trigger_file::CustomTextTriggerFile;
use crate::map_data::data_ini::DataIni;
use crate::map_data::doodad_map::DoodadMap;
use crate::map_data::import_file::ImportFile;
use crate::map_data::mmp_file::MMPFile;
use crate::map_data::pathmap_file::PathMapFile;
use crate::map_data::region_file::RegionFile;
use crate::map_data::shadowmap_file::ShadowMapFile;
use crate::map_data::sound_file::SoundFile;
use crate::map_data::terrain_file::TerrainFile;
use crate::map_data::trigger_string_file::TriggerStringFile;
use crate::map_data::triggers::TriggersFile;
use crate::map_data::unit_map::UnitItemMap;
use crate::map_data::w3i_file::W3iFile;

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
    custom_scripts: CustomTextTriggerFile,
    doodad_map: DoodadMap,
    unit_item_map: UnitItemMap,
    triggers: TriggersFile,
    import_listing: Option<ImportFile>,
}

impl<'a> Map<'a> {
    pub fn open(path: String, game_data: &'a GameData) -> Self{
        let mut map = Archive::open(path.to_owned()).unwrap();

        let w3i = W3iFile::read_file(&mut map);
//        w3i.debug();
        let mmp = MMPFile::read_file(&mut map);
//        mmp.debug();
        let regions = RegionFile::read_file(&mut map);
//        regions.debug();
        let cameras = CameraFile::read_file(&mut map);
//        cameras.debug();
        let sounds = SoundFile::read_file(&mut map);
//        sounds.debug();
        let pathing = PathMapFile::read_file(&mut map);
//        pathing.debug();
        let shaders = ShadowMapFile::read_file(&mut map);
//        shaders.debug();
        let environment = TerrainFile::read_file(&mut map);
//        environment.debug();
//        let mmap = MinimapFile::read_file(&mut map);
//        mmap.debug();
        let trigstrs = TriggerStringFile::read_file(&mut map);
//        trigstrs.debug();
        let triggers_ct = CustomTextTriggerFile::read_file(&mut map);
//        triggers_ct.debug();
        let triggers = TriggersFile::read_file(&mut map, game_data.get_trigger_data());
        let doodad_map = DoodadMap::read_file(&mut map);
        let unit_item_map = UnitItemMap::read_file(&mut map);
        let import_listing = ImportFile::read_file(&mut map);

        Self{
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
        }
    }
}
