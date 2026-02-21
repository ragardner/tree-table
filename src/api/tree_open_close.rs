use crate::EventData;
use crate::Table;
use crate::args::TreeOpenArgs;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub(crate) fn tree_set_open_or_closed(
        &mut self,
        kwargs: TreeOpenArgs,
        open: bool,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let mut event_data: EventData = self.cr_new_event(true);
        let to_show = if kwargs.rows.is_empty() {
            let end_row = Self::acr_total_rows(&self.grid.index.cells);
            self.cr_tree_open_close((0..end_row).into_iter(), open, Some(&mut event_data))?
        } else {
            let mut rows = Vec::new();
            for iid in kwargs.rows.into_iter() {
                rows.push(Self::acr_pos(&self.grid.index.iid_to_row, &iid)?);
            }
            rows.sort_unstable();
            rows.dedup();
            self.cr_tree_open_close(rows.into_iter(), open, Some(&mut event_data))?
        };
        if !to_show.is_empty() {
            Self::acr_set_row_vis(
                &mut self.grid.index.cells,
                to_show.into_iter(),
                open,
                Some(&mut event_data),
            )?;
            Self::acr_remake_rows(
                &mut self.grid.canvas_rows,
                &mut self.grid.disp_rows,
                &self.grid.index.cells,
                self.opts.horizontal_grid_line_width.clone(),
            );
        }
        if !event_data.changes.is_empty() {
            self.cr_event_modified(true)?;
            let shared = Rc::new(RefCell::new(event_data));
            if self.opts.enabled_undo && kwargs.undo && self.grid.undo_stack.get_maxlen() > 0 {
                self.grid.undo_stack.push_back(shared.clone());
            }
            Ok(kwargs.emit.then_some(shared))
        } else {
            Ok(None)
        }
    }

    /// Sets items/rows to open. If used without args then all rows are opened.
    pub fn tree_open(
        &mut self,
        kwargs: Option<TreeOpenArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.tree_set_open_or_closed(kwargs.unwrap_or_default(), true)
    }

    /// Sets items/rows to closed. If used without args then all rows are closed.
    pub fn tree_close(
        &mut self,
        kwargs: Option<TreeOpenArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.tree_set_open_or_closed(kwargs.unwrap_or_default(), false)
    }
}
