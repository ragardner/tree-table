use crate::Change;
use crate::EventData;
use crate::SelectionKind;
use crate::SelectionState;
use crate::Table;
use crate::args::MoveArgs;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_borrowed;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    pub fn move_rows(
        &mut self,
        kwargs: Option<MoveArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells) as u64;
        if num_rows == 0 {
            return Ok(None);
        }

        let kwargs: MoveArgs = kwargs.unwrap_or(MoveArgs {
            old_to_new: Some(vec![(0, num_rows.saturating_sub(1))]),
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        });

        // Build old_to_new and new_to_old as Vec<(usize, usize)>
        let mut seen_old: HashSet<usize> = HashSet::new();
        let mut seen_new: HashSet<usize> = HashSet::new();
        let mut old_to_new: Vec<(usize, usize)> = Vec::new();
        let mut new_to_old: HashMap<usize, usize> = HashMap::new();

        for (old_pos, new_pos) in kwargs
            .old_to_new
            .unwrap_or(vec![(0, num_rows.saturating_sub(1))])
        {
            let old = old_pos as usize;
            let new = new_pos as usize;

            if old >= num_rows as usize {
                return Err(str_err!(
                    "Old pos {} is out of bounds. Number of rows: {}",
                    old,
                    num_rows
                ));
            }
            if new >= num_rows as usize {
                return Err(str_err!(
                    "New pos {} exceeds number of rows. Number of rows: {}",
                    new,
                    num_rows
                ));
            }
            if !seen_old.insert(old) {
                return Err(str_err!("Duplicate old pos: {}", old));
            }
            if !seen_new.insert(new) {
                return Err(str_err!("Duplicate new pos: {}", new));
            }

            old_to_new.push((old, new));
            new_to_old.insert(new, old);
        }

        if old_to_new.is_empty() {
            return Ok(None);
        }

        let mut event_data = self.cr_new_event(false);
        let boxes = kwargs
            .save_selection
            .then_some(Self::acr_new_change_with_old_boxes(&self.sel));

        self.cr_move_rows(
            &mut event_data,
            seen_old,
            old_to_new,
            new_to_old,
            None,
            None,
            true,
        )?;

        if kwargs.select {
            Self::acr_deselect_all(&mut self.sel);
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

        if let Some(mut boxes) = boxes {
            Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
            event_data.changes.push(boxes);
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
