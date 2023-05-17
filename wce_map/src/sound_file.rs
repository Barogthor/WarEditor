#[cfg(test)]
use pretty_assertions::assert_eq;

use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::BinaryConverter;
use wce_formats::MapArchive;

use crate::globals::MAP_SOUNDS;

const DEFAULT_FLOAT: f32 = 4.2949673e+009;

#[derive(Debug, Derivative)]
#[derivative(Default(new="true"), PartialEq)]
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
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    pitch: f32,
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    unknown1: f32,
    unknown2: i32,
    channel: i32,
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    min_dist: f32,
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    max_dist: f32,
    dist_cutoff: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: i32,
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    unknown6: f32,
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    unknown7: f32,
    #[derivative(Default(value="DEFAULT_FLOAT"))]
    unknown8: f32,
}
impl BinaryConverter for Sound {
    fn read(reader: &mut BinaryReader) -> Self {
        let mut sound: Sound = Default::default();
        sound.id = reader.read_c_string().into_string().unwrap();
        sound.file = reader.read_c_string().into_string().unwrap();
        sound.effect = reader.read_c_string().into_string().unwrap();
        sound.flags = reader.read_i32();
        sound.looping = sound.flags & 0x00000001 == 1;
        sound.sound_3d = sound.flags & 0x00000002 == 2;
        sound.stop_oof = sound.flags & 0x00000004 == 4;
        sound.music = sound.flags & 0x00000008 == 8;
        sound.unknown_flag = sound.flags & 0x00000010 == 16;
        sound.fadein = reader.read_i32();
        sound.fadeout = reader.read_i32();
        sound.volume = reader.read_i32();
        sound.pitch = reader.read_f32();
        sound.unknown1 = reader.read_f32();
        sound.unknown2 = reader.read_i32();
        sound.channel = reader.read_i32();
        sound.min_dist = reader.read_f32();
        sound.max_dist = reader.read_f32();
        sound.dist_cutoff = reader.read_f32();
        sound.unknown3 = reader.read_f32();
        sound.unknown4 = reader.read_f32();
        sound.unknown5 = reader.read_i32();
        sound.unknown6 = reader.read_f32();
        sound.unknown7 = reader.read_f32();
        sound.unknown8 = reader.read_f32();
        sound
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct SoundFile {
    version: u32,
    sounds: Vec<Sound>,
}

impl SoundFile {
    pub fn read_file(map: &mut MapArchive) -> Option<Self>{
        let file = map.open_file(MAP_SOUNDS);

        match file{
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(map, &mut buffer).unwrap();
                let mut reader = BinaryReader::new(buffer);
                Some(reader.read::<SoundFile>())
            }
            _ => None
        }

    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for SoundFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let version = reader.read_u32();
        let count_sound = reader.read_u32() as usize;
        let sounds = reader.read_vec::<Sound>(count_sound);
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_SOUNDS, reader.size() - reader.pos() as usize);
        SoundFile {
            version,
            sounds
        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[cfg(test)]
mod w3s_test{
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;

    use crate::sound_file::{DEFAULT_FLOAT, Sound, SoundFile};

    fn mock_sounds() -> Vec<Sound>{
        vec![
            Sound{
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
                unknown8: DEFAULT_FLOAT
            },
            Sound{
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
                unknown8: DEFAULT_FLOAT
            },
            Sound{
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
                unknown8: DEFAULT_FLOAT
            },
            Sound{
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
                unknown8: DEFAULT_FLOAT
            },
            Sound{
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
                unknown8: DEFAULT_FLOAT
            },
        ]
    }

    #[test]
    fn no_failure(){
        let mut w3s = File::open("../resources/Scenario/Sandbox_roc/war3map.w3s").unwrap();
        let mut reader = BinaryReader::from(&mut w3s);
        let _sound_file = reader.read::<SoundFile>();
    }

    #[test]
    fn check_values(){
        let mut w3s = File::open("../resources/Scenario/Sandbox_roc/war3map.w3s").unwrap();
        let mut reader = BinaryReader::from(&mut w3s);
        let sound_file = reader.read::<SoundFile>();
        let mock = mock_sounds();
        assert_eq!(sound_file.sounds, mock);
    }
}