use std::fmt::{Display, Formatter};
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use crate::slk::record::cell::CellValue;
use crate::slk::document::SLKDocument;

pub mod record;
pub mod slk;
pub mod document;

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
        match id.to_uppercase().as_ref() {
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


type SLKValueMapping = HashMap<u32,Vec<(u32,CellValue)>>;

pub fn merge_slk(base: &mut SLKValueMapping, document: &SLKDocument){
    let map = document.get_cells_value_sorted_by_line();
    for tuple in map.iter(){
        let key = tuple.0;
        let line = tuple.1;
        if base.contains_key(key){
            let target_line = base.get_mut(key).unwrap();
            let mut last_column = target_line.last().unwrap().0;
            for cell in line.iter(){
                last_column+=1;
                let tuple = (last_column, cell.1.to_owned());
                target_line.push(tuple);
            }
        }
    }
}