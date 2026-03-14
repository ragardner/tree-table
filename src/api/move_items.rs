use crate::Change;
use crate::EventData;
use crate::NO_END;
use crate::Table;
use crate::TreeMove;
use crate::args::MoveItemsArgs;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_borrowed;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    pub fn move_items(
        &mut self,
        kwargs: MoveItemsArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells);
        if num_rows == 0 {
            return Ok(None);
        }

        // Item ids
        if let Some(items) = kwargs.items {
            // Parent
            let new_par: String = match kwargs.par {
                Some(par) => par,
                None => "".to_string(),
            };
            if !new_par.is_empty() && !Self::acr_tree_exists(&self.grid.index.iid_to_row, &new_par)
            {
                return Err(str_err!("Parent does not exist: {}", new_par));
            };

            // Check items
            let mut hash_items: HashSet<Rc<str>> = HashSet::new();
            for iid in items.into_iter() {
                if !Self::acr_tree_exists(&self.grid.index.iid_to_row, &iid) {
                    return Err(str_err!("iid does not exist: {}", &iid));
                }
                if Self::acr_tree_move_pid_causes_recursive_loop(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    &iid,
                    &new_par,
                )? {
                    return Err(str_err!(
                        "Moving '{}' to '{}' would cause a recursive loop",
                        &iid,
                        new_par
                    ));
                }
                hash_items.insert(Rc::from(iid));
            }

            // Create a new event
            let mut event_data = self.cr_new_event(kwargs.save_selection);

            // Set up TreeMove object
            let move_op: TreeMove = TreeMove {
                iids: hash_items,
                new_par: Rc::from(new_par),
                pos: kwargs.pos,
            };

            // Perform move operation
            self.cr_move_rows(
                &mut event_data,
                HashSet::new(),
                Vec::new(),
                HashMap::new(),
                None,
                Some(move_op),
                true,
            )?;

            // Select moved rows if true
            if kwargs.select {
                Self::acr_deselect_all(&mut self.sel, None);
                let mut moved: Vec<usize> = event_data
                    .changes
                    .iter()
                    .filter_map(|change| match change {
                        Change::MoveRows(moves) => Some(moves.iter().map(|&(_, to)| to)),
                        _ => None,
                    })
                    .flatten()
                    .filter_map(|n| Self::acr_data_to_disp_row_opt(&self.grid.disp_rows, n))
                    .collect();
                if !moved.is_empty() {
                    moved.sort_unstable();
                    for (start, end) in consecutive_ranges_borrowed(moved.iter()) {
                        Self::acr_add_selection_box(
                            &mut self.sel,
                            &self.grid,
                            start,
                            end,
                            0,
                            NO_END,
                            true,
                        );
                    }
                }
            }

            return self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit);
        }

        Ok(None)
    }
}
