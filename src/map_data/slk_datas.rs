use std::collections::{HashMap, BTreeMap};
use crate::map_data::slk_datas::adapter::{ScannerAdapter, DocumentAdapter};
use slkparser::record::cell::{Cell};

type MetaID = String;
type FieldColumn = u32;
const HEADER_ROW: u32 = 1;

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
    headers: BTreeMap<FieldColumn, String>,
//    map: HashMap<FieldID, VecMap<CellValue>>
    lines: HashMap<MetaID, BTreeMap<FieldColumn,String>>
}

fn process_cells(cells: &Vec<Cell>) -> (BTreeMap<FieldColumn, String>, HashMap<MetaID, BTreeMap<FieldColumn,String>>){
    let mut headers = BTreeMap::new();
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
        if row == HEADER_ROW{
            let header_pos = cell.get_column();
            let header_label = cell.get_value().unwrap().to_string();
            let header_label =
                if header_label.is_empty() {
                    String::from("Unknown")
                } else {
                    header_label
                };

            headers.insert(header_pos, header_label);
        } else {
            let column_header = cell.get_column();
            let field_value = cell.get_value().unwrap_or(Default::default()).to_string();
            if cell.get_row().is_some(){
                meta_id_holder = field_value;
                lines.insert(meta_id_holder.clone(), BTreeMap::new());
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
                self.lines.insert(meta_id.clone(), BTreeMap::new());
            }
            let self_parameters = self.lines.get_mut(&meta_id).unwrap();
            for (column, parameter) in parameters{
                self_parameters.insert(headers_count + column, parameter);
            }
        }
        for (column, label) in headers {
            self.headers.insert(headers_count+column, label);
        }


    }

//    pub fn debug(&self){
//        println!("[Header]: {:?}",self.header);
//        for (id, value) in self.map.iter() {
//            println!("[{:?}] : {:?}",*id,*value);
//        }
//    }

    pub fn get(&self, id: &MetaID) -> Option<&BTreeMap<FieldColumn,String>>{
        self.lines.get(id)
    }

    pub fn headers(&self) -> &BTreeMap<FieldColumn, String>{
        &self.headers
    }

    pub fn get_formatted(&self, id: &MetaID) -> Option<BTreeMap<String,String>>{
        let v = self.get(id);
        let mut counter = 1;
        if v.is_none() {
            return None;
        }
        let meta = v.unwrap();
        let mut res = BTreeMap::new();
        for (column, value) in meta {
            let key = self.headers.get(column);
            let key = if key.is_none() {
                    format!("Unknown{}", counter)
                } else {
                    key.unwrap().to_string()
                };
            res.insert(key.clone(), value.clone());
        }
        Some(res)
    }
//    pub fn get_mut(&mut self, id: &str) -> Option<&mut HashMap<FieldName,CellValue>>{
//        self.map.get_mut(id)
//    }

}
