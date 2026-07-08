//! Loads Warcraft III SLK game-data tables (unit/ability/doodad/upgrade metadata, …) into
//! `SLKData`, a header-indexed lookup keyed by each row's meta ID; used by `GameData`.

use std::collections::{BTreeMap, HashMap};

use slkparser::{cell::Cell, SLKError};

use crate::slk_datas::adapter::{DocumentAdapter, ScannerAdapter};

type MetaID = String;
type FieldColumn = u32;
const HEADER_ROW: u32 = 1;

mod adapter {
    use slkparser::cell::Cell;
    use slkparser::document::Document;
    use slkparser::{SLKError, SLKScanner};

    pub struct ScannerAdapter {
        scanner: SLKScanner,
    }

    impl ScannerAdapter {
        pub fn open(path: &str) -> Result<Self, SLKError> {
            Ok(ScannerAdapter {
                scanner: SLKScanner::open(path)?,
            })
        }
    }

    pub struct DocumentAdapter {
        document: Document,
    }

    impl DocumentAdapter {
        pub fn load(scanner: ScannerAdapter) -> Result<DocumentAdapter, SLKError> {
            let mut document = Document::default();
            document.load(scanner.scanner)?;
            Ok(DocumentAdapter { document })
        }

        pub fn get_contents(&self) -> &Vec<Cell> {
            self.document.get_contents()
        }
    }
}

#[derive(Debug)]
pub struct SLKData {
    headers: BTreeMap<FieldColumn, String>,
    //    map: HashMap<FieldID, VecMap<CellValue>>
    lines: HashMap<MetaID, BTreeMap<FieldColumn, String>>,
}

fn process_cells(
    cells: &Vec<Cell>,
) -> (
    BTreeMap<FieldColumn, String>,
    HashMap<MetaID, BTreeMap<FieldColumn, String>>,
) {
    let mut headers = BTreeMap::new();
    let mut lines = HashMap::new();
    let mut row = 0;
    let mut meta_id_holder = String::default();
    for cell in cells {
        let value = cell.value();
        if value.is_none() {
            log::warn!("Value is none: {cell:?}, row: {row}");
        }
        if let Some(cell_row) = cell.get_row() {
            row = cell_row;
        }
        if row == HEADER_ROW {
            let header_pos = cell.get_column();
            let header_label = match value {
                Some(label) if !label.is_empty() => label.to_owned(),
                _ => String::from("Unknown"),
            };
            headers.insert(header_pos, header_label);
        } else {
            let column_header = cell.get_column();
            let field_value = value.unwrap_or("").to_owned();
            if cell.get_row().is_some() {
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

fn in_file(path: &str, source: SLKError) -> SLKError {
    SLKError::InFile {
        path: path.to_string(),
        source: Box::new(source),
    }
}

impl Default for SLKData {
    fn default() -> Self {
        Self::new()
    }
}

impl SLKData {
    pub fn new() -> Self {
        Self {
            headers: Default::default(),
            lines: Default::default(),
        }
    }
    pub fn load(path: &str) -> Result<Self, SLKError> {
        // println!("========== Parse file: {}",path);
        let scanner = ScannerAdapter::open(path).map_err(|e| in_file(path, e))?;
        let document = DocumentAdapter::load(scanner).map_err(|e| in_file(path, e))?;
        let cells = document.get_contents();

        let (headers, lines) = process_cells(cells);

        Ok(SLKData { headers, lines })
    }

    pub fn merge(&mut self, path: &str) -> Result<(), SLKError> {
        // println!("========== Merge file: {}",path);
        let scanner = ScannerAdapter::open(path).map_err(|e| in_file(path, e))?;
        let document = DocumentAdapter::load(scanner).map_err(|e| in_file(path, e))?;
        let cells = document.get_contents();
        let (headers, lines) = process_cells(cells);
        let headers_count = self.headers.len() as u32;

        for (meta_id, parameters) in lines {
            if !self.lines.contains_key(&meta_id) {
                self.lines.insert(meta_id.clone(), BTreeMap::new());
            }
            let self_parameters = self.lines.get_mut(&meta_id).unwrap();
            for (column, parameter) in parameters {
                if parameter == "#VALUE!" || parameter == "-" || parameter == "_" {
                    self_parameters.insert(headers_count + column, String::new());
                } else {
                    self_parameters.insert(headers_count + column, parameter.trim().to_string());
                }
            }
        }
        for (column, label) in headers {
            self.headers.insert(headers_count + column, label);
        }
        Ok(())
    }

    // Reachable only from the cfg(test) gamedata_snapshot regression test
    // (no production accessor consumes per-row lookups yet).
    #[allow(dead_code)]
    pub fn get(&self, id: &MetaID) -> Option<&BTreeMap<FieldColumn, String>> {
        self.lines.get(id)
    }

    // Reachable only from the cfg(test) gamedata_snapshot regression test.
    #[allow(dead_code)]
    pub fn headers(&self) -> &BTreeMap<FieldColumn, String> {
        &self.headers
    }

    // Reachable only from the cfg(test) gamedata_snapshot regression test.
    #[allow(dead_code)]
    pub(crate) fn lines(&self) -> &HashMap<MetaID, BTreeMap<FieldColumn, String>> {
        &self.lines
    }

    // Reachable only from the cfg(test) gamedata_snapshot regression test.
    #[allow(dead_code)]
    pub fn get_formatted(&self, id: &MetaID) -> Option<BTreeMap<String, String>> {
        let v = self.get(id);
        let counter = 1;
        v?;
        let meta = v.unwrap();
        let mut res = BTreeMap::new();
        for (column, value) in meta {
            let key = self.headers.get(column);
            let key = if key.is_none() {
                format!("Unknown{counter}")
            } else {
                key.map(String::to_string).unwrap()
            };
            res.insert(key.clone(), value.clone());
        }
        Some(res)
    }
    //    pub fn get_mut(&mut self, id: &str) -> Option<&mut HashMap<FieldName,CellValue>>{
    //        self.map.get_mut(id)
    //    }
}
