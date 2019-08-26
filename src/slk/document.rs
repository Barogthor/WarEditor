use crate::slk::RecordType;
use std::collections::HashMap;
use crate::slk::record::cell::{RecordCell, CellValue};

#[derive(Debug, Getters, Setters, Default)]
pub struct SLKDocument {
    #[get = "pub with_prefix"] #[set = "pub with_prefix"]
    rows: u32,
    #[get = "pub with_prefix"] #[set = "pub with_prefix"]
    columns: u32,
    #[get = "pub with_prefix"]
    cells: Vec<RecordCell>,
    //TODO others
}

impl SLKDocument {

    pub fn add_cell(&mut self, cell: RecordCell){
        self.cells.push(cell);
    }

    pub fn get_cells_value_sorted_by_line(&self) -> HashMap<u32,Vec<(u32,CellValue)>>{
//        let mut lines: Vec<(u32,Vec<(u32,Value)>)> = vec![];
        let mut lines: HashMap<u32,Vec<(u32,CellValue)>> = HashMap::new();
        let mut current_line = 0;
        for cell in self.cells.iter(){
            if cell.row().is_some(){
                lines.insert(cell.row().unwrap(), vec![]);
                current_line = cell.row().unwrap().to_owned();
            }

            if cell.value().is_some() {
                let line = lines.get_mut(&current_line).unwrap();
                let val: &Option<CellValue> = cell.value();
                let val = val.to_owned().unwrap();
                line.push((cell.column().to_owned(), val ));
            }
        }
        lines
    }

    pub fn debug(&self){
        println!("{:#?}", self);
    }
}
