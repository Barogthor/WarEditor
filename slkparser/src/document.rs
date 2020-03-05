use crate::record::cell::Cell;
use crate::SLKScanner;
use crate::slk_type::Record;

#[derive(Default, Debug)]
pub struct Document {
    rows: u32,
    columns: u32,

    contents: Vec<Cell>,
}

impl Document {
    pub fn load(&mut self, scanner: SLKScanner){
        for record in scanner{
            match record {
                Record::Info(rows, columns) => {
                    self.rows = rows;
                    self.columns = columns;
                },
                Record::CellContent(cell) => self.contents.push(cell),
                _ => ()
            }
        }
    }

    pub fn get_contents(&self) -> &Vec<Cell>{
        &self.contents
    }

    pub fn debug(&self){
        println!("{:#?}", self);
    }
}

