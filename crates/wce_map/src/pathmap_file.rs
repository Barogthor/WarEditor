use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::{BinaryConverter, ReadError};
use wce_formats::{MapArchive, MpqError, WriteError};

use crate::globals::MAP_PATH_MAP;
use crate::MapError;

#[derive(Debug, Error)]
pub enum PathmapError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse pathmap datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save pathmap data. {0}")]
    SaveError(WriteError),
}
impl From<PathmapError> for MapError {
    fn from(value: PathmapError) -> Self {
        MapError::PathingMap(value)
    }
}

type Flag = u8;
#[derive(Debug)]
pub struct PathCell {
    flags: Flag,
}
impl PathCell {
    pub fn walkable(&self) -> bool {
        self.flags & 0x02 == 0
    }
    pub fn flyable(&self) -> bool {
        self.flags & 0x04 == 0
    }
    pub fn buildable(&self) -> bool {
        self.flags & 0x08 == 0
    }
    pub fn blight(&self) -> bool {
        self.flags & 0x20 == 0
    }
    pub fn water(&self) -> bool {
        self.flags & 0x40 == 0
    }
    pub fn normal(&self) -> bool {
        self.flags & 0x80 == 0 || !self.blight()
    }

    pub fn update_flags(&mut self, value: Flag) {
        self.flags = value;
    }
}

#[derive(Debug)]
pub struct PathMapFile {
    id: String,
    version: u32,
    pathmap_width: u32,
    pathmap_height: u32,
    pathing: Vec<PathCell>,
}

impl PathMapFile {
    pub const FILE_NAME: &str = MAP_PATH_MAP;

