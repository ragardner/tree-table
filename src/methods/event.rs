use crate::EventData;
use crate::Events;
use crate::Table;
use crate::TableGrid;
use crate::statics::{EVENT_MODIFIED, EVENT_SELECT};

impl Table {
    pub(crate) fn cr_callback_event(&self, events: &Events, event: &str, data: &EventData) {
        if let Some(vec) = events.bound_event_fns.get(event) {
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
        &self,
        grid: &mut TableGrid,
        events: &Events,
        data: &EventData,
        emit: bool,
        purge_redo: bool,
    ) -> Result<(), String> {
        if emit {
            self.cr_callback_event(&events, EVENT_MODIFIED, data);
        }
        if purge_redo {
            grid.redo_stack.clear();
        }
        Ok(())
    }

    /// The table's selection was modified, serialize event_data and send to bound events.
    /// Purge redo is required when not performing undo or redo.
    pub(crate) fn cr_event_select(
        &self,
        grid: &mut TableGrid,
        events: &Events,
        data: &EventData,
        emit: bool,
        purge_redo: bool,
    ) -> Result<(), String> {
        if emit {
            self.cr_callback_event(&events, EVENT_SELECT, data);
        }
        if purge_redo {
            grid.redo_stack.clear();
        }
        Ok(())
    }
}
