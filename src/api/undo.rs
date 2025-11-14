use crate::Table;
use crate::args::UndoRedoArgs;

impl Table {
    pub fn undo(&mut self, kwargs: Option<UndoRedoArgs>) -> Result<(), String> {
        self.cr_undo(kwargs)
    }

    pub fn redo(&mut self, kwargs: Option<UndoRedoArgs>) -> Result<(), String> {
        self.cr_redo(kwargs)
    }
}
