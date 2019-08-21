
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EndParsing {}
impl Error for EndParsing {}
impl Display for EndParsing {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{}", "The parser is done".to_string())
    }
}
impl EndParsing {
    pub fn new() -> Self { EndParsing {}}
}

#[derive(Debug)]
pub enum FieldValue {
    Row(u32),
    Column(u32),
    Value(CellValue),
    Expression(String),
    RefColumn(),
    RefRow(),
    SharedValue(),
    SharedValueOrExpr(),
    Protected(bool),
    Hidden(bool),
}

#[derive(Debug)]
pub enum CellValue {
    Text(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
}


pub mod record;
pub mod slk;
