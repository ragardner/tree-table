use crate::EventData;
use crate::Table;
use std::collections::HashMap;

impl Table {
    pub(crate) fn cr_new_event(&self, boxes: bool) -> EventData {
        EventData {
            widget_name: self.opts.widget_name.clone(),
            widget_id: self.opts.widget_id.clone(),
            table_cells: HashMap::new(),
            index_cells: HashMap::new(),
            header_cells: HashMap::new(),
            added_rows: Vec::new(),
            added_cols: Vec::new(),
            added_progress_bars: HashMap::new(),
            removed_index: Vec::new(),
            removed_header: Vec::new(),
            removed_table_cells: HashMap::new(),
            removed_progress_bars: HashMap::new(),
            moved_rows: HashMap::new(),
            moved_cols: HashMap::new(),
            selection_boxes: if boxes {
                self.sel.selection_boxes.clone()
            } else {
                Vec::new()
            },
            selected: if boxes {
                self.sel.selected.clone()
            } else {
                None
            },
            key: None,
            tree_changes: HashMap::new(),
            row: None,
            col: None,
            table_data: HashMap::new(),
            index_data: HashMap::new(),
            header_data: HashMap::new(),
        }
    }
}
