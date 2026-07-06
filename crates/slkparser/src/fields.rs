//! Splits the fields of an SLK record.
//!
//! Spec rule (`specs/SLKFormat.txt:19-20`): `;` separates fields,
//! `;;` encodes a literal `;` within a field. Consequence: a truly
//! empty field is only representable at the end of a line (`...;`).
//!
//! [`FieldIter`] yields `Cow<[u8]>`: borrowed as long as no escape
//! is encountered, allocated otherwise.

use std::borrow::Cow;

/// Iterator over the fields of a record line (line ending excluded),
/// with `;;` → `;` unescaping.
pub(crate) struct FieldIter<'a> {
    line: &'a [u8],
    pos: usize,
    done: bool,
}

impl<'a> FieldIter<'a> {
    /// Prepares iteration over the fields of `line` (line ending excluded).
    pub(crate) fn new(line: &'a [u8]) -> Self {
        FieldIter {
            line,
            pos: 0,
            done: line.is_empty(),
        }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Cow<'a, [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let start = self.pos;
        let mut unescaped: Option<Vec<u8>> = None;
        let mut i = self.pos;
        while i < self.line.len() {
            if self.line[i] == b';' {
                if self.line.get(i + 1) == Some(&b';') {
                    let buf = unescaped.get_or_insert_with(|| self.line[start..i].to_vec());
                    buf.push(b';');
                    i += 2;
                    continue;
                }
                self.pos = i + 1;
                return Some(match unescaped {
                    Some(buf) => Cow::Owned(buf),
                    None => Cow::Borrowed(&self.line[start..i]),
                });
            }
            if let Some(buf) = unescaped.as_mut() {
                buf.push(self.line[i]);
            }
            i += 1;
        }
        self.done = true;
        Some(match unescaped {
            Some(buf) => Cow::Owned(buf),
            None => Cow::Borrowed(&self.line[start..]),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::FieldIter;

    fn fields(line: &[u8]) -> Vec<Vec<u8>> {
        FieldIter::new(line).map(|f| f.into_owned()).collect()
    }

    #[test]
    fn splits_on_semicolon() {
        assert_eq!(
            fields(b"C;X1;Y2"),
            vec![b"C".to_vec(), b"X1".to_vec(), b"Y2".to_vec()]
        );
    }

    #[test]
    fn double_semicolon_is_a_literal_semicolon() {
        assert_eq!(
            fields(b"P;PGeneral;;suffix"),
            vec![b"P".to_vec(), b"PGeneral;suffix".to_vec()]
        );
    }

    #[test]
    fn escape_then_separator() {
        // `C;;`: `;;` = literal, then `;` = separator → ["C;", "X1"]
        assert_eq!(fields(b"C;;;X1"), vec![b"C;".to_vec(), b"X1".to_vec()]);
    }

    #[test]
    fn real_excel_format_string() {
        // Real case from AbilityBuffMetaData.slk / sample_1.slk (P records)
        assert_eq!(
            fields(b"P;P#,##0_);;\\-#,##0_)"),
            vec![b"P".to_vec(), b"P#,##0_);\\-#,##0_)".to_vec()]
        );
    }

    #[test]
    fn empty_line_yields_nothing() {
        assert!(fields(b"").is_empty());
    }

    #[test]
    fn single_field_line() {
        assert_eq!(fields(b"E"), vec![b"E".to_vec()]);
    }

    #[test]
    fn trailing_separator_yields_trailing_empty_field() {
        assert_eq!(fields(b"O;"), vec![b"O".to_vec(), vec![]]);
    }

    #[test]
    fn no_allocation_without_escape() {
        let line: &[u8] = b"C;X1;K\"abc\"";
        assert!(FieldIter::new(line).all(|f| matches!(f, Cow::Borrowed(_))));
    }
}
