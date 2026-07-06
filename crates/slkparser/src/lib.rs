use std::num::ParseIntError;
use std::{fs::File, io};
// #[warn(unused_variables)]
use std::io::Read;
use thiserror::Error;

use crate::slk_type::{Record, RecordType};

pub mod document;
pub mod record;
pub mod slk_type;
mod fields;
#[cfg(target_os = "macos")]
pub const END_RECORD: &str = "\n";
#[cfg(not(target_os = "macos"))]
pub const END_RECORD: &str = "\r\n";
pub const FIELD_SEPARATOR: &str = ";";

#[derive(Debug, Error)]
pub enum SLKError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid SLK record type: '{record_type}'")]
    InvalidType { record_type: String },
    #[error("Unexpected end of file while parsing SLK")]
    Eof,
    #[error("Failed to parse {record_type:?} record '{content}': {source}")]
    Parsing {
        record_type: RecordType,
        content: String,
        #[source]
        source: ParseIntError,
    },
}

pub struct SLKScanner {
    //    buffer: Vec<String>,
    buffer: String,
    pos: usize,
}

impl SLKScanner {
    pub fn open(path: &str) -> Result<Self, SLKError> {
        let mut f = File::open(path).map_err(SLKError::IoError)?;
        let mut buffer: String = Default::default();
        f.read_to_string(&mut buffer).map_err(SLKError::IoError)?;
        //        let buffer = buffer.split(END_RECORD).map(|slice: &str| String::from(slice)).collect();
        Ok(SLKScanner { buffer, pos: 0 })
    }

    fn get_record_type(&mut self) -> Result<RecordType, SLKError> {
        let start_pos = self.pos;
        let t = &self.buffer[self.pos..self.pos + 1];
        if t == "E" {
            return Ok(RecordType::EOF);
        }
        while &self.buffer[self.pos..self.pos + 1] != FIELD_SEPARATOR {
            self.pos += 1;
        }
        let res = RecordType::from_id(&self.buffer[start_pos..self.pos]);
        self.pos += 1;
        res
    }

    pub fn parse_record(&mut self) -> Result<Record, SLKError> {
        if self.pos >= self.buffer.len() {
            return Err(SLKError::Eof);
        }
        let record_type = self.get_record_type()?;
        if record_type == RecordType::EOF {
            self.pos = self.buffer.len();
            return Ok(Record::EOF);
        }
        let mut fields: Vec<String> = vec![];
        let mut field_start_pos = self.pos;
        while self.pos < self.buffer.len() - END_RECORD.len()
            && &self.buffer[self.pos..self.pos + END_RECORD.len()] != END_RECORD
        {
            if &self.buffer[self.pos..self.pos + 1] == FIELD_SEPARATOR {
                fields.push(String::from(&self.buffer[field_start_pos..self.pos]));
                field_start_pos = self.pos + 1;
            }
            self.pos += 1;
        }
        let field = String::from(&self.buffer[field_start_pos..self.pos]);
        fields.push(field.replace("\r", ""));
        self.pos += END_RECORD.len();
        Record::from(record_type, &fields)
    }
}

impl Iterator for SLKScanner {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let record = self.parse_record();
        match record {
            Ok(Record::EOF) => None,
            Ok(record) => Some(record),
            Err(msg) => panic!("{:?}", msg),
        }
    }
}

#[cfg(test)]
mod big_sample {
    use crate::get_resources_path;
    use crate::SLKScanner;

    #[test]
    fn ability_data_record_count() {
        let slk_reader =
            SLKScanner::open(&format!("{}slk/AbilityData.slk", get_resources_path()))
                .unwrap_or_else(|e| panic!("{:?}", e));
        assert_eq!(slk_reader.count(), 67387);
    }
}

#[cfg(test)]
fn get_resources_path() -> String {
    use std::path::Path;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("Should have parent directory");
    format!("{}/resources/", workspace_root.to_string_lossy())
}

#[cfg(test)]
mod sample {
    use crate::document::Document;
    use crate::record::cell::Cell;
    use crate::slk_type::Record;
    use crate::{get_resources_path, SLKError, SLKScanner};

    fn get_path(path: &str) -> String {
        let prefix = get_resources_path();
        format!("{prefix}/slk/{path}")
    }

    #[test]
    fn test_open() {
        SLKScanner::open(&get_path("sample_1.slk")).unwrap_or_else(|e| panic!("{:?}", e));
    }

    #[test]
    fn parse_record_one_by_one() -> Result<(), SLKError> {
        let to_s = |s: &str| String::from(s);
        let mut slk_reader =
            SLKScanner::open(&get_path("sample_1.slk")).unwrap_or_else(|e| panic!("{:?}", e));
        let fetch = slk_reader.parse_record()?;
        assert_eq!(fetch, Record::Header);

        while let Ok(Record::CellFormat) = slk_reader.parse_record() {}
        let fetch = slk_reader.parse_record()?;
        assert_eq!(fetch, Record::Info(3, 4));
        assert_eq!(slk_reader.parse_record()?, Record::Options);

        let fetch = slk_reader.parse_record()?;
        let cell = Cell::new(1u32, Some(1u32), Some(to_s("a")));
        assert_eq!(fetch, Record::CellContent(cell));

        for _ in 0..11 {
            slk_reader.parse_record().expect("Failed to parse slk");
        }
        let fetch = slk_reader.parse_record()?;
        assert_eq!(fetch, Record::EOF);
        Ok(())
    }

    #[test]
    fn parse_iterator() {
        let slk_reader =
            SLKScanner::open(&get_path("sample_1.slk")).unwrap_or_else(|e| panic!("{:?}", e));
        let mut count = 0;
        for record in slk_reader {
            println!("{record:?}");
            count += 1;
        }
        assert_eq!(count, 92);
    }

    #[test]
    fn document_test() {
        let slk_reader =
            SLKScanner::open(&get_path("sample_1.slk")).unwrap_or_else(|e| panic!("{:?}", e));
        let mut document = Document::default();
        document.load(slk_reader);
        document.debug();
    }

    #[test]
    fn test_to_string() {
        let slk_reader =
            SLKScanner::open(&get_path("sample_1.slk")).unwrap_or_else(|e| panic!("{:?}", e));
        let mut document = Document::default();
        document.load(slk_reader);
        let cells = document.get_contents();
        let cell1 = &cells[0].get_value().unwrap();
        let cell6 = &cells[6].get_value().unwrap();

        println!("{cell1:?}");
        println!("{cell6:?}");
    }
}
