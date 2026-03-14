use crate::Change;
use crate::EventData;
use crate::NO_END;
use crate::Table;
use crate::args::MoveArgs;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_borrowed;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Table {
    pub fn move_cols(
        &mut self,
        kwargs: Option<MoveArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_cols = Self::acr_total_cols(&self.grid.header.cells) as u64;
        if num_cols == 0 {
            return Ok(None);
        }
        let kwargs: MoveArgs = kwargs.unwrap_or(MoveArgs {
            old_to_new: Some(vec![(0, num_cols.saturating_sub(1))]),
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        });
        // Validate old_to_new, create inverse
        let mut seen_old: HashSet<usize> = HashSet::new();
        let mut seen_new: HashSet<usize> = HashSet::new();
        let mut old_to_new: Vec<(usize, usize)> = Vec::new();
        let mut new_to_old: HashMap<usize, usize> = HashMap::new();

        for (old_pos, new_pos) in kwargs
            .old_to_new
            .unwrap_or(vec![(0, num_cols.saturating_sub(1))])
            .iter()
        {
            let size_old_pos = *old_pos as usize;
            let size_new_pos = *new_pos as usize;
            // Validation
            if old_pos >= &num_cols {
                // Check if old pos is within bounds
                return Err(str_err!(
                    "Old pos {} is out of bounds. Number of cols: {}",
                    old_pos,
                    num_cols
                ));
            } else if new_pos >= &num_cols {
                // Check if new pos exceeds num_cols
                return Err(str_err!(
                    "New pos {} exceeds number of cols. Number of cols: {}",
                    new_pos,
                    num_cols
                ));
            } else if !seen_old.insert(size_old_pos) {
                // Check for duplicate old indices
                return Err(str_err!("Duplicate old pos: {}", old_pos));
            } else if !seen_new.insert(size_new_pos) {
                // Check for duplicate new indices
                return Err(str_err!("Duplicate new pos: {}", new_pos));
            } else {
                old_to_new.push((size_old_pos, size_new_pos));
                new_to_old.insert(size_new_pos, size_old_pos);
            }
        }

        if old_to_new.is_empty() {
            return Ok(None);
        }

        let mut event_data = self.cr_new_event(kwargs.save_selection);
        self.cr_move_cols(&mut event_data, seen_old, old_to_new, &new_to_old)?;

        if kwargs.select {
            Self::acr_deselect_all(&mut self.sel, None);
            let mut moved: Vec<usize> = event_data
                .changes
                .iter()
                .filter_map(|change| match change {
                    Change::MoveCols(moves) => Some(moves.iter().map(|&(_, to)| to)),
                    _ => None,
                })
                .flatten()
                .filter_map(|n| Self::acr_data_to_disp_col_opt(&self.grid.disp_cols, n))
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

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
