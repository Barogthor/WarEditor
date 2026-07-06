//! Parser de fichiers SLK (SYLK), le format tabulaire des données de jeu
//! de Warcraft III (voir `specs/SLKFormat.txt`).
//!
//! Point d'entrée : [`SLKScanner`], un itérateur faillible de
//! [`slk_type::Record`]s. Le scan travaille sur octets : les fins de ligne
//! sont détectées (`\n`, `\r` optionnel devant), pas configurées par OS,
//! et les échappements de la spec (`;;` dans un champ, `""` dans une
//! valeur quotée) sont appliqués.
//!
//! ```no_run
//! use slkparser::SLKScanner;
//!
//! let scanner = SLKScanner::open("resources/slk/sample_1.slk")?;
//! for record in scanner {
//!     let record = record?;
//!     // ...
//! }
//! # Ok::<(), slkparser::SLKError>(())
//! ```

use std::io;
use std::num::ParseIntError;

use thiserror::Error;

use crate::fields::FieldIter;
use crate::slk_type::{Record, RecordType};

pub mod cell;
pub mod document;
mod fields;
pub mod slk_type;

/// Erreurs de lecture et de parsing SLK.
///
/// `record_index` est l'ordinal (base 1) du record dans le fichier,
/// lignes vides exclues.
#[derive(Debug, Error)]
pub enum SLKError {
    /// Échec d'I/O à l'ouverture ou la lecture du fichier.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    /// Type d'enregistrement (RTD) inconnu.
    #[error("record {record_index}: invalid SLK record type '{record_type}'")]
    InvalidType {
        record_index: usize,
        record_type: String,
    },
    /// Champ numérique (`X`/`Y`) illisible.
    #[error("record {record_index}: invalid number in field '{field}' of {record_type:?} record: {source}")]
    ParseInt {
        record_index: usize,
        record_type: RecordType,
        field: String,
        #[source]
        source: ParseIntError,
    },
    /// Record structurellement invalide (octets non UTF-8 dans un champ
    /// numérique, etc.).
    #[error("record {record_index}: malformed record: {reason}")]
    Malformed { record_index: usize, reason: String },
    /// Contexte fichier ajouté par l'appelant (p. ex. `SLKData::load` dans `wce_map`).
    #[error("in file '{path}': {source}")]
    InFile {
        path: String,
        #[source]
        source: Box<SLKError>,
    },
}

/// Scanner d'un fichier SLK : itère des [`Record`]s faillibles.
///
/// L'itération s'arrête au record `E` (non yieldé) ou à la fin du tampon —
/// un fichier tronqué sans `E` final est toléré.
pub struct SLKScanner {
    buffer: Vec<u8>,
    pos: usize,
    record_index: usize,
    finished: bool,
}

impl SLKScanner {
    /// Ouvre un fichier SLK et prépare le scan. Ne lit aucun record.
    pub fn open(path: &str) -> Result<Self, SLKError> {
        Ok(Self::from_bytes(std::fs::read(path)?))
    }

    /// Scanner sur un tampon en mémoire — testable sans fichier.
    pub fn from_bytes(buffer: Vec<u8>) -> Self {
        SLKScanner {
            buffer,
            pos: 0,
            record_index: 0,
            finished: false,
        }
    }

    /// Bornes `(début, longueur)` de la prochaine ligne non vide, fin de
    /// ligne exclue. Un record se termine à `\n`, avec `\r` optionnel juste
    /// avant, quel que soit l'OS. Les lignes vides sont ignorées (spec :
    /// « Empty records are ignored »).
    fn next_line_bounds(&mut self) -> Option<(usize, usize)> {
        while self.pos < self.buffer.len() {
            let start = self.pos;
            let rest = &self.buffer[start..];
            let (line_len, skip) = match rest.iter().position(|&b| b == b'\n') {
                Some(nl) if nl > 0 && rest[nl - 1] == b'\r' => (nl - 1, nl + 1),
                Some(nl) => (nl, nl + 1),
                None if rest.ends_with(b"\r") => (rest.len() - 1, rest.len()),
                None => (rest.len(), rest.len()),
            };
            self.pos = start + skip;
            if line_len > 0 {
                return Some((start, line_len));
            }
        }
        None
    }
}

impl Iterator for SLKScanner {
    type Item = Result<Record, SLKError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let (start, len) = self.next_line_bounds()?;
        self.record_index += 1;
        let line = &self.buffer[start..start + len];
        let mut fields = FieldIter::new(line);
        // FieldIter yield toujours au moins un champ sur une ligne non vide.
        let type_field = fields.next()?;
        let record_type = match RecordType::from_bytes(&type_field, self.record_index) {
            Ok(record_type) => record_type,
            Err(e) => return Some(Err(e)),
        };
        if record_type == RecordType::EOF {
            self.finished = true;
            return None;
        }
        Some(Record::from_fields(record_type, fields, self.record_index))
    }
}

#[cfg(test)]
mod big_sample {
    use crate::get_resources_path;
    use crate::SLKScanner;

    #[test]
    fn ability_data_record_count() {
        let scanner = SLKScanner::open(&format!("{}slk/AbilityData.slk", get_resources_path()))
            .unwrap_or_else(|e| panic!("{:?}", e));
        let mut count = 0;
        for record in scanner {
            record.expect("record invalide");
            count += 1;
        }
        assert_eq!(count, 67387);
    }
}

