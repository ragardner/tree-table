use crate::Change;
use crate::EventData;
use crate::Table;
use crate::args::UndoRedoArgs;
use crate::str_err;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_get_next_undo(&mut self) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        if self.grid.undo_stack.is_empty() {
            return Ok(None);
        }
        let change = self
            .grid
            .undo_stack
            .last()
            .ok_or(str_err!("Undo stack unexpectedly empty"))?
            .clone();
        Ok(Some(change))
    }

    pub(crate) fn cr_get_next_redo(&mut self) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        if self.grid.redo_stack.is_empty() {
            return Ok(None);
        }
        let change = self
            .grid
            .redo_stack
            .last()
            .ok_or(str_err!("Redo stack unexpectedly empty"))?
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
            .ok_or(str_err!("Undo stack unexpectedly empty"))?;
        let event_data = self.cr_undo_and_invert_event(change, kwargs.invalid_indices, false)?;
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
            .ok_or(str_err!("Redo stack unexpectedly empty"))?;
        let event_data = self.cr_undo_and_invert_event(change, kwargs.invalid_indices, false)?;
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
        recursive: bool,
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

        // paste -> add rows -> set data
        // undo -> rev -> set data -> del rows
        // redo -> rev -> add rows -> set data
        // undoing selection box changes are fine because they are saved and not mutated based on cmds...
        // but perhaps we should mutate the selection boxes in the cmds, put it all in there
        // we would still have selection box changes in event data but effectively they'd be limited to
        // only when directly changing selection boxes
        // how possible is this
        // TODO

        let mut event_data = self.cr_new_event(false);
        let mut to_undo_mut = to_undo.borrow_mut();
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
                    let (old, old_to_new, new_to_old) = prepare_move_mappings(new_to_old);
                    self.cr_move_rows(
                        &mut event_data,
                        old,
                        old_to_new,
                        new_to_old,
                        Some(&mut tree_changes),
                        None,
                        true,
                    )
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
                    self.cr_add_rows(removed_rows, sorted_seq, &mut event_data, true, true)
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
                Change::SaveSelection {
                    old_selection_boxes,
                    old_selected,
                } => {
                    let change = Change::UpdateSelection {
                        old_selection_boxes: self.sel.selection_boxes.clone(),
                        old_selected: self.sel.selected.clone(),
                        new_selection_boxes: old_selection_boxes.clone(),
                        new_selected: old_selected.clone(),
                    };
                    self.sel.selection_boxes = old_selection_boxes;
                    self.sel.selected = old_selected;
                    event_data.changes.push(change);
                    Ok(())
                }
                Change::UpdateSelection {
                    old_selection_boxes,
                    old_selected,
                    new_selection_boxes,
                    new_selected,
                } => {
                    let change = Change::UpdateSelection {
                        old_selection_boxes: new_selection_boxes,
                        old_selected: new_selected,
                        new_selection_boxes: old_selection_boxes.clone(),
                        new_selected: old_selected.clone(),
                    };
                    self.sel.selection_boxes = old_selection_boxes;
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
            if !recursive {
                let roll_back = Rc::new(RefCell::new(event_data));
                let _ = self.cr_undo_and_invert_event(roll_back, vec![], true);
            }
            return Err(e);
        }

        Ok(event_data)
    }
}
