use crate::ChangeHeaderCell;
use crate::ChangeIndexCell;
use crate::EventData;
use crate::Events;
use crate::Options;
use crate::Selection;
use crate::Table;
use crate::TableGrid;
use crate::ValidationMessage;
use crate::args::UndoRedoArgs;
use crate::str_err;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(PartialEq, Eq)]
pub enum UndoType {
    Undo,
    Redo,
}

impl Table {
    pub(crate) fn cr_undo(&self, kwargs: Option<UndoRedoArgs>) -> Result<(), String> {
        let (mut grid, opts, sel) = self.cr_mut_borrows();
        let events = self.events.borrow();
        let kwargs = kwargs.unwrap_or(UndoRedoArgs { emit: true });
        if grid.undo_stack.is_empty() {
            return Ok(());
        }
        let change = grid
            .undo_stack
            .pop_back()
            .ok_or(str_err!("Undo stack unexpectedly empty"))?;
        let event_data =
            self.cr_undo_and_invert_event(&mut grid, &opts, &sel, &events, change, UndoType::Undo)?;
        self.cr_event_modified(&mut grid, &events, &event_data, kwargs.emit, false)?;
        grid.redo_stack.push_back(event_data);
        Ok(())
    }

    pub(crate) fn cr_redo(&self, kwargs: Option<UndoRedoArgs>) -> Result<(), String> {
        let (mut grid, opts, sel) = self.cr_mut_borrows();
        let events = self.events.borrow();
        let kwargs = kwargs.unwrap_or(UndoRedoArgs { emit: true });
        if grid.redo_stack.is_empty() {
            return Ok(());
        }
        let change = grid
            .redo_stack
            .pop_back()
            .ok_or(str_err!("Redo stack unexpectedly empty"))?;
        let event_data =
            self.cr_undo_and_invert_event(&mut grid, &opts, &sel, &events, change, UndoType::Redo)?;
        self.cr_event_modified(&mut grid, &events, &event_data, kwargs.emit, false)?;
        grid.undo_stack.push_back(event_data);
        Ok(())
    }

