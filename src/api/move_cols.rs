use crate::SelectionKind;
use crate::SelectionState;
use crate::Table;
use crate::args::MoveArgs;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_borrowed;
use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;

impl Table {
    pub fn move_cols(&self, kwargs: Option<MoveArgs>) -> Result<(), String> {
        let (mut grid, opts, mut sel) = self.cr_mut_borrows();
        let mut grid = grid.deref_mut();
        let num_cols = Self::acr_total_cols(&grid.header.cells) as u64;
        if num_cols == 0 {
            return Ok(());
        }
        let kwargs: MoveArgs = kwargs.unwrap_or(MoveArgs {
            old_to_new: Some(vec![(0, num_cols.saturating_sub(1))]),
            select: false,
            emit: true,
            undo: true,
        });
        // Validate old_to_new, create inverse
        let mut seen_old_indices: HashSet<u64> = HashSet::new();
        let mut seen_new_indices: HashSet<u64> = HashSet::new();
        let mut old_to_new: HashMap<usize, usize> = HashMap::new();
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
                return Err(str_err!(format!(
                    "Old pos {} is out of bounds. Number of cols: {}",
                    old_pos, num_cols
                )));
            } else if new_pos >= &num_cols {
                // Check if new pos exceeds num_cols
                return Err(str_err!(format!(
                    "New pos {} exceeds number of cols. Number of cols: {}",
                    new_pos, num_cols
                )));
            } else if !seen_old_indices.insert(*old_pos) {
                // Check for duplicate old indices
                return Err(str_err!(format!("Duplicate old pos: {}", old_pos)));
            } else if !seen_new_indices.insert(*new_pos) {
                // Check for duplicate new indices
                return Err(str_err!(format!("Duplicate new pos: {}", new_pos)));
            } else {
                old_to_new.insert(size_old_pos, size_new_pos);
                new_to_old.insert(size_new_pos, size_old_pos);
            }
        }

        let mut event_data = self.cr_new_event(true, &opts, &sel);
        event_data.moved_cols = old_to_new;
        self.cr_move_cols(&mut grid, &opts, &mut event_data, &new_to_old)?;

        if kwargs.select {
            Self::acr_deselect_all(&mut sel);
            let mut sorted_moved: Vec<usize> = event_data
                .moved_cols
                .values()
                .filter_map(|n| Self::acr_data_to_disp_col_opt(&grid.disp_cols, *n))
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
                        SelectionKind::Cols,
                        SelectionState::Normal,
                        true,
                    );
                }
            }
        }
        self.cr_event_modified(
            &mut grid,
            &self.events.borrow(),
            &event_data,
            kwargs.emit,
            true,
        )?;
        if opts.enabled_undo && kwargs.undo {
            grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
