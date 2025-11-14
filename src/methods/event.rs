use crate::EventData;
use crate::Table;
use crate::statics::{EVENT_MODIFIED, EVENT_SELECT};

impl Table {
    pub(crate) fn cr_callback_event(&self, event: &str, data: &EventData) {
        if let Some(vec) = self.events.bound_event_fns.get(event) {
            if !vec.is_empty() {
                for func in vec.iter() {
                    let _ = func(&data);
                }
            }
        }
    }

    /// The table was modified, serialize event_data and send to bound events.
    /// Purge redo is required when not performing undo or redo.
    pub(crate) fn cr_event_modified(
        &mut self,
        data: &EventData,
        emit: bool,
        purge_redo: bool,
    ) -> Result<(), String> {
        if emit {
            self.cr_callback_event(EVENT_MODIFIED, data);
        }
        if purge_redo {
            self.grid.redo_stack.clear();
        }
        Ok(())
    }

    /// The table's selection was modified, serialize event_data and send to bound events.
    /// Purge redo is required when not performing undo or redo.
    pub(crate) fn cr_event_select(
        &mut self,
        data: &EventData,
        emit: bool,
        purge_redo: bool,
    ) -> Result<(), String> {
        if emit {
            self.cr_callback_event(EVENT_SELECT, data);
        }
        if purge_redo {
            self.grid.redo_stack.clear();
        }
        Ok(())
    }
}
