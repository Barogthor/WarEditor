use std::ffi::CString;

use mpq::Archive;

use crate::globals::MAP_SOUNDS;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

const DEFAULT_FLOAT: f32 = 4.2949673e+009;

#[derive(Debug)]
pub struct Sound {
    id: CString,
    file: CString,
    effect: CString,
    flags: i32,
    looping: bool,
    sound_3d: bool,
    stop_oof: bool,
    music: bool,
    unknown_flag: bool,
    fadein: i32,
    fadeout: i32,
    volume: i32,
    pitch: f32,
    unknown1: f32,
    unknown2: i32,
    channel: u32,
    min_dist: f32,
    max_dist: f32,
    dist_cutoff: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: i32,
    unknown6: f32,
    unknown7: f32,
    unknown8: f32,
}
impl Default for Sound {
    fn default() -> Self {
        Sound {
            id: Default::default(),
            file: Default::default(),
            effect: Default::default(),
            flags: 0,
            looping: false,
            sound_3d: false,
            stop_oof: false,
            music: false,
            unknown_flag: false,
            fadein: 0,
            fadeout: 0,
            volume: -1,
            pitch: DEFAULT_FLOAT,
            unknown1: DEFAULT_FLOAT,
            unknown2: 0,
            channel: 0,
            min_dist: DEFAULT_FLOAT,
            max_dist: DEFAULT_FLOAT,
            dist_cutoff: DEFAULT_FLOAT,
            unknown3: DEFAULT_FLOAT,
            unknown4: DEFAULT_FLOAT,
            unknown5: 0,
            unknown6: DEFAULT_FLOAT,
            unknown7: DEFAULT_FLOAT,
            unknown8: DEFAULT_FLOAT
        }
    }
}
impl BinaryConverter for Sound {
    fn read(reader: &mut BinaryReader) -> Self {
        let mut sound = Self::default();
        sound.id = reader.read_c_string();
        sound.file = reader.read_c_string();
        sound.effect = reader.read_c_string();
        sound.flags = reader.read_i32();
        sound.fadein = reader.read_i32();
        sound.fadeout = reader.read_i32();
        sound.volume = reader.read_i32();
        sound.pitch = reader.read_f32();
        sound.unknown1 = reader.read_f32();
        sound.unknown2 = reader.read_i32();
        sound.channel = reader.read_u32();
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

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct SoundFile {
    version: u32,
    sounds: Vec<Sound>,
}

impl SoundFile {
    pub fn read_file(mpq: &mut Archive) -> Option<Self>{
        let file = mpq.open_file(MAP_SOUNDS);

        match file{
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(mpq, &mut buffer).unwrap();
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

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}