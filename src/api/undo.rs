use crate::Table;
use crate::args::UndoRedoArgs;

impl Table {
    pub fn undo(&self, kwargs: Option<UndoRedoArgs>) -> Result<(), String> {
        self.cr_undo(kwargs)
    }

    pub fn redo(&self, kwargs: Option<UndoRedoArgs>) -> Result<(), String> {
        self.cr_redo(kwargs)
    }
}
