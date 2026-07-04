use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError, WriteError};

use crate::globals::MAP_MENU_MINIMAP;
use crate::MapError;

type RGBA = Vec<u8>;
#[derive(Debug, Error)]
pub enum MenuMinimapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse menu minimap datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save menu minimap data. {0}")]
    SaveError(WriteError),
}
impl From<MenuMinimapError> for MapError {
    fn from(value: MenuMinimapError) -> Self {
        MapError::MenuMinimap(value)
    }
}

#[derive(Debug)]
pub struct MMPDataset {
    icon_type: u32,
    x: i32,
    y: i32,
    color: RGBA,
}
impl BinaryConverter for MMPDataset {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let icon_type = reader.read_u32()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let color = reader.read_bytes(4)?;
        Ok(MMPDataset {
            icon_type,
            x,
            y,
            color,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_u32(self.icon_type)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_bytes(&self.color)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MMPFile {
    unknown: i32,
    datasets: Vec<MMPDataset>,
}

impl BinaryConverter for MMPFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let unknown = reader.read_i32()?;
        let count_dataset = reader.read_i32()? as usize;
        let datasets = reader.read_vec::<MMPDataset>(count_dataset)?;
        Ok(MMPFile { unknown, datasets })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_i32(self.unknown)?;
        writer.write_i32(self.datasets.len() as i32)?;
        writer.write_vec(&self.datasets)?;
        Ok(())
    }
}

impl MMPFile {
    pub const FILE_NAME: &str = MAP_MENU_MINIMAP;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map
            .read_file(MAP_MENU_MINIMAP)
            .map_err(MenuMinimapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(MenuMinimapError::InitReader)?;
        let mmp = reader
            .read::<MMPFile>()
            .map_err(MenuMinimapError::Parsing)?;
        Ok(mmp)
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer.write(self).map_err(MenuMinimapError::SaveError)?;
        Ok(writer)
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

#[cfg(test)]
mod mmp_test {
    use wce_formats::MapArchive;

    use crate::{get_resources_path, mmp_file::MMPFile};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        MMPFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));
    }
}
