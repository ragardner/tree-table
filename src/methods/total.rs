use crate::HeaderCell;
use crate::IndexCell;
use crate::Table;

impl Table {
    pub(crate) fn acr_total_cols(cells: &[HeaderCell]) -> usize {
        cells.len()
    }

    pub(crate) fn acr_total_rows(cells: &[IndexCell]) -> usize {
        cells.len()
    }

    pub(crate) fn acr_total_disp_cols(disp_cols: &[usize]) -> usize {
        disp_cols.len()
    }

    pub(crate) fn acr_total_disp_rows(disp_rows: &[usize]) -> usize {
        disp_rows.len()
    }
}
