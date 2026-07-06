//! Découpe des champs d'un enregistrement SLK.
//!
//! Règle de la spec (`specs/SLKFormat.txt:19-20`) : `;` sépare les champs,
//! `;;` encode un `;` littéral dans un champ. Conséquence : un champ
//! réellement vide n'est représentable qu'en fin de ligne (`...;`).
//!
//! [`FieldIter`] yield des `Cow<[u8]>` : emprunté tant qu'aucun
//! échappement n'est rencontré, alloué sinon.

use std::borrow::Cow;

/// Itérateur sur les champs d'une ligne de record (fin de ligne exclue),
/// avec dé-échappement `;;` → `;`.
pub(crate) struct FieldIter<'a> {
    line: &'a [u8],
    pos: usize,
    done: bool,
}

impl<'a> FieldIter<'a> {
    /// Prépare l'itération sur les champs de `line` (fin de ligne exclue).
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
        // `C;;` : `;;` = littéral, puis `;` = séparateur → ["C;", "X1"]
        assert_eq!(fields(b"C;;;X1"), vec![b"C;".to_vec(), b"X1".to_vec()]);
    }

    #[test]
    fn real_excel_format_string() {
        // Cas réel d'AbilityBuffMetaData.slk / sample_1.slk (records P)
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
