//! Minimal INI-style parser for Warcraft III profile data files (e.g. `TriggerData.txt`),
//! merging sections/key-value pairs from one or more files into a `DataIni` lookup table.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use regex::{Captures, Regex};

const EOL: &str = "\r\n";

lazy_static! {
    static ref REG_SECTION: Regex = Regex::new(r"^\s*\[(.+)\]\s*$").unwrap();
    static ref SEC_PROP: Regex = Regex::new(r"^\s*([^=]+)=(.*)\s*$").unwrap();
}

fn parse_ini(path: &str) -> Result<HashMap<String, HashMap<String, String>>, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let buffer: Vec<&str> = buffer.split(EOL).collect();

    let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section: String = String::new();
    for line in buffer.iter() {
        let line = String::from(*line);
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        if REG_SECTION.is_match(&line) {
            let capture: Captures = REG_SECTION.captures(&line).unwrap();
            current_section = String::from(capture.get(1).unwrap().as_str());
            map.insert(current_section.to_owned(), HashMap::new());
        } else if SEC_PROP.is_match(&line) && !current_section.is_empty() {
            let capture: Captures = SEC_PROP.captures(&line).unwrap();
            let id = String::from(capture.get(1).unwrap().as_str());
            let value = String::from(capture.get(2).unwrap().as_str());
            map.get_mut(&current_section).unwrap().insert(id, value);
        }
    }
    Ok(map)
}

#[derive(Debug)]
pub struct DataIni {
    datas: HashMap<String, HashMap<String, String>>,
}

impl Default for DataIni {
    fn default() -> Self {
        Self::new()
    }
}

impl DataIni {
    pub fn new() -> Self {
        Self {
            datas: Default::default(),
        }
    }

    pub fn fit(&mut self) {
        self.datas.shrink_to_fit();
    }

    pub fn merge(&mut self, path: &str) -> Result<(), crate::GameDataError> {
        let ini = parse_ini(path).map_err(|source| crate::GameDataError::Ini {
            path: path.to_string(),
            source,
        })?;
        // println!("========== Parse file: {}",path);
        for (sec, prop) in ini.iter() {
            let mut sec_props = HashMap::new();
            for (id, value) in prop.iter() {
                sec_props.insert(id.to_owned(), value.to_owned());
            }
            if self.datas.contains_key(sec) {
                let mut first = false;
                let before = self.datas.get(sec).unwrap();
                for (id, value) in before.iter() {
                    if !first {
                        //                        println!("WARN: section {} may be overwritten",sec);
                        //                        println!("before: {:?}", before);
                        //                        println!("after: {:?}", sec_props);
                        first = true;
                    }
                    if !sec_props.contains_key(id) {
                        //                        println!("Added new value '{}': {}",*id, *value);
                        sec_props.insert(id.to_owned(), value.to_owned());
                    }
                }
            };
            self.datas.insert(sec.to_owned(), sec_props);
        }
        Ok(())
    }

    pub fn get_sector(&self, sector: &str) -> Option<&HashMap<String, String>> {
        self.datas.get(sector)
    }

    pub fn get_prop(&self, sector: &str, id: &str) -> Option<&String> {
        let sector_res = self.datas.get(sector);
        sector_res?.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_missing_file_returns_err_not_panic() {
        let mut d = DataIni::new();
        let r = d.merge("does/not/exist.ini");
        assert!(r.is_err());
    }
}
