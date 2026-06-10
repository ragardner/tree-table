use crate::{Change, EventData, NO_END, Table};
use alloc::string::String;
use alloc::vec::Vec;

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

    pub(crate) fn cr_adjust_selections_after_col_delete(
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
            .map(|s| (s.coords.clone(), s.coords.kind()));

        // Flag to detect if the exact box the focused cell was attached to was fully deleted
        let mut focused_box_was_deleted = false;

        // Rebuild surviving selection boxes
        let mut new_boxes = Vec::with_capacity(self.sel.selection_boxes.len());

        for mut b in self.sel.selection_boxes.drain(..) {
            if b.end_col == NO_END {
                new_boxes.push(b);
                continue;
            }

            // Use the minimal reusable helper (only column range)
            if let Some((new_start, new_end)) =
                Self::acr_adjust_range_after_delete(b.col, b.end_col, deleted_disp_sorted)
            {
                b.col = new_start;
                b.end_col = new_end;
                new_boxes.push(b);
            } else {
                // Fully covered → remove
                // Check if this was the exact box the focused cell was attached to
                if let Some((ref orig_coords, ref orig_kind)) = original_focus {
                    if orig_coords == &b && orig_kind == &b.kind() {
                        focused_box_was_deleted = true;
                    }
                }
                continue;
            }
        }

        self.sel.selection_boxes = new_boxes;

        if let Some(sel) = &mut self.sel.selected {
            if sel.coords.end_col == NO_END {
                return;
            }

            let range_adjusted = Self::acr_adjust_range_after_delete(
                sel.coords.col,
                sel.coords.end_col,
                deleted_disp_sorted,
            );

            if focused_box_was_deleted || range_adjusted.is_none() {
                // The exact box this focused cell was attached to was deleted
                // (or extremely unlikely error case) → attach to last surviving box
                if let Some(last_box) = self.sel.selection_boxes.last() {
                    sel.row = last_box.row;
                    sel.col = last_box.col;
                    sel.coords = *last_box;
                } else {
                    self.sel.selected = None;
                }
            } else if let Some((new_start, new_end)) = range_adjusted {
                // Box survived → shift cursor independently + adjust coords to match box
                let old_col = sel.col;
                let deleted_before_cursor = deleted_disp_sorted.partition_point(|&d| d < old_col);
                sel.col = old_col - deleted_before_cursor;

                sel.coords.col = new_start;
                sel.coords.end_col = new_end;
            }
        }
    }
}
