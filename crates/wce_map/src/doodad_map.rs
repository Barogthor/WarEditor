//! Parser and writer for `war3map.doo` (terrain doodad and destructable placements).

use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::GameVersion::{self, RoC, TFT};
use wce_formats::{BinaryConverter, BinaryConverterVersion};
use wce_formats::{MapArchive, MpqError, ReadError, WriteError};

use crate::doodad_map::DestructableFlag::{InvisibleNonSolid, VisibleNonSolid, VisibleSolid};
use crate::globals::MAP_TERRAIN_DOODADS;
use crate::unit_map::Drops;
use crate::MapError;

pub type Radian = f32;

#[derive(Debug, Error)]
pub enum DoodadError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse doodads datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save doodads data. {0}")]
    SaveError(WriteError),
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum DestructableFlag {
    InvisibleNonSolid = 0,
    VisibleNonSolid = 1,
    VisibleSolid = 2,
}

impl DestructableFlag {
    pub fn from(value: u8) -> Result<Self, String> {
        match value {
            0 => Ok(InvisibleNonSolid),
            1 => Ok(VisibleNonSolid),
            2 => Ok(VisibleSolid),
            _ => Err(format!("Unknown destructable flag {value}")),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Destructable {
    model_id: String,
    variation: u32,
    coord_x: f32,
    coord_y: f32,
    coord_z: f32,
    angle: Radian,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    flags: u8,
    life: u8,
    drops: Drops,
    creation_id: u32,
}

impl BinaryConverterVersion for Destructable {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let model_id = reader.read_string_utf8_safe(4)?;
        let variation = reader.read_u32()?;
        let coord_x = reader.read_f32()?;
        let coord_y = reader.read_f32()?;
        let coord_z = reader.read_f32()?;
        let angle = reader.read_f32()?;
        let scale_x = reader.read_f32()?;
        let scale_y = reader.read_f32()?;
        let scale_z = reader.read_f32()?;
        let flags = reader.read_u8()?;
        // let flags = DestructableFlag::from(flags).map_err(ReadError::Reason)?;
        let life = reader.read_u8()?;
        let drops = Self::load_drops(reader, game_version)?;

        let creation_id = reader.read_u32()?;
        Ok(Destructable {
            model_id,
            variation,
            coord_x,
            coord_y,
            coord_z,
            angle,
            scale_x,
            scale_y,
            scale_z,
            flags,
            life,
            drops,
            creation_id,
        })
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        writer.write_string_utf8(&self.model_id)?;
        writer.write_u32(self.variation)?;
        writer.write_f32(self.coord_x)?;
        writer.write_f32(self.coord_y)?;
        writer.write_f32(self.coord_z)?;
        writer.write_f32(self.angle)?;
        writer.write_f32(self.scale_x)?;
        writer.write_f32(self.scale_y)?;
        writer.write_f32(self.scale_z)?;
        writer.write_u8(self.flags)?;
        writer.write_u8(self.life)?;
        Self::write_drops(&self.drops, writer, game_version)?;
        writer.write_u32(self.creation_id)?;
        Ok(())
    }
}

impl Destructable {
    fn load_drops(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Drops> {
        let drops = match *game_version {
            RoC => Drops::Empty,
            _ => Drops::read_version(reader, game_version)?,
        };
        Ok(drops)
    }

    fn write_drops(
        drops: &Drops,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        match *game_version {
            RoC => {
                // RoC destructables don't have drop data at all
            }
            _ => {
                writer.write_version(drops, game_version)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct SpecialDoodad {
    model_id: String,
    coord_x: f32,
    coord_y: f32,
    coord_z: f32,
}

impl BinaryConverter for SpecialDoodad {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let model_id = reader.read_string_utf8(4)?;
        let coord_x = reader.read_f32()?;
        let coord_y = reader.read_f32()?;
        let coord_z = reader.read_f32()?;
        Ok(SpecialDoodad {
            model_id,
            coord_x,
            coord_y,
            coord_z,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_string_utf8(&self.model_id)?;
        writer.write_f32(self.coord_x)?;
        writer.write_f32(self.coord_y)?;
        writer.write_f32(self.coord_z)?;
        Ok(())
    }
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq)]
pub struct DoodadMap {
    id: String,
    version: GameVersion,
    #[derivative(PartialEq = "ignore")]
    subversion: u32,
    destructables: Vec<Destructable>,
    special_doodad_version: u32,
    special_doodads: Vec<SpecialDoodad>,
}

impl DoodadMap {
    pub const FILE_NAME: &str = MAP_TERRAIN_DOODADS;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map
            .read_file(MAP_TERRAIN_DOODADS)
            .map_err(DoodadError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(DoodadError::InitReader)?;
        let doodads = reader.read::<DoodadMap>().map_err(DoodadError::Parsing)?;
        Ok(doodads)
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer.write(self).map_err(DoodadError::SaveError)?;
        Ok(writer)
    }
}

impl BinaryConverter for DoodadMap {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let id = reader.read_string_utf8(4)?;
        let version = reader.read_u32()?;
        let version = to_game_version(version).map_err(ReadError::Reason)?;
        let subversion = reader.read_u32()?;
        let count_destructables = reader.read_u32()?;
        let destructables =
            reader.read_vec_version::<Destructable>(count_destructables as usize, &version)?;
        let special_doodad_version = reader.read_u32()?;
        let count_special_doodads = reader.read_u32()?;
        let special_doodads = reader.read_vec::<SpecialDoodad>(count_special_doodads as usize)?;
        if reader.size() != reader.pos() as usize {
            return Err(ReadError::TrailingBytes {
                file: MAP_TERRAIN_DOODADS.into(),
                expected: reader.size(),
                actual: reader.pos() as usize,
            });
        }
        Ok(DoodadMap {
            id,
            version,
            subversion,
            destructables,
            special_doodad_version,
            special_doodads,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_string_utf8(&self.id)?;
        writer.write_u32(from_game_version(&self.version))?;
        writer.write_u32(self.subversion)?;
        writer.write_u32(self.destructables.len() as u32)?;
        writer.write_vec_version(&self.destructables, &self.version)?;
        writer.write_u32(self.special_doodad_version)?;
        writer.write_u32(self.special_doodads.len() as u32)?;
        writer.write_vec(&self.special_doodads)?;
        Ok(())
    }
}

fn from_game_version(game_version: &GameVersion) -> u32 {
    match game_version {
        RoC => 7,
        TFT => 8,
        GameVersion::Reforged => unimplemented!(),
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, String> {
    match value {
        7 => Ok(RoC),
        8 => Ok(TFT),
        _ => Err(format!("Unknown or unsupported game version '{value}'")),
    }
}

#[cfg(test)]
mod doodads_test {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::BinaryConverter;
    use wce_formats::GameVersion::RoC;
    use wce_formats::ReadError;

    use crate::{
        doodad_map::{Destructable, DoodadMap, Drops},
        get_resources_path,
    };

    fn mock_destructable_roc() -> Vec<Destructable> {
        vec![
            Destructable {
                model_id: "LTlt".to_string(),
                variation: 0,
                coord_x: -1280.0,
                coord_y: 1600.0,
                coord_z: 0.0,
                angle: 4.712389,
                scale_x: 0.9766412,
                scale_y: 0.9766412,
                scale_z: 0.9766412,
                flags: 2,
                life: 100,
                drops: Drops::Empty,
                creation_id: 0,
            },
            Destructable {
                model_id: "LRrk".to_string(),
                variation: 4,
                coord_x: 1088.0,
                coord_y: 1216.0,
                coord_z: 79.5,
                angle: 0.5061455,
                scale_x: 0.9194495,
                scale_y: 0.9194495,
                scale_z: 0.9194495,
                flags: 2,
                life: 255,
                drops: Drops::Empty,
                creation_id: 55,
            },
            Destructable {
                model_id: "LRrk".to_string(),
                variation: 0,
                coord_x: 960.0,
                coord_y: 1280.0,
                coord_z: 46.5,
                angle: 5.969026,
                scale_x: 1.0382886,
                scale_y: 1.0382886,
                scale_z: 1.0382886,
                flags: 2,
                life: 255,
                drops: Drops::Empty,
                creation_id: 168,
            },
        ]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure_roc() {
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_roc/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file).unwrap();
        let _doodad_map = reader.read::<DoodadMap>();
    }

    #[test]
    fn check_roc() {
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_roc/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file).unwrap();
        let doodad_map = reader.read::<DoodadMap>().unwrap();
        let mock_destructables = mock_destructable_roc();
        assert_eq!(doodad_map.id, "W3do".to_string());
        assert_eq!(doodad_map.version, RoC);
        let destructables: Vec<Destructable> = doodad_map
            .destructables
            .into_iter()
            .filter(|destructable| {
                let creat_id = destructable.creation_id;
                matches!(creat_id, 168 | 55 | 0)
            })
            .collect();
        assert_eq!(destructables, mock_destructables);
    }

    #[test]
    fn no_failure_tft() {
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_tft/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file).unwrap();
        let _doodad_map = reader
            .read::<DoodadMap>()
            .unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn write_read_roundtrip_roc() {
        // Read original data
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_roc/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file).unwrap();
        let original_map = reader
            .read::<DoodadMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write DoodadMap");

        println!("Original buffer size: {}", reader.size());
        println!("Written buffer size: {}", writer.into_buffer().len());

        // Write to buffer again for comparison
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write DoodadMap");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_map = reader
            .read::<DoodadMap>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare
        assert_eq!(original_map.id, written_map.id);
        assert_eq!(original_map.version, written_map.version);
        assert_eq!(original_map.subversion, written_map.subversion);
        assert_eq!(
            original_map.destructables.len(),
            written_map.destructables.len()
        );
        assert_eq!(
            original_map.special_doodad_version,
            written_map.special_doodad_version
        );
        assert_eq!(
            original_map.special_doodads.len(),
            written_map.special_doodads.len()
        );

        // Compare individual destructables
        for (original, written) in original_map
            .destructables
            .iter()
            .zip(written_map.destructables.iter())
        {
            assert_eq!(original.model_id, written.model_id);
            assert_eq!(original.creation_id, written.creation_id);
            assert_eq!(original.variation, written.variation);
            assert_eq!(original.flags, written.flags);
            assert_eq!(original.life, written.life);
        }

        // Compare special doodads
        for (original, written) in original_map
            .special_doodads
            .iter()
            .zip(written_map.special_doodads.iter())
        {
            assert_eq!(original.model_id, written.model_id);
        }
    }

    #[test]
    fn write_read_roundtrip_tft() {
        // Read original data
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_tft/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file).unwrap();
        let original_map = reader
            .read::<DoodadMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write DoodadMap");

        // Read back from buffer
        let mut reader = BinaryReader::new(writer.into_buffer());
        let written_map = reader
            .read::<DoodadMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Compare
        assert_eq!(original_map.id, written_map.id);
        assert_eq!(original_map.version, written_map.version);
        assert_eq!(original_map.subversion, written_map.subversion);
        assert_eq!(
            original_map.destructables.len(),
            written_map.destructables.len()
        );
        assert_eq!(
            original_map.special_doodad_version,
            written_map.special_doodad_version
        );
        assert_eq!(
            original_map.special_doodads.len(),
            written_map.special_doodads.len()
        );
    }

    #[test]
    fn trailing_bytes_after_valid_doodads_returns_error_not_panic() {
        // Read valid original data, same as write_read_roundtrip_roc
        let mut doodad_file = File::open(get_path("Scenario/Sandbox_roc/war3map.doo"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut doodad_file).unwrap();
        let original_map = reader
            .read::<DoodadMap>()
            .unwrap_or_else(|e| panic!("{}", e));

        // Serialize back to a valid, fully-consumable buffer
        let mut writer = BinaryWriter::new();
        original_map
            .write(&mut writer)
            .expect("Failed to write DoodadMap");
        let mut bytes = writer.into_buffer();

        // Append junk bytes so pos() < size() once the real record is fully read
        bytes.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);

        let mut reader = BinaryReader::new(bytes);
        let result = reader.read::<DoodadMap>();
        assert!(
            matches!(result, Err(ReadError::TrailingBytes { .. })),
            "expected TrailingBytes, got {result:?}"
        );
    }
}
