

use regex::Regex;
use std::collections::HashMap;
use crate::slk::CellValue;

type Type = String;
type Value = String;

#[derive(Debug)]
pub struct SLKRecord {
    kind: Type,
    fields: HashMap<Type, Value>,
}

impl SLKRecord{
    pub fn parse(record: Vec<&str>) -> SLKRecord{
        let mut iter = record.iter();
        let kind: Type = String::from(*iter.next().unwrap());
        let mut fields: HashMap<Type, Value> = HashMap::new();

        for field in iter {
            if field.len() == 0{ break; }
            fields.insert(field[0..1].parse().unwrap(), field[1..].parse().unwrap());
        }
        SLKRecord{kind, fields}
    }
    pub fn get_kind(&self) -> &Type{
        &self.kind
    }
    pub fn get_fields(&self) -> &HashMap<Type,Value>{
        &self.fields
    }
}


