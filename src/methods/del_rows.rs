use crate::Change;
use crate::EventData;
use crate::IndexCell;
use crate::NO_END;
use crate::Table;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    /// Core del rows function
    pub(crate) fn cr_del_rows(
        &mut self,
        mut positions: Vec<usize>,
        event_data: &mut EventData,
        validate: bool,
    ) -> Result<(), String> {
        // TODO: close dropdowns and editors?

        // Deal with tree, copies affected index cells
        if self.opts.tree_mode {
            if validate {
                // Descendants of rows are also deleted
                // In order not to make mistakes any descendant rows are removed first
                // Then all the descendants are added back in
                positions = self.cr_tree_remove_and_add_descendants(
                    positions
                        .iter()
                        .filter_map(|rn| Self::acr_copy_riid(&self.grid.index.cells, *rn).ok())
                        .collect::<HashSet<Rc<str>>>(),
                )?;
            }
            Self::acr_tree_del_rows(
                &mut self.grid.index.cells,
                &self.grid.index.iid_to_row,
                &positions,
                event_data,
            )?;
        }

        // Retrieve data from index
        let removed = self.grid.index.del_rows(&positions)?;

        event_data.changes.push(Change::RemoveRows(removed));

        // Pull outstanding row numbers
        self.grid.pull_rows(&positions);

        // Remake rows
        Self::acr_remake_rows(
            &mut self.grid.canvas_rows,
            &mut self.grid.disp_rows,
            &self.grid.index.cells,
            self.opts.horizontal_grid_line_width.clone(),
        );

        Ok(())
    }

    #[inline]
    pub(crate) fn acr_compute_deleted_disp_rows(
        disp_rows: &[usize],
        deleted_data_positions: &[usize], // already sorted + unique from caller
    ) -> Vec<usize> {
        // No sort/dedup needed — output is guaranteed sorted ascending
        deleted_data_positions
            .iter()
            .filter_map(|&d| Self::acr_data_to_disp_row_opt(disp_rows, d))
            .collect()
    }

    #[inline]
    pub(crate) fn acr_adjust_range_after_delete(
        start: usize,
        end: usize,
        deleted_disp_sorted: &[usize],
    ) -> Option<(usize, usize)> {
        let deleted_before = deleted_disp_sorted.partition_point(|&d| d < start);
        let deleted_inside = deleted_disp_sorted.partition_point(|&d| d <= end) - deleted_before;

        let surviving_width = (end - start + 1).saturating_sub(deleted_inside);

        if surviving_width == 0 {
            return None;
        }

        let new_start = start - deleted_before;
        let new_end = new_start + surviving_width - 1;

        Some((new_start, new_end))
    }

    pub(crate) fn cr_adjust_selections_after_row_delete(
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
            if b.end_row == NO_END {
                new_boxes.push(b);
                continue;
            }

            // Use the minimal reusable helper (only row range)
            if let Some((new_start, new_end)) =
                Self::acr_adjust_range_after_delete(b.row, b.end_row, deleted_disp_sorted)
            {
                b.row = new_start;
                b.end_row = new_end;
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
            if sel.coords.end_row == NO_END {
                return;
            }

            let range_adjusted = Self::acr_adjust_range_after_delete(
                sel.coords.row,
                sel.coords.end_row,
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
                let old_row = sel.row;
                let deleted_before_cursor = deleted_disp_sorted.partition_point(|&d| d < old_row);
                sel.row = old_row - deleted_before_cursor;

                sel.coords.row = new_start;
                sel.coords.end_row = new_end;
            }
        }
    }

    /// Stores index cells prior to change
    pub(crate) fn acr_tree_del_rows(
        mut cells: &mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        positions: &[usize],
        mut event_data: &mut EventData,
    ) -> Result<(), String> {
        // positons should be sorted and unique
        for pos in positions.iter() {
            let cell = Self::acr_icell_mut(&mut cells, *pos)?;
            let change = cell.record_set_open(false);
            event_data
                .changes
                .push(Change::IndexCell { row: *pos, change });
            let iid = cell.iid.clone();
            Self::acr_tree_remove_iid_from_pars_chn(
                &mut cells,
                &iid_to_row,
                &iid,
                &mut event_data,
            )?;
        }
        Ok(())
    }
}
