//! Agrégat en mémoire d'un fichier SLK : dimensions (record `B`) et
//! cellules (records `C`), dans l'ordre du fichier.

use crate::record::cell::Cell;
use crate::slk_type::Record;
use crate::{SLKError, SLKScanner};

/// Contenu d'un fichier SLK chargé : dimensions et cellules.
#[derive(Default, Debug)]
pub struct Document {
    rows: u32,
    columns: u32,
    contents: Vec<Cell>,
}

impl Document {
    /// Charge tous les records du scanner.
    /// S'arrête à la première erreur et la propage.
    pub fn load(&mut self, scanner: SLKScanner) -> Result<(), SLKError> {
        for record in scanner {
            match record? {
                Record::Info(rows, columns) => {
                    self.rows = rows;
                    self.columns = columns;
                }
                Record::CellContent(cell) => self.contents.push(cell),
                _ => (),
            }
        }
        Ok(())
    }

    /// Cellules dans l'ordre du fichier.
    pub fn get_contents(&self) -> &Vec<Cell> {
        &self.contents
    }

    /// Nombre de lignes annoncé par le record `B`.
    pub fn row_count(&self) -> u32 {
        self.rows
    }

    /// Nombre de colonnes annoncé par le record `B`.
    pub fn column_count(&self) -> u32 {
        self.columns
    }
}
