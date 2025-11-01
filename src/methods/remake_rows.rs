use crate::IndexCell;
use crate::Table;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    /// Remake table displayed rows, row canvas positions.
    pub(crate) fn acr_remake_rows(
        canvas_rows: &mut Vec<u64>,
        disp_rows: &mut Vec<usize>,
        cells: &Vec<Rc<RefCell<IndexCell>>>,
    ) {
        disp_rows.clear();
        canvas_rows.clear();
        canvas_rows.push(0);
        let mut acc: u64 = 0;
        for (i, cell) in cells.iter().enumerate() {
            let cb = cell.borrow();
            if cb.vis {
                disp_rows.push(i);
                acc += cb.dim;
                canvas_rows.push(acc);
            }
        }
    }
}
