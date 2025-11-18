use crate::Table;
use crate::args::DelArgs;

impl Table {
    /// Delete rows from the table.
    pub fn del_rows(&mut self, kwargs: Option<DelArgs>) -> Result<(), String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells);
        if num_rows == 0 {
            return Ok(());
        }
        let kwargs = kwargs.unwrap_or(DelArgs {
            items: Some(vec![
                Self::acr_borrow_riid(&self.grid.index.cells, num_rows.saturating_sub(1))?
                    .to_string(),
            ]),
            emit: true,
            undo: true,
        });
        let items = kwargs.items.unwrap_or(vec![
            Self::acr_borrow_riid(&self.grid.index.cells, num_rows.saturating_sub(1))?.to_string(),
        ]);
        let mut event_data = self.cr_new_event(true);
        let mut positions: Vec<usize> = Vec::new();
        for iid in items.iter() {
            positions.push(Self::acr_pos(&self.grid.index.iid_to_row, iid)?);
        }
        if positions.is_empty() {
            return Ok(());
        } else {
            positions.sort_unstable();
            positions.dedup();
        }
        self.cr_del_rows(positions, &mut event_data, true)?;
        self.cr_event_modified(&event_data, kwargs.emit, true)?;
        if self.opts.enabled_undo && kwargs.undo {
            self.grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
