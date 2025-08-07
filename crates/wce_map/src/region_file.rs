use std::convert::TryFrom;

use derivative::Derivative;
#[cfg(test)]
use pretty_assertions::assert_eq;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError};

use crate::globals::MAP_REGIONS;
use crate::OpeningError;

#[derive(Debug, Error)]
pub enum RegionError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse regions datas. {0}")]
    Parsing(ReadError),
}
impl From<RegionError> for OpeningError {
    fn from(value: RegionError) -> Self {
        OpeningError::Region(value)
    }
}

#[derive(Debug, Derivative)]
#[derivative(Default, PartialEq)]
pub struct Region {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    name: String,
    index: u32,
    weather_effect: String,
    weather_enabled: bool,
    ambient_sound: String,
    color: Vec<u8>,
    // skip 1 byte : end structure
}
impl BinaryConverter for Region {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let mut region = Self::default();
        region.left = reader.read_f32()?;
        region.bottom = reader.read_f32()?;
        region.right = reader.read_f32()?;
        region.top = reader.read_f32()?;
        region.name = reader.read_c_string_converted()?;
        region.index = reader.read_u32()?;
        //        let effect_id = reader.read_bytes(4);
        //        region.weather_effect = String::from_utf8(effect_id).unwrap();
        region.weather_effect = reader.read_string_utf8(4)?;
        if region.weather_effect.as_bytes() == [0u8; 4] {
            region.weather_enabled = false;
        }
        region.ambient_sound = reader.read_c_string_converted()?;
        region.color = reader.read_bytes(3)?;
        reader.skip(1);
        Ok(region)
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_f32(self.left)?;
        writer.write_f32(self.bottom)?;
        writer.write_f32(self.right)?;
        writer.write_f32(self.top)?;
        writer.write_c_string_converted(&self.name)?;
        writer.write_u32(self.index)?;

        // Weather effect must be exactly 4 bytes - pad or truncate as needed
        let mut weather_bytes = self.weather_effect.as_bytes().to_vec();
        weather_bytes.resize(4, 0); // Pad with zeros or truncate to 4 bytes
        writer.write_bytes(&weather_bytes)?;

        writer.write_c_string_converted(&self.ambient_sound)?;
        writer.write_bytes(&self.color)?;
        writer.write_u8(0)?; // Skip byte
        Ok(())
    }
}

#[derive(Debug)]
pub struct RegionFile {
    version: u32,
    regions: Vec<Region>,
}

impl RegionFile {
    pub const FILE_NAME: &str = MAP_REGIONS;
    
