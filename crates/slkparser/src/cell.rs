//! Contents of an SLK cell (`C` record): column, optional row and
//! value (`K` field), unescaped from the file's raw bytes.

use std::borrow::Cow;

use crate::slk_type::{parse_u32, RecordType};
use crate::SLKError;

/// A cell of an SLK file: its column (`X`), its optional row
/// (`Y`, absent if unchanged since the previous cell) and its value
/// (`K`).
#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Cell {
    column: u32,
    row: Option<u32>,
    value: Option<String>,
}

impl Cell {
    /// Column of the cell (`X` field).
    pub fn get_column(&self) -> u32 {
        self.column
    }
    /// Row of the cell (`Y` field), if present in the record.
    pub fn get_row(&self) -> Option<u32> {
        self.row
    }
}

impl Cell {
    /// Builds a cell from its already resolved components.
    pub fn new(column: u32, row: Option<u32>, value: Option<String>) -> Self {
        Cell { column, row, value }
    }

    /// Value of the cell (`K` field), without copying.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Builds a cell from the (already unescaped) fields of a
    /// `C` record. Only `X`, `Y` and `K` are retained; the other FTDs
    /// (`;E`, `;S`, …) and empty fields are ignored.
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

/// Decodes a `K` value: strips the surrounding quotes and unescapes
/// `""` into `"`. Tolerant: an opening quote with no closing one (`K"abc`)
/// keeps the content as-is. The UTF-8 conversion is lossy — Blizzard
/// SLKs are ASCII, this is a safety net, not a hot path.
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
    use super::Cell;
    use crate::fields::FieldIter;
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
        // FieldIter has already unescaped `;;` → the K field contains `a;b`
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
        // line ending with `;` → trailing empty field, ignored
        let parsed = cell(b"X1;Y1;").unwrap();
        assert_eq!(parsed.get_column(), 1);
        assert_eq!(parsed.value(), None);
    }

    #[test]
    fn non_utf8_value_is_lossy() {
        assert_eq!(
            cell(b"X1;K\"caf\xe9\"").unwrap().value(),
            Some("caf\u{FFFD}")
        );
    }

    #[test]
    fn non_utf8_coordinate_is_error() {
        assert!(matches!(
            cell(b"X\xff;Y1"),
            Err(SLKError::Malformed {
                record_index: 1,
                ..
            })
        ));
    }

    #[test]
    fn coordinates_are_parsed() {
        let parsed = cell(b"Y2;X3;K\"v\"").unwrap();
        assert_eq!(parsed.get_row(), Some(2));
        assert_eq!(parsed.get_column(), 3);
    }
}
