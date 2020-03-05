use crate::map_data::environment_file::EnvironmentFile;
use crate::map_data::camera_file::CameraFile;
use crate::map_data::region_file::RegionFile;
use crate::map_data::minimap_file::MinimapFile;
use crate::map_data::mmp_file::MMPFile;
use crate::map_data::shadowmap_file::ShadowMapFile;
use crate::map_data::sound_file::SoundFile;
use crate::map_data::trigger_string_file::TriggerStringFile;
use crate::map_data::custom_text_trigger_file::CustomTextTriggerFile;
use crate::map_data::triggers_names_file::TriggersNameFile;
use crate::map_data::import_file::ImportFile;
use crate::map_data::w3i_file::W3iFile;
use mpq::Archive;
use crate::map_data::pathmap_file::PathMapFile;

pub struct Map{
    path: String,
    infos: W3iFile,
    terrain: EnvironmentFile,
    cameras: CameraFile,
    regions: RegionFile,
    minimap: MinimapFile,
    menu_minimap: MMPFile,
    shaders: ShadowMapFile,
    sounds: SoundFile,
    strings: TriggerStringFile,
    custom_scripts: CustomTextTriggerFile,
//    triggers: TriggersNameFile,
//    import_listing: ImportFile,
}

impl Map {
    pub fn open(path: String) -> Self{
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
        let environment = EnvironmentFile::read_file(&mut map);
//        environment.debug();
        let mmap = MinimapFile::read_file(&mut map);
//        mmap.debug();
        let trigstrs = TriggerStringFile::read_file(&mut map);
//        trigstrs.debug();
        let triggers_ct = CustomTextTriggerFile::read_file(&mut map);
//        triggers_ct.debug();

        Self{
            path,
            infos: w3i,
            terrain: environment,
            cameras,
            regions,
            minimap: mmap,
            menu_minimap: mmp,
            shaders,
            sounds,
            strings: trigstrs,
            custom_scripts: triggers_ct,
//            import_listing: ()
        }
    }
}