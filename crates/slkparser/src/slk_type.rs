use std::borrow::Cow;

use crate::{cell::Cell, SLKError};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Record {
    Header,
    Info(u32, u32),
    CellContent(Cell),
    CellFormat,
    Format,
    Options,
    Substitution,
    ExtLink,
    NameDefinitions,
    WindowDefinitions,
    ChartExtLink,
    EOF,
}

impl Record {
    /// Construit un enregistrement depuis ses champs déjà découpés et
    /// dé-échappés. Seuls Info (`B`) et CellContent (`C`) portent des
    /// données ; les autres types sont reconnus mais leurs champs ignorés.
    pub fn from_fields<'a, I>(
        record_type: RecordType,
        fields: I,
        record_index: usize,
    ) -> Result<Record, SLKError>
    where
        I: Iterator<Item = Cow<'a, [u8]>>,
    {
        match record_type {
            RecordType::EOF => Ok(Record::EOF),
            RecordType::Header => Ok(Record::Header),
            RecordType::Info => {
                let mut rows = 0u32;
                let mut columns = 0u32;
                for field in fields {
                    match field.split_first() {
                        Some((b'Y', content)) => {
                            rows = parse_u32(content, record_type, "Y", record_index)?
                        }
                        Some((b'X', content)) => {
                            columns = parse_u32(content, record_type, "X", record_index)?
                        }
                        _ => (),
                    }
                }
                Ok(Record::Info(rows, columns))
            }
            RecordType::CellContent => Ok(Record::CellContent(Cell::from_fields(
                fields,
                record_index,
            )?)),
            RecordType::Format => Ok(Record::Format),
            RecordType::ChartExtLink => Ok(Record::ChartExtLink),
            RecordType::CellFormat => Ok(Record::CellFormat),
            RecordType::Options => Ok(Record::Options),
            RecordType::Substitution => Ok(Record::Substitution),
            RecordType::ExtLink => Ok(Record::ExtLink),
            RecordType::NameDefinitions => Ok(Record::NameDefinitions),
            RecordType::WindowDefinitions => Ok(Record::WindowDefinitions),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum RecordType {
    Header,
    Info,
    CellContent,
    CellFormat,
    Format,
    Options,
    Substitution,
    ExtLink,
    NameDefinitions,
    WindowDefinitions,
    ChartExtLink,
    EOF,
}

impl RecordType {
    /// Identifie le type d'enregistrement depuis son RTD brut (premier champ).
    pub fn from_bytes(id: &[u8], record_index: usize) -> Result<Self, SLKError> {
        match id {
            b"ID" => Ok(RecordType::Header),
            b"B" => Ok(RecordType::Info),
            b"C" => Ok(RecordType::CellContent),
            b"P" => Ok(RecordType::CellFormat),
            b"F" => Ok(RecordType::Format),
            b"O" => Ok(RecordType::Options),
            b"NU" => Ok(RecordType::Substitution),
            b"NE" => Ok(RecordType::ExtLink),
            b"NN" => Ok(RecordType::NameDefinitions),
            b"W" => Ok(RecordType::WindowDefinitions),
            b"NL" => Ok(RecordType::ChartExtLink),
            b"E" => Ok(RecordType::EOF),
            _ => Err(SLKError::InvalidType {
                record_index,
                record_type: String::from_utf8_lossy(id).into_owned(),
            }),
        }
    }
}

/// Parse un entier `u32` depuis les octets d'un contenu de champ.
///
/// Erreurs : [`SLKError::Malformed`] si les octets ne sont pas de l'UTF-8,
/// [`SLKError::ParseInt`] si le texte n'est pas un entier.
pub(crate) fn parse_u32(
    bytes: &[u8],
    record_type: RecordType,
    field: &str,
    record_index: usize,
) -> Result<u32, SLKError> {
    let text = std::str::from_utf8(bytes).map_err(|_| SLKError::Malformed {
        record_index,
        reason: format!("non-UTF-8 bytes in numeric field '{field}'"),
    })?;
    text.parse::<u32>().map_err(|source| SLKError::ParseInt {
        record_index,
        record_type,
        field: field.into(),
        source,
    })
}

#[cfg(test)]
mod from_fields_tests {
    use crate::fields::FieldIter;
    use crate::cell::Cell;
    use crate::slk_type::{Record, RecordType};
    use crate::SLKError;

    #[test]
    fn record_type_from_bytes() {
        assert_eq!(RecordType::from_bytes(b"ID", 1).unwrap(), RecordType::Header);
        assert_eq!(RecordType::from_bytes(b"B", 1).unwrap(), RecordType::Info);
        assert_eq!(RecordType::from_bytes(b"C", 1).unwrap(), RecordType::CellContent);
        assert_eq!(RecordType::from_bytes(b"E", 1).unwrap(), RecordType::EOF);
        assert!(matches!(
            RecordType::from_bytes(b"ZZ", 7),
            Err(SLKError::InvalidType { record_index: 7, .. })
        ));
    }

    #[test]
    fn info_record_from_fields() {
        let fields = FieldIter::new(b"Y3;X4");
        let record = Record::from_fields(RecordType::Info, fields, 2).unwrap();
        assert_eq!(record, Record::Info(3, 4));
    }

    #[test]
    fn info_record_bad_number_is_error() {
        let fields = FieldIter::new(b"Yabc;X4");
        assert!(matches!(
            Record::from_fields(RecordType::Info, fields, 2),
            Err(SLKError::ParseInt { record_index: 2, .. })
        ));
    }

    #[test]
    fn cell_record_from_fields() {
        let fields = FieldIter::new(b"X1;Y1;K\"a\"");
        let record = Record::from_fields(RecordType::CellContent, fields, 5).unwrap();
        let expected = Cell::new(1, Some(1), Some(String::from("a")));
        assert_eq!(record, Record::CellContent(expected));
    }

    #[test]
    fn ignored_record_types_pass_through() {
        let fields = FieldIter::new(b"PGeneral;extra");
        let record = Record::from_fields(RecordType::CellFormat, fields, 3).unwrap();
        assert_eq!(record, Record::CellFormat);
    }
}
