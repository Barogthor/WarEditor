use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use regex::{Captures, Regex};

const EOL: &str = "\r\n";

lazy_static!{
    static ref REG_SECTION: Regex = Regex::new(r"^\s*\[(.+)\]\s*$").unwrap();
    static ref SEC_PROP: Regex = Regex::new(r"^\s*([^=]+)=(.*)\s*$").unwrap();
}

fn parse_ini(path: &str) -> HashMap<String, HashMap<String,String>>{
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let buffer: Vec<&str> = buffer.split(EOL).collect();

    let mut map: HashMap<String, HashMap<String,String>> = HashMap::new();
    let mut current_section: String = String::new();
    for line in buffer.iter(){
        let line = String::from(*line);
        if line.starts_with("//") || line.len() == 0{continue;}
        if REG_SECTION.is_match(&line){
            let capture: Captures = REG_SECTION.captures(&line).unwrap();
            current_section = String::from(capture.get(1).unwrap().as_str());
            map.insert(current_section.to_owned(),HashMap::new());
        }
        else if SEC_PROP.is_match(&line) && current_section.len() > 0{
            let capture: Captures = SEC_PROP.captures(&line).unwrap();
            let id = String::from(capture.get(1).unwrap().as_str());
            let value = String::from(capture.get(2).unwrap().as_str());
            map.get_mut(&current_section).unwrap().insert(id,value);
        }
    }
    map
}

#[derive(Debug)]
pub struct DataIni {
    datas: HashMap<String, HashMap<String, String>>
}

impl DataIni {
    pub fn new() -> Self{ Self{ datas: Default::default() }}

    pub fn fit(&mut self){
        self.datas.shrink_to_fit();
    }

    pub fn merge(&mut self, path: &str){
//        let ini = Ini::load_from_file(path).unwrap();
        let ini = parse_ini(path);
        println!("========== Parse file: {}",path);
        for (sec,prop) in ini.iter(){
            let sec = sec;
            let mut sec_props = HashMap::new();
            for (id,value) in prop.iter(){
                sec_props.insert(id.to_owned(),value.to_owned());
            }
            if self.datas.contains_key(sec){
                let mut first = false;
                let before = self.datas.get(sec).unwrap();
                for (id, value) in before.iter(){
                    if !first {
//                        println!("WARN: section {} may be overwritten",sec);
//                        println!("before: {:?}", before);
//                        println!("after: {:?}", sec_props);
                        first = true;
                    }
                    if !sec_props.contains_key(id){
//                        println!("Added new value '{}': {}",*id, *value);
                        sec_props.insert(id.to_owned(),value.to_owned());
                    }
                }
            };
            self.datas.insert(sec.to_owned(),sec_props);
        }
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }

    pub fn get_sector(&self, sector: &str) -> Option<&HashMap<String,String>>{
        self.datas.get(sector)
    }

    pub fn get_prop(&self, sector: &str, id: &str) -> Option<&String>{
        let sector = self.datas.get(sector);
        if sector.is_none() { return None; }
        sector.unwrap().get(id)
    }
}

