//! Parser and writer for `war3map.w3c` (camera definitions).

use std::convert::TryFrom;

#[cfg(test)]
use pretty_assertions::assert_eq;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError, WriteError};

use crate::globals::MAP_CAMERAS;
use crate::MapError;

type Degree = f32;

#[derive(Debug, Error)]
pub enum CameraError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse cameras data. {0}")]
    Parsing(ReadError),
    #[error("Failed to save cameras data. {0}")]
    SaveError(WriteError),
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Default)]
pub struct Camera {
    x: f32,
    y: f32,
    z: f32,
    rotation: Degree,
    aoa: Degree,
    dist: f32,
    roll: f32,
    fov: Degree,
    far_clip: f32,
    #[derivative(Default(value = "100.0"))]
    unknown: f32,
    name: String,
}

impl BinaryConverter for Camera {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let mut camera = Self::default();
        camera.x = reader.read_f32()?;
        camera.y = reader.read_f32()?;
        camera.z = reader.read_f32()?;
        camera.rotation = reader.read_f32()?;
        camera.aoa = reader.read_f32()?;
        camera.dist = reader.read_f32()?;
        camera.roll = reader.read_f32()?;
        camera.fov = reader.read_f32()?;
        camera.far_clip = reader.read_f32()?;
        camera.unknown = reader.read_f32()?;
        camera.name = reader.read_c_string_converted()?;
        Ok(camera)
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_f32(self.x)?;
        writer.write_f32(self.y)?;
        writer.write_f32(self.z)?;
        writer.write_f32(self.rotation)?;
        writer.write_f32(self.aoa)?;
        writer.write_f32(self.dist)?;
        writer.write_f32(self.roll)?;
        writer.write_f32(self.fov)?;
        writer.write_f32(self.far_clip)?;
        writer.write_f32(self.unknown)?;
        writer.write_c_string_converted(&self.name)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CameraFile {
    version: u32,
    cameras: Vec<Camera>,
}

impl CameraFile {
    pub const FILE_NAME: &str = MAP_CAMERAS;

    pub fn read_file(map: &mut MapArchive) -> Result<Option<Self>, MapError> {
        let file = map.read_file(MAP_CAMERAS);
        match file {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(CameraError::InitReader)?;
                Self::read_opt(&mut reader)
            }
            _ => Ok(None),
        }
    }

    fn read_opt(reader: &mut BinaryReader) -> Result<Option<Self>, MapError> {
        if reader.size() > 0 {
            let camera = reader.read::<CameraFile>().map_err(CameraError::Parsing)?;
            Ok(Some(camera))
        } else {
            Ok(None)
        }
    }

    pub fn prepare_write(&self) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        writer.write(self).map_err(CameraError::SaveError)?;
        Ok(writer)
    }
}

impl BinaryConverter for CameraFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let count_camera = reader.read_u32()? as usize;
        let cameras = reader.read_vec::<Camera>(count_camera)?;

        if reader.size() != reader.pos() as usize {
            return Err(ReadError::TrailingBytes {
                file: MAP_CAMERAS.into(),
                expected: reader.size(),
                actual: reader.pos() as usize,
            });
        }
        Ok(CameraFile { version, cameras })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        if !self.cameras.is_empty() {
            writer.write_u32(self.version)?;
            writer.write_u32(self.cameras.len() as u32)?;
            for camera in &self.cameras {
                camera.write(writer)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod w3c_test {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::BinaryConverter;

    use crate::{
        camera_file::{Camera, CameraFile},
        get_resources_path,
    };

    fn mock_cameras() -> Vec<Camera> {
        vec![Camera {
            x: 758.24,
            y: 178.15,
            z: 13.5,
            rotation: 90.0,
            aoa: 304.0,
            dist: 1996.5,
            roll: 2.4,
            fov: 70.0,
            far_clip: 5000.0,
            unknown: 100.0,
            name: "Camera 001".to_string(),
        }]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure_roc() {
        let mut w3c = File::open(get_path("Scenario/Sandbox_Roc/war3map.w3c"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3c).unwrap();
        reader
            .read::<CameraFile>()
            .unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn no_failure_tft() {
        let mut w3c = File::open(get_path("Scenario/Sandbox_TFT/war3map.w3c"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3c).unwrap();
        reader
            .read::<CameraFile>()
            .unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn check_values_roc() {
        let mut w3c = File::open(get_path("Scenario/Sandbox_roc/war3map.w3c"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3c).unwrap();
        let camera_file = reader
            .read::<CameraFile>()
            .unwrap_or_else(|e| panic!("{}", e));
        let mock_cameras = mock_cameras();
        assert_eq!(camera_file.cameras, mock_cameras);
    }

    #[test]
    fn check_values_tft() {
        let mut w3c = File::open(get_path("Scenario/Sandbox_tft/war3map.w3c"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3c).unwrap();
        let camera_file = reader
            .read::<CameraFile>()
            .unwrap_or_else(|e| panic!("{}", e));
        let mock_cameras = mock_cameras();
        assert_eq!(camera_file.cameras, mock_cameras);
    }

    #[test]
    fn test_camera_round_trip() {
        let original = Camera {
            x: 100.5,
            y: 200.75,
            z: 15.25,
            rotation: 45.0,
            aoa: 30.0,
            dist: 1500.0,
            roll: 5.0,
            fov: 60.0,
            far_clip: 3000.0,
            unknown: 50.0,
            name: "TestCamera".to_string(),
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = Camera::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.x, reconstructed.x);
        assert_eq!(original.y, reconstructed.y);
        assert_eq!(original.z, reconstructed.z);
        assert_eq!(original.rotation, reconstructed.rotation);
        assert_eq!(original.aoa, reconstructed.aoa);
        assert_eq!(original.dist, reconstructed.dist);
        assert_eq!(original.roll, reconstructed.roll);
        assert_eq!(original.fov, reconstructed.fov);
        assert_eq!(original.far_clip, reconstructed.far_clip);
        assert_eq!(original.unknown, reconstructed.unknown);
        assert_eq!(original.name, reconstructed.name);
    }

    #[test]
    fn test_camera_file_round_trip() {
        let original = CameraFile {
            version: 0,
            cameras: mock_cameras(),
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = CameraFile::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.version, reconstructed.version);
        assert_eq!(original.cameras.len(), reconstructed.cameras.len());

        for (orig, recon) in original.cameras.iter().zip(reconstructed.cameras.iter()) {
            assert_eq!(orig.x, recon.x);
            assert_eq!(orig.y, recon.y);
            assert_eq!(orig.z, recon.z);
            assert_eq!(orig.rotation, recon.rotation);
            assert_eq!(orig.aoa, recon.aoa);
            assert_eq!(orig.dist, recon.dist);
            assert_eq!(orig.roll, recon.roll);
            assert_eq!(orig.fov, recon.fov);
            assert_eq!(orig.far_clip, recon.far_clip);
            assert_eq!(orig.unknown, recon.unknown);
            assert_eq!(orig.name, recon.name);
        }
    }

    #[test]
    fn write_empty_edge_case() {
        let empty_camera_file = CameraFile {
            version: 0,
            cameras: vec![],
        };

        let mut writer = BinaryWriter::new();
        empty_camera_file
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        assert_eq!(buffer.len(), 0, "Empty cameras should produce empty buffer");

        let mut reader = BinaryReader::new(buffer);
        assert!(
            CameraFile::read_opt(&mut reader)
                .unwrap_or_else(|e| panic!("{}", e))
                .is_none(),
            "Empty buffer shouldn't return camera file."
        );
    }
}
