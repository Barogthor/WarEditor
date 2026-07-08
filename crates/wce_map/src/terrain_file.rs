//! Parser and writer for `war3map.w3e` (heightmap, tileset palette and per-tile ground
//! textures/cliffs that make up the map terrain).

use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError, WriteError};

use crate::globals::MAP_TERRAIN;
use crate::MapError;

#[derive(Debug, Error)]
pub enum TerrainError {
    #[error("MPQ opening failure. {0} ")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse terrain data. {0}")]
    Parsing(ReadError),
    #[error("Failed to save terrain data. {0}")]
    SaveError(WriteError),
}

#[derive(Debug)]
pub struct TilePoint {
    ground_height: i16,
    water_level: u16,
    ground_texture_and_flags: u8,
    texture_details: u8,
    cliff_texture_and_layer_height: u8,
}

impl TilePoint {
    pub fn boundary_flag(&self) -> bool {
        self.water_level & 0x4000 == 0x4000
    }

    pub fn get_water_level(&self) -> i16 {
        (self.water_level & 0xBFFF) as i16
    }

    pub fn ramp(&self) -> bool {
        self.ground_texture_and_flags & 0x0010 == 0x0010
    }

    pub fn blight(&self) -> bool {
        self.ground_texture_and_flags & 0x0020 == 0x0020
    }

    pub fn water(&self) -> bool {
        self.ground_texture_and_flags & 0x0040 == 0x0040
    }

    pub fn ground_texture(&self) -> u8 {
        self.ground_texture_and_flags >> 4
    }

    pub fn cliff_texture(&self) -> u8 {
        self.cliff_texture_and_layer_height
    }

    pub fn layer_height(&self) -> u8 {
        self.cliff_texture_and_layer_height >> 4
    }

    pub fn set_boundary_flag(&mut self, value: bool) {
        if value {
            self.water_level |= 0x4000;
        } else {
            self.water_level &= 0xBFFF;
        }
    }
}

