use crate::EventData;
use crate::Table;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
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
