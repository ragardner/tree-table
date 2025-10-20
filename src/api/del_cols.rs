use crate::Table;
use crate::args::DelArgs;

impl Table {
    /// Delete cols from the table.
    pub fn _del_cols(&self, kwargs: Option<DelArgs>) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        let num_cols = Self::acr_total_cols(&grid.header.cells);
        if num_cols == 0 {
            return Ok(());
        }
        let kwargs = kwargs.unwrap_or(DelArgs {
            items: Some(vec![
                Self::acr_borrow_ciid(&grid.header.cells, &num_cols.saturating_sub(1))?.clone(),
            ]),
            emit: true,
            undo: true,
        });
        let items = kwargs.items.unwrap_or(vec![
            Self::acr_borrow_ciid(&grid.header.cells, &num_cols.saturating_sub(1))?.clone(),
        ]);
        let mut event_data = self.cr_new_event(true);
        let mut positions: Vec<usize> = Vec::new();
        for iid in items.iter() {
            positions.push(Self::acr_copy_pos(&grid.header.iid_to_col, iid)?);
        }
        if positions.is_empty() {
            return Ok(());
        } else {
            positions.sort_unstable();
            positions.dedup();
        }
        self.cr_del_cols(positions, &mut event_data)?;
        self.cr_event_modified(&event_data, kwargs.emit, true)?;
        if self.opts.borrow().enabled_undo && kwargs.undo {
            grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
