//! In-memory aggregate of an SLK file: dimensions (`B` record) and
//! cells (`C` records), in file order.

use crate::cell::Cell;
use crate::slk_type::Record;
use crate::{SLKError, SLKScanner};

/// Contents of a loaded SLK file: dimensions and cells.
#[derive(Default, Debug)]
pub struct Document {
    rows: u32,
    columns: u32,
    contents: Vec<Cell>,
}

impl Document {
    /// Loads all records from the scanner.
    /// Stops at the first error and propagates it.
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

    /// Cells in file order.
    pub fn get_contents(&self) -> &Vec<Cell> {
        &self.contents
    }

    /// Row count announced by the `B` record.
    pub fn row_count(&self) -> u32 {
        self.rows
    }

    /// Column count announced by the `B` record.
    pub fn column_count(&self) -> u32 {
        self.columns
    }
}
