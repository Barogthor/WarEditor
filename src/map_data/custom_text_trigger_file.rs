use std::ffi::CString;

use mpq::Archive;

use crate::globals::{MAP_TRIGGERS_SCRIPT, GameVersion};
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;
use crate::globals::GameVersion::{RoC, TFT};

type TextScript = CString;

#[derive(Debug)]
pub struct CustomTextTriggerFile {
    version: GameVersion,
    global_comment: CString,
    global_script: TextScript,
    triggers_script: Vec<TextScript>,
}

impl CustomTextTriggerFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_TRIGGERS_SCRIPT).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<CustomTextTriggerFile>()
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for CustomTextTriggerFile {
    fn read(reader: &mut BinaryReader) -> Self {

        let version = reader.read_u32();
        let version = to_game_version(version);
        let count_triggers: usize;
        let mut global_comment: CString = Default::default();
        let mut global_script: CString = Default::default();
        let mut text_triggers: Vec<TextScript> = Vec::new();
        match version {
            RoC => {
                count_triggers = reader.read_u32() as usize;
                for _ in 0..count_triggers{
                    let s = reader.read_u32() as usize;
                    text_triggers.push(reader.read_c_string_sized(s));
                }
            },
            _  => {
                global_comment = reader.read_c_string();
                let s = reader.read_u32() as usize;
                println!("pos: {},  length: {}", reader.pos(), s);
                global_script = if s > 0 {
                    let scr = reader.read_c_string_sized(s-1);
                    reader.skip(1);
                    scr
                } else { Default::default() };
                count_triggers = reader.read_u32() as usize;
                println!("count: {}, bufsize: {}",count_triggers, reader.size());
                for i in 0..count_triggers{
                    let length = reader.read_u32() as usize;
                    if length == 0 { continue; }
                    println!("i: {} || left: {},  length: {}", i, (reader.size() - reader.pos() as usize), length);
                    text_triggers.push(reader.read_c_string_sized(length - 1));
                    reader.skip(1);
                }
            }
        }
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_TRIGGERS_SCRIPT, reader.size() - reader.pos() as usize);

        CustomTextTriggerFile {
            version,
            global_comment,
            global_script,
            triggers_script: text_triggers,

        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


fn to_game_version(value: u32) -> GameVersion{
    match value{
        0 => RoC,
        1 => TFT,
        _ => panic!("Unknown or unsupported game version '{}'", value)
    }
}

