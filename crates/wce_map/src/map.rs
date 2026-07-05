use std::fs::File;
use std::io::Write;

use wce_formats::{MapArchive, MapArchiveWriter};

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
use crate::{GameData, MapError};

pub struct Map<'a> {
    game_data: &'a GameData,
    path: String,
    /// Original 512-byte `HM3W` map header, preserved to be written back when
    /// repackaging into an archive. Empty for a bare MPQ source.
    header: Vec<u8>,
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
    /// Archive entries not modelled as typed components — imported assets
    /// (models, textures, sounds) carried verbatim so they survive a repack.
    /// Stored as `(archive_name, raw_bytes)`.
    extra_files: Vec<(String, Vec<u8>)>,
    /// Archive entries that could not be read on open (e.g. Huffman/ADPCM
    /// MPQ compression the `mpq` crate does not support). Opening stays
    /// possible for inspection, but saving refuses rather than silently
    /// dropping these files.
    unreadable_files: Vec<String>,
}

impl<'a> Map<'a> {
    /// Every archive file name modelled as a typed component. Used on open to
    /// tell imported assets apart from files that round-trip via their own
    /// `prepare_write()`.
    const KNOWN_COMPONENT_FILES: [&'static str; 22] = [
        W3iFile::FILE_NAME,
        TerrainFile::FILE_NAME,
        MMPFile::FILE_NAME,
        MinimapFile::FILE_NAME,
        ShadowMapFile::FILE_NAME,
        MapStringFile::FILE_NAME,
        TriggerJassFile::FILE_NAME,
        PathMapFile::FILE_NAME,
        TriggersFile::FILE_NAME,
        DoodadMap::FILE_NAME,
        UnitItemMap::FILE_NAME,
        CameraFile::FILE_NAME,
        RegionFile::FILE_NAME,
        SoundFile::FILE_NAME,
        ImportFile::FILE_NAME,
        CustomUnitFile::FILE_NAME,
        CustomAbilityFile::FILE_NAME,
        CustomBuffFile::FILE_NAME,
        CustomDoodadFile::FILE_NAME,
        CustomDestructableFile::FILE_NAME,
        CustomItemFile::FILE_NAME,
        CustomUpgradeFile::FILE_NAME,
    ];

    /// Read every archive entry that is neither a typed component nor an
    /// MPQ-internal file (`(listfile)`, `(attributes)`, `(signature)`) as a raw
    /// blob, so imported assets survive an open/save round-trip. Returns the
    /// captured `(name, bytes)` blobs plus the names of entries that could not
    /// be read (unsupported MPQ compression such as Huffman/ADPCM on old
    /// imported `.wav` files) — those must block a later save, not the open.
    /// Both lists are empty when the archive has no `(listfile)` to enumerate.
    fn capture_extra_files(map: &mut MapArchive) -> (Vec<(String, Vec<u8>)>, Vec<String>) {
        let names = match map.files() {
            Some(names) => names,
            None => return (Vec::new(), Vec::new()),
        };
        let mut extra = Vec::new();
        let mut unreadable = Vec::new();
        for name in names {
            // The writer regenerates its own (listfile); (attributes)/(signature)
            // would be stale after content changes. Skip all MPQ-internal files.
            if name.starts_with('(') {
                continue;
            }
            // Typed components are re-emitted via their prepare_write().
            if Self::KNOWN_COMPONENT_FILES
                .iter()
                .any(|known| known.eq_ignore_ascii_case(&name))
            {
                continue;
            }
            match map.read_file(&name) {
                Ok(buffer) => extra.push((name, buffer.inner())),
                Err(err) => {
                    log::warn!("Cannot capture archive entry '{name}' for repack: {err:?}");
                    unreadable.push(name);
                }
            }
        }
        (extra, unreadable)
    }

    pub fn open(path: String, game_data: &'a GameData) -> Result<Self, MapError> {
        let mut map = MapArchive::open(path.to_owned()).map_err(MapError::Protected)?;

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

        let header = map.header().to_vec();
        let (extra_files, unreadable_files) = Self::capture_extra_files(&mut map);

        Ok(Self {
            game_data,
            path,
            header,
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
            extra_files,
            unreadable_files,
        })
    }

    fn save_file(bytes: &[u8], file_path: &str) -> Result<(), MapError> {
        let to_err = |source| MapError::SaveFileIo {
            path: file_path.to_string(),
            source,
        };
        let mut f = File::create(file_path).map_err(to_err)?;
        f.write_all(bytes).map_err(to_err)
    }

