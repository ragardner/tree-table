use crate::EventData;
use crate::Table;
use crate::Val;
use crate::args::SortRowsArgs;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    pub fn sort_rows_by_cols(
        &mut self,
        kwargs: Option<SortRowsArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells) as u64;
        if num_rows == 0 {
            return Ok(None);
        }

        let kwargs: SortRowsArgs = kwargs.unwrap_or_default();
        if kwargs.keys.is_empty() {
            return Ok(None);
        }

        let span = self.cr_key_to_span(kwargs.span)?;
        let rows: Vec<usize> = span.rows(&self.grid).collect();

        if rows.len() <= 1 {
            return Ok(None);
        }

        let compare_fn: fn(&Val, &Val) -> Ordering = kwargs.sort_key.get_cmp();
        let mut indices: Vec<usize> = rows.clone();

        indices.sort_by(|&a, &b| {
            for &(col_u64, ascending) in &kwargs.keys {
                let col = col_u64 as usize;
                let va =
                    Self::acr_tcell_val(&self.grid.index.cells, a, col).unwrap_or(&Val::Nil(()));
                let vb =
                    Self::acr_tcell_val(&self.grid.index.cells, b, col).unwrap_or(&Val::Nil(()));

                let mut cmp = compare_fn(va, vb);
                if cmp != Ordering::Equal {
                    if !ascending {
                        cmp = cmp.reverse();
                    }
                    return cmp;
                }
            }
            Ordering::Equal // stable tie-breaker
        });

        if indices == rows {
            return Ok(None);
        }

        let mut old_to_new: Vec<(usize, usize)> = Vec::new();
        let mut new_to_old: HashMap<usize, usize> = HashMap::new();
        let mut seen_old: HashSet<usize> = HashSet::new();
        for (new_pos, &old) in indices.iter().enumerate() {
            let new = rows[new_pos];
            if old == new {
                continue;
            }
            seen_old.insert(old);
            old_to_new.push((old, new));
            new_to_old.insert(new, old);
        }

        if old_to_new.is_empty() {
            return Ok(None);
        }

        let mut event_data = self.cr_new_event(kwargs.save_selection);

        if let Err(e) = self.cr_move_rows(
            &mut event_data,
            seen_old,
            old_to_new,
            new_to_old,
            None,
            true,
        ) {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

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
}
