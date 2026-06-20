use crate::args::UndoRedoArgs;
use crate::{EventData, Table};
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;

impl Table {
    pub fn undo(
        &mut self,
        kwargs: Option<UndoRedoArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.cr_undo(kwargs)
    }

    pub fn redo(
        &mut self,
        kwargs: Option<UndoRedoArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.cr_redo(kwargs)
    }

    pub fn get_next_redo(&self) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.cr_get_next_redo()
    }

    pub fn get_next_undo(&self) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.cr_get_next_undo()
    }
}
