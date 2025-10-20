use crate::{ChangeHeaderCell, ChangeIndexCell, EventData};
use std::collections::HashMap;

pub struct ValidationMessage {
    pub table_data: Vec<(u64, Vec<ChangeIndexCell>)>,
    pub index_data: Vec<(u64, Vec<ChangeIndexCell>)>,
    pub header_data: Vec<(u64, Vec<ChangeHeaderCell>)>,
}

#[derive(Debug)]
pub struct Events {
    pub bound_event_fns: HashMap<String, Vec<fn(&EventData) -> Result<(), String>>>,
    pub bound_validation_fn: Option<fn(ValidationMessage) -> Result<ValidationMessage, String>>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            bound_event_fns: HashMap::new(),
            bound_validation_fn: None,
        }
    }
}
