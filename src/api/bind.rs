use crate::EventData;
use crate::Table;
use crate::ValidationMessage;
use crate::statics::EVENT_CLIPBOARD;
use crate::statics::EVENT_MODIFIED;
use crate::statics::EVENT_SELECT;

impl Table {
    pub fn bind_validation(
        &mut self,
        callback: fn(ValidationMessage) -> Result<ValidationMessage, String>,
    ) -> Result<(), String> {
        self.events.bound_validation_fn = Some(callback);
        Ok(())
    }

    pub fn unbind_validation(&mut self) -> Result<(), String> {
        self.events.bound_validation_fn = None;
        Ok(())
    }

    // Helper for binding an event callback
    fn bind_event(
        &mut self,
        event: &str,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        let vec = self
            .events
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
    fn unbind_event(&mut self, event: &str) -> Result<(), String> {
        if let Some(vec) = self.events.bound_event_fns.get_mut(event) {
            vec.clear();
        }
        Ok(())
    }

    pub fn bind_modified(
        &mut self,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        self.bind_event(EVENT_MODIFIED, callback, add)
    }

    pub fn unbind_modified(&mut self) -> Result<(), String> {
        self.unbind_event(EVENT_MODIFIED)
    }

    pub fn bind_select(
        &mut self,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        self.bind_event(EVENT_SELECT, callback, add)
    }

    pub fn unbind_select(&mut self) -> Result<(), String> {
        self.unbind_event(EVENT_SELECT)
    }

    pub fn bind_clipboard(
        &mut self,
        callback: fn(&EventData) -> Result<(), String>,
        add: bool,
    ) -> Result<(), String> {
        self.bind_event(EVENT_CLIPBOARD, callback, add)
    }

    pub fn unbind_clipboard(&mut self) -> Result<(), String> {
        self.unbind_event(EVENT_CLIPBOARD)
    }
}
