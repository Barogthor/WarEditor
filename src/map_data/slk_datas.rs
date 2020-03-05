use std::collections::HashMap;
use crate::slk::record::cell::{CellValue, RecordCell};
use crate::slk::document::SLKDocument;
use crate::slk::slk::SLKReader;
use vec_map::VecMap;

type MetaID = String;
type FieldName = String;

#[derive(Debug)]
pub struct SLKData {
    header: VecMap<String>,
//    map: HashMap<FieldID, VecMap<CellValue>>
    map: HashMap<MetaID, HashMap<FieldName,CellValue>>
}

impl SLKData {

    pub fn new() -> Self{
        Self{
            header: Default::default(),
            map: Default::default()
        }
    }

    pub fn debug(&self){
        println!("[Header]: {:?}",self.header);
        for (id, value) in self.map.iter() {
            println!("[{:?}] : {:?}",*id,*value);
        }
    }
    pub fn sizeof(&self) -> u32{
        let mut size = 0;
        size
    }

    fn process_cells(slk: &SLKDocument) -> VecMap<Vec<(FieldName,CellValue)>>{
        let mut header = VecMap::new();
        let mut lines = VecMap::new();
        let mut current_line = 0usize;
        for cell in slk.get_cells().iter(){
            if cell.row().is_some() {
                current_line = match cell.row() {
                    Some(line) => *line as usize,
                    _ => panic!("row can't be anything but integer")
                };
                if current_line > 1 {
                    lines.insert(current_line.to_owned(),Vec::new());
                }
            }
            let value = cell.value();
            if value.is_none() {continue;}
            let value = value.to_owned().unwrap();
            let x = cell.column();
            if current_line == 1{
                   let column_text = match value{
                     CellValue::Text(text) => text,
                       _ => panic!("Column head should be a string")
                   };
                header.insert(*x as usize, column_text);
            }
            else {
                let field_name = header.get(*x as usize);
                if field_name.is_none(){
                    continue;
                }
                let field_name = field_name.unwrap();
                let line = lines.get_mut(current_line).unwrap();
//                line.push((*x, value));
                line.push((field_name.to_owned(), value));
            }
        }
        lines
    }

    pub fn open<'a>(path: &str) -> Self{
        let doc = SLKReader::open_file(path.to_string()).parse().unwrap();
        let lines = Self::process_cells(&doc);
        let mut map: HashMap<MetaID, HashMap<FieldName,CellValue>> = HashMap::new();
        for (line, values) in lines{
            let mut id = String::new();
            let mut iter = values.iter();
            let (name,id_value) = iter.next().unwrap();
            let id = match id_value{
              CellValue::Text(id) => id,
                _ => panic!("First field value should be a string")
            };
            map.insert(id.to_owned(),HashMap::new());
            map.get_mut(id).unwrap().insert(name.to_owned(),id_value.to_owned());
            for (name,value) in iter{
                map.get_mut(id).unwrap().insert(name.to_owned(),value.to_owned());
            }
        }
        Self{
            map,
            header: Default::default()
        }
    }

    pub fn merge(&mut self, path: &str){
        let mut last_pos = self.header.len();
        let mut datas = Self::open(path);
        for (id, newline) in datas.map.iter(){
            if !self.map.contains_key(id){
                self.map.insert(id.to_owned(),newline.to_owned());
            }
            else {
                let current_line = self.map.get_mut(id).unwrap();
                for (field, value) in newline.iter(){
                    if !current_line.contains_key(field) {
                        current_line.insert(field.to_owned(), value.to_owned());
                    }
                }
            }

        }
    }

    pub fn get(&self, id: &str) -> Option<&HashMap<FieldName,CellValue>>{
        self.map.get(id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut HashMap<FieldName,CellValue>>{
        self.map.get_mut(id)
    }

}
