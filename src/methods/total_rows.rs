use crate::IndexCell;
use crate::Table;

impl Table {
    pub(crate) fn acr_total_rows(cells: &Vec<IndexCell>) -> usize {
        cells.len()
    }

    pub(crate) fn acr_total_disp_rows(disp_rows: &Vec<IndexCell>) -> usize {
        disp_rows.len()
    }
}
