use crate::Table;
use crate::TreeMove;
use crate::args::MoveItemsArgs;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_borrowed;
use crate::{SelectionKind, SelectionState};
use std::collections::HashMap;
use std::collections::HashSet;

impl Table {
    pub fn move_items(&self, kwargs: MoveItemsArgs) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        let num_rows = Self::acr_total_rows(&grid.index.cells);
        if num_rows == 0 {
            return Ok(());
        }
        // Item ids
        if let Some(items) = kwargs.items {
            // Parent
            let new_par: String = match kwargs.parent {
                Some(parent) => parent,
                None => "".to_string(),
            };
            if !new_par.is_empty() && !Self::acr_tree_exists(&grid.index.iid_to_row, &new_par) {
                return Err(str_err!(format!("Parent does not exist: {}", new_par)));
            };

            // Check items
            let mut hash_items: HashSet<String> = HashSet::new();
            for iid in items.into_iter() {
                if !Self::acr_tree_exists(&grid.index.iid_to_row, &iid) {
                    return Err(str_err!(format!("iid does not exist: {}", &iid)));
                }
                if Self::acr_tree_move_pid_causes_recursive_loop(
                    &grid.index.cells,
                    &grid.index.iid_to_row,
                    &iid,
                    &new_par,
                )? {
                    return Err(str_err!(format!(
                        "Moving '{}' to '{}' would cause a recursive loop",
                        &iid, new_par
                    )));
                }
                hash_items.insert(iid);
            }

            // Create a new event
            let mut event_data = self.cr_new_event(true);

            // Set up TreeMove object
            let move_op: TreeMove = TreeMove {
                iids: hash_items,
                new_par,
                pos: kwargs.pos,
            };

            // Perform move operation
            self.cr_move_rows(&mut event_data, HashMap::new(), None, Some(move_op), true)?;

            // Select moved rows if true
            if kwargs.select {
                let mut sel = self.sel.borrow_mut();
                Self::acr_deselect_all(&mut sel);
                let mut sorted_moved: Vec<usize> = event_data
                    .moved_rows
                    .values()
                    .filter_map(|n| {
                        let data_row = *n;
                        if Self::acr_row_is_vis_opt(&grid.index.cells, &data_row).is_some() {
                            Self::acr_data_to_disp_row_opt(&grid.disp_rows, data_row)
                        } else {
                            None
                        }
                    })
                    .collect();
                if !sorted_moved.is_empty() {
                    sorted_moved.sort_unstable();
                    for (start, end) in consecutive_ranges_borrowed(sorted_moved.iter()) {
                        Self::acr_add_selection_box(
                            &mut sel,
                            &grid,
                            start,
                            Some(end),
                            0,
                            None,
                            SelectionKind::Rows,
                            SelectionState::Normal,
                            true,
                        );
                    }
                }
            }

            // Sheet has been modified
            self.cr_event_modified(&event_data, kwargs.emit, true)?;

            // Add to undo stack if undo is enabled and undo is true in kwargs
            if self.opts.borrow().enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
        }

        Ok(())
    }
}
