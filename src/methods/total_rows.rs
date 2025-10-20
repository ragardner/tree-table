use crate::IndexCell;
use crate::Table;

impl Table {
    pub(crate) fn cr_total_rows(&self) -> usize {
        self.grid.borrow().index.cells.len()
    }

    pub(crate) fn cr_total_disp_rows(&self) -> usize {
        self.grid.borrow().disp_rows.len()
    }

    pub(crate) fn acr_total_rows(cells: &Vec<IndexCell>) -> usize {
        cells.len()
    }

    pub(crate) fn acr_total_disp_rows(disp_rows: &Vec<usize>) -> usize {
        disp_rows.len()
    }
}
