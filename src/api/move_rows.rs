use crate::Change;
use crate::EventData;
use crate::Table;
use crate::args::MoveArgs;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use crate::str_err;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use hashbrown::HashSet;

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

        let mut event_data = self.cr_new_event(kwargs.save_selection);

        match self.cr_move_rows(
            &mut event_data,
            seen_old,
            old_to_new,
            new_to_old,
            None,
            true,
        ) {
            Err(e) => {
                self.cr_roll_back(Rc::new(RefCell::new(event_data)));
                return Err(e);
            }
            _ => {}
        };

        if kwargs.select {
            Self::acr_deselect_all(&mut self.sel, None);
            let mut moved: Vec<usize> = self.extract_disp_moved_rows(&event_data);
            if !moved.is_empty() {
                moved.sort_unstable();
                self.cr_select_rows_from(moved);
            }
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub(crate) fn extract_disp_moved_rows(&self, event_data: &EventData) -> Vec<usize> {
        let moved: Vec<usize> = event_data
            .changes
            .iter()
            .filter_map(|change| match change {
                Change::MoveRows(moves) => Some(moves.iter().map(|&(_, to)| to)),
                _ => None,
            })
            .flatten()
            .filter_map(|n| Self::acr_data_to_disp_row_opt(&self.grid.disp_rows, n))
            .collect();
        moved
    }
}
