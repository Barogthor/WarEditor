use crate::record::cell::Cell;

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
    pub fn from(record_type: Result<RecordType, String>, fields: &Vec<String>) -> Result<Record, String>{
//        println!("{:?}",record_type);
        match record_type{
            Ok(RecordType::EOF) => {
                Ok(Record::EOF)
            },
            Ok(RecordType::Header) => {
                Ok(Record::Header)
            },
            Ok(RecordType::Info) => {
                let mut columns = 0u32;
                let mut rows = 0u32;
                for field in fields.iter(){
                    let field_id = &field[0..1];
                    let field_content = &field[1..];
                    match field_id{
                        "Y" => rows = field_content.parse::<u32>().unwrap(),
                        "X" => columns = field_content.parse::<u32>().unwrap(),
                        _ => ()
                    }
                }
                Ok(Record::Info(rows,columns))
            },
            Ok(RecordType::CellContent) => {
                Ok(Record::CellContent(Cell::parse(fields, None)))
            },
            Ok(RecordType::Format) => Ok(Record::Format),
            Ok(RecordType::ChartExtLink) => Ok(Record::ChartExtLink),
            Ok(RecordType::CellFormat) => Ok(Record::CellFormat),
            Ok(RecordType::Options) => Ok(Record::Options),
            Ok(RecordType::Substitution) => Ok(Record::Substitution),
            Ok(RecordType::ExtLink) => Ok(Record::ExtLink),
            Ok(RecordType::NameDefinitions) => Ok(Record::NameDefinitions),
            Ok(RecordType::WindowDefinitions) => Ok(Record::WindowDefinitions),
            Err(msg) => Err(msg),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
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
    pub fn is_eof(&self) -> bool{
        *self == RecordType::EOF
    }

    pub fn from_id(id: &str) -> Result<Self, String>{
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
            _ => Err(format!("Unknown record {}", id))
        }
    }
}