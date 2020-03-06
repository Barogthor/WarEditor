//const TRUE: &str = "TRUE";
//const FALSE: &str = "FALSE";

//#[derive(Debug, PartialOrd, PartialEq, Clone)]
//pub enum CellValue {
//    Text(String),
//    Integer(i64),
//    Float(f64),
//    Bool(bool),
//}

//impl ToString for CellValue {
//    fn to_string(&self) -> String {
//        match self.clone(){
//            CellValue::Text(text) => text,
//            CellValue::Integer(value) => value.to_string(),
//            CellValue::Float(value) => value.to_string(),
//            CellValue::Bool(value) => {
//                if value{
//                    "true".to_string()
//                } else {
//                    "false".to_string()
//                }
//            }
//        }
//    }
//}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Cell {
    column: u32,
    row: Option<u32>,
//    expression: Option<String>,
    value: Option<String>,
//    column_ref: Option<String>,
//    row_ref: Option<String>,
//    shared_value_definition: Option<String>,
//    shared_expression_definition: Option<String>,
//    shared_expression_or_value_refs: Option<String>,
//    is_protected: bool,
//    hidden: bool,
//    inside_matrix: bool,
//    matrix_expression: Option<String>,
}

impl Cell{
    pub fn get_value(&self) -> Option<String>{
        self.value.clone()
    }

    pub fn get_column(&self) -> u32{
        self.column
    }
    pub fn get_row(&self) -> Option<u32>{
        self.row
    }
}

impl Cell {
    pub fn new(column: u32, row: Option<u32>, value: Option<String>) -> Self{
        Cell{
            column,
            row,
            value
        }
    }

    pub fn parse(fields: &Vec<String>, line: Option<u32>) -> Self{
        let mut cell = Cell::default();
        for field in fields.iter(){
            let field_id = &field[0..1];
            let field_content = &field[1..];
//            println!("{:?}",field_content);
            match field_id{
                "Y" => cell.row = Some(field_content.parse::<u32>().unwrap()),
                "X" => cell.column = field_content.parse::<u32>().unwrap(),
                "K" => {
                    if field_content.starts_with("\""){
                        let slice = &field_content[1..field_content.len()-1];
                        cell.value = Some(String::from(slice));
                    }
                    else{
                        cell.value = Some(String::from(field_content));
                    }
                }
                _ => ()
            }
        }
        cell
    }
}

fn is_numeric(string: &str) -> bool{
    let try_parse = string.parse::<f64>();
    try_parse.is_ok()
}
