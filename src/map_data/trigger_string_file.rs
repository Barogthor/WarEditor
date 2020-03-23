
use mpq::Archive;
use regex::Regex;

use crate::globals::MAP_STRINGS;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+";
type TRIGSTR = (u32, String);


#[derive(Debug)]
pub struct TriggerStringFile {
    trigger_strings: Vec<TRIGSTR>,
}

impl TriggerStringFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_STRINGS).unwrap();
        let mut buf: Vec<u8> = vec![0; file.size() as usize];
        file.read(mpq, &mut buf).unwrap();
        let buffer = String::from_utf8(buf).unwrap();
        let REG: Regex = Regex::new(EXTRACT_DATA).unwrap();

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportPathType{
    STANDARD(u8),
    CUSTOM(u8),
}