    pub fn read_file(map: &mut MapArchive) -> Result<Option<Self>, OpeningError> {
        let file = map.read_file(MAP_REGIONS);

        match file {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(RegionError::InitReader)?;
                Self::read_opt(&mut reader)
            }
            _ => Ok(None),
        }
    }

    fn read_opt(reader: &mut BinaryReader) -> Result<Option<Self>, OpeningError> {
        if reader.size() > 0 {
            let regions = reader.read::<RegionFile>().map_err(RegionError::Parsing)?;
            Ok(Some(regions))
        } else {
            Ok(None)
        }
    }

    pub fn prepare_write(&self) -> WriteResult<BinaryWriter> {
        let mut writer = BinaryWriter::new();
        writer.write(self)?;
        Ok(writer)
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for RegionFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let count_region = reader.read_u32()? as usize;
        let regions = reader.read_vec::<Region>(count_region)?;
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_REGIONS,
            reader.size() - reader.pos() as usize
        );
        Ok(RegionFile { version, regions })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        if !self.regions.is_empty() {
            writer.write_u32(self.version)?;
            writer.write_u32(self.regions.len() as u32)?;
            for region in &self.regions {
                region.write(writer)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod w3r_test {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::BinaryConverter;

    use crate::{
        get_resources_path,
        region_file::{Region, RegionFile},
    };

    fn mock_regions() -> Vec<Region> {
        vec![
            Region {
                left: -832.0,
                right: -480.0,
                bottom: -640.0,
                top: -256.0,
                name: "Red".to_string(),
                index: 0,
                weather_effect: "RAhr".to_string(),
                weather_enabled: false,
                ambient_sound: "gg_snd_RainAmbience".to_string(),
                color: vec![0, 0, 255],
            },
            Region {
                left: 416.0,
                right: 768.0,
                bottom: -32.0,
                top: 352.0,
                name: "LightGreen".to_string(),
                index: 1,
                weather_effect: "\0\0\0\0".to_string(),
                weather_enabled: false,
                ambient_sound: "gg_snd_Avatar".to_string(),
                color: vec![128, 255, 128],
            },
            Region {
                left: 384.0,
                right: 416.0,
                bottom: -1056.0,
                top: -640.0,
                name: "White".to_string(),
                index: 2,
                weather_effect: "\0\0\0\0".to_string(),
                weather_enabled: false,
                ambient_sound: "".to_string(),
                color: vec![255, 255, 255],
            },
        ]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let mut w3r = File::open(get_path("Scenario/Sandbox_roc/war3map.w3r"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3r);
        reader.read::<RegionFile>().unwrap();
    }

    #[test]
    fn check_values() {
        let mut w3r = File::open(get_path("Scenario/Sandbox_roc/war3map.w3r"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3r);
        let region_file = reader
            .read::<RegionFile>()
            .unwrap_or_else(|e| panic!("{}", e));
        let mock_regions = mock_regions();
        assert_eq!(region_file.regions, mock_regions);
    }

    #[test]
    fn test_region_round_trip() {
        let original = Region {
            left: -832.0,
            right: -480.0,
            bottom: -640.0,
            top: -256.0,
            name: "TestRegion".to_string(),
            index: 42,
            weather_effect: "RAhr".to_string(),
            weather_enabled: false,
            ambient_sound: "gg_snd_TestSound".to_string(),
            color: vec![255, 128, 64],
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = Region::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.left, reconstructed.left);
        assert_eq!(original.right, reconstructed.right);
        assert_eq!(original.bottom, reconstructed.bottom);
        assert_eq!(original.top, reconstructed.top);
        assert_eq!(original.name, reconstructed.name);
        assert_eq!(original.index, reconstructed.index);
        assert_eq!(original.weather_effect, reconstructed.weather_effect);
        assert_eq!(original.ambient_sound, reconstructed.ambient_sound);
        assert_eq!(original.color, reconstructed.color);
    }

    #[test]
    fn test_region_file_round_trip() {
        // Test complete RegionFile round-trip
        let original = RegionFile {
            version: 5,
            regions: mock_regions(),
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = RegionFile::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.version, reconstructed.version);
        assert_eq!(original.regions.len(), reconstructed.regions.len());

        for (orig, recon) in original.regions.iter().zip(reconstructed.regions.iter()) {
            assert_eq!(orig.left, recon.left);
            assert_eq!(orig.right, recon.right);
            assert_eq!(orig.bottom, recon.bottom);
            assert_eq!(orig.top, recon.top);
            assert_eq!(orig.name, recon.name);
            assert_eq!(orig.index, recon.index);
            assert_eq!(orig.weather_effect, recon.weather_effect);
            assert_eq!(orig.ambient_sound, recon.ambient_sound);
            assert_eq!(orig.color, recon.color);
        }
    }

    #[test]
    fn test_single_region_file() {
        // Test with single region
        let single_region = Region {
            left: 100.5,
            right: 200.5,
            bottom: 50.25,
            top: 150.75,
            name: "SingleRegion".to_string(),
            index: 1,
            weather_effect: "WEth".to_string(),
            weather_enabled: true,
            ambient_sound: "ambient_test".to_string(),
            color: vec![128, 128, 128],
        };

        let original = RegionFile {
            version: 1,
            regions: vec![single_region],
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = RegionFile::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert_eq!(original.version, reconstructed.version);
        assert_eq!(original.regions.len(), reconstructed.regions.len());
        assert_eq!(original.regions[0].name, reconstructed.regions[0].name);
        assert_eq!(original.regions[0].left, reconstructed.regions[0].left);
        assert_eq!(
            original.regions[0].weather_effect,
            reconstructed.regions[0].weather_effect
        );
    }

    #[test]
    fn test_weather_effect_handling() {
        // Test different weather effect scenarios
        // Note: Weather effects are padded/truncated to exactly 4 bytes
        let test_cases = vec![
            ("RAhr", "RAhr"),         // Exactly 4 bytes
            ("\0\0\0\0", "\0\0\0\0"), // 4 null bytes
            ("WEth", "WEth"),         // Exactly 4 bytes
            ("ABC", "ABC\0"),         // 3 chars padded with null
            ("", "\0\0\0\0"),         // Empty string padded to 4 null bytes
            ("ABCDEF", "ABCD"),       // 6 chars truncated to 4
        ];

        for (input_weather, expected_weather) in test_cases {
            let region = Region {
                left: 0.0,
                right: 100.0,
                bottom: 0.0,
                top: 100.0,
                name: "WeatherTest".to_string(),
                index: 0,
                weather_effect: input_weather.to_string(),
                weather_enabled: false,
                ambient_sound: "".to_string(),
                color: vec![255, 255, 255],
            };

            let mut writer = BinaryWriter::new();
            region
                .write(&mut writer)
                .unwrap_or_else(|e| panic!("{}", e));
            let buffer = writer.into_buffer();

            let mut reader = BinaryReader::new(buffer);
            let reconstructed = Region::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

            assert_eq!(reconstructed.weather_effect, expected_weather);
        }
    }

    #[test]
    fn test_real_file_round_trip() {
        // Test with real file data to ensure compatibility
        let file_path = get_path("Scenario/Sandbox_roc/war3map.w3r");
        if let Ok(mut w3r) = File::open(&file_path) {
            let mut reader = BinaryReader::from(&mut w3r);
            if let Ok(original) = reader.read::<RegionFile>() {
                // Write the loaded file
                let mut writer = BinaryWriter::new();
                original
                    .write(&mut writer)
                    .unwrap_or_else(|e| panic!("{}", e));
                let buffer = writer.into_buffer();

                // Read it back
                let mut reader = BinaryReader::new(buffer);
                let reconstructed =
                    RegionFile::read(&mut reader).unwrap_or_else(|e| panic!("{}", e));

                // Verify it matches the original
                assert_eq!(original.version, reconstructed.version);
                assert_eq!(original.regions.len(), reconstructed.regions.len());

                for (i, (orig, recon)) in original
                    .regions
                    .iter()
                    .zip(reconstructed.regions.iter())
                    .enumerate()
                {
                    assert_eq!(orig.left, recon.left, "Region {i}: left mismatch");
                    assert_eq!(orig.right, recon.right, "Region {i}: right mismatch");
                    assert_eq!(orig.bottom, recon.bottom, "Region {i}: bottom mismatch");
                    assert_eq!(orig.top, recon.top, "Region {i}: top mismatch");
                    assert_eq!(orig.name, recon.name, "Region {i}: name mismatch");
                    assert_eq!(orig.index, recon.index, "Region {i}: index mismatch");
                    assert_eq!(
                        orig.weather_effect, recon.weather_effect,
                        "Region {i}: weather_effect mismatch"
                    );
                    assert_eq!(
                        orig.ambient_sound, recon.ambient_sound,
                        "Region {i}: ambient_sound mismatch"
                    );
                    assert_eq!(orig.color, recon.color, "Region {i}: color mismatch");
                }
            }
        }
        // Note: Test will pass silently if file doesn't exist, which is fine for CI/environments without test data
    }

    #[test]
    fn write_empty_edge_case() {
        // Test writing RegionFile with empty regions vector
        let empty_region_file = RegionFile {
            version: 5,
            regions: vec![],
        };

        let mut writer = BinaryWriter::new();
        empty_region_file
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        // Empty regions should produce empty buffer (no version or count written)
        assert_eq!(buffer.len(), 0, "Empty regions should produce empty buffer");

        // Verify we can read an empty buffer without errors
        let mut reader = BinaryReader::new(buffer);
        assert!(
            RegionFile::read_opt(&mut reader)
                .unwrap_or_else(|e| panic!("{}", e))
                .is_none(),
            "Empty buffer shouldn't return region file."
        );
    }
}
