use crate::EventData;
use crate::Table;
use crate::ValidationMessage;
use crate::log;
use crate::statics::EVENT_CLIPBOARD;
use crate::statics::EVENT_MODIFIED;
use crate::statics::EVENT_SELECT;

impl Table {
    pub fn bind_validation(
        &self,
        callback: fn(ValidationMessage) -> Result<ValidationMessage, String>,
    ) -> Result<(), String> {
        let events = &mut *self.events.borrow_mut();
        events.bound_validation_fn = Some(callback);
        Ok(())
    }

    pub fn unbind_validation(&self) -> Result<(), String> {
        let events = &mut *self.events.borrow_mut();
        events.bound_validation_fn = None;
        Ok(())
    }

    // Helper for binding an event callback
    fn bind_event(
        &self,
        event: &str,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        let events = &mut *self.events.borrow_mut();
        let vec = events
            .bound_event_fns
            .entry(event.to_string())
            .or_insert_with(Vec::new);
        if !add {
            vec.clear();
        }
        vec.push(callback);
        Ok(())
    }

    // Helper for unbinding an event
    fn unbind_event(&self, event: &str) -> Result<(), String> {
        let events = &mut *self.events.borrow_mut();
        if let Some(vec) = events.bound_event_fns.get_mut(event) {
            vec.clear();
        }
        Ok(())
    }

    pub fn bind_modified(
        &self,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        self.bind_event(EVENT_MODIFIED, callback, add)
    }

    pub fn unbind_modified(&self) -> Result<(), String> {
        self.unbind_event(EVENT_MODIFIED)
    }

    pub fn bind_select(
        &self,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        self.bind_event(EVENT_SELECT, callback, add)
    }

    pub fn unbind_select(&self) -> Result<(), String> {
        self.unbind_event(EVENT_SELECT)
    }

    pub fn bind_clipboard(
        &self,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        self.bind_event(EVENT_CLIPBOARD, callback, add)
    }

    pub fn unbind_clipboard(&self) -> Result<(), String> {
        self.unbind_event(EVENT_CLIPBOARD)
    }
}
