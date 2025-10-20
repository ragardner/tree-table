use crate::ChangeHeaderCell;
use crate::ChangeIndexCell;
use crate::EventData;
use crate::Table;
use crate::ValidationMessage;
use crate::str_err;

impl Table {
    pub(crate) fn cr_bulk_edit_validation(
        &self,
        validation_message: Option<ValidationMessage>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        if let Some(callback) = &self.events.borrow().bound_validation_fn
            && let Some(message) = validation_message
        {
            let response: ValidationMessage = callback(message)?;
            let num_rows = self.cr_total_rows();
            let num_cols = self.cr_total_cols();
            // Header
            let mut cells = &mut self.grid.borrow_mut().header.cells;
            let mut iid_to_col = &mut self.grid.borrow_mut().header.iid_to_col;
            for (pos, changes) in response.header_data.into_iter() {
                let pos = pos as usize;
                if pos < num_cols {
                    let mut val_changes = Vec::new();
                    let mut new_changes = Vec::new();
                    for change in changes {
                        match &change {
                            ChangeHeaderCell::SetVal(_) => val_changes.push(change),
                            _ => {
                                new_changes.push(change.invert(&mut cells, &mut iid_to_col, pos)?)
                            }
                        }
                    }
                    for change in val_changes {
                        match &change {
                            ChangeHeaderCell::SetVal(val) => {
                                if Self::acr_val_valid_h(cells, &pos, val, false) {
                                    new_changes.push(change.invert(
                                        &mut cells,
                                        &mut iid_to_col,
                                        pos,
                                    )?);
                                }
                            }
                            _ => return Err(str_err!("Unexpected change in val_changes")),
                        }
                    }
                    if !new_changes.is_empty() {
                        event_data.header_cells.insert(pos, new_changes);
                    }
                }
            }
            // Index
            let mut cells = &mut self.grid.borrow_mut().index.cells;
            let mut iid_to_row = &mut self.grid.borrow_mut().index.iid_to_row;
            for (pos, changes) in response.index_data.into_iter() {
                let pos = pos as usize;
                if pos < num_rows {
                    let mut new_changes = Vec::new();
                    let mut val_changes = Vec::new();
                    for change in changes {
                        match &change {
                            ChangeIndexCell::SetVal(_) => val_changes.push(change),
                            _ => {
                                new_changes.push(change.invert(&mut cells, &mut iid_to_row, pos)?)
                            }
                        }
                    }
                    for change in val_changes {
                        match &change {
                            ChangeIndexCell::SetVal(val) => {
                                if Self::acr_val_valid_i(cells, &pos, val, false) {
                                    new_changes.push(change.invert(
                                        &mut cells,
                                        &mut iid_to_row,
                                        pos,
                                    )?);
                                }
                            }
                            _ => return Err(str_err!("Unexpected change in val_changes")),
                        }
                    }
                    if !new_changes.is_empty() {
                        event_data.index_cells.insert(pos, new_changes);
                    }
                }
            }
            // Table
            let mut cells = &mut self.grid.borrow_mut().index.cells;
            let mut iid_to_row = &mut self.grid.borrow_mut().index.iid_to_row;
            for (pos, changes) in response.table_data.into_iter() {
                let pos = pos as usize;
                if pos < num_rows {
                    let mut new_changes = Vec::new();
                    let mut val_changes = Vec::new();
                    for change in changes {
                        match &change {
                            ChangeIndexCell::SetTableCellVal { col, .. } => {
                                if col < &num_cols {
                                    val_changes.push(change)
                                }
                            }
                            ChangeIndexCell::SetTableCellProps { col, .. } => {
                                if col < &num_cols {
                                    new_changes.push(change.invert(
                                        &mut cells,
                                        &mut iid_to_row,
                                        pos,
                                    )?)
                                }
                            }
                            _ => {
                                new_changes.push(change.invert(&mut cells, &mut iid_to_row, pos)?)
                            }
                        }
                    }
                    for change in val_changes {
                        match &change {
                            ChangeIndexCell::SetTableCellVal { col, val } => {
                                if Self::acr_val_valid_t(cells, &pos, col, val, false) {
                                    new_changes.push(change.invert(
                                        &mut cells,
                                        &mut iid_to_row,
                                        pos,
                                    )?);
                                }
                            }
                            _ => return Err(str_err!("Unexpected change in val_changes")),
                        }
                    }
                    if !new_changes.is_empty() {
                        event_data.table_cells.insert(pos, new_changes);
                    }
                }
            }
        }
        Ok(())
    }
}
