use crate::HeaderCell;
use crate::IndexCell;
use crate::Table;

impl Table {
    /// Remake table displayed rows, row canvas positions.
    pub(crate) fn acr_remake_rows(
        canvas_rows: &mut Vec<u64>,
        disp_rows: &mut Vec<usize>,
        cells: &[IndexCell],
        mut line_width: u64,
    ) {
        line_width -= 1;
        disp_rows.clear();
        canvas_rows.clear();
        canvas_rows.push(0);
        let mut acc: u64 = 0;
        let mut it = cells.iter().enumerate().skip_while(|(_, cell)| !cell.vis);

        if let Some((first_vis_pos, first_vis_cell)) = it.next() {
            disp_rows.push(first_vis_pos);
            acc += first_vis_cell.dim;
            canvas_rows.push(acc);
            for (i, cell) in it {
                if cell.vis {
                    disp_rows.push(i);
                    acc += line_width + cell.dim;
                    canvas_rows.push(acc);
                }
            }
        }
    }

    /// Remake table displayed cols, col canvas positions.
    pub(crate) fn acr_remake_cols(
        canvas_cols: &mut Vec<u64>,
        disp_cols: &mut Vec<usize>,
        cells: &[HeaderCell],
        mut line_width: u64,
    ) {
        line_width -= 1;
        disp_cols.clear();
        canvas_cols.clear();
        canvas_cols.push(0);
        let mut acc: u64 = 0;
        let mut it = cells.iter().enumerate().skip_while(|(_, cell)| !cell.vis);

        if let Some((first_vis_pos, first_vis_cell)) = it.next() {
            disp_cols.push(first_vis_pos);
            acc += first_vis_cell.dim;
            canvas_cols.push(acc);
            for (i, cell) in it {
                if cell.vis {
                    disp_cols.push(i);
                    acc += line_width + cell.dim;
                    canvas_cols.push(acc);
                }
            }
        }
    }
}