#[cfg(test)]
fn get_resources_path() -> String {
    use std::path::Path;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("Should have parent directory");
    format!("{}/resources/", workspace_root.to_string_lossy())
}

#[cfg(test)]
mod sample {
    use crate::cell::Cell;
    use crate::document::Document;
    use crate::slk_type::Record;
    use crate::{get_resources_path, SLKScanner};

    fn get_path(path: &str) -> String {
        format!("{}slk/{path}", get_resources_path())
    }

    #[test]
    fn test_open() {
        SLKScanner::open(&get_path("sample_1.slk")).unwrap_or_else(|e| panic!("{:?}", e));
    }

    #[test]
    fn parse_all_records() {
        let scanner = SLKScanner::open(&get_path("sample_1.slk")).unwrap();
        let records: Vec<Record> = scanner.map(|r| r.expect("record invalide")).collect();
        assert_eq!(records.len(), 92);
        assert_eq!(records[0], Record::Header);
        assert!(records.contains(&Record::Info(3, 4)));
        let first_cell = Cell::new(1, Some(1), Some(String::from("a")));
        assert!(records.contains(&Record::CellContent(first_cell)));
    }

    #[test]
    fn document_loads_sample() {
        let scanner = SLKScanner::open(&get_path("sample_1.slk")).unwrap();
        let mut document = Document::default();
        document.load(scanner).expect("chargement sample_1");
        assert_eq!(document.row_count(), 3);
        assert_eq!(document.column_count(), 4);
        let cells = document.get_contents();
        assert_eq!(cells.len(), 12);
        assert_eq!(cells[0].value(), Some("a"));
        assert_eq!(cells[6].value(), Some("3"));
    }
}

#[cfg(test)]
mod malformed {
    use crate::slk_type::Record;
    use crate::{SLKError, SLKScanner};

    fn records(bytes: &[u8]) -> Vec<Result<Record, SLKError>> {
        SLKScanner::from_bytes(bytes.to_vec()).collect()
    }

    fn only_cell_value(bytes: &[u8]) -> Option<String> {
        records(bytes).into_iter().find_map(|r| match r {
            Ok(Record::CellContent(cell)) => cell.value().map(String::from),
            _ => None,
        })
    }

    #[test]
    fn semicolon_escape_in_cell_value() {
        assert_eq!(
            only_cell_value(b"ID;PWXL\r\nC;X1;Y1;K\"a;;b\"\r\nE\r\n"),
            Some(String::from("a;b"))
        );
    }

    #[test]
    fn mixed_line_endings_in_one_file() {
        let recs = records(b"ID;PWXL\nC;X1;Y1;K\"a\"\r\nE\n");
        assert_eq!(recs.len(), 2);
        assert!(recs.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn empty_buffer_yields_nothing() {
        assert!(records(b"").is_empty());
    }

    #[test]
    fn blank_lines_are_ignored() {
        let recs = records(b"ID;PWXL\r\n\r\n\nC;X1;Y1;K1\r\nE\r\n");
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn truncated_last_record_is_parsed() {
        // fichier coupé en plein record, sans fin de ligne ni record E
        assert_eq!(
            only_cell_value(b"ID;PWXL\r\nC;X1;Y1;K\"tronqu"),
            Some(String::from("tronqu"))
        );
    }

    #[test]
    fn missing_e_record_ends_cleanly() {
        let recs = records(b"ID;PWXL\r\nC;X1;Y1;K1\r\n");
        assert_eq!(recs.len(), 2);
        assert!(recs.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn records_after_e_are_not_read() {
        let recs = records(b"ID;PWXL\r\nE\r\nC;X1;Y1;K1\r\n");
        assert_eq!(recs.len(), 1);
    }

    #[test]
    fn unknown_record_type_is_an_error_not_a_panic() {
        let recs = records(b"ZZ;X1\r\nE\r\n");
        assert_eq!(recs.len(), 1);
        assert!(matches!(
            recs[0],
            Err(SLKError::InvalidType {
                record_index: 1,
                ..
            })
        ));
    }

    #[test]
    fn invalid_number_is_an_error_not_a_panic() {
        let recs = records(b"C;Xabc;Y1\r\nE\r\n");
        assert!(matches!(recs[0], Err(SLKError::ParseInt { .. })));
    }

    #[test]
    fn double_semicolon_merges_fields_per_spec() {
        // `X1;;Y2` = un seul champ `X1;Y2` → nombre invalide, erreur propre.
        // (Un champ vide en milieu de record n'est pas représentable : `;;`
        // est un échappement — c'est la spec, pas un choix.)
        let recs = records(b"C;X1;;Y2;K\"v\"\r\nE\r\n");
        assert!(matches!(recs[0], Err(SLKError::ParseInt { .. })));
    }

    #[test]
    fn fuzz_mutations_never_panic() {
        // Mini-fuzz déterministe : mutations xorshift d'un SLK valide.
        // Le parser peut retourner Err, jamais paniquer.
        let base = b"ID;PWXL\r\nB;Y3;X4\r\nC;Y1;X1;K\"a;;b\"\r\nC;X2;K\"say \"\"hi\"\"\"\r\nC;Y2;X1;K12\r\nE\r\n".to_vec();
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..2000 {
            let mut mutated = base.clone();
            let mutations = next() % 8 + 1;
            for _ in 0..mutations {
                let idx = (next() as usize) % mutated.len();
                mutated[idx] = (next() & 0xFF) as u8;
            }
            for record in SLKScanner::from_bytes(mutated) {
                let _ = record;
            }
        }
    }
}