    fn cr_edit_cells_using_change(
        &self,
        grid: &mut TableGrid,
        events: &Events,
        opts: &Options,
        change: &mut EventData,
        event_data: &mut EventData,
    ) -> Result<Option<ValidationMessage>, String> {
        if events.bound_validation_fn.is_some() && opts.validation_includes_undo {
            let mut table_data: HashMap<u64, Vec<ChangeIndexCell>> = HashMap::new();
            let mut index_data: HashMap<u64, Vec<ChangeIndexCell>> = HashMap::new();
            let mut header_data: HashMap<u64, Vec<ChangeHeaderCell>> = HashMap::new();
            // Separate the Val changes from the non-Val changes
            // to send them to bound validation functions
            // Index
            for (key, mut changes) in change.index_cells.drain() {
                let mut val_changes: Vec<ChangeIndexCell> = Vec::new();
                let mut non_val_changes: Vec<ChangeIndexCell> = Vec::new();
                for ch in changes.drain(..) {
                    match ch {
                        ChangeIndexCell::SetVal(_) => {
                            val_changes.push(ch);
                        }
                        _ => {
                            non_val_changes.push(ch);
                        }
                    }
                }
                if !val_changes.is_empty() {
                    match index_data.entry(key as u64) {
                        Entry::Vacant(entry) => {
                            entry.insert(val_changes);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().extend(val_changes);
                        }
                    }
                }
                if !non_val_changes.is_empty() {
                    let mut inverted_non_val = Vec::with_capacity(non_val_changes.len());
                    for change in non_val_changes.into_iter() {
                        inverted_non_val.push(change.invert(
                            &mut grid.index.cells,
                            &mut grid.index.iid_to_row,
                            key,
                        )?);
                    }
                    match event_data.index_cells.entry(key) {
                        Entry::Vacant(entry) => {
                            entry.insert(inverted_non_val);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().extend(inverted_non_val);
                        }
                    }
                }
            }
            // Header
            for (key, mut changes) in change.header_cells.drain() {
                let mut val_changes: Vec<ChangeHeaderCell> = Vec::new();
                let mut non_val_changes: Vec<ChangeHeaderCell> = Vec::new();
                for ch in changes.drain(..) {
                    match ch {
                        ChangeHeaderCell::SetVal(_) => {
                            val_changes.push(ch);
                        }
                        _ => {
                            non_val_changes.push(ch);
                        }
                    }
                }
                if !val_changes.is_empty() {
                    match header_data.entry(key as u64) {
                        Entry::Vacant(entry) => {
                            entry.insert(val_changes);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().extend(val_changes);
                        }
                    }
                }
                if !non_val_changes.is_empty() {
                    let mut inverted_non_val = Vec::with_capacity(non_val_changes.len());
                    for change in non_val_changes.into_iter() {
                        inverted_non_val.push(change.invert(
                            &mut grid.header.cells,
                            &mut grid.header.iid_to_col,
                            key,
                        )?);
                    }
                    match event_data.header_cells.entry(key) {
                        Entry::Vacant(entry) => {
                            entry.insert(inverted_non_val);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().extend(inverted_non_val);
                        }
                    }
                }
            }
            // Table
            for (key, mut changes) in change.table_cells.drain() {
                let mut val_changes: Vec<ChangeIndexCell> = Vec::new();
                let mut non_val_changes: Vec<ChangeIndexCell> = Vec::new();
                for ch in changes.drain(..) {
                    match ch {
                        ChangeIndexCell::SetTableCellVal { .. } => {
                            val_changes.push(ch);
                        }
                        _ => {
                            non_val_changes.push(ch);
                        }
                    }
                }
                if !val_changes.is_empty() {
                    match table_data.entry(key as u64) {
                        Entry::Vacant(entry) => {
                            entry.insert(val_changes);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().extend(val_changes);
                        }
                    }
                }
                if !non_val_changes.is_empty() {
                    let mut inverted_non_val = Vec::with_capacity(non_val_changes.len());
                    for change in non_val_changes.into_iter() {
                        inverted_non_val.push(change.invert(
                            &mut grid.index.cells,
                            &mut grid.index.iid_to_row,
                            key,
                        )?);
                    }
                    match event_data.table_cells.entry(key) {
                        Entry::Vacant(entry) => {
                            entry.insert(inverted_non_val);
                        }
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().extend(inverted_non_val);
                        }
                    }
                }
            }
            let mut validation_message = ValidationMessage {
                table_data: Vec::new(),
                index_data: Vec::new(),
                header_data: Vec::new(),
            };
            for (row, changes) in table_data.into_iter() {
                validation_message.table_data.push((row, changes));
            }
            for (row, changes) in index_data.into_iter() {
                validation_message.index_data.push((row, changes));
            }
            for (row, changes) in header_data.into_iter() {
                validation_message.header_data.push((row, changes));
            }
            Ok(Some(validation_message))
        } else {
            // Invert changes, fill event_data with replaced values.
            for (pos, changes) in std::mem::take(&mut change.index_cells).into_iter() {
                let mut new_changes = Vec::new();
                for change in changes.into_iter() {
                    new_changes.push(change.invert(
                        &mut grid.index.cells,
                        &mut grid.index.iid_to_row,
                        pos,
                    )?);
                }
                event_data.index_cells.insert(pos, new_changes);
            }
            for (pos, changes) in std::mem::take(&mut change.header_cells).into_iter() {
                let mut new_changes = Vec::new();
                for change in changes.into_iter() {
                    new_changes.push(change.invert(
                        &mut grid.header.cells,
                        &mut grid.header.iid_to_col,
                        pos,
                    )?);
                }
                event_data.header_cells.insert(pos, new_changes);
            }
            for (pos, changes) in std::mem::take(&mut change.table_cells).into_iter() {
                let mut new_changes = Vec::new();
                for change in changes.into_iter() {
                    new_changes.push(change.invert(
                        &mut grid.index.cells,
                        &mut grid.index.iid_to_row,
                        pos,
                    )?);
                }
                event_data.table_cells.insert(pos, new_changes);
            }
            Ok(None)
        }
    }

    pub(crate) fn cr_undo_and_invert_event(
        &self,
        mut grid: &mut TableGrid,
        opts: &Options,
        sel: &Selection,
        events: &Events,
        mut to_undo: EventData,
        undo_type: UndoType,
    ) -> Result<EventData, String> {
        let mut event_data = self.cr_new_event(false, &opts, &sel);
        event_data.selection_boxes = to_undo.selection_boxes.clone();
        event_data.selected = to_undo.selected.clone();

        if undo_type == UndoType::Undo {
            if let Some(to_validate) = self.cr_edit_cells_using_change(
                &mut grid,
                &events,
                &opts,
                &mut to_undo,
                &mut event_data,
            )? {
                self.cr_bulk_edit_validation(
                    &mut grid,
                    &self.events.borrow(),
                    to_validate,
                    &mut event_data,
                )?;
            }
        }

        if !to_undo.moved_cols.is_empty() {
            let new_to_old = std::mem::take(&mut to_undo.moved_cols);
            event_data.moved_cols = new_to_old.iter().map(|(k, v)| (*v, *k)).collect();
            self.cr_move_cols(&mut grid, &opts, &mut event_data, &new_to_old)?;
        }

        if !to_undo.moved_rows.is_empty() {
            let new_to_old = std::mem::take(&mut to_undo.moved_rows);
            event_data.moved_rows = new_to_old.iter().map(|(k, v)| (*v, *k)).collect();
            self.cr_move_rows(
                &mut grid,
                &opts,
                &mut event_data,
                new_to_old,
                Some(&mut to_undo),
                None,
                true,
            )?;
        }

        if !to_undo.added_cols.is_empty() {
            self.cr_del_cols(
                &mut grid,
                &opts,
                std::mem::take(&mut to_undo.added_cols),
                &mut event_data,
            )?;
        }

        if !to_undo.added_rows.is_empty() {
            self.cr_del_rows(
                &mut grid,
                &opts,
                std::mem::take(&mut to_undo.added_rows),
                &mut event_data,
                false,
            )?;
        }

        if !to_undo.removed_header.is_empty() {
            let sorted_seq = to_undo.removed_header.iter().map(|t| t.0.clone()).collect();
            self.cr_add_cols(
                &mut grid,
                &opts,
                &sel,
                std::mem::take(&mut to_undo.removed_table_cells),
                std::mem::take(&mut to_undo.removed_header),
                sorted_seq,
                &mut event_data,
                false,
                true,
            )?;
        }

        if !to_undo.removed_index.is_empty() {
            let sorted_seq = to_undo.removed_index.iter().map(|t| t.0.clone()).collect();
            self.cr_add_rows(
                &mut grid,
                &opts,
                &sel,
                std::mem::take(&mut to_undo.removed_index),
                sorted_seq,
                &mut event_data,
                false,
                true,
                true,
            )?;
        }

        if undo_type == UndoType::Redo {
            if let Some(to_validate) = self.cr_edit_cells_using_change(
                &mut grid,
                &events,
                &opts,
                &mut to_undo,
                &mut event_data,
            )? {
                self.cr_bulk_edit_validation(
                    &mut grid,
                    &self.events.borrow(),
                    to_validate,
                    &mut event_data,
                )?;
            }
        }

        // TODO: Handle selection boxes
        // TODO: See/scroll to correct section of table

        Ok(event_data)
    }
}
