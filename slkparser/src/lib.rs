// #[warn(unused_variables)]
use std::io::{Read};
use std::fs::File;
use crate::slk_type::{RecordType, Record};

pub mod slk_type;
pub mod record;
pub mod document;
#[cfg(target_os = "macos")]
pub const END_RECORD: &str = "\n";
#[cfg(target_os = "windows")]
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
        f.read_to_string(&mut buffer).unwrap();
//        let buffer = buffer.split(END_RECORD).map(|slice: &str| String::from(slice)).collect();
        SLKScanner{
            buffer,
            pos: 0
        }
    }

    fn get_record_type(&mut self) -> Result<RecordType, String>{
        let start_pos = self.pos;
        let t = &self.buffer[self.pos..self.pos+1];
        if t == "E"{
            return Ok(RecordType::EOF);
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
        if record_type == Ok(RecordType::EOF){
            self.pos=self.buffer.len();
            return Ok(Record::EOF);
        }
        let mut fields: Vec<String> = vec![];
        let mut field_start_pos = self.pos;
        while self.pos < self.buffer.len()- END_RECORD.len() && &self.buffer[self.pos..self.pos+END_RECORD.len()] != END_RECORD{
            if &self.buffer[self.pos..self.pos+1] == FIELD_SEPARATOR{
                fields.push(String::from(&self.buffer[field_start_pos..self.pos]));
                field_start_pos=self.pos+1;
            }
            self.pos+=1;
        };
        let field = String::from(&self.buffer[field_start_pos..self.pos]);
        fields.push(field.replace("\r", ""));
        self.pos+=END_RECORD.len();
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



#[cfg(test)]
mod big_sample {
    use std::time::Instant;
    use crate::SLKScanner;
    use crate::document::Document;
    use crate::elapsed_time;

    #[test]
    fn test_ability_data() {
        let now = Instant::now();
        let slk_reader = SLKScanner::open("../resources/slk/AbilityData.slk");
        let mut document = Document::default();
        document.load(slk_reader);
        elapsed_time(&now);
//        for _ in document.get_contents(){}
        elapsed_time(&now);

    }
}

#[cfg(test)]
fn elapsed_time(instant: &std::time::Instant) {
    let elasped = instant.elapsed().as_millis();
    let millis = elasped % 1000;
    let seconds = (elasped / 1000) % 60;
    let mins = elasped / 60000;
    let hours = elasped / 3600000;
    println!("Elapsed time: {}:{}:{}::{}", hours, mins, seconds, millis);
}


#[cfg(test)]
mod sample {
    use crate::SLKScanner;
    use crate::slk_type::{Record};
    use crate::record::cell::{Cell};
    use crate::document::Document;

    #[test]
    fn test_open(){
        SLKScanner::open("resources/sample_1.slk");
    }

    #[test]
    fn parse_record_one_by_one() {
        let to_s = |s: &str| String::from(s);
        let mut slk_reader = SLKScanner::open("resources/sample_1.slk");
        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Ok( Record::Header ));

        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Ok( Record::Info(3, 4) ));

        let fetch = slk_reader.parse_record();
        let cell = Cell::new(1u32, Some(1u32), Some(to_s("a")) );
        assert_eq!(fetch, Ok( Record::CellContent(cell) ));

        for _ in 0..11 {
            slk_reader.parse_record();
        }
        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Ok( Record::EOF ));
        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Err(to_s("EOF")));
    }

    #[test]
    fn parse_iterator() {
        let slk_reader = SLKScanner::open("resources/sample_1.slk");
        let mut count = 0;
        for record in slk_reader {
            println!("{:?}", record);
            count+=1;
        }
        assert_eq!(count,14);
    }

    #[test]
    fn document_test() {
        let slk_reader = SLKScanner::open("resources/sample_1.slk");
        let mut document = Document::default();
        document.load(slk_reader);
        document.debug();
    }

    #[test]
    fn test_to_string() {
        let slk_reader = SLKScanner::open("resources/sample_1.slk");
        let mut document = Document::default();
        document.load(slk_reader);
        let cells = document.get_contents();
        let cell1 = &cells[0].get_value().unwrap();
        let cell6 = &cells[6].get_value().unwrap();

        println!("{:?}", cell1);
        println!("{:?}", cell6);
    }
}
