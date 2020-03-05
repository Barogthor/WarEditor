const TRUE: &str = "TRUE";
const FALSE: &str = "FALSE";

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum CellValue {
    Text(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

impl ToString for CellValue {
    fn to_string(&self) -> String {
        match self.clone(){
            CellValue::Text(text) => text,
            CellValue::Integer(value) => value.to_string(),
            CellValue::Float(value) => value.to_string(),
            CellValue::Bool(value) => {
                if value{
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
        }
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Cell {
    column: u32,
    row: Option<u32>,
//    expression: Option<String>,
    value: Option<CellValue>,
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
    pub fn get_value(&self) -> Option<CellValue>{
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
    pub fn new(column: u32, row: Option<u32>, value: Option<CellValue>) -> Self{
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
            println!("{:?}",field_content);
            match field_id{
                "Y" => cell.row = Some(field_content.parse::<u32>().unwrap()),
                "X" => cell.column = field_content.parse::<u32>().unwrap(),
                "K" => {
                    if field_content.starts_with("\"") && field_content.ends_with("\""){
                        let slice = &field_content[1..field_content.len()-1];
                        cell.value = Some(CellValue::Text(String::from(slice)));
                    }
                    else if field_content == TRUE || field_content == FALSE{
                        cell.value = Some(CellValue::Bool(field_content == "true"));
                    }
                    else if field_content.contains(",") || field_content.contains("."){
                        let v = field_content.parse::<f64>().unwrap();
                        cell.value = Some(CellValue::Float(v));
                    }
                    else {
                        let v= field_content.parse::<i64>();
                        cell.value = Some(CellValue::Integer(v.unwrap()));
                    }
                }
                _ => ()
            }
        }
        cell
    }
}

