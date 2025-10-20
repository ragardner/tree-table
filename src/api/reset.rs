use crate::Options;
use crate::Table;
use crate::args::ResetArgs;

impl Table {
    /// Reset the table.
    pub fn reset(&self, kwargs: Option<ResetArgs>) -> Result<(), String> {
        let kwargs: ResetArgs = kwargs.unwrap_or_default();
        if kwargs.opts {
            *self.opts.borrow_mut() = Options::new();
        }
        let mut grid = self.grid.borrow_mut();
        if kwargs.grid {
            grid.reset();
        }
        if kwargs.index {
            grid.index.reset();
        }
        if kwargs.header {
            grid.header.reset();
        }
        if kwargs.selections {
            Self::acr_deselect_all(&mut self.sel.borrow_mut());
        }
        if kwargs.undo_stack {
            grid.undo_stack.clear();
            grid.redo_stack.clear();
        }
        Ok(())
    }
}
