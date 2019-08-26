use regex::Regex;
use lazy_static::lazy;
//static R: Regex = Regex::new(r"-?[0-9]+(\.[0-9]*)?");
lazy_static!{
    static ref REG_FLOAT: Regex = Regex::new(r"^-?[0-9]+(\.[0-9]*)?$").unwrap();
    static ref REG_INT: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
    static ref REG_BOOL: Regex = Regex::new(r"^(?i)true|false$").unwrap();
    static ref REG_STR: Regex = Regex::new(r#"^".*"$"#).unwrap();
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum CellValue {
    Text(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, Default, Getters)]
#[get = "pub"]
pub struct RecordCell {
    column: u32,
    row: Option<u32>,
    expression: Option<String>,
    value: Option<CellValue>,
    column_ref: Option<String>,
    row_ref: Option<String>,
    shared_value_definition: Option<String>,
    shared_expression_definition: Option<String>,
    shared_expression_or_value_refs: Option<String>,
    is_protected: bool,
    hidden: bool,
    inside_matrix: bool,
    matrix_expression: Option<String>,
}

impl RecordCell {
    pub fn parse(tokens: Vec<&str>) -> Self{
//        let REG_FLOAT: Regex = Regex::new(r"-?[0-9]+(\.[0-9]*)?").unwrap();
//        let REG_INT: Regex = Regex::new(r"-?[0-9]+").unwrap();
//        let REG_BOOL: Regex = Regex::new(r"true|false").unwrap();
//        let REG_STR: Regex = Regex::new(r#"".*""#).unwrap();
        let mut cell = Self::default();
        let mut iter = tokens.iter();
        iter.next();
        for token in iter {
            if token.len() <= 0 {continue;}
            let field_type = &token[..1];
            let content = &token[1..];
            let field = CellField::from_id(field_type).unwrap();
            match field{
                CellField::Column => {
                    cell.column = content.parse::<u32>().unwrap();
                },
                CellField::Value => {
                    println!("debug: {}", content);
                    if REG_BOOL.is_match(content){
                        cell.value = Some(CellValue::Bool(content.to_lowercase() == "true".to_string()));
                    }
                    else if REG_INT.is_match(content){
                        cell.value = Some(CellValue::Integer(content.parse::<i32>().unwrap()));
                    }
                    else if REG_FLOAT.is_match(content){
                        cell.value = Some(CellValue::Float(content.parse::<f32>().unwrap()));
                    }
                    else if REG_STR.is_match(content){
                        cell.value = Some(CellValue::Text(String::from(&content[1..(content.len() - 1)])));
                    }
                    else {
                        cell.value = None;
                    }
                },
                CellField::Row => {
                    cell.row = Some(content.parse::<u32>().unwrap());
                },
                CellField::Expression => {
                    cell.expression = Some(String::from(content));
                },
                CellField::ColumnRef => {
                    cell.column_ref = Some(String::from(content));
                },
                CellField::RowRef => {
                    cell.row_ref = Some(String::from(content));
                },
                CellField::SharedValue => {
                    cell.shared_value_definition = Some(String::from(content));
                },
                CellField::SharedExpression => {
                    cell.shared_expression_definition = Some(String::from(content));
                },
                CellField::SharedExpressionOrValueRefs => {
                    cell.shared_expression_or_value_refs = Some(String::from(content));
                },
                CellField::NFlag => {

                },
                CellField::PFlag => {

                },
                CellField::Hidden => {
                    cell.hidden = true;
                },
                CellField::MatrixExpression => {
                    cell.matrix_expression = Some(String::from(content));
                },
                CellField::InsideMatrix => {
                    cell.inside_matrix = true;
                },
            }
        }

        cell
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum CellField {
    Column,
    Row,
    Value,
    Expression,
    ColumnRef,
    RowRef,
    SharedValue,
    SharedExpression,
    SharedExpressionOrValueRefs,
    NFlag,
    PFlag,
    Hidden,
    MatrixExpression,
    InsideMatrix,
}

impl CellField {
    pub fn from_id(id: &str) -> Option<Self>{
        match id.to_uppercase().as_ref() {
            "X" => Some(CellField::Column),
            "Y" => Some(CellField::Row),
            "K" => Some(CellField::Value),
            "E" => Some(CellField::Expression),
            "C" => Some(CellField::ColumnRef),
            "R" => Some(CellField::RowRef),
            "G" => Some(CellField::SharedValue),
            "D" => Some(CellField::SharedExpression),
            "S" => Some(CellField::SharedExpressionOrValueRefs),
            "N" => Some(CellField::NFlag),
            "P" => Some(CellField::PFlag),
            "H" => Some(CellField::Hidden),
            "M" => Some(CellField::MatrixExpression),
            "I" => Some(CellField::InsideMatrix),
            _ => None
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FieldValue {
    Row(u32),
    Column(u32),
    //    Value(CellValue),
    Expression(String),
    RefColumn(),
    RefRow(),
    SharedValue(),
    SharedValueOrExpr(),
    Protected(bool),
    Hidden(bool),
}