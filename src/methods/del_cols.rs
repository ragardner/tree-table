use crate::Change;
use crate::Coords;
use crate::EventData;
use crate::SelectionBox;
use crate::SelectionKind;
use crate::Table;

impl Table {
    pub(crate) fn cr_del_cols(
        &mut self,
        positions: Vec<usize>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        // TODO: close dropdowns and editors?
        let removed_header = self.grid.header.del_cols(&positions)?;
        let removed_table = self.grid.index.del_cols(&positions);
        event_data
            .changes
            .push(Change::remove_cols(removed_header, removed_table));

        self.grid.pull_cols(&positions);

        Self::acr_remake_cols(
            &mut self.grid.canvas_cols,
            &mut self.grid.disp_cols,
            &self.grid.header.cells,
            self.opts.vertical_grid_line_width.clone(),
        );

        Ok(())
    }

    #[inline]
    pub(crate) fn acr_compute_deleted_disp_cols(
        disp_cols: &[usize],
        deleted_data_positions: &[usize], // already sorted + unique from caller
    ) -> Vec<usize> {
        // No sort/dedup needed — output is guaranteed sorted ascending
        deleted_data_positions
            .iter()
            .filter_map(|&d| Self::acr_data_to_disp_col_opt(disp_cols, d))
            .collect()
    }

    pub(crate) fn acr_adjust_selections_after_col_delete(
        &mut self,
        deleted_disp_sorted: &[usize], // must be sorted ascending
    ) {
        if deleted_disp_sorted.is_empty() {
            return;
        }

        // Save the ORIGINAL focused cell's coords + kind BEFORE we touch anything
        // This gives us an exact match for "was this the box the focused cell was attached to?"
        let original_focus = self
            .sel
            .selected
            .as_ref()
            .map(|s| (s.coords.clone(), s.kind.clone()));

        // Flag to detect if the exact box the focused cell was attached to was fully deleted
        let mut focused_box_was_deleted = false;

        // Rebuild surviving selection boxes
        let mut new_boxes = Vec::with_capacity(self.sel.selection_boxes.len());

        for b in self.sel.selection_boxes.drain(..) {
            if b.kind == SelectionKind::Rows {
                new_boxes.push(b);
                continue;
            }

            let start = b.coords.col;
            let end = b.coords.end_col.unwrap_or(start); // inclusive

            // Single binary-search step for both counts
            let deleted_before = deleted_disp_sorted.partition_point(|&d| d < start);
            let deleted_inside =
                deleted_disp_sorted.partition_point(|&d| d <= end) - deleted_before;

            let surviving_width = (end - start + 1).saturating_sub(deleted_inside);

            if surviving_width == 0 {
                // Fully covered → remove
                // Check if this was the exact box the focused cell was attached to
                if let Some((ref orig_coords, ref orig_kind)) = original_focus {
                    if orig_coords == &b.coords && orig_kind == &b.kind {
                        focused_box_was_deleted = true;
                    }
                }
                continue;
            }

            let new_start = start - deleted_before;
            let new_end = new_start + surviving_width - 1;

            let new_coords = Coords::new(
                b.coords.row,
                b.coords.end_row,
                new_start,
                if new_start == new_end {
                    Some(new_start)
                } else {
                    Some(new_end)
                },
            );

            new_boxes.push(SelectionBox {
                coords: new_coords,
                kind: b.kind.clone(),
                state: b.state.clone(),
            });
        }

        self.sel.selection_boxes = new_boxes;

        if let Some(sel) = &mut self.sel.selected {
            if sel.kind == SelectionKind::Rows {
                return;
            }

            let old_col = sel.col;

            if focused_box_was_deleted {
                // The exact box this focused cell was attached to was deleted
                // → attach to the LAST remaining box (most recent) at its top-left corner
                if let Some(last_box) = self.sel.selection_boxes.last() {
                    let new_row = last_box.coords.row;
                    let new_col = last_box.coords.col;

                    sel.row = new_row;
                    sel.col = new_col;
                    sel.kind = last_box.kind.clone(); // ← NEW: copy the box kind
                    sel.coords = Coords::new(
                        new_row,
                        last_box.coords.end_row.map(|_| new_row), // single cell
                        new_col,
                        last_box.coords.end_col.map(|_| new_col), // single cell
                    );
                } else {
                    // No boxes left at all → clear focus
                    self.sel.selected = None;
                }
            } else {
                // Box survived (possibly shrunk/shifted) → just shift column indices left
                let deleted_before = deleted_disp_sorted.partition_point(|&d| d < old_col);
                let new_col = old_col - deleted_before;

                sel.col = new_col;
                sel.coords.col = new_col;
                if let Some(ec) = &mut sel.coords.end_col {
                    *ec = new_col;
                }
            }
        }
    }
}
