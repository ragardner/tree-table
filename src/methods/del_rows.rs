use crate::Change;
use crate::Coords;
use crate::EventData;
use crate::IndexCell;
use crate::SelectionBox;
use crate::SelectionKind;
use crate::Table;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
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
        event_data
            .changes
            .push(Change::RemoveRows(self.grid.index.del_rows(&positions)?));

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

    pub(crate) fn acr_adjust_selections_after_row_delete(
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
            if b.kind == SelectionKind::Cols {
                new_boxes.push(b);
                continue;
            }

            let start = b.coords.row;
            let end = b.coords.end_row.unwrap_or(start); // inclusive

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
                new_start,
                if new_start == new_end {
                    Some(new_start)
                } else {
                    Some(new_end)
                },
                b.coords.col,
                b.coords.end_col,
            );

            new_boxes.push(SelectionBox {
                coords: new_coords,
                kind: b.kind.clone(),
                state: b.state.clone(),
            });
        }

        self.sel.selection_boxes = new_boxes;

        if let Some(sel) = &mut self.sel.selected {
            if sel.kind == SelectionKind::Cols {
                return;
            }

            let old_row = sel.row;

            if focused_box_was_deleted {
                // The exact box this focused cell was attached to was deleted
                // → attach to the LAST remaining box (most recent) at its top-left corner
                if let Some(last_box) = self.sel.selection_boxes.last() {
                    let new_row = last_box.coords.row;
                    let new_col = last_box.coords.col;

                    sel.row = new_row;
                    sel.col = new_col;
                    sel.kind = last_box.kind.clone(); // copy the box kind
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
                // Box survived (possibly shrunk/shifted) → just shift row indices left
                let deleted_before = deleted_disp_sorted.partition_point(|&d| d < old_row);
                let new_row = old_row - deleted_before;

                sel.row = new_row;
                sel.coords.row = new_row;
                if let Some(er) = &mut sel.coords.end_row {
                    *er = new_row;
                }
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
            event_data.add_tree_change(&cell.iid, change);
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
