use crate::Table;

impl Table {
    /// The table was modified in some way.
    /// Purge redo is required when not performing undo or redo.
    pub(crate) fn cr_event_modified(&mut self, purge_redo: bool) -> Result<(), String> {
        if purge_redo {
            self.grid.redo_stack.clear();
        }
        Ok(())
    }
}
