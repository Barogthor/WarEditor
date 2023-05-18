use std::collections::HashMap;

use regex::Regex;

use wce_formats::MapArchive;

use crate::globals::MAP_STRINGS;

const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)";
//const EXTRACT_DATA: &str = r"STRING\s+([0-9]+)\s+";
type TRIGSTR =  String;


#[derive(Debug)]
pub struct TriggerStringFile {
    trigger_strings: HashMap<String, TRIGSTR>,
}

impl TriggerStringFile {
    pub fn read_file(map: &mut MapArchive) -> Self{
        let file = map.open_file(MAP_STRINGS).unwrap();
        let mut buf: Vec<u8> = vec![0; file.size() as usize];
        file.read(map, &mut buf).unwrap();
        let buffer = String::from_utf8_lossy(&buf).to_string();
        // let buffer = unsafe { String::from_utf8_unchecked(buf) };
        let reg: Regex = Regex::new(EXTRACT_DATA).unwrap();

        let mut trigger_strings = HashMap::new();
        for caps in reg.captures_iter(buffer.as_str()){
            let id = caps.get(1).unwrap().as_str().to_string();
            let content = String::from(caps.get(2).unwrap().as_str());
            trigger_strings.insert(id,content);
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
