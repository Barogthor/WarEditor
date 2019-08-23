use std::ffi::CString;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use std::fs::File;
use std::io::Read;
use std::borrow::Borrow;
use crate::map_data::binary_writer::BinaryWriter;
use regex::Regex;
use crate::map_data::{PREFIX_SAMPLE_PATH, concat_path};

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+";
type TRIGSTR = (u32, String);


#[derive(Debug)]
pub struct TriggerStringFile {
    trigger_strings: Vec<TRIGSTR>,
}

impl TriggerStringFile {
    pub fn read_file() -> Self{
//        let REG = Regex::new(EXTRACT_DATA).unwrap();

        let REG: Regex = Regex::new(EXTRACT_DATA).unwrap();
        let mut f = File::open(concat_path("war3map.wts")).unwrap();
        let mut buffer= String::new();
        f.read_to_string(&mut buffer).unwrap();
        let buffer_size = buffer.len();
//        let str_buf = buffer.as_str();
        let mut trigger_strings: Vec<TRIGSTR> = vec![];
        for caps in REG.captures_iter(buffer.as_str()){
            let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let content = String::from(caps.get(2).unwrap().as_str());
            trigger_strings.push((id,content));
        }
        TriggerStringFile{
            trigger_strings
        }
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for TriggerStringFile {
    fn read(reader: &mut BinaryReader) -> Self {
        unimplemented!()
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportPathType{
    STANDARD(u8),
    CUSTOM(u8),
}
