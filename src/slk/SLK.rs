use std::io::{Read};
use std::fs::File;
use crate::slk::record::SLKRecord;
use std::collections::HashMap;
use crate::slk::document::SLKDocument;
use crate::slk::RecordType;
use std::cell::Cell;
use crate::slk::record::cell::RecordCell;

//const END_RECORD: char = '\n';
const END_RECORD: &str = "\r\n";
const FIELD_SEPARATOR: &str = ";";

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

    pub fn parse(&mut self) -> Result<SLKDocument,String>{
        let mut document = SLKDocument::default();
        for record in self.buffer.iter(){
            let line: Vec<&str> = record.split(FIELD_SEPARATOR).collect();
            let id = RecordType::from_id(line[0]);

            match id{
                Some(RecordType::Info) => {
                    let mut iter = line.iter();
                    iter.next();
                    for field in iter{
                        let kind = &field[..1];
                        if kind == "X" {
                            document.set_columns(field[1..].parse::<u32>().unwrap());
                        }
                        else if kind == "Y" {
                            document.set_rows(field[1..].parse::<u32>().unwrap());
                        }
                    }
                }
                Some(RecordType::CellContent) => {
                    let record = RecordCell::parse(line);

                    document.add_cell(record);
                },
                Some(RecordType::EOF) => return Ok(document),
                None => return Err("Unknown record type".to_string()),
                _ => {}
            }

        }
        Err("Missing EOF: File invalid".to_string())
    }

}


