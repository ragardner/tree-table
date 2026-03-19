use crate::Change;
use crate::EventData;
use crate::Index;
use crate::Table;
use crate::args::UndoRedoArgs;
use crate::str_err;
use crate::types::event_data::TreeChange;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_get_next_undo(&self) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        if self.grid.undo_stack.is_empty() {
            return Ok(None);
        }
        let change = self
            .grid
            .undo_stack
            .last()
            .ok_or_else(|| str_err!("Undo stack unexpectedly empty"))?
            .clone();
        Ok(Some(change))
    }

    pub(crate) fn cr_get_next_redo(&self) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        if self.grid.redo_stack.is_empty() {
            return Ok(None);
        }
        let change = self
            .grid
            .redo_stack
            .last()
            .ok_or_else(|| str_err!("Redo stack unexpectedly empty"))?
            .clone();
        Ok(Some(change))
    }

    pub(crate) fn cr_undo(
        &mut self,
        kwargs: Option<UndoRedoArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        if self.grid.undo_stack.is_empty() {
            return Ok(None);
        }
        let change = self
            .grid
            .undo_stack
            .pop_back()
            .ok_or_else(|| str_err!("Undo stack unexpectedly empty"))?;
        let event_data = self.cr_undo_and_invert_event(change, kwargs.invalid_indices, true)?;
        self.cr_event_modified(false)?;
        let shared = Rc::new(RefCell::new(event_data));
        if self.opts.enabled_undo && self.grid.redo_stack.get_maxlen() > 0 {
            self.grid.redo_stack.push_back(shared.clone());
        }
        Ok(kwargs.emit.then_some(shared))
    }

    pub(crate) fn cr_redo(
        &mut self,
        kwargs: Option<UndoRedoArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        if self.grid.redo_stack.is_empty() {
            return Ok(None);
        }
        let change = self
            .grid
            .redo_stack
            .pop_back()
            .ok_or_else(|| str_err!("Redo stack unexpectedly empty"))?;
        let event_data = self.cr_undo_and_invert_event(change, kwargs.invalid_indices, true)?;
        self.cr_event_modified(false)?;
        let shared = Rc::new(RefCell::new(event_data));
        if self.opts.enabled_undo && self.grid.undo_stack.get_maxlen() > 0 {
            self.grid.undo_stack.push_back(shared.clone());
        }
        Ok(kwargs.emit.then_some(shared))
    }

    pub(crate) fn cr_undo_and_invert_event(
        &mut self,
        to_undo: Rc<RefCell<EventData>>,
        mut invalid_indices: Vec<u64>,
        repeat_on_err: bool,
    ) -> Result<EventData, String> {
        // For move rows, move cols
        fn prepare_move_mappings(
            new_to_old_inp: Vec<(usize, usize)>,
        ) -> (HashSet<usize>, Vec<(usize, usize)>, HashMap<usize, usize>) {
            let capacity = new_to_old_inp.len();
            let mut new_to_old: HashMap<usize, usize> = HashMap::with_capacity(capacity);
            let mut old_to_new: Vec<(usize, usize)> = Vec::with_capacity(capacity);
            let mut old: HashSet<usize> = HashSet::with_capacity(capacity);
            for (new_pos, old_pos) in new_to_old_inp {
                new_to_old.insert(new_pos, old_pos);
                old_to_new.push((old_pos, new_pos)); // reverse mapping
                old.insert(old_pos);
            }
            (old, old_to_new, new_to_old)
        }

        let mut event_data = self.cr_new_event(false);
        let mut to_undo_mut = to_undo.borrow_mut();

        // All changes are consumed
        let changes = std::mem::take(&mut to_undo_mut.changes);
        let mut tree_changes = std::mem::take(&mut to_undo_mut.tree_changes);

        let invalid_indices_set: HashSet<usize> =
            invalid_indices.drain(..).map(|x| x as usize).collect();
        let mut error: Option<String> = None;
        for (i, change) in changes.into_iter().enumerate().rev() {
            if invalid_indices_set.contains(&i) {
                continue;
            }
            let result = match change {
                Change::AddCols(cols) => self.cr_del_cols(cols, &mut event_data),
                Change::AddRows(rows) => self.cr_del_rows(rows, &mut event_data, false),
                Change::MoveCols(new_to_old) => {
                    let (old, old_to_new, new_to_old) = prepare_move_mappings(new_to_old);
                    self.cr_move_cols(&mut event_data, old, old_to_new, &new_to_old)
                }
                Change::MoveRows(new_to_old) => {
                    Self::acr_tree_restore_cells(
                        &mut self.grid.index,
                        &mut tree_changes,
                        Some(&mut event_data),
                    )?;
                    let (old, old_to_new, new_to_old) = prepare_move_mappings(new_to_old);
                    self.cr_move_rows(&mut event_data, old, old_to_new, new_to_old, None, false)
                }
                Change::RemoveCols {
                    removed_header,
                    removed_table,
                } => {
                    let sorted_seq = removed_header.iter().map(|t| t.0.clone()).collect();
                    self.cr_add_cols(
                        removed_table,
                        removed_header,
                        sorted_seq,
                        &mut event_data,
                        true,
                    );
                    Ok(())
                }
                Change::RemoveRows(removed_rows) => {
                    let sorted_seq = removed_rows.iter().map(|t| t.0.clone()).collect();
                    Self::acr_tree_restore_cells(&mut self.grid.index, &mut tree_changes, None)?;
                    self.cr_add_rows(removed_rows, sorted_seq, &mut event_data, false, true)
                }
                Change::IndexCell { row, change } => match change.invert(
                    &mut self.grid.index.cells,
                    &mut self.grid.index.iid_to_row,
                    row,
                ) {
                    Ok(inv) => {
                        event_data
                            .changes
                            .push(Change::IndexCell { row, change: inv });
                        Ok(())
                    }
                    Err(e) => Err(e),
                },
                Change::HeaderCell { col, change } => match change.invert(
                    &mut self.grid.header.cells,
                    &mut self.grid.header.iid_to_col,
                    col,
                ) {
                    Ok(inv) => {
                        event_data
                            .changes
                            .push(Change::HeaderCell { col, change: inv });
                        Ok(())
                    }
                    Err(e) => Err(e),
                },
                Change::SetSelectionBoxes {
                    old_selection_boxes,
                    old_selected,
                } => {
                    let change = Change::set_selection_boxes(
                        self.sel.selection_boxes.clone(),
                        self.sel.selected.clone(),
                    );
                    self.sel.selection_boxes = old_selection_boxes;
                    self.sel.selected = old_selected;
                    event_data.changes.push(change);
                    Ok(())
                }
                Change::AddSelectionBoxes {
                    count,
                    old_selected,
                } => {
                    let split_at = self.sel.selection_boxes.len().saturating_sub(count);
                    let change = Change::remove_selection_boxes(
                        self.sel.selection_boxes.split_off(split_at),
                        self.sel.selected.clone(),
                    );
                    self.sel.selected = old_selected;
                    event_data.changes.push(change);
                    Ok(())
                }
                Change::RemoveSelectionBoxes {
                    removed,
                    old_selected,
                } => {
                    let count = removed.len();
                    self.sel.selection_boxes.extend(removed);
                    let change = Change::add_selection_boxes(count, self.sel.selected.clone());
                    self.sel.selected = old_selected;
                    event_data.changes.push(change);
                    Ok(())
                }
            };

            if let Err(e) = result {
                error = Some(e);
                break;
            }
        }

        if let Some(e) = error {
            if repeat_on_err {
                self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            }
            return Err(e);
        }

        Ok(event_data)
    }

    pub(crate) fn cr_roll_back(&mut self, event_data: Rc<RefCell<EventData>>) {
        let _ = self.cr_undo_and_invert_event(event_data, vec![], false);
    }

    pub(crate) fn acr_tree_restore_cells(
        index: &mut Index,
        restore_from: &mut HashMap<Rc<str>, TreeChange>,
        record_to: Option<&mut EventData>,
    ) -> Result<(), String> {
        let mut temp = None;
        let target: &mut EventData =
            record_to.unwrap_or_else(|| temp.get_or_insert_with(EventData::default));
        if let Err(e) = (|| -> Result<(), String> {
            for (iid, tree_change) in restore_from.drain() {
                let rn = Self::acr_pos(&index.iid_to_row, &iid)?;
                if let Some(change) = tree_change.par {
                    let inv = change.invert(&mut index.cells, &mut index.iid_to_row, rn)?;
                    target.add_tree_change(&iid, inv);
                }
                if let Some(change) = tree_change.chn {
                    let inv = change.invert(&mut index.cells, &mut index.iid_to_row, rn)?;
                    target.add_tree_change(&iid, inv);
                }
            }
            Ok(())
        })() {
            // Roll back in the event of an error
            for (iid, tree_change) in target.tree_changes.drain() {
                if let Ok(rn) = Self::acr_pos(&index.iid_to_row, &iid) {
                    if let Some(change) = tree_change.par {
                        let _ = change.invert(&mut index.cells, &mut index.iid_to_row, rn);
                    }
                    if let Some(change) = tree_change.chn {
                        let _ = change.invert(&mut index.cells, &mut index.iid_to_row, rn);
                    }
                }
            }
            return Err(e);
        }
        Ok(())
    }
}
