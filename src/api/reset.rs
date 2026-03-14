use crate::Options;
use crate::Table;
use crate::args::ResetArgs;

impl Table {
    /// Reset the table.
    pub fn reset(&mut self, kwargs: Option<ResetArgs>) -> Result<(), String> {
        let kwargs: ResetArgs = kwargs.unwrap_or_default();
        if kwargs.opts {
            self.opts = Options::new();
        }
        if kwargs.grid {
            self.grid.reset();
        }
        if kwargs.index {
            self.grid.index.reset();
        }
        if kwargs.header {
            self.grid.header.reset();
        }
        if kwargs.selections {
            Self::acr_deselect_all(&mut self.sel, None);
        }
        if kwargs.undo_stack {
            self.grid.undo_stack.clear();
            self.grid.redo_stack.clear();
        }
        Ok(())
    }
}