impl BinaryConverter for TilePoint {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let ground_height = reader.read_i16()?;
        let water_level = reader.read_u16()?;
        let ground_texture_and_flags = reader.read_u8()?;
        let texture_details = reader.read_u8()?;
        let cliff_texture_and_layer_height = reader.read_u8()?;
        Ok(TilePoint {
            ground_height,
            water_level,
            ground_texture_and_flags,
            texture_details,
            cliff_texture_and_layer_height,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_i16(self.ground_height)?;
        writer.write_u16(self.water_level)?;
        writer.write_u8(self.ground_texture_and_flags)?;
        writer.write_u8(self.texture_details)?;
        writer.write_u8(self.cliff_texture_and_layer_height)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TerrainFile {
    id: String,
    version: u32,
    main_tileset: u8,
    custom_tileset: bool,
    // from 32 bit integer
    ground_tilesets: Vec<String>,
    // max 16 [4]
    cliff_tilesets: Vec<String>,
    // max 16 [4]
    my_height: u32,
    mx_width: u32,
    center_offset_x: f32,
    center_offset_y: f32,
    tilepoints: Vec<TilePoint>, // [Mx*My]
}

impl TerrainFile {
    pub const FILE_NAME: &str = MAP_TERRAIN;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map.read_file(MAP_TERRAIN).map_err(TerrainError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(TerrainError::InitReader)?;
        let terrain = reader
            .read::<TerrainFile>()
            .map_err(TerrainError::Parsing)?;
        Ok(terrain)
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer.write(self).map_err(TerrainError::SaveError)?;
        Ok(writer)
    }
}

impl BinaryConverter for TerrainFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let id = reader.read_string_utf8_safe(4)?;
        let version = reader.read_u32()?;
        let main_tileset = reader.read_u8()?;
        let custom_tileset = reader.read_u32()? == 1;

        let count_ground_tiles = reader.read_u32()?; //TODO Warning for > 16
        let mut ground_tilesets: Vec<String> = Vec::new();
        for _i in 0..count_ground_tiles {
            ground_tilesets.push(reader.read_string_utf8_safe(4)?)
        }
        let count_cliff_tiles = reader.read_u32()?; //TODO Warning for > 16
        let mut cliff_tilesets: Vec<String> = Vec::new();
        for _i in 0..count_cliff_tiles {
            cliff_tilesets.push(reader.read_string_utf8_safe(4)?);
        }

        let my_height = reader.read_u32()?;
        let mx_width = reader.read_u32()?;
        let center_offset_x = reader.read_f32()?;
        let center_offset_y = reader.read_f32()?;
        let count_tilepoints: usize = (mx_width * my_height) as usize;
        let tilepoints = reader.read_vec::<TilePoint>(count_tilepoints)?;

        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_TERRAIN,
            reader.size() - reader.pos() as usize
        );
        Ok(TerrainFile {
            id,
            version,
            main_tileset,
            custom_tileset,
            ground_tilesets,
            cliff_tilesets,
            my_height,
            mx_width,
            center_offset_x,
            center_offset_y,
            tilepoints,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        // Write header
        writer.write_string_utf8(&self.id)?;
        writer.write_u32(self.version)?;
        writer.write_u8(self.main_tileset)?;
        writer.write_u32(if self.custom_tileset { 1 } else { 0 })?;

        // Write ground tilesets
        writer.write_u32(self.ground_tilesets.len() as u32)?;
        for tileset in &self.ground_tilesets {
            writer.write_string_utf8(tileset)?;
        }

        // Write cliff tilesets
        writer.write_u32(self.cliff_tilesets.len() as u32)?;
        for tileset in &self.cliff_tilesets {
            writer.write_string_utf8(tileset)?;
        }

        // Write dimensions and offsets
        writer.write_u32(self.my_height)?;
        writer.write_u32(self.mx_width)?;
        writer.write_f32(self.center_offset_x)?;
        writer.write_f32(self.center_offset_y)?;

        // Write tilepoints
        for tilepoint in &self.tilepoints {
            tilepoint.write(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod terrain_tests {
    use super::*;
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;

    #[test]
    fn test_tilepoint_round_trip() {
        // Create a test TilePoint
        let original = TilePoint {
            ground_height: 123,
            water_level: 0x4000 | 45, // boundary flag set + water level 45
            ground_texture_and_flags: 0x70, // ground texture 7, with some flags
            texture_details: 42,
            cliff_texture_and_layer_height: 0x85, // cliff texture 5, layer height 8
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = TilePoint::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.ground_height, reconstructed.ground_height);
        assert_eq!(original.water_level, reconstructed.water_level);
        assert_eq!(
            original.ground_texture_and_flags,
            reconstructed.ground_texture_and_flags
        );
        assert_eq!(original.texture_details, reconstructed.texture_details);
        assert_eq!(
            original.cliff_texture_and_layer_height,
            reconstructed.cliff_texture_and_layer_height
        );

        // Verify helper methods work correctly
        assert_eq!(original.boundary_flag(), reconstructed.boundary_flag());
        assert_eq!(original.get_water_level(), reconstructed.get_water_level());
        assert_eq!(original.ground_texture(), reconstructed.ground_texture());
    }

    #[test]
    fn test_terrain_file_round_trip() {
        let original = TerrainFile {
            id: "W3E!".to_string(),
            version: 11,
            main_tileset: 0,
            custom_tileset: true,
            ground_tilesets: vec!["Ashe".to_string(), "Barr".to_string()],
            cliff_tilesets: vec!["Ashe".to_string()],
            my_height: 2,
            mx_width: 2,
            center_offset_x: 1.5,
            center_offset_y: 2.5,
            tilepoints: vec![
                TilePoint {
                    ground_height: 100,
                    water_level: 50,
                    ground_texture_and_flags: 0x10,
                    texture_details: 1,
                    cliff_texture_and_layer_height: 0x20,
                },
                TilePoint {
                    ground_height: 200,
                    water_level: 0x4000 | 75, // boundary flag set
                    ground_texture_and_flags: 0x30,
                    texture_details: 2,
                    cliff_texture_and_layer_height: 0x40,
                },
                TilePoint {
                    ground_height: 150,
                    water_level: 25,
                    ground_texture_and_flags: 0x20,
                    texture_details: 3,
                    cliff_texture_and_layer_height: 0x30,
                },
                TilePoint {
                    ground_height: 175,
                    water_level: 100,
                    ground_texture_and_flags: 0x50,
                    texture_details: 4,
                    cliff_texture_and_layer_height: 0x60,
                },
            ],
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = TerrainFile::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.id, reconstructed.id);
        assert_eq!(original.version, reconstructed.version);
        assert_eq!(original.main_tileset, reconstructed.main_tileset);
        assert_eq!(original.custom_tileset, reconstructed.custom_tileset);
        assert_eq!(original.ground_tilesets, reconstructed.ground_tilesets);
        assert_eq!(original.cliff_tilesets, reconstructed.cliff_tilesets);
        assert_eq!(original.my_height, reconstructed.my_height);
        assert_eq!(original.mx_width, reconstructed.mx_width);
        assert_eq!(original.center_offset_x, reconstructed.center_offset_x);
        assert_eq!(original.center_offset_y, reconstructed.center_offset_y);
        assert_eq!(original.tilepoints.len(), reconstructed.tilepoints.len());

        for (orig, recon) in original
            .tilepoints
            .iter()
            .zip(reconstructed.tilepoints.iter())
        {
            assert_eq!(orig.ground_height, recon.ground_height);
            assert_eq!(orig.water_level, recon.water_level);
            assert_eq!(
                orig.ground_texture_and_flags,
                recon.ground_texture_and_flags
            );
            assert_eq!(orig.texture_details, recon.texture_details);
            assert_eq!(
                orig.cliff_texture_and_layer_height,
                recon.cliff_texture_and_layer_height
            );
        }
    }

    #[test]
    fn test_read_no_failure() {
        let resources_path = crate::get_resources_path();
        let map_path = format!("{resources_path}/Scenario/Sandbox_1.w3x");

        let mut map_archive =
            wce_formats::MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let terrain_file =
            TerrainFile::read_file(&mut map_archive).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(terrain_file.id, "W3E!");
        assert!(terrain_file.version > 0);
        assert_eq!(
            terrain_file.tilepoints.len(),
            (terrain_file.mx_width * terrain_file.my_height) as usize
        );
    }

    #[test]
    fn test_real_terrain_file_round_trip() {
        let resources_path = crate::get_resources_path();
        let map_path = format!("{resources_path}/Scenario/Sandbox_1.w3x");

        let mut map_archive =
            wce_formats::MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let original_terrain =
            TerrainFile::read_file(&mut map_archive).unwrap_or_else(|e| panic!("{}", e));
        println!("Testing round-trip with real terrain file...");

        let mut writer = BinaryWriter::new();
        original_terrain
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed_terrain =
            TerrainFile::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original_terrain.id, reconstructed_terrain.id);
        assert_eq!(original_terrain.version, reconstructed_terrain.version);
        assert_eq!(
            original_terrain.main_tileset,
            reconstructed_terrain.main_tileset
        );
        assert_eq!(
            original_terrain.custom_tileset,
            reconstructed_terrain.custom_tileset
        );
        assert_eq!(
            original_terrain.ground_tilesets,
            reconstructed_terrain.ground_tilesets
        );
        assert_eq!(
            original_terrain.cliff_tilesets,
            reconstructed_terrain.cliff_tilesets
        );
        assert_eq!(original_terrain.my_height, reconstructed_terrain.my_height);
        assert_eq!(original_terrain.mx_width, reconstructed_terrain.mx_width);
        assert_eq!(
            original_terrain.center_offset_x,
            reconstructed_terrain.center_offset_x
        );
        assert_eq!(
            original_terrain.center_offset_y,
            reconstructed_terrain.center_offset_y
        );
        assert_eq!(
            original_terrain.tilepoints.len(),
            reconstructed_terrain.tilepoints.len()
        );

        for (i, (orig, recon)) in original_terrain
            .tilepoints
            .iter()
            .zip(reconstructed_terrain.tilepoints.iter())
            .enumerate()
        {
            assert_eq!(
                orig.ground_height, recon.ground_height,
                "Tilepoint {i}: ground_height mismatch"
            );
            assert_eq!(
                orig.water_level, recon.water_level,
                "Tilepoint {i}: water_level mismatch"
            );
            assert_eq!(
                orig.ground_texture_and_flags, recon.ground_texture_and_flags,
                "Tilepoint {i}: ground_texture_and_flags mismatch"
            );
            assert_eq!(
                orig.texture_details, recon.texture_details,
                "Tilepoint {i}: texture_details mismatch"
            );
            assert_eq!(
                orig.cliff_texture_and_layer_height, recon.cliff_texture_and_layer_height,
                "Tilepoint {i}: cliff_texture_and_layer_height mismatch"
            );

            assert_eq!(
                orig.boundary_flag(),
                recon.boundary_flag(),
                "Tilepoint {i}: boundary_flag mismatch"
            );
            assert_eq!(
                orig.get_water_level(),
                recon.get_water_level(),
                "Tilepoint {i}: get_water_level mismatch"
            );
            assert_eq!(
                orig.ground_texture(),
                recon.ground_texture(),
                "Tilepoint {i}: ground_texture mismatch"
            );
        }

        println!("✅ Real terrain file round-trip test passed successfully!");
        println!(
            "   Verified {} tilepoints",
            original_terrain.tilepoints.len()
        );
    }
}
