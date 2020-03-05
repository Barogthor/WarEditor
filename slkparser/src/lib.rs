use std::io::{Read};
use std::fs::File;
use std::borrow::BorrowMut;
use crate::slk_type::{RecordType, Record};
use crate::record::cell::Cell;

pub mod slk_type;
pub mod record;
pub mod document;
pub const END_RECORD: &str = "\r\n";
pub const FIELD_SEPARATOR: &str = ";";


pub struct SLKScanner{
//    buffer: Vec<String>,
    buffer: String,
    pos: usize
}

impl SLKScanner {
    pub fn open(path: &str) -> Self{
        let mut f = File::open(path).unwrap();
        let mut buffer: String = Default::default();
        f.read_to_string(&mut buffer);
//        let buffer = buffer.split(END_RECORD).map(|slice: &str| String::from(slice)).collect();
        SLKScanner{
            buffer,
            pos: 0
        }
    }

    fn get_record_type(&mut self) -> Option<RecordType>{
        let start_pos = self.pos;
        let t = &self.buffer[self.pos..self.pos+1];
        if t == "E"{
            return Some(RecordType::EOF);
        }
        while &self.buffer[self.pos..self.pos+1] != FIELD_SEPARATOR{
            self.pos+=1;
        }
        let res = RecordType::from_id(&self.buffer[start_pos..self.pos]);
        self.pos+=1;
        res
    }

    pub fn parse_record(&mut self) -> Result<Record, String> {
        if self.pos >= self.buffer.len(){
            return Err(String::from("EOF"));
        }
        let record_type = self.get_record_type();
        let star_pos = self.pos;
        let mut fields: Vec<String> = vec![];
        let mut field_start_pos = self.pos;
        while self.pos < self.buffer.len() && &self.buffer[self.pos..self.pos+2] != END_RECORD{
            if &self.buffer[self.pos..self.pos+1] == FIELD_SEPARATOR{
                fields.push(String::from(&self.buffer[field_start_pos..self.pos]));
                field_start_pos=self.pos+1;
            }
            self.pos+=1;
        };
        fields.push(String::from(&self.buffer[field_start_pos..self.pos]));
        self.pos+=2;
        Record::from(record_type, &fields)
    }
}

impl Iterator for SLKScanner{
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let record = self.parse_record();
        match record {
            Ok(Record::EOF) => None,
            Ok(record) => Some(record),
            Err(msg) => panic!(msg)
        }
    }
}

fn is_end_of_record(buffer: &str) -> bool{
    let eor_buffer= &buffer[buffer.len()-2..];
    eor_buffer.eq(END_RECORD)
}