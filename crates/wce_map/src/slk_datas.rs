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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_preserves_prior_value_from_base_table() {
        // UnitData and UnitBalance are both keyed by unit id (see GameData::new, which merges
        // UnitData -> UnitBalance -> UnitUI -> UnitAbilities -> UnitWeapons into `unit_data`),
        // so merging them exercises the same-key column-offset branch of `merge` (as opposed to
        // two tables with disjoint id namespaces, where every row would just be inserted fresh).
        let base = format!("{}slk/UnitData.slk", crate::get_resources_path());
        assert!(
            std::path::Path::new(&base).exists(),
            "fixture missing: {base}"
        );
        let other = format!("{}slk/UnitBalance.slk", crate::get_resources_path());
        assert!(
            std::path::Path::new(&other).exists(),
            "fixture missing: {other}"
        );

        let a = SLKData::load(&base).unwrap();
        assert!(!a.headers().is_empty(), "no headers parsed from {base}");
        assert!(!a.lines().is_empty(), "no rows parsed from {base}");

        let b = SLKData::load(&other).unwrap();
        assert!(!b.headers().is_empty(), "no headers parsed from {other}");
        assert!(!b.lines().is_empty(), "no rows parsed from {other}");

        // Confirm the two tables actually share meta ids before relying on that to exercise the
        // same-key branch; a disjoint pair would make the rest of this test pass vacuously.
        let shared_ids: Vec<&MetaID> = a
            .lines()
            .keys()
            .filter(|id| b.lines().contains_key(*id))
            .collect();
        assert!(
            !shared_ids.is_empty(),
            "UnitData and UnitBalance share no ids; pick a different table pair"
        );

        let meta_id = shared_ids[0].clone();
        let base_row = a.get(&meta_id).unwrap().clone();
        let (column, value) = base_row
            .iter()
            .next()
            .map(|(col, val)| (*col, val.clone()))
            .expect("expected shared row to have at least one field value");
        let base_column_count = base_row.len();

        // Merge a table that shares this id; its columns are offset by the current header
        // count, so it must never overwrite columns already populated by the base table.
        let mut a = a;
        a.merge(&other).unwrap();

        let merged_fields = a
            .get(&meta_id)
            .unwrap_or_else(|| panic!("row {meta_id} dropped by merge"));

        // (a) the base table's value for this id/column survives unchanged.
        assert_eq!(
            merged_fields.get(&column),
            Some(&value),
            "merge overwrote a prior value for {meta_id}/{column}"
        );

        // (b) the same id gained columns from the second table (offset-merged, not replaced).
        assert!(
            merged_fields.len() > base_column_count,
            "merge did not add UnitBalance columns to {meta_id} (still has only {base_column_count} columns)"
        );
    }
}
