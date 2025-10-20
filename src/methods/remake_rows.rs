use crate::IndexCell;
use crate::Table;

impl Table {
    /// Remake table displayed rows, row canvas positions.
    pub(crate) fn acr_remake_rows(
        canvas_rows: &mut Vec<u64>,
        disp_rows: &mut Vec<usize>,
        cells: &Vec<IndexCell>,
    ) {
        disp_rows.clear();
        canvas_rows.clear();
        canvas_rows.push(0);
        let mut acc: u64 = 0;
        for (i, cell) in cells.iter().enumerate() {
            if cell.vis {
                disp_rows.push(i);
                acc += cell.dim;
                canvas_rows.push(acc);
            }
        }
    }
}
