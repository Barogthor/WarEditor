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
    pub fn from(record_type: Option<RecordType>, fields: &Vec<String>) -> Result<Record, String>{
        match record_type{
            Some(RecordType::EOF) => {
                Ok(Record::EOF)
            },
            Some(RecordType::Header) => {
                Ok(Record::Header)
            },
            Some(RecordType::Info) => {
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
            Some(RecordType::CellContent) => {
                Ok(Record::CellContent(Cell::parse(fields, None)))
            },
            _ => Err(String::from("Unkown Record"))
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

    pub fn from_id(id: &str) -> Option<Self>{
        match id {
            "ID" => Some(RecordType::Header),
            "B" => Some(RecordType::Info),
            "C" => Some(RecordType::CellContent),
            "P" => Some(RecordType::CellFormat),
            "F" => Some(RecordType::Format),
            "O" => Some(RecordType::Options),
            "NU" => Some(RecordType::Substitution),
            "NE" => Some(RecordType::ExtLink),
            "NN" => Some(RecordType::NameDefinitions),
            "W" => Some(RecordType::WindowDefinitions),
            "NL" => Some(RecordType::ChartExtLink),
            "E" => Some(RecordType::EOF),
            _ => None
        }
    }
}