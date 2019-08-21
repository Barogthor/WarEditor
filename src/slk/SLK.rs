use std::io::{Read};
use std::fs::File;
use crate::slk::record::SLKRecord;
use std::collections::HashMap;
use crate::slk::EndParsing;

//const END_RECORD: char = '\n';
const END_RECORD: &str = "\r\n";
const FIELD_SEPARATOR: &str = ";";
const EOF: &str = "E";

type Line= u32;
type Column= u32;
type Cell = (Column,String);

pub struct SLK{
    cells: HashMap<Line, Cell>
}

pub struct SLKReader {
    buffer: Vec<String>,
    pos: usize,
}

impl SLKReader{
    pub fn open_file(path: String) -> Self{
        let mut f = File::open(path).unwrap();
        let mut buffer: String = Default::default();
        f.read_to_string(&mut buffer);
        let buffer = buffer.split(END_RECORD).map(|slice: &str| String::from(slice)).collect();
        SLKReader{
            buffer,
            pos:0,
        }
    }

    pub fn parse(&mut self) -> Vec<SLKRecord>{
        let mut records: Vec<SLKRecord> = Vec::new();
        for record in self.buffer.iter(){
            let line: Vec<&str> = record.split(FIELD_SEPARATOR).collect();
            if line[0] == EOF {
                return records
            }
            let record = self.read_record(line);
            records.push(record);
        }
        records
    }

    pub fn read_record(&self, line: Vec<&str>) -> SLKRecord{
        SLKRecord::parse(line)
    }
}


