use crate::HeaderCell;
use crate::Table;

impl Table {
    pub(crate) fn acr_total_cols(cells: &Vec<HeaderCell>) -> usize {
        cells.len()
    }

    pub(crate) fn acr_total_disp_cols(disp_cols: &Vec<usize>) -> usize {
        disp_cols.len()
    }
}
