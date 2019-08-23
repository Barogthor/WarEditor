use std::ffi::CString;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use std::fs::File;
use std::io::{Read, ErrorKind};
use std::borrow::Borrow;
use crate::map_data::binary_writer::BinaryWriter;
use crate::map_data::{PREFIX_SAMPLE_PATH, concat_path};
use regex::Error;

type TextScript = CString;

#[derive(Debug)]
pub struct CustomTextTriggerFile {
    version: Version,
    global_comment: CString,
    global_script: TextScript,
    triggers_script: Vec<TextScript>,
}

impl CustomTextTriggerFile {
    pub fn read_file() -> Self{
        let mut f = File::open(concat_path("war3map.wct")).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<CustomTextTriggerFile>()
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for CustomTextTriggerFile {
    fn read(reader: &mut BinaryReader) -> Self {

        let version = Version::from(reader.read_u32()).unwrap();
        let mut count_triggers: usize;
        let mut global_comment: CString = Default::default();
        let mut global_script: CString = Default::default();
        let mut text_triggers: Vec<TextScript> = Vec::new();
        match version {
            Version::TFT => {
                global_comment = reader.read_c_string();
                let s = reader.read_u32() as usize;
                println!("pos: {},  length: {}", reader.pos(), s);
                global_script = reader.read_string_sized(s-1);
                reader.skip(1);
                count_triggers = reader.read_u32() as usize;
                println!("count: {}, bufsize: {}",count_triggers, reader.size());
                for i in 0..count_triggers{
                    let length = reader.read_u32() as usize;
                    if length == 0 { continue; }
                    println!("i: {} || left: {},  length: {}", i, (reader.size() - reader.pos() as usize), length);
                    text_triggers.push(reader.read_string_sized(length-1));
                    reader.skip(1);
                }
            }
            Version::RoC => {
                count_triggers = reader.read_u32() as usize;
                for _ in 0..count_triggers{
                    let s = reader.read_u32() as usize;
                    text_triggers.push(reader.read_string_sized(s));
                }
            },
        }

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Version{
    RoC,
    TFT
}

impl Version{
    pub fn from(n: u32) -> Option<Version> {
        match n{
            0 => Some(Version::RoC),
            1 => Some(Version::TFT),
            _ => None
        }
    }
}