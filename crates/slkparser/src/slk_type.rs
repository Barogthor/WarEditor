use crate::{record::cell::Cell, SLKError};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Record {
    Header,
    Info(u32, u32),
    CellContent(Cell),
    CellFormat,
    Format,
    Options,
    Substitution,
    ExtLink,
    NameDefinitions,
    WindowDefinitions,
    ChartExtLink,
    EOF,
}

impl Record {
    pub fn from(record_type: RecordType, fields: &[String]) -> Result<Record, SLKError> {
        //        println!("{:?}",record_type);
        match record_type {
            RecordType::EOF => Ok(Record::EOF),
            RecordType::Header => Ok(Record::Header),
            RecordType::Info => {
                let mut columns = 0u32;
                let mut rows = 0u32;
                for field in fields.iter() {
                    let field_id = &field[0..1];
                    let field_content = &field[1..];
                    match field_id {
                        "Y" => {
                            rows = field_content
                                .parse::<u32>()
                                .map_err(|e| SLKError::Parsing(record_type, "Y".into(), e))?
                        }
                        "X" => {
                            columns = field_content
                                .parse::<u32>()
                                .map_err(|e| SLKError::Parsing(record_type, "X".into(), e))?
                        }
                        _ => (),
                    }
                }
                Ok(Record::Info(rows, columns))
            }
            RecordType::CellContent => Ok(Record::CellContent(Cell::parse(fields, None)?)),
            RecordType::Format => Ok(Record::Format),
            RecordType::ChartExtLink => Ok(Record::ChartExtLink),
            RecordType::CellFormat => Ok(Record::CellFormat),
            RecordType::Options => Ok(Record::Options),
            RecordType::Substitution => Ok(Record::Substitution),
            RecordType::ExtLink => Ok(Record::ExtLink),
            RecordType::NameDefinitions => Ok(Record::NameDefinitions),
            RecordType::WindowDefinitions => Ok(Record::WindowDefinitions),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum RecordType {
    Header,
    Info,
    CellContent,
    CellFormat,
    Format,
    Options,
    Substitution,
    ExtLink,
    NameDefinitions,
    WindowDefinitions,
    ChartExtLink,
    EOF,
}

impl RecordType {
    pub fn is_eof(&self) -> bool {
        *self == RecordType::EOF
    }

    pub fn from_id(id: &str) -> Result<Self, SLKError> {
        match id {
            "ID" => Ok(RecordType::Header),
            "B" => Ok(RecordType::Info),
            "C" => Ok(RecordType::CellContent),
            "P" => Ok(RecordType::CellFormat),
            "F" => Ok(RecordType::Format),
            "O" => Ok(RecordType::Options),
            "NU" => Ok(RecordType::Substitution),
            "NE" => Ok(RecordType::ExtLink),
            "NN" => Ok(RecordType::NameDefinitions),
            "W" => Ok(RecordType::WindowDefinitions),
            "NL" => Ok(RecordType::ChartExtLink),
            "E" => Ok(RecordType::EOF),
            _ => Err(SLKError::InvalidType(format!("Unknown record {id}"))),
        }
    }
}