    /// Emit every component file as `(file_name, bytes)`. Both `save` (loose
    /// files) and `save_as_archive` (MPQ) route through here so the two can
    /// never drift apart on which files they write. Errors with
    /// [`MapError::UnreadableArchiveEntries`] when entries could not be
    /// captured on open — writing would silently drop them.
    fn for_each_component_file<F>(&self, game_data: &GameData, mut emit: F) -> Result<(), MapError>
    where
        F: FnMut(&str, Vec<u8>) -> Result<(), MapError>,
    {
        if !self.unreadable_files.is_empty() {
            return Err(MapError::UnreadableArchiveEntries(
                self.unreadable_files.clone(),
            ));
        }
        let game_version = self.infos.game_version();
        emit(
            W3iFile::FILE_NAME,
            self.infos.prepare_write()?.into_buffer(),
        )?;
        emit(
            TerrainFile::FILE_NAME,
            self.terrain.prepare_write()?.into_buffer(),
        )?;
        emit(
            MMPFile::FILE_NAME,
            self.menu_minimap.prepare_write()?.into_buffer(),
        )?;
        emit(
            MinimapFile::FILE_NAME,
            self.minimap.prepare_write()?.into_buffer(),
        )?;
        emit(
            ShadowMapFile::FILE_NAME,
            self.shaders.prepare_write()?.into_buffer(),
        )?;
        emit(
            MapStringFile::FILE_NAME,
            self.strings.prepare_write()?.into_buffer(),
        )?;
        emit(
            TriggerJassFile::FILE_NAME,
            self.custom_scripts.prepare_write()?.into_buffer(),
        )?;
        emit(
            PathMapFile::FILE_NAME,
            self.path_map.prepare_write()?.into_buffer(),
        )?;
        emit(
            TriggersFile::FILE_NAME,
            self.triggers
                .prepare_write(&game_data.trigger_data)?
                .into_buffer(),
        )?;
        emit(
            DoodadMap::FILE_NAME,
            self.doodad_map.prepare_write()?.into_buffer(),
        )?;
        emit(
            UnitItemMap::FILE_NAME,
            self.unit_item_map.prepare_write()?.into_buffer(),
        )?;
        if let Some(f) = &self.cameras {
            emit(CameraFile::FILE_NAME, f.prepare_write()?.into_buffer())?;
        }
        if let Some(f) = &self.regions {
            emit(RegionFile::FILE_NAME, f.prepare_write()?.into_buffer())?;
        }
        if let Some(f) = &self.sounds {
            emit(SoundFile::FILE_NAME, f.prepare_write()?.into_buffer())?;
        }
        if let Some(f) = &self.import_listing {
            emit(
                ImportFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.unit_datas {
            emit(
                CustomUnitFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.ability_datas {
            emit(
                CustomAbilityFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.buff_datas {
            emit(
                CustomBuffFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.doodad_datas {
            emit(
                CustomDoodadFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.destructable_datas {
            emit(
                CustomDestructableFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.item_datas {
            emit(
                CustomItemFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        if let Some(f) = &self.upgrade_datas {
            emit(
                CustomUpgradeFile::FILE_NAME,
                f.prepare_write(&game_version)?.into_buffer(),
            )?;
        }
        // Imported assets and any other non-component entries, carried verbatim.
        for (name, bytes) in &self.extra_files {
            emit(name, bytes.clone())?;
        }
        Ok(())
    }

    /// Save every component as a loose file under `path`.
    pub fn save(&self, path: String, game_data: &'a GameData) -> Result<(), MapError> {
        self.for_each_component_file(game_data, |file_name, bytes| {
            Self::save_file(&bytes, &format!("{path}/{file_name}"))
        })
    }

    /// Repackage the map into a single MPQ archive at `path`, preserving the
    /// original 512-byte `HM3W` header. Imported assets captured on open are
    /// carried over verbatim.
    pub fn save_as_archive(&self, path: &str, game_data: &'a GameData) -> Result<(), MapError> {
        let mut archive = MapArchiveWriter::new();
        self.for_each_component_file(game_data, |file_name, bytes| {
            archive.add_file(file_name, bytes);
            Ok(())
        })?;
        archive
            .save_archive(path, &self.header)
            .map_err(MapError::Archive)
    }
}

#[cfg(test)]
mod map_tests {
    use super::Map;
    use crate::{get_resources_path, GameData};
    use std::fs;

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

        assert!(
            fs::metadata(&w3i_path).is_ok(),
            "war3map.w3i should be created"
        );
        assert!(
            fs::metadata(&env_path).is_ok(),
            "war3map.w3e should be created"
        );
        assert!(
            fs::metadata(&triggers_path).is_ok(),
            "war3map.wtg should be created"
        );
        assert!(
            fs::metadata(&units_path).is_ok(),
            "war3mapUnits.doo should be created"
        );

        // Previously-missing mandatory files must now be written on save.
        for name in [
            "war3map.wts",    // trigger strings
            "war3map.wct",    // custom scripts
            "war3map.mmp",    // menu minimap
            "war3mapMap.blp", // minimap
            "war3map.shd",    // shadow map
        ] {
            let p = format!("{}/{}", output_dir, name);
            assert!(fs::metadata(&p).is_ok(), "{} should be created", name);
        }

        println!(
            "RoC map open/save test passed. Files saved to: {}",
            output_dir
        );

        // Clean up
        let _ = fs::remove_dir_all(&output_dir);
    }

    #[test]
    fn save_into_missing_directory_returns_error() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        let map = Map::open(get_path("Scenario/Sandbox_1.w3m"), &game_data)
            .unwrap_or_else(|e| panic!("Failed to open map: {:?}", e));

        // The directory intentionally does not exist: the I/O failure must
        // surface as a MapError, not abort the process.
        let missing_dir = get_path("Scenario/this-dir-does-not-exist/nested");
        let result = map.save(missing_dir, &game_data);
        assert!(
            result.is_err(),
            "saving into a missing directory must return Err, not panic"
        );
    }

    #[test]
    fn map_save_as_archive_reopenable() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let map = Map::open(map_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to open map: {:?}", e));

        let output_dir = get_path("Scenario/archive-test");
        let _ = fs::remove_dir_all(&output_dir);
        fs::create_dir_all(&output_dir)
            .unwrap_or_else(|e| panic!("Failed to create output directory: {:?}", e));
        let out_path = format!("{}/repacked.w3m", output_dir);

        // Repackage all component files into a single MPQ archive.
        map.save_as_archive(&out_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to save archive: {:?}", e));

        // The archive must keep the original 512-byte HM3W header.
        let bytes = fs::read(&out_path).unwrap_or_else(|e| panic!("{:?}", e));
        assert_eq!(&bytes[0..4], b"HM3W", "archive must keep the HM3W header");

        // The repackaged archive must be a real, re-openable Warcraft III map.
        let reopened = Map::open(out_path.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to re-open repacked archive: {:?}", e));
        assert_eq!(
            reopened.infos.game_version(),
            map.infos.game_version(),
            "re-opened map should report the same game version"
        );

        let _ = fs::remove_dir_all(&output_dir);
    }

    /// Recreate an MPQ archive and leave it on disk (in the git-ignored
    /// `resources/repacked/`) so it can be opened in the World Editor or
    /// Warcraft III for manual inspection. Ignored by default (persistent
    /// output); run on demand:
    ///
    /// ```text
    /// cargo test -p wce_map --lib repack_persistent -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore = "writes a repacked .w3m to resources/repacked/ for manual inspection"]
    fn repack_persistent() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        let map = Map::open(get_path("Scenario/Sandbox_1.w3m"), &game_data)
            .unwrap_or_else(|e| panic!("Failed to open map: {:?}", e));

        let output_dir = get_path("repacked");
        fs::create_dir_all(&output_dir).unwrap_or_else(|e| panic!("{:?}", e));
        let out_path = format!("{}/Sandbox_1.w3m", output_dir);

        map.save_as_archive(&out_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to save archive: {:?}", e));
        // Sanity: the persisted archive must re-open.
        Map::open(out_path.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Repacked archive is not re-openable: {:?}", e));

        println!("repacked archive written to: {out_path}");
        // No cleanup: left on disk for manual inspection.
    }

    /// Imported assets (here `war3mapImported\Grid256.blp`) are not modelled as
    /// typed components; they must be captured on open and re-emitted byte-exact
    /// on repack, otherwise a real map would lose its imports. Covers both the
    /// RoC (`.w3m`) and TFT (`.w3x`) sandbox maps.
    fn assert_import_survives_repack(source: &str, out_name: &str) {
        const IMPORT: &str = r"war3mapImported\Grid256.blp";

        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        let map = Map::open(get_path(source), &game_data)
            .unwrap_or_else(|e| panic!("Failed to open {source}: {:?}", e));

        // The import must be captured on open, verbatim.
        let original = map
            .extra_files
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(IMPORT))
            .unwrap_or_else(|| {
                panic!(
                    "{source} should carry the imported {IMPORT}; captured: {:?}",
                    map.extra_files.iter().map(|(n, _)| n).collect::<Vec<_>>()
                )
            })
            .clone();
        assert!(!original.1.is_empty(), "captured import must not be empty");

        // Unique per call so the RoC and TFT tests can run in parallel.
        let output_dir = get_path(&format!("Scenario/import-repack-test-{out_name}"));
        let _ = fs::remove_dir_all(&output_dir);
        fs::create_dir_all(&output_dir).unwrap_or_else(|e| panic!("{:?}", e));
        let out_path = format!("{}/{}", output_dir, out_name);

        map.save_as_archive(&out_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to repack {source}: {:?}", e));

        // Re-open the repacked archive: the import must round-trip byte-exact.
        let reopened = Map::open(out_path, &game_data)
            .unwrap_or_else(|e| panic!("Failed to re-open repacked {source}: {:?}", e));
        let roundtripped = reopened
            .extra_files
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(IMPORT))
            .unwrap_or_else(|| {
                panic!(
                    "repacked {source} lost its import {IMPORT}",
                    source = source,
                    IMPORT = IMPORT
                )
            });
        assert_eq!(
            roundtripped.1, original.1,
            "imported {IMPORT} must survive repack byte-exact"
        );

        let _ = fs::remove_dir_all(&output_dir);
    }

    /// An archive entry that could not be read on open (unsupported MPQ
    /// compression, e.g. Huffman/ADPCM `.wav` imports in old maps) must not
    /// fail the open, but saving must refuse rather than silently drop it.
    #[test]
    fn save_refuses_when_an_entry_was_unreadable_on_open() {
        let game_data = GameData::new(&get_resources_path()).unwrap_or_else(|e| panic!("{:?}", e));
        let mut map = Map::open(get_path("Scenario/Sandbox_1.w3m"), &game_data)
            .unwrap_or_else(|e| panic!("Failed to open map: {:?}", e));
        map.unreadable_files
            .push(r"war3mapImported\Huffman.wav".to_string());

        let output_dir = get_path("Scenario/unreadable-entry-test");
        let _ = fs::remove_dir_all(&output_dir);
        fs::create_dir_all(&output_dir).unwrap_or_else(|e| panic!("{:?}", e));

        let archive_err = map
            .save_as_archive(&format!("{output_dir}/repacked.w3m"), &game_data)
            .expect_err("save_as_archive must refuse a lossy repack");
        assert!(
            matches!(&archive_err, crate::MapError::UnreadableArchiveEntries(files)
                if files == &[r"war3mapImported\Huffman.wav".to_string()]),
            "unexpected error: {archive_err:?}"
        );
        let save_err = map
            .save(output_dir.clone(), &game_data)
            .expect_err("save must refuse a lossy write");
        assert!(matches!(
            save_err,
            crate::MapError::UnreadableArchiveEntries(_)
        ));

        let _ = fs::remove_dir_all(&output_dir);
    }

    #[test]
    fn import_survives_repack_roc() {
        assert_import_survives_repack("Scenario/Sandbox_1.w3m", "repacked.w3m");
    }

    #[test]
    fn import_survives_repack_tft() {
        assert_import_survives_repack("Scenario/Sandbox_1.w3x", "repacked.w3x");
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

        assert!(
            fs::metadata(&w3i_path).is_ok(),
            "war3map.w3i should be created"
        );
        assert!(
            fs::metadata(&env_path).is_ok(),
            "war3map.w3e should be created"
        );
        assert!(
            fs::metadata(&triggers_path).is_ok(),
            "war3map.wtg should be created"
        );
        assert!(
            fs::metadata(&units_path).is_ok(),
            "war3mapUnits.doo should be created"
        );

        // Previously-missing mandatory files must now be written on save.
        for name in [
            "war3map.wts",    // trigger strings
            "war3map.wct",    // custom scripts
            "war3map.mmp",    // menu minimap
            "war3mapMap.blp", // minimap
            "war3map.shd",    // shadow map
        ] {
            let p = format!("{}/{}", output_dir, name);
            assert!(fs::metadata(&p).is_ok(), "{} should be created", name);
        }

        println!(
            "TFT map open/save test passed. Files saved to: {}",
            output_dir
        );

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
        roc_map
            .save(roc_output.clone(), &game_data)
            .unwrap_or_else(|e| panic!("Failed to save RoC map: {:?}", e));

        tft_map
            .save(tft_output.clone(), &game_data)
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
