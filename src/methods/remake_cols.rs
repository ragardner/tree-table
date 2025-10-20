use crate::HeaderCell;
use crate::Table;

impl Table {
    /// Remake table displayed cols, col canvas positions.
    pub(crate) fn acr_remake_cols(
        canvas_cols: &mut Vec<u64>,
        disp_cols: &mut Vec<usize>,
        cells: &Vec<HeaderCell>,
    ) {
        disp_cols.clear();
        canvas_cols.clear();
        canvas_cols.push(0);
        let mut acc: u64 = 0;
        for (i, cell) in cells.iter().enumerate() {
            if cell.vis {
                disp_cols.push(i);
                acc += cell.dim;
                canvas_cols.push(acc);
            }
        }
    }
}
