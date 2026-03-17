use crate::EventData;
use crate::Table;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    /// The table was modified in some way.
    /// Purge redo is required when not performing undo or redo.
    pub(crate) fn cr_event_modified(&mut self, purge_redo: bool) -> Result<(), String> {
        if purge_redo {
            self.grid.redo_stack.clear();
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn cr_finalize_event_if_any_non_selection_change(
        &mut self,
        event_data: EventData,
        undo: bool,
        emit: bool,
        reverse: bool,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        if event_data.has_non_selection_change(reverse) {
            self.cr_event_modified(true)?;
            let shared = Rc::new(RefCell::new(event_data));
            if self.opts.enabled_undo && undo && self.grid.undo_stack.get_maxlen() > 0 {
                self.grid.undo_stack.push_back(shared.clone());
            }
            Ok(emit.then_some(shared))
        } else {
            Ok(None)
        }
    }

    #[inline(always)]
    pub(crate) fn cr_finalize_event(
        &mut self,
        event_data: EventData,
        undo: bool,
        emit: bool,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.cr_event_modified(true)?;
        let shared = Rc::new(RefCell::new(event_data));
        if self.opts.enabled_undo && undo && self.grid.undo_stack.get_maxlen() > 0 {
            self.grid.undo_stack.push_back(shared.clone());
        }
        Ok(emit.then_some(shared))
    }
}

impl Table {}
