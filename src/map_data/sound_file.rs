use std::ffi::CString;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use std::fs::File;
use std::io::Read;
use crate::map_data::binary_writer::BinaryWriter;


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
            pitch: 4.2949673e+009,
            unknown1: 4.2949673e+009,
            unknown2: 0,
            channel: 0,
            min_dist: 4.2949673e+009,
            max_dist: 4.2949673e+009,
            unknown3: 4.2949673e+009,
            unknown4: 4.2949673e+009,
            unknown5: 0,
            unknown6: 4.2949673e+009,
            unknown7: 4.2949673e+009,
            unknown8: 4.2949673e+009
        }
    }
}
impl BinaryConverter for Sound {
    fn read(reader: &mut BinaryReader) -> Self {
        let mut sound = Self::default();
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
    pub fn read_file() -> Self{
        let mut f = File::open("resources/war3map.w3s").unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<SoundFile>()
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
        SoundFile {
            version,
            sounds
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}