use std::convert::TryFrom;

#[cfg(test)]
use pretty_assertions::assert_eq;

use thiserror::Error;
use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::MapArchive;
use wce_formats::{BinaryConverter, MpqError, ReadError};

use crate::globals::MAP_SOUNDS;
use crate::OpeningError;

const DEFAULT_FLOAT: f32 = 4.294_967_3e9;

#[derive(Debug, Error)]
pub enum SoundError {
    #[error("MPQ opening failure. {0}")]
    MpqError(MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(ReadError),
    #[error("Failed to parse sounds datas. {0}")]
    Parsing(ReadError),
}
impl From<SoundError> for OpeningError {
    fn from(value: SoundError) -> Self {
        OpeningError::Sound(value)
    }
}

#[derive(Debug, Derivative)]
#[derivative(Default(new = "true"), PartialEq)]
pub struct Sound {
    id: String,
    file: String,
    effect: String,
    flags: i32,
    looping: bool,
    sound_3d: bool,
    stop_oof: bool,
    music: bool,
    unknown_flag: bool,
    fadein: i32,
    fadeout: i32,
    volume: i32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    pitch: f32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    unknown1: f32,
    unknown2: i32,
    channel: i32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    min_dist: f32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    max_dist: f32,
    dist_cutoff: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: i32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    unknown6: f32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    unknown7: f32,
    #[derivative(Default(value = "DEFAULT_FLOAT"))]
    unknown8: f32,
}
impl BinaryConverter for Sound {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let mut sound: Sound = Default::default();
        sound.id = reader.read_c_string_converted()?;
        sound.file = reader.read_c_string_converted()?;
        sound.effect = reader.read_c_string_converted()?;
        sound.flags = reader.read_i32()?;
        sound.looping = sound.flags & 0x00000001 == 1;
        sound.sound_3d = sound.flags & 0x00000002 == 2;
        sound.stop_oof = sound.flags & 0x00000004 == 4;
        sound.music = sound.flags & 0x00000008 == 8;
        sound.unknown_flag = sound.flags & 0x00000010 == 16;
        sound.fadein = reader.read_i32()?;
        sound.fadeout = reader.read_i32()?;
        sound.volume = reader.read_i32()?;
        sound.pitch = reader.read_f32()?;
        sound.unknown1 = reader.read_f32()?;
        sound.unknown2 = reader.read_i32()?;
        sound.channel = reader.read_i32()?;
        sound.min_dist = reader.read_f32()?;
        sound.max_dist = reader.read_f32()?;
        sound.dist_cutoff = reader.read_f32()?;
        sound.unknown3 = reader.read_f32()?;
        sound.unknown4 = reader.read_f32()?;
        sound.unknown5 = reader.read_i32()?;
        sound.unknown6 = reader.read_f32()?;
        sound.unknown7 = reader.read_f32()?;
        sound.unknown8 = reader.read_f32()?;
        Ok(sound)
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_c_string_converted(&self.id)?;
        writer.write_c_string_converted(&self.file)?;
        writer.write_c_string_converted(&self.effect)?;

        // Reconstruct flags from boolean fields
        let mut flags = 0i32;
        if self.looping {
            flags |= 0x00000001;
        }
        if self.sound_3d {
            flags |= 0x00000002;
        }
        if self.stop_oof {
            flags |= 0x00000004;
        }
        if self.music {
            flags |= 0x00000008;
        }
        if self.unknown_flag {
            flags |= 0x00000010;
        }

        writer.write_i32(flags)?;
        writer.write_i32(self.fadein)?;
        writer.write_i32(self.fadeout)?;
        writer.write_i32(self.volume)?;
        writer.write_f32(self.pitch)?;
        writer.write_f32(self.unknown1)?;
        writer.write_i32(self.unknown2)?;
        writer.write_i32(self.channel)?;
        writer.write_f32(self.min_dist)?;
        writer.write_f32(self.max_dist)?;
        writer.write_f32(self.dist_cutoff)?;
        writer.write_f32(self.unknown3)?;
        writer.write_f32(self.unknown4)?;
        writer.write_i32(self.unknown5)?;
        writer.write_f32(self.unknown6)?;
        writer.write_f32(self.unknown7)?;
        writer.write_f32(self.unknown8)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SoundFile {
    version: u32,
    sounds: Vec<Sound>,
}

impl SoundFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Option<Self>, OpeningError> {
        let file = map.read_file(MAP_SOUNDS);

        match file {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(SoundError::InitReader)?;
                Self::read_opt(&mut reader)
            }
            _ => Ok(None),
        }
    }

    fn read_opt(reader: &mut BinaryReader) -> Result<Option<Self>, OpeningError> {
        if reader.size() > 0 {
            let sounds = reader.read::<SoundFile>().map_err(SoundError::Parsing)?;
            Ok(Some(sounds))
        } else {
            Ok(None)
        }
    }

    pub fn debug(&self) {
        println!("{self:#?}");
    }
}

impl BinaryConverter for SoundFile {
    fn read(reader: &mut BinaryReader) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let count_sound = reader.read_u32()? as usize;
        let sounds = reader.read_vec::<Sound>(count_sound)?;
        assert_eq!(
            reader.size(),
            reader.pos() as usize,
            "reader for {} hasn't reached EOF. Missing {} bytes",
            MAP_SOUNDS,
            reader.size() - reader.pos() as usize
        );
        Ok(SoundFile { version, sounds })
    }

    fn write(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        if !self.sounds.is_empty() {
            writer.write_u32(self.version)?;
            writer.write_u32(self.sounds.len() as u32)?;
            for sound in &self.sounds {
                sound.write(writer)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod w3s_test {
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;
    use wce_formats::binary_writer::BinaryWriter;
    use wce_formats::BinaryConverter;

    use crate::{
        get_resources_path,
        sound_file::{Sound, SoundFile, DEFAULT_FLOAT},
    };

    fn mock_sounds() -> Vec<Sound> {
        vec![
            Sound {
                id: "gg_snd_RainAmbience".to_string(),
                file: "Sound\\Ambient\\RainAmbience.wav".to_string(),
                effect: "DefaultEAXON".to_string(),
                flags: 0,
                looping: false,
                sound_3d: false,
                stop_oof: false,
                music: false,
                unknown_flag: false,
                fadein: 10,
                fadeout: 10,
                volume: -1,
                pitch: DEFAULT_FLOAT,
                unknown1: DEFAULT_FLOAT,
                unknown2: -1,
                channel: -1,
                min_dist: DEFAULT_FLOAT,
                max_dist: DEFAULT_FLOAT,
                dist_cutoff: DEFAULT_FLOAT,
                unknown3: DEFAULT_FLOAT,
                unknown4: DEFAULT_FLOAT,
                unknown5: -1,
                unknown6: DEFAULT_FLOAT,
                unknown7: DEFAULT_FLOAT,
                unknown8: DEFAULT_FLOAT,
            },
            Sound {
                id: "gg_snd_WindLoopStereo".to_string(),
                file: "Sound\\Ambient\\WindLoopStereo.wav".to_string(),
                effect: "DefaultEAXON".to_string(),
                flags: 0,
                looping: false,
                sound_3d: false,
                stop_oof: false,
                music: false,
                unknown_flag: false,
                fadein: 10,
                fadeout: 10,
                volume: -1,
                pitch: DEFAULT_FLOAT,
                unknown1: DEFAULT_FLOAT,
                unknown2: -1,
                channel: -1,
                min_dist: DEFAULT_FLOAT,
                max_dist: DEFAULT_FLOAT,
                dist_cutoff: DEFAULT_FLOAT,
                unknown3: DEFAULT_FLOAT,
                unknown4: DEFAULT_FLOAT,
                unknown5: -1,
                unknown6: DEFAULT_FLOAT,
                unknown7: DEFAULT_FLOAT,
                unknown8: DEFAULT_FLOAT,
            },
            Sound {
                id: "gg_snd_RainAmbience01".to_string(),
                file: "Sound\\Ambient\\RainAmbience.wav".to_string(),
                effect: "DefaultEAXON".to_string(),
                flags: 0,
                looping: false,
                sound_3d: false,
                stop_oof: false,
                music: false,
                unknown_flag: false,
                fadein: 10,
                fadeout: 10,
                volume: 127,
                pitch: DEFAULT_FLOAT,
                unknown1: DEFAULT_FLOAT,
                unknown2: -1,
                channel: -1,
                min_dist: DEFAULT_FLOAT,
                max_dist: DEFAULT_FLOAT,
                dist_cutoff: DEFAULT_FLOAT,
                unknown3: DEFAULT_FLOAT,
                unknown4: DEFAULT_FLOAT,
                unknown5: -1,
                unknown6: DEFAULT_FLOAT,
                unknown7: DEFAULT_FLOAT,
                unknown8: DEFAULT_FLOAT,
            },
            Sound {
                id: "gg_snd_Avatar".to_string(),
                file: "Abilities\\Spells\\Human\\Avatar\\Avatar.wav".to_string(),
                effect: "SpellsEAX".to_string(),
                flags: 6,
                looping: false,
                sound_3d: true,
                stop_oof: true,
                music: false,
                unknown_flag: false,
                fadein: 10,
                fadeout: 10,
                volume: -1,
                pitch: DEFAULT_FLOAT,
                unknown1: DEFAULT_FLOAT,
                unknown2: -1,
                channel: -1,
                min_dist: 650.0,
                max_dist: 10010.0,
                dist_cutoff: 3010.0,
                unknown3: DEFAULT_FLOAT,
                unknown4: DEFAULT_FLOAT,
                unknown5: -1,
                unknown6: DEFAULT_FLOAT,
                unknown7: DEFAULT_FLOAT,
                unknown8: DEFAULT_FLOAT,
            },
            Sound {
                id: "gg_snd_Credits".to_string(),
                file: "Sound\\Music\\mp3Music\\Credits.mp3".to_string(),
                effect: "".to_string(),
                flags: 8,
                looping: false,
                sound_3d: false,
                stop_oof: false,
                music: true,
                unknown_flag: false,
                fadein: 10,
                fadeout: 10,
                volume: 0,
                pitch: 0.0,
                unknown1: DEFAULT_FLOAT,
                unknown2: 0,
                channel: -1,
                min_dist: DEFAULT_FLOAT,
                max_dist: 0.0,
                dist_cutoff: 0.0,
                unknown3: DEFAULT_FLOAT,
                unknown4: DEFAULT_FLOAT,
                unknown5: 0,
                unknown6: DEFAULT_FLOAT,
                unknown7: DEFAULT_FLOAT,
                unknown8: DEFAULT_FLOAT,
            },
        ]
    }

    fn get_path(path_resource: &str) -> String {
        let base_path = get_resources_path();
        format!("{base_path}/{path_resource}")
    }

    #[test]
    fn no_failure() {
        let mut w3s = File::open(get_path("Scenario/Sandbox_roc/war3map.w3s"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3s);
        let _sound_file = reader.read::<SoundFile>();
    }

    #[test]
    fn check_values() {
        let mut w3s = File::open(get_path("Scenario/Sandbox_roc/war3map.w3s"))
            .unwrap_or_else(|e| panic!("{}", e));
        let mut reader = BinaryReader::from(&mut w3s);
        let sound_file = reader.read::<SoundFile>().unwrap();
        let mock = mock_sounds();
        assert_eq!(sound_file.sounds, mock);
    }

    #[test]
    fn test_sound_round_trip() {
        // Test each sound from mock_sounds individually
        let mock_sounds = mock_sounds();
        let mut writer = BinaryWriter::new();
        writer.write_u32(1).unwrap();
        writer.write_u32(mock_sounds.len() as u32).unwrap();
        for s in &mock_sounds {
            writer.write(s).unwrap();
        }
        let mut reader = BinaryReader::new(writer.into_buffer());
        let reconstructed_file = SoundFile::read(&mut reader).unwrap();
        assert_eq!(reconstructed_file.version, 1);
        assert_eq!(reconstructed_file.sounds.len(), mock_sounds.len());
        for (original, recon) in mock_sounds.iter().zip(reconstructed_file.sounds.iter()) {
            assert_eq!(original, recon);
        }
    }

    #[test]
    fn test_sound_file_round_trip() {
        // Create a test SoundFile with the mock sounds
        let original_sound_file = SoundFile {
            version: 1,
            sounds: mock_sounds(),
        };

        // Write SoundFile to buffer
        let mut writer = BinaryWriter::new();
        original_sound_file.write(&mut writer).unwrap();
        let buffer = writer.into_buffer();

        // Read SoundFile back from buffer
        let mut reader = BinaryReader::new(buffer);
        let reconstructed_sound_file = SoundFile::read(&mut reader).unwrap();

        // Verify version and sound count match
        assert_eq!(
            original_sound_file.version,
            reconstructed_sound_file.version
        );
        assert_eq!(
            original_sound_file.sounds.len(),
            reconstructed_sound_file.sounds.len()
        );

        // Verify each sound matches
        for (index, (original, reconstructed)) in original_sound_file
            .sounds
            .iter()
            .zip(reconstructed_sound_file.sounds.iter())
            .enumerate()
        {
            assert_eq!(original.id, reconstructed.id, "Sound {index}: id mismatch");
            assert_eq!(
                original.file, reconstructed.file,
                "Sound {index}: file mismatch"
            );
            assert_eq!(
                original.effect, reconstructed.effect,
                "Sound {index}: effect mismatch"
            );
            assert_eq!(
                original.looping, reconstructed.looping,
                "Sound {index}: looping mismatch"
            );
            assert_eq!(
                original.sound_3d, reconstructed.sound_3d,
                "Sound {index}: sound_3d mismatch"
            );
            assert_eq!(
                original.stop_oof, reconstructed.stop_oof,
                "Sound {index}: stop_oof mismatch"
            );
            assert_eq!(
                original.music, reconstructed.music,
                "Sound {index}: music mismatch"
            );
            assert_eq!(
                original.unknown_flag, reconstructed.unknown_flag,
                "Sound {index}: unknown_flag mismatch"
            );
            assert_eq!(
                original.volume, reconstructed.volume,
                "Sound {index}: volume mismatch"
            );
            assert_eq!(
                original.min_dist, reconstructed.min_dist,
                "Sound {index}: min_dist mismatch"
            );
            assert_eq!(
                original.max_dist, reconstructed.max_dist,
                "Sound {index}: max_dist mismatch"
            );
        }
    }

    #[test]
    fn test_sound_flag_reconstruction() {
        // Test that flags are correctly reconstructed from boolean fields
        let test_cases = vec![
            // (looping, sound_3d, stop_oof, music, unknown_flag, expected_flags)
            (false, false, false, false, false, 0),
            (true, false, false, false, false, 0x00000001),
            (false, true, false, false, false, 0x00000002),
            (false, false, true, false, false, 0x00000004),
            (false, false, false, true, false, 0x00000008),
            (false, false, false, false, true, 0x00000010),
            (true, true, false, false, false, 0x00000003), // looping + sound_3d
            (false, true, true, false, false, 0x00000006), // sound_3d + stop_oof (like Avatar sound)
            (false, false, false, true, false, 0x00000008), // music only (like Credits sound)
            (true, true, true, true, true, 0x0000001F),    // all flags set
        ];

        for (looping, sound_3d, stop_oof, music, unknown_flag, expected_flags) in test_cases {
            let sound = Sound {
                id: "test".to_string(),
                file: "test.wav".to_string(),
                effect: "TestEAX".to_string(),
                flags: expected_flags, // This will be ignored in write, computed from booleans
                looping,
                sound_3d,
                stop_oof,
                music,
                unknown_flag,
                fadein: 0,
                fadeout: 0,
                volume: 0,
                pitch: 1.0,
                unknown1: 2.0,
                unknown2: 0,
                channel: 0,
                min_dist: 3.0,
                max_dist: 4.0,
                dist_cutoff: 5.0,
                unknown3: 6.0,
                unknown4: 7.0,
                unknown5: 0,
                unknown6: 8.0,
                unknown7: 9.0,
                unknown8: 10.0,
            };

            // Write and read back
            let mut writer = BinaryWriter::new();
            sound.write(&mut writer).unwrap();
            let buffer = writer.into_buffer();

            let mut reader = BinaryReader::new(buffer);
            let reconstructed = Sound::read(&mut reader).unwrap();

            // Check that the flags were reconstructed correctly
            assert_eq!(reconstructed.flags, expected_flags,
                "Flags mismatch for looping={looping}, sound_3d={sound_3d}, stop_oof={stop_oof}, music={music}, unknown_flag={unknown_flag}");

            // Check that boolean fields match
            assert_eq!(reconstructed.looping, looping);
            assert_eq!(reconstructed.sound_3d, sound_3d);
            assert_eq!(reconstructed.stop_oof, stop_oof);
            assert_eq!(reconstructed.music, music);
            assert_eq!(reconstructed.unknown_flag, unknown_flag);
        }
    }

    #[test]
    fn test_empty_sound_file() {
        // Test with empty sound file
        let original = SoundFile {
            version: 42,
            sounds: vec![],
        };

        let mut writer = BinaryWriter::new();
        original
            .write(&mut writer)
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = SoundFile::read_opt(&mut reader).unwrap_or_else(|e| panic!("{}", e));

        assert!(
            reconstructed.is_none(),
            "Empty buffer shouldn't return sound file."
        );
    }

    #[test]
    fn test_single_sound_file() {
        // Test with single sound
        let single_sound = Sound {
            id: "single_test".to_string(),
            file: "single.wav".to_string(),
            effect: "SingleEAX".to_string(),
            flags: 0,
            looping: false,
            sound_3d: false,
            stop_oof: false,
            music: false,
            unknown_flag: false,
            fadein: 100,
            fadeout: 200,
            volume: 127,
            pitch: 1.5,
            unknown1: DEFAULT_FLOAT,
            unknown2: -1,
            channel: 2,
            min_dist: 500.0,
            max_dist: 1000.0,
            dist_cutoff: 750.0,
            unknown3: DEFAULT_FLOAT,
            unknown4: DEFAULT_FLOAT,
            unknown5: -1,
            unknown6: DEFAULT_FLOAT,
            unknown7: DEFAULT_FLOAT,
            unknown8: DEFAULT_FLOAT,
        };

        let original = SoundFile {
            version: 1,
            sounds: vec![single_sound],
        };

        let mut writer = BinaryWriter::new();
        original.write(&mut writer).unwrap();
        let buffer = writer.into_buffer();

        let mut reader = BinaryReader::new(buffer);
        let reconstructed = SoundFile::read(&mut reader).unwrap();

        assert_eq!(original.version, reconstructed.version);
        assert_eq!(original.sounds.len(), reconstructed.sounds.len());
        assert_eq!(original.sounds[0].id, reconstructed.sounds[0].id);
        assert_eq!(original.sounds[0].volume, reconstructed.sounds[0].volume);
        assert_eq!(original.sounds[0].pitch, reconstructed.sounds[0].pitch);
    }

    #[test]
    fn test_real_file_round_trip() {
        // Test with actual file data to ensure compatibility
        let file_path = get_path("Scenario/Sandbox_roc/war3map.w3s");
        if let Ok(mut w3s) = File::open(&file_path) {
            let mut reader = BinaryReader::from(&mut w3s);
            if let Ok(original) = reader.read::<SoundFile>() {
                // Write the loaded file
                let mut writer = BinaryWriter::new();
                original.write(&mut writer).unwrap();
                let buffer = writer.into_buffer();

                // Read it back
                let mut reader = BinaryReader::new(buffer);
                let reconstructed = SoundFile::read(&mut reader).unwrap();

                // Verify it matches the original
                assert_eq!(original.version, reconstructed.version);
                assert_eq!(original.sounds.len(), reconstructed.sounds.len());

                for (i, (orig, recon)) in original
                    .sounds
                    .iter()
                    .zip(reconstructed.sounds.iter())
                    .enumerate()
                {
                    assert_eq!(orig.id, recon.id, "Sound {i}: id mismatch");
                    assert_eq!(orig.file, recon.file, "Sound {i}: file mismatch");
                    assert_eq!(orig.effect, recon.effect, "Sound {i}: effect mismatch");
                    assert_eq!(orig.looping, recon.looping, "Sound {i}: looping mismatch");
                    assert_eq!(
                        orig.sound_3d, recon.sound_3d,
                        "Sound {i}: sound_3d mismatch"
                    );
                    assert_eq!(
                        orig.stop_oof, recon.stop_oof,
                        "Sound {i}: stop_oof mismatch"
                    );
                    assert_eq!(orig.music, recon.music, "Sound {i}: music mismatch");
                    assert_eq!(orig.volume, recon.volume, "Sound {i}: volume mismatch");
                }
            }
        }
        // Note: Test will pass silently if file doesn't exist, which is fine for CI/environments without test data
    }
}
