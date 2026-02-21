use crate::Change;
use crate::EventData;
use crate::Table;
use std::collections::HashMap;

impl Table {
    pub(crate) fn cr_new_event(&self, boxes: bool) -> EventData {
        EventData {
            widget_name: self.opts.widget_name.clone(),
            widget_id: self.opts.widget_id.clone(),
            changes: if boxes {
                vec![Change::SaveSelection {
                    old_selection_boxes: self.sel.selection_boxes.clone(),
                    old_selected: self.sel.selected.clone(),
                }]
            } else {
                vec![]
            },
            tree_changes: HashMap::new(),
        }
    }
}
