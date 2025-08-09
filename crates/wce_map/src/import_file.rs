use std::convert::TryFrom;
use std::ffi::CString;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::GameVersion::RoC;
use wce_formats::{BinaryConverterVersion, GameVersion};
use wce_formats::{MapArchive, MpqError, ReadError, WriteError};

use crate::globals::MAP_IMPORT_LIST;
use crate::MapError;

type ImportPath = Vec<(ImportPathType, CString)>;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse imports datas. {0}")]
    Parsing(ReadError),
    #[error("Failed to save import data. {0}")]
    SaveError(WriteError),
}

impl From<ImportError> for MapError {
    fn from(value: ImportError) -> Self {
        MapError::Import(value)
    }
}

#[derive(Debug)]
pub struct ImportFile {
    files: ImportPath,
}

impl ImportFile {
    pub const FILE_NAME: &str = MAP_IMPORT_LIST;

    pub fn read_file(
        map: &mut MapArchive,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        let file = map.read_file(MAP_IMPORT_LIST);
        match file {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(ImportError::InitReader)?;
                Self::read_opt(&mut reader, game_version)
            }
            _ => Ok(None),
        }
    }

    fn read_opt(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        if reader.size() > 0 {
            let v = reader
                .read_version::<ImportFile>(game_version)
                .map_err(ImportError::Parsing)?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    pub fn prepare_write(&self, game_version: &GameVersion) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer
            .write_version(self, game_version)
            .map_err(ImportError::SaveError)?;
        Ok(writer)
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverterVersion for ImportFile {
    fn read_version(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self>
    where
        Self: Sized,
    {
        reader.skip(4); // Roc and TFT maps are 1
        let count = reader.read_u32()?;
        let mut files: ImportPath = vec![];
        for _ in 0..count {
            let path_type = reader.read_u8()?;
            let path_type = match *game_version {
                RoC => ImportPathType::RoC,
                _ => ImportPathType::from_u8(path_type).ok_or_else(|| {
                    ReadError::Reason(format!(
                        "Invalid import type '{path_type}' at {}/{}.",
                        reader.pos(),
                        reader.size()
                    ))
                })?,
            };
            let path = reader.read_c_string()?;
            files.push((path_type, path));
        }

        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_IMPORT_LIST,
            reader.size() - reader.pos() as usize
        );
        Ok(ImportFile { files })
    }

    fn write_version(
        &self,
        writer: &mut BinaryWriter,
        _game_version: &GameVersion,
    ) -> WriteResult<()> {
        if !self.files.is_empty() {
            writer.write_u32(1)?;
            writer.write_u32(self.files.len() as u32)?;
            for (path_type, path) in &self.files {
                writer.write_u8(path_type.to_u8())?;
                writer.write_c_string(path)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportPathType {
    STANDARD(u8),
    CUSTOM(u8),
    RoC,
}

impl ImportPathType {
    pub fn from_u8(n: u8) -> Option<ImportPathType> {
        match n {
            5 | 8 => Some(ImportPathType::STANDARD(n)),
            10 | 13 => Some(ImportPathType::CUSTOM(n)),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ImportPathType::STANDARD(n) => *n,
            ImportPathType::CUSTOM(n) => *n,
            ImportPathType::RoC => 0,
        }
    }
}

#[cfg(test)]
mod import_file_test {
    use std::ffi::CString;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::{BinaryConverterVersion, GameVersion};

    use crate::import_file::{ImportFile, ImportPathType};

    fn mock_import_files_tft() -> Vec<(ImportPathType, CString)> {
        vec![
            (
                ImportPathType::STANDARD(5),
                CString::new("Units\\Human\\Footman\\Footman.mdx").unwrap(),
            ),
            (
                ImportPathType::CUSTOM(10),
                CString::new("CustomTextures\\Grass.blp").unwrap(),
            ),
        ]
    }

    fn mock_import_files_roc() -> Vec<(ImportPathType, CString)> {
        vec![
            (
                ImportPathType::RoC,
                CString::new("Units\\Orc\\Grunt\\Grunt.mdx").unwrap(),
            ),
            (
                ImportPathType::RoC,
                CString::new("Textures\\Stone.blp").unwrap(),
            ),
        ]
    }

    #[test]
    fn test_import_file_round_trip_tft() {
        let original = ImportFile {
            files: mock_import_files_tft(),
        };

        let mut writer = BinaryWriter::new();
        original
            .write_version(&mut writer, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = ImportFile::read_version(&mut reader, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.files.len(), reconstructed.files.len());

        for ((orig_type, orig_path), (recon_type, recon_path)) in
            original.files.iter().zip(reconstructed.files.iter())
        {
            assert_eq!(orig_type, recon_type);
            assert_eq!(orig_path, recon_path);
        }
    }

    #[test]
    fn test_import_file_round_trip_roc() {
        let original = ImportFile {
            files: mock_import_files_roc(),
        };

        let mut writer = BinaryWriter::new();
        original
            .write_version(&mut writer, &GameVersion::RoC)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = ImportFile::read_version(&mut reader, &GameVersion::RoC)
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.files.len(), reconstructed.files.len());

        for ((orig_type, orig_path), (recon_type, recon_path)) in
            original.files.iter().zip(reconstructed.files.iter())
        {
            // For RoC, all path types should be RoC
            assert_eq!(*recon_type, ImportPathType::RoC);
            assert_eq!(orig_path, recon_path);
        }
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_import_file = ImportFile { files: vec![] };

        let mut writer = BinaryWriter::new();
        empty_import_file
            .write_version(&mut writer, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(buffer.len(), 0, "Empty imports should produce empty buffer");

        let mut reader = BinaryReader::new(buffer);
        assert!(
            ImportFile::read_opt(&mut reader, &GameVersion::TFT)
                .unwrap_or_else(|e| panic!("{}", e))
                .is_none(),
            "Empty buffer shouldn't return import file."
        );
    }

    #[test]
    fn test_import_path_type_conversions() {
        // Test STANDARD types
        assert_eq!(ImportPathType::STANDARD(5).to_u8(), 5);
        assert_eq!(ImportPathType::STANDARD(8).to_u8(), 8);

        // Test CUSTOM types
        assert_eq!(ImportPathType::CUSTOM(10).to_u8(), 10);
        assert_eq!(ImportPathType::CUSTOM(13).to_u8(), 13);

        // Test RoC type
        assert_eq!(ImportPathType::RoC.to_u8(), 0);

        // Test round-trip conversions
        assert_eq!(
            ImportPathType::from_u8(5),
            Some(ImportPathType::STANDARD(5))
        );
        assert_eq!(
            ImportPathType::from_u8(8),
            Some(ImportPathType::STANDARD(8))
        );
        assert_eq!(
            ImportPathType::from_u8(10),
            Some(ImportPathType::CUSTOM(10))
        );
        assert_eq!(
            ImportPathType::from_u8(13),
            Some(ImportPathType::CUSTOM(13))
        );
        assert_eq!(ImportPathType::from_u8(99), None);
    }

    #[test]
    fn test_single_import_file() {
        let single_import = ImportFile {
            files: vec![(
                ImportPathType::STANDARD(5),
                CString::new("Test\\File.mdx").unwrap(),
            )],
        };

        let mut writer = BinaryWriter::new();
        single_import
            .write_version(&mut writer, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = ImportFile::read_version(&mut reader, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(single_import.files.len(), 1);
        assert_eq!(reconstructed.files.len(), 1);
        assert_eq!(single_import.files[0].0, reconstructed.files[0].0);
        assert_eq!(single_import.files[0].1, reconstructed.files[0].1);
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = crate::get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn test_real_file_roc() {
        use wce_formats::MapArchive;

        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let import_file =
            ImportFile::read_file(&mut map, &GameVersion::RoC).unwrap_or_else(|e| panic!("{}", e));

        assert!(import_file.is_some(), "RoC map should have import file");
        let import_file = import_file.unwrap();

        assert_eq!(import_file.files.len(), 1);

        let (path_type, path) = &import_file.files[0];
        assert_eq!(*path_type, ImportPathType::RoC);
        assert_eq!(
            path.to_str().unwrap(),
            "Grid256.blp",
            "Expected Grid256.blp import"
        );
    }

    #[test]
    fn test_real_file_tft() {
        use wce_formats::MapArchive;

        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let import_file =
            ImportFile::read_file(&mut map, &GameVersion::TFT).unwrap_or_else(|e| panic!("{}", e));

        assert!(import_file.is_some(), "TFT map should have import file");
        let import_file = import_file.unwrap();

        assert_eq!(import_file.files.len(), 1);

        let (path_type, path) = &import_file.files[0];
        assert_eq!(*path_type, ImportPathType::STANDARD(8));
        assert_eq!(
            path.to_str().unwrap(),
            "Grid256.blp",
            "Expected Grid256.blp import"
        );
    }

    #[test]
    fn test_real_file_round_trip_roc() {
        use wce_formats::MapArchive;

        let map_path = get_path("Scenario/Sandbox_1.w3m");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let original = ImportFile::read_file(&mut map, &GameVersion::RoC)
            .unwrap_or_else(|e| panic!("{}", e))
            .expect("RoC map should have import file");

        // Write the loaded file
        let mut writer = BinaryWriter::new();
        original
            .write_version(&mut writer, &GameVersion::RoC)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        // Read it back
        let mut reader = BinaryReader::new(buffer);
        let reconstructed = ImportFile::read_version(&mut reader, &GameVersion::RoC)
            .unwrap_or_else(|e| panic!("{}", e));

        // Verify it matches the original
        assert_eq!(original.files.len(), reconstructed.files.len());

        for ((orig_type, orig_path), (recon_type, recon_path)) in
            original.files.iter().zip(reconstructed.files.iter())
        {
            // For RoC, the reconstructed type should always be RoC regardless of original
            assert_eq!(*recon_type, ImportPathType::RoC);
            assert_eq!(orig_path, recon_path);
        }
    }

    #[test]
    fn test_real_file_round_trip_tft() {
        use wce_formats::MapArchive;

        let map_path = get_path("Scenario/Sandbox_1.w3x");
        let mut map = MapArchive::open(map_path).unwrap_or_else(|e| panic!("{}", e));
        let original = ImportFile::read_file(&mut map, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e))
            .expect("TFT map should have import file");

        // Write the loaded file
        let mut writer = BinaryWriter::new();
        original
            .write_version(&mut writer, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        // Read it back
        let mut reader = BinaryReader::new(buffer);
        let reconstructed = ImportFile::read_version(&mut reader, &GameVersion::TFT)
            .unwrap_or_else(|e| panic!("{}", e));

        // Verify it matches the original
        assert_eq!(original.files.len(), reconstructed.files.len());

        for ((orig_type, orig_path), (recon_type, recon_path)) in
            original.files.iter().zip(reconstructed.files.iter())
        {
            assert_eq!(orig_type, recon_type);
            assert_eq!(orig_path, recon_path);
        }
    }
}
