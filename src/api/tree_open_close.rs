use crate::EventData;
use crate::Table;
use crate::args::TreeOpenArgs;

impl Table {
    pub(crate) fn tree_set_open_or_closed(
        &self,
        kwargs: TreeOpenArgs,
        open: bool,
    ) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        let mut event_data: EventData = self.cr_new_event(true);
        let to_show = if kwargs.rows.is_empty() {
            self.cr_tree_open_close(
                (0..Self::acr_total_rows(&grid.index.cells)).into_iter(),
                open,
                Some(&mut event_data),
            )?
        } else {
            let mut rows = Vec::new();
            for iid in kwargs.rows.into_iter() {
                rows.push(*Self::acr_borrow_pos(&grid.index.iid_to_row, &iid)?);
            }
            rows.sort_unstable();
            rows.dedup();
            self.cr_tree_open_close(rows.into_iter(), open, Some(&mut event_data))?
        };
        if !to_show.is_empty() {
            Self::acr_set_row_vis(
                &mut grid.index.cells,
                to_show.into_iter(),
                open,
                Some(&mut event_data),
            )?;
            Self::acr_remake_rows(
                &mut grid.canvas_rows,
                &mut grid.disp_rows,
                &grid.index.cells,
            );
        }
        if !event_data.index_cells.is_empty() && self.opts.borrow().enabled_undo && kwargs.undo {
            // TODO: do we need a new event type for open/close/hide/unhide?
            self.cr_event_modified(&event_data, kwargs.emit, true)?;
            grid.undo_stack.push_back(event_data);
        }
        // TODO: set refresh timer
        Ok(())
    }

    /// Sets items/rows to open. If used without args then all rows are opened.
    pub fn tree_open(&self, kwargs: Option<TreeOpenArgs>) -> Result<(), String> {
        self.tree_set_open_or_closed(kwargs.unwrap_or_default(), true)
    }

    /// Sets items/rows to closed. If used without args then all rows are closed.
    pub fn tree_close(&self, kwargs: Option<TreeOpenArgs>) -> Result<(), String> {
        self.tree_set_open_or_closed(kwargs.unwrap_or_default(), false)
    }
}
