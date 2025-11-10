use std::cell::RefCell;
use std::rc::Rc;

use crate::HeaderCell;
use crate::IndexCell;
use crate::Table;
use crate::TableCell;
use crate::Val;

impl Table {
    /// Checks if a val is valid for the table's current table cell at coords.
    #[inline]
    pub(crate) fn acr_val_valid_t(
        cells: &Vec<IndexCell>,
        row: &usize,
        col: &usize,
        val: &Val,
        readonly: bool,
    ) -> bool {
        true
    }

    /// Checks if a val is valid for the index's current index cell at coords.
    #[inline]
    pub(crate) fn acr_val_valid_i(
        cells: &Vec<IndexCell>,
        pos: &usize,
        val: &Val,
        readonly: bool,
    ) -> bool {
        true
    }

    /// Checks if a val is valid for the header's current header cell at coords.
    #[inline]
    pub(crate) fn acr_val_valid_h(
        cells: &Vec<HeaderCell>,
        pos: &usize,
        val: &Val,
        readonly: bool,
    ) -> bool {
        true
    }

    /// Checks if a val is valid for a specific table cell.
    #[inline]
    pub(crate) fn acr_val_valid_tcell(cell: &TableCell, val: &Val, readonly: bool) -> bool {
        true
    }

    /// Checks if a val is valid for a specific index cell.
    #[inline]
    pub(crate) fn acr_val_valid_icell(cell: &IndexCell, val: &Val, readonly: bool) -> bool {
        true
    }

    /// Checks if a val is valid for a specific header cell.
    #[inline]
    pub(crate) fn acr_val_valid_hcell(cell: &HeaderCell, val: &Val, readonly: bool) -> bool {
        true
    }
}
