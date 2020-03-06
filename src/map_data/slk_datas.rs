use std::collections::HashMap;
use crate::map_data::slk_datas::adapter::{ScannerAdapter, DocumentAdapter};
use slkparser::record::cell::{Cell};

type MetaID = String;
type FieldColumn = u32;

mod adapter{
    use slkparser::document::Document;
    use slkparser::SLKScanner;
    use slkparser::slk_type::Record;
    use slkparser::record::cell::Cell;

    pub struct ScannerAdapter {
        scanner: SLKScanner
    }

    impl ScannerAdapter {
        pub fn open(path: &str) -> ScannerAdapter{
            ScannerAdapter{
                scanner: SLKScanner::open(path)
            }
        }
    }
    impl Iterator for ScannerAdapter{
        type Item = Record;

        fn next(&mut self) -> Option<Self::Item> {
            self.scanner.next()
        }
    }

    pub struct DocumentAdapter {
        document: Document
    }

    impl DocumentAdapter {
        pub fn load(scanner: ScannerAdapter) -> DocumentAdapter{
            let mut document = Document::default();
            document.load(scanner.scanner);
            DocumentAdapter{
                document
            }
        }

        pub fn get_contents(&self) -> &Vec<Cell>{
            self.document.get_contents()
        }
        pub fn row_count(&self) -> u32 {self.document.row_count()}
        pub fn column_count(&self) -> u32 {self.document.column_count()}
    }
}

#[derive(Debug)]
pub struct SLKData {
    headers: HashMap<FieldColumn, String>,
//    map: HashMap<FieldID, VecMap<CellValue>>
    lines: HashMap<MetaID, HashMap<FieldColumn,String>>
}

fn process_cells(cells: &Vec<Cell>) -> (HashMap<FieldColumn, String>, HashMap<MetaID, HashMap<FieldColumn,String>>){
    let mut headers = HashMap::new();
    let mut lines = HashMap::new();
    let mut row = 0;
    let mut meta_id_holder = String::default();
    for cell in cells{
        if cell.get_value().is_none(){
            println!("Value is none: {:?}, row: {}",cell, row);
        }
        if cell.get_row().is_some(){
            row = cell.get_row().unwrap();
        }
        if row == 1{
            let header_pos = cell.get_column();
            let header_label = cell.get_value().unwrap().to_string();
            headers.insert(header_pos, header_label);
        } else {
            let column_header = cell.get_column();
            let field_value = cell.get_value().unwrap_or(Default::default()).to_string();
            if cell.get_row().is_some(){
                meta_id_holder = field_value;
                lines.insert(meta_id_holder.clone(), HashMap::new());
            } else {
                let parameters = lines.get_mut(&meta_id_holder).unwrap();
                parameters.insert(column_header, field_value);
            }
        }
    }
    (headers, lines)
}

impl SLKData {
    pub fn new() -> Self{
        Self{
            headers: Default::default(),
            lines: Default::default()
        }
    }
    pub fn load(path: &str) -> Self{
        println!("========== Parse file: {}",path);
        let scanner = ScannerAdapter::open(path);
        let document = DocumentAdapter::load(scanner);
        let cells = document.get_contents();

        let (headers, lines) = process_cells(cells);

        SLKData{
            headers,
            lines
        }
    }

    pub fn merge(&mut self, path: &str){
        println!("========== Merge file: {}",path);
        let scanner = ScannerAdapter::open(path);
        let document = DocumentAdapter::load(scanner);
        let cells = document.get_contents();
        let (headers, lines) = process_cells(cells);
        let headers_count = self.headers.len() as u32;
        for (meta_id, parameters) in lines{
            if !self.lines.contains_key(&meta_id){
                self.lines.insert(meta_id.clone(), HashMap::new());
            }
            let self_parameters = self.lines.get_mut(&meta_id).unwrap();
            for (column, parameter) in parameters{
                self_parameters.insert(headers_count + column, parameter);
            }
        }

    }

//
//    pub fn debug(&self){
//        println!("[Header]: {:?}",self.header);
//        for (id, value) in self.map.iter() {
//            println!("[{:?}] : {:?}",*id,*value);
//        }
//    }
//    pub fn sizeof(&self) -> u32{
//        let mut size = 0;
//        size
//    }

//    pub fn get(&self, id: &str) -> Option<&HashMap<FieldName,CellValue>>{
//        self.map.get(id)
//    }
//    pub fn get_mut(&mut self, id: &str) -> Option<&mut HashMap<FieldName,CellValue>>{
//        self.map.get_mut(id)
//    }

}
