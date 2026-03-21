use crate::Change;
use crate::EventData;
use crate::NO_END;
use crate::Selected;
use crate::Selection;
use crate::Span;
use crate::Table;
use crate::TableGrid;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;

impl Table {
    pub(crate) fn acr_deselect_all(sel: &mut Selection, event_data: Option<&mut EventData>) {
        if let Some(event_data) = event_data {
            event_data.changes.push(Change::RemoveSelectionBoxes {
                removed: std::mem::take(&mut sel.selection_boxes),
                old_selected: sel.selected.take(),
            })
        } else {
            sel.selection_boxes.clear();
            sel.selected = None;
        }
    }

    pub(crate) fn acr_add_selection_box(
        mut sel: &mut Selection,
        grid: &TableGrid,
        mut row: usize,
        mut end_row: usize,
        mut col: usize,
        mut end_col: usize,
        current: bool,
    ) -> usize {
        if grid.disp_rows.is_empty() {
            row = 0;
            end_row = 0;
        };
        if grid.disp_cols.is_empty() {
            col = 0;
            end_col = 0;
        };
        let coords = Span {
            row,
            end_row,
            col,
            end_col,
        };
        if current || sel.selected.is_none() {
            Self::acr_set_current_core(&mut sel, row, col, coords.clone());
        }
        sel.selection_boxes.push(coords);
        sel.selection_boxes.len() - 1
    }

    #[inline]
    pub(crate) fn acr_set_current_core(sel: &mut Selection, row: usize, col: usize, coords: Span) {
        sel.selected = Some(Selected { row, col, coords });
    }

    pub(crate) fn cr_select_rows_from(&mut self, rows: impl IntoIterator<Item = usize>) {
        for (start, end) in consecutive_ranges_owned(rows.into_iter()) {
            Self::acr_add_selection_box(&mut self.sel, &self.grid, start, end, 0, NO_END, true);
        }
    }
}
