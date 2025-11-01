use std::cell::RefCell;
use std::rc::Rc;

use crate::IndexCell;
use crate::Table;

impl Table {
    pub(crate) fn acr_total_rows(cells: &Vec<Rc<RefCell<IndexCell>>>) -> usize {
        cells.len()
    }

    pub(crate) fn acr_total_disp_rows(disp_rows: &Vec<Rc<RefCell<IndexCell>>>) -> usize {
        disp_rows.len()
    }
}
