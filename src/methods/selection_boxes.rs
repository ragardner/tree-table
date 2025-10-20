use crate::Selected;
use crate::Selection;
use crate::SelectionBox;
use crate::Table;
use crate::TableGrid;
use crate::{Coords, SelectionKind, SelectionState};

impl Table {
    pub(crate) fn acr_deselect_all(sel: &mut Selection) {
        sel.selection_boxes.clear();
        sel.selected = None;
    }

    pub(crate) fn acr_add_selection_box(
        mut sel: &mut Selection,
        grid: &TableGrid,
        mut row: usize,
        mut end_row: Option<usize>,
        mut col: usize,
        mut end_col: Option<usize>,
        kind: SelectionKind,
        state: SelectionState,
        current: bool,
    ) -> usize {
        if grid.disp_rows.is_empty() {
            row = 0;
            end_row = Some(0);
        };
        if grid.disp_cols.is_empty() {
            col = 0;
            end_col = Some(0);
        };
        let coords = Coords {
            row,
            end_row,
            col,
            end_col,
        };
        let pos = sel.selection_boxes.len();
        sel.selection_boxes.push(SelectionBox {
            coords: coords.clone(),
            kind: kind.clone(),
            state,
        });
        if current || sel.selected.is_none() {
            Self::acr_set_current_core(&mut sel, row, col, coords, kind, pos);
        }
        pos
    }

    #[inline]
    pub(crate) fn acr_set_current_core(
        sel: &mut Selection,
        row: usize,
        col: usize,
        coords: Coords,
        kind: SelectionKind,
        pos: usize,
    ) {
        sel.selected = Some(Selected {
            row,
            col,
            coords,
            kind,
            pos,
        });
    }
}