    pub fn read_file(map: &mut MapArchive) -> Result<Self, MapError> {
        let buffer = map
            .read_file(MAP_PATH_MAP)
            .map_err(PathmapError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(PathmapError::InitReader)?;
        let pathmaps = reader
            .read::<PathMapFile>()
            .map_err(PathmapError::Parsing)?;
        Ok(pathmaps)
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer.write(self).map_err(PathmapError::SaveError)?;
        Ok(writer)
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for PathMapFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let id = reader.read_string_utf8_safe(4)?;
        let version = reader.read_u32()?;
        let pathmap_width = reader.read_u32()?;
        let pathmap_height = reader.read_u32()?;
        let mut pathing: Vec<PathCell> = Vec::new();
        for _i in 0..pathmap_width * pathmap_height {
            let flags = reader.read_u8()?;

            //            println!("{:x}",flags);
            pathing.push(PathCell { flags });
        }
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_PATH_MAP,
            reader.size() - reader.pos() as usize
        );
        Ok(PathMapFile {
            id,
            version,
            pathmap_width,
            pathmap_height,
            pathing,
        })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_string_utf8(&self.id)?;
        writer.write_u32(self.version)?;
        writer.write_u32(self.pathmap_width)?;
        writer.write_u32(self.pathmap_height)?;

        for cell in &self.pathing {
            writer.write_u8(cell.flags)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod pathmap_test {
    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::BinaryConverter;
    use wce_formats::MapArchive;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use crate::{get_resources_path, pathmap_file::PathMapFile};

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        PathMapFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn pathmap_basic_test() {
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let pathmap = PathMapFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));

        // Basic validation
        assert_eq!(pathmap.id, "MP3W"); // Standard pathmap ID
        assert!(pathmap.pathmap_width > 0);
        assert!(pathmap.pathmap_height > 0);
        assert_eq!(
            pathmap.pathing.len(),
            (pathmap.pathmap_width * pathmap.pathmap_height) as usize
        );
    }

    #[test]
    fn write_read_roundtrip_test() {
        // Read original data from map archive
        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let original_pathmap = PathMapFile::read_file(&mut map).unwrap_or_else(|e| panic!("{}", e));

        // Write to buffer
        let mut writer = BinaryWriter::new();
        original_pathmap
            .write(&mut writer)
            .expect("Failed to write PathMapFile");

        println!("Written buffer size: {}", writer.into_buffer().len());

        // Write to buffer again for comparison
        let mut writer = BinaryWriter::new();
        original_pathmap
            .write(&mut writer)
            .expect("Failed to write PathMapFile");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_pathmap = reader
            .read::<PathMapFile>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare basic properties
        assert_eq!(original_pathmap.id, written_pathmap.id);
        assert_eq!(original_pathmap.version, written_pathmap.version);
        assert_eq!(
            original_pathmap.pathmap_width,
            written_pathmap.pathmap_width
        );
        assert_eq!(
            original_pathmap.pathmap_height,
            written_pathmap.pathmap_height
        );
        assert_eq!(
            original_pathmap.pathing.len(),
            written_pathmap.pathing.len()
        );

        // Compare each path cell's flags
        for (original, written) in original_pathmap
            .pathing
            .iter()
            .zip(written_pathmap.pathing.iter())
        {
            assert_eq!(original.flags, written.flags);
        }
    }

    #[test]
    fn test_pathcell_flag_interpretation() {
        // Test PathCell flag interpretation methods
        let walkable_cell = crate::pathmap_file::PathCell { flags: 0x00 }; // All bits 0 = walkable/flyable/buildable
        let blocked_cell = crate::pathmap_file::PathCell { flags: 0xFF }; // All bits 1 = blocked

        // Test walkable cell (flags & 0x02 == 0)
        assert!(walkable_cell.walkable());
        assert!(walkable_cell.flyable());
        assert!(walkable_cell.buildable());

        // Test blocked cell - these should be false since flags & bit != 0
        assert!(!blocked_cell.walkable());
        assert!(!blocked_cell.flyable());
        assert!(!blocked_cell.buildable());

        // Test specific flag combinations
        let water_cell = crate::pathmap_file::PathCell { flags: 0x40 }; // Water flag set
        assert!(!water_cell.water()); // Should be false since flags & 0x40 != 0

        let blight_cell = crate::pathmap_file::PathCell { flags: 0x20 }; // Blight flag set
        assert!(!blight_cell.blight()); // Should be false since flags & 0x20 != 0
    }

    #[test]
    fn test_pathcell_flag_updates() {
        let mut cell = crate::pathmap_file::PathCell { flags: 0x00 };

        // Initial state - should be walkable
        assert!(cell.walkable());

        // Update flags to make it unwalkable
        cell.update_flags(0x02);
        assert!(!cell.walkable());

        // Update flags to make it walkable again
        cell.update_flags(0x00);
        assert!(cell.walkable());
    }

    #[test]
    fn test_create_custom_pathmap() {
        // Create a custom PathMapFile for testing
        let custom_pathmap = PathMapFile {
            id: "MP3W".to_string(),
            version: 0,
            pathmap_width: 2,
            pathmap_height: 2,
            pathing: vec![
                crate::pathmap_file::PathCell { flags: 0x00 }, // Walkable
                crate::pathmap_file::PathCell { flags: 0x02 }, // Not walkable
                crate::pathmap_file::PathCell { flags: 0x04 }, // Not flyable
                crate::pathmap_file::PathCell { flags: 0x08 }, // Not buildable
            ],
        };

        // Write to buffer
        let mut writer = BinaryWriter::new();
        custom_pathmap
            .write(&mut writer)
            .expect("Failed to write custom PathMapFile");

        // Read back from buffer
        let buffer = writer.into_buffer();
        let mut reader = BinaryReader::new(buffer);
        let written_pathmap = reader
            .read::<PathMapFile>()
            .unwrap_or_else(|e| panic!("Failed to read back: {}", e));

        // Compare
        assert_eq!(custom_pathmap.id, written_pathmap.id);
        assert_eq!(custom_pathmap.version, written_pathmap.version);
        assert_eq!(custom_pathmap.pathmap_width, written_pathmap.pathmap_width);
        assert_eq!(
            custom_pathmap.pathmap_height,
            written_pathmap.pathmap_height
        );
        assert_eq!(custom_pathmap.pathing.len(), written_pathmap.pathing.len());

        // Test specific path cell properties
        assert!(written_pathmap.pathing[0].walkable()); // First cell should be walkable
        assert!(!written_pathmap.pathing[1].walkable()); // Second cell should not be walkable
        assert!(!written_pathmap.pathing[2].flyable()); // Third cell should not be flyable
        assert!(!written_pathmap.pathing[3].buildable()); // Fourth cell should not be buildable
    }
}
