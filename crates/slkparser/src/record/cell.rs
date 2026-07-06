use std::borrow::Cow;

use crate::slk_type::{parse_u32, RecordType};
use crate::SLKError;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Cell {
    column: u32,
    row: Option<u32>,
    value: Option<String>,
}

impl Cell {
    pub fn get_value(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn get_column(&self) -> u32 {
        self.column
    }
    pub fn get_row(&self) -> Option<u32> {
        self.row
    }
}

impl Cell {
    pub fn new(column: u32, row: Option<u32>, value: Option<String>) -> Self {
        Cell { column, row, value }
    }

    pub fn parse(fields: &[String], _line: Option<u32>) -> Result<Self, SLKError> {
        let mut cell = Cell::default();
        for field in fields.iter() {
            let field_id = &field[0..1];
            let field_content = &field[1..];
            //            println!("{:?}",field_content);
            match field_id {
                "Y" => {
                    cell.row =
                        Some(
                            field_content
                                .parse::<u32>()
                                .map_err(|e| SLKError::Parsing {
                                    record_type: RecordType::CellContent,
                                    content: "Y".into(),
                                    source: e,
                                })?,
                        )
                }
                "X" => {
                    cell.column = field_content
                        .parse::<u32>()
                        .map_err(|e| SLKError::Parsing {
                            record_type: RecordType::CellContent,
                            content: "X".into(),
                            source: e,
                        })?
                }
                "K" => {
                    if field_content.starts_with("\"") {
                        let slice = &field_content[1..field_content.len() - 1];
                        cell.value = Some(String::from(slice));
                    } else {
                        cell.value = Some(String::from(field_content));
                    }
                }
                _ => (),
            }
        }
        Ok(cell)
    }

    /// Valeur de la cellule (champ `K`), sans copie.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Construit une cellule depuis les champs (déjà dé-échappés) d'un
    /// record `C`. Seuls `X`, `Y` et `K` sont retenus ; les autres FTD
    /// (`;E`, `;S`, …) et les champs vides sont ignorés.
    pub(crate) fn from_fields<'a, I>(fields: I, record_index: usize) -> Result<Self, SLKError>
    where
        I: Iterator<Item = Cow<'a, [u8]>>,
    {
        let mut cell = Cell::default();
        for field in fields {
            match field.split_first() {
                Some((b'Y', content)) => {
                    cell.row = Some(parse_u32(
                        content,
                        RecordType::CellContent,
                        "Y",
                        record_index,
                    )?)
                }
                Some((b'X', content)) => {
                    cell.column = parse_u32(content, RecordType::CellContent, "X", record_index)?
                }
                Some((b'K', content)) => cell.value = Some(decode_value(content)),
                _ => (),
            }
        }
        Ok(cell)
    }
}

/// Décode une valeur `K` : retire les guillemets englobants et dé-échappe
/// `""` en `"`. Tolérant : un guillemet ouvrant sans fermant (`K"abc`)
/// garde le contenu tel quel. La conversion UTF-8 est lossy — les SLK
/// Blizzard sont ASCII, c'est un garde-fou, pas un chemin chaud.
fn decode_value(content: &[u8]) -> String {
    let inner = match content.split_first() {
        Some((b'"', after)) => match after.split_last() {
            Some((b'"', mid)) => mid,
            _ => after,
        },
        _ => content,
    };
    if inner.windows(2).any(|w| w == b"\"\"") {
        let mut buf = Vec::with_capacity(inner.len());
        let mut i = 0;
        while i < inner.len() {
            if inner[i] == b'"' && inner.get(i + 1) == Some(&b'"') {
                buf.push(b'"');
                i += 2;
            } else {
                buf.push(inner[i]);
                i += 1;
            }
        }
        match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
        }
    } else {
        String::from_utf8_lossy(inner).into_owned()
    }
}

#[cfg(test)]
mod from_fields_tests {
    use crate::fields::FieldIter;
    use crate::record::cell::Cell;
    use crate::SLKError;

    fn cell(line: &[u8]) -> Result<Cell, SLKError> {
        Cell::from_fields(FieldIter::new(line), 1)
    }

    #[test]
    fn quoted_value() {
        assert_eq!(cell(b"X1;Y1;K\"a\"").unwrap().value(), Some("a"));
    }

    #[test]
    fn unquoted_numeric_value() {
        assert_eq!(cell(b"X2;K1").unwrap().value(), Some("1"));
    }

    #[test]
    fn escaped_semicolon_in_value() {
        // FieldIter a déjà dé-échappé `;;` → le champ K contient `a;b`
        assert_eq!(cell(b"X1;Y1;K\"a;;b\"").unwrap().value(), Some("a;b"));
    }

    #[test]
    fn doubled_quotes_are_unescaped() {
        assert_eq!(
            cell(b"X1;Y1;K\"say \"\"hi\"\"\"").unwrap().value(),
            Some(r#"say "hi""#)
        );
    }

    #[test]
    fn lone_quote_is_empty_value_not_panic() {
        assert_eq!(cell(b"X1;Y1;K\"").unwrap().value(), Some(""));
    }

    #[test]
    fn unterminated_quote_keeps_content() {
        assert_eq!(cell(b"X1;Y1;K\"abc").unwrap().value(), Some("abc"));
    }

    #[test]
    fn empty_field_is_ignored_not_panic() {
        // ligne finissant par `;` → champ vide final, ignoré
        let parsed = cell(b"X1;Y1;").unwrap();
        assert_eq!(parsed.get_column(), 1);
        assert_eq!(parsed.value(), None);
    }

    #[test]
    fn non_utf8_value_is_lossy() {
        assert_eq!(cell(b"X1;K\"caf\xe9\"").unwrap().value(), Some("caf\u{FFFD}"));
    }

    #[test]
    fn non_utf8_coordinate_is_error() {
        assert!(matches!(
            cell(b"X\xff;Y1"),
            Err(SLKError::Malformed { record_index: 1, .. })
        ));
    }

    #[test]
    fn coordinates_are_parsed() {
        let parsed = cell(b"Y2;X3;K\"v\"").unwrap();
        assert_eq!(parsed.get_row(), Some(2));
        assert_eq!(parsed.get_column(), 3);
    }
}
