use crate::Table;
use crate::args::DelArgs;

impl Table {
    /// Delete rows from the table.
    pub fn del_rows(&self, kwargs: Option<DelArgs>) -> Result<(), String> {
        let (mut grid, opts, sel) = self.cr_mut_borrows();
        let num_rows = Self::acr_total_rows(&grid.index.cells);
        if num_rows == 0 {
            return Ok(());
        }
        let kwargs = kwargs.unwrap_or(DelArgs {
            items: Some(vec![
                Self::acr_borrow_riid(&grid.index.cells, num_rows.saturating_sub(1))?.to_string(),
            ]),
            emit: true,
            undo: true,
        });
        let items = kwargs.items.unwrap_or(vec![
            Self::acr_borrow_riid(&grid.index.cells, num_rows.saturating_sub(1))?.to_string(),
        ]);
        let mut event_data = self.cr_new_event(true, &opts, &sel);
        let mut positions: Vec<usize> = Vec::new();
        for iid in items.iter() {
            positions.push(Self::acr_pos(&grid.index.iid_to_row, iid)?);
        }
        if positions.is_empty() {
            return Ok(());
        } else {
            positions.sort_unstable();
            positions.dedup();
        }
        self.cr_del_rows(&mut grid, &opts, positions, &mut event_data, true)?;
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
