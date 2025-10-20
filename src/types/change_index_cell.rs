use crate::CellBorders;
use crate::CellProps;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::str_err;
use std::collections::HashMap;

// Enum for changes to IndexCell fields. Each variant stores the *old* value,
// since the new value is already in the Table. Inversion will fetch current (new) as old for the inverse.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeIndexCell {
    SetVal(Val),                                               // Old val
    SetDim(u64),                                               // Old dim
    SetVis(bool),                                              // Old vis
    SetIid(String),                                            // Old iid (for renames, etc.)
    SetPar(String),                                            // Old par
    SetChn(Vec<String>),       // Old chn (clone is fine assuming small)
    SetOpen(bool),             // Old open
    SetProps(ChangeCellProps), // Sub-enum for props changes
    SetTableCellVal { col: usize, val: Val }, // For vals map: Old val per col
    SetTableCellProps { col: usize, change: ChangeCellProps }, // For table cell props changes
}

// Sub-enum for CellProps changes.
// storing old values.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeCellProps {
    SetAlign(Option<String>),            // Old align
    SetBorders(Option<CellBorders>),     // Old borders
    SetCheckbox(Option<Checkbox>),       // Old checkbox
    SetDataFormat(Option<DataFormat>),   // Old data_format
    SetDropdown(Option<Dropdown>),       // Old dropdown
    SetHighlight(Option<Highlight>),     // Old highlight
    SetNote(Option<Note>),               // Old note
    SetProgressBar(Option<ProgressBar>), // Old progress_bar
    SetReadonly(bool),                   // Old readonly
}

// Inversion function: Takes a mutable Table (to apply old and fetch current), the change (with old value),
// applies the old value (performs the undo), fetches the current value (which becomes old for inverse),
// and returns the inverted ChangeIndexCell (or ChangeCellProps for props).
impl ChangeIndexCell {
    pub fn invert(
        &self,
        mut cells: &mut Vec<IndexCell>,
        iid_to_row: &mut HashMap<String, usize>,
        row: usize,
    ) -> Result<ChangeIndexCell, String> {
        match self {
            ChangeIndexCell::SetVal(old_val) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_val = std::mem::replace(&mut cell.val, old_val.clone());
                    Ok(ChangeIndexCell::SetVal(current_val))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetDim(old_dim) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_dim = std::mem::replace(&mut cell.dim, *old_dim);
                    Ok(ChangeIndexCell::SetDim(current_dim))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetVis(old_vis) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_vis = std::mem::replace(&mut cell.vis, *old_vis);
                    Ok(ChangeIndexCell::SetVis(current_vis))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetIid(old_iid) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_iid = std::mem::replace(&mut cell.iid, old_iid.clone());
                    let removed_pos = iid_to_row
                        .remove(&current_iid)
                        .ok_or(str_err!(format!("iid not found: {}", current_iid)))?;
                    iid_to_row.insert(old_iid.clone(), removed_pos);
                    Ok(ChangeIndexCell::SetIid(current_iid))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetPar(old_par) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_par = std::mem::replace(&mut cell.par, old_par.clone());
                    Ok(ChangeIndexCell::SetPar(current_par))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetChn(old_chn) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_chn = std::mem::replace(&mut cell.chn, old_chn.clone());
                    Ok(ChangeIndexCell::SetChn(current_chn))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetOpen(old_open) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    let current_open = std::mem::replace(&mut cell.open, *old_open);
                    Ok(ChangeIndexCell::SetOpen(current_open))
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetProps(change) => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    if let Some(props) = cell.props.as_mut() {
                        let inverted = change.invert(props);
                        Ok(ChangeIndexCell::SetProps(inverted))
                    } else {
                        Err(str_err!(format!("No props for inversion in row: {}", row)))
                    }
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetTableCellVal { col, val: old_val } => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    if let Some(table_cell) = cell.vals.get_mut(col) {
                        // Assuming vals: HashMap<usize, TableCell>
                        let current_val = std::mem::replace(&mut table_cell.val, old_val.clone());
                        Ok(ChangeIndexCell::SetTableCellVal {
                            col: *col,
                            val: current_val,
                        })
                    } else {
                        Err(str_err!(format!(
                            "Table cell not found for col {} in row: {}",
                            col, row
                        )))
                    }
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
            ChangeIndexCell::SetTableCellProps { col, change } => {
                if let Ok(cell) = Table::acr_icell_mut(&mut cells, &row) {
                    if let Some(table_cell) = cell.vals.get_mut(col) {
                        if let Some(props) = table_cell.props.as_mut() {
                            let inverted = change.invert(props);
                            Ok(ChangeIndexCell::SetTableCellProps {
                                col: *col,
                                change: inverted,
                            })
                        } else {
                            Err(str_err!(format!(
                                "No props for table cell col {} in row: {}",
                                col, row
                            )))
                        }
                    } else {
                        Err(str_err!(format!(
                            "Table cell not found for col {} in row: {}",
                            col, row
                        )))
                    }
                } else {
                    Err(str_err!(format!("Row not found for inversion: {}", row)))
                }
            }
        }
    }
}

// Similar inversion for ChangeCellProps: Applies old, returns inverted with current as old.
impl ChangeCellProps {
    pub fn invert(&self, props: &mut CellProps) -> ChangeCellProps {
        match self {
            ChangeCellProps::SetAlign(old_align) => {
                let current_align = std::mem::replace(&mut props.align, old_align.clone());
                ChangeCellProps::SetAlign(current_align)
            }
            ChangeCellProps::SetBorders(old_borders) => {
                let current_borders = std::mem::replace(&mut props.borders, old_borders.clone());
                ChangeCellProps::SetBorders(current_borders)
            }
            ChangeCellProps::SetCheckbox(old_checkbox) => {
                let current_checkbox = std::mem::replace(&mut props.checkbox, old_checkbox.clone());
                ChangeCellProps::SetCheckbox(current_checkbox)
            }
            ChangeCellProps::SetDataFormat(old_data_format) => {
                let current_data_format =
                    std::mem::replace(&mut props.data_format, old_data_format.clone());
                ChangeCellProps::SetDataFormat(current_data_format)
            }
            ChangeCellProps::SetDropdown(old_dropdown) => {
                let current_dropdown = std::mem::replace(&mut props.dropdown, old_dropdown.clone());
                ChangeCellProps::SetDropdown(current_dropdown)
            }
            ChangeCellProps::SetHighlight(old_highlight) => {
                let current_highlight =
                    std::mem::replace(&mut props.highlight, old_highlight.clone());
                ChangeCellProps::SetHighlight(current_highlight)
            }
            ChangeCellProps::SetNote(old_note) => {
                let current_note = std::mem::replace(&mut props.note, old_note.clone());
                ChangeCellProps::SetNote(current_note)
            }
            ChangeCellProps::SetProgressBar(old_progress_bar) => {
                let current_progress_bar =
                    std::mem::replace(&mut props.progress_bar, old_progress_bar.clone());
                ChangeCellProps::SetProgressBar(current_progress_bar)
            }
            ChangeCellProps::SetReadonly(old_readonly) => {
                let current_readonly = std::mem::replace(&mut props.readonly, *old_readonly);
                ChangeCellProps::SetReadonly(current_readonly)
            }
        }
    }
}

// Recording functions: These apply a new value (or change) to the IndexCell and return a ChangeIndexCell
// encapsulating the old value. This is for the recording phase when making changes - apply the new,
// capture old in the change for the undo stack.
// The difference with invert is that these do not require a ChangeIndexCell to begin with.
impl IndexCell {
    pub fn record_set_iid(
        &mut self,
        new_iid: String,
        iid_to_row: &mut HashMap<String, usize>,
    ) -> Result<ChangeIndexCell, String> {
        let current_iid = std::mem::replace(&mut self.iid, new_iid.clone());
        let removed_pos = iid_to_row
            .remove(&current_iid)
            .ok_or(str_err!(format!("iid not found: {}", current_iid)))?;
        iid_to_row.insert(new_iid, removed_pos);
        Ok(ChangeIndexCell::SetIid(current_iid))
    }
    pub fn record_set_val(&mut self, new_val: Val) -> ChangeIndexCell {
        ChangeIndexCell::SetVal(std::mem::replace(&mut self.val, new_val))
    }
    pub fn record_set_dim(&mut self, new_dim: u64) -> ChangeIndexCell {
        ChangeIndexCell::SetDim(std::mem::replace(&mut self.dim, new_dim))
    }
    pub fn record_set_vis(&mut self, new_vis: bool) -> ChangeIndexCell {
        ChangeIndexCell::SetVis(std::mem::replace(&mut self.vis, new_vis))
    }
    pub fn record_set_par(&mut self, new_par: String) -> ChangeIndexCell {
        ChangeIndexCell::SetPar(std::mem::replace(&mut self.par, new_par))
    }
    pub fn record_set_chn(&mut self, new_chn: Vec<String>) -> ChangeIndexCell {
        ChangeIndexCell::SetChn(std::mem::replace(&mut self.chn, new_chn))
    }
    pub fn record_set_open(&mut self, new_open: bool) -> ChangeIndexCell {
        ChangeIndexCell::SetOpen(std::mem::replace(&mut self.open, new_open))
    }
    // IndexCell CellProps
    pub fn record_set_props_align(&mut self, new_align: Option<String>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_align = std::mem::replace(&mut props.align, new_align);
        ChangeIndexCell::SetProps(ChangeCellProps::SetAlign(old_align))
    }
    pub fn record_set_props_borders(
        &mut self,
        new_borders: Option<CellBorders>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_borders = std::mem::replace(&mut props.borders, new_borders);
        ChangeIndexCell::SetProps(ChangeCellProps::SetBorders(old_borders))
    }
    pub fn record_set_props_checkbox(&mut self, new_checkbox: Option<Checkbox>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_checkbox = std::mem::replace(&mut props.checkbox, new_checkbox);
        ChangeIndexCell::SetProps(ChangeCellProps::SetCheckbox(old_checkbox))
    }
    pub fn record_set_props_data_format(
        &mut self,
        new_data_format: Option<DataFormat>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_data_format = std::mem::replace(&mut props.data_format, new_data_format);
        ChangeIndexCell::SetProps(ChangeCellProps::SetDataFormat(old_data_format))
    }
    pub fn record_set_props_dropdown(&mut self, new_dropdown: Option<Dropdown>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_dropdown = std::mem::replace(&mut props.dropdown, new_dropdown);
        ChangeIndexCell::SetProps(ChangeCellProps::SetDropdown(old_dropdown))
    }
    pub fn record_set_props_highlight(
        &mut self,
        new_highlight: Option<Highlight>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_highlight = std::mem::replace(&mut props.highlight, new_highlight);
        ChangeIndexCell::SetProps(ChangeCellProps::SetHighlight(old_highlight))
    }
    pub fn record_set_props_note(&mut self, new_note: Option<Note>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_note = std::mem::replace(&mut props.note, new_note);
        ChangeIndexCell::SetProps(ChangeCellProps::SetNote(old_note))
    }
    pub fn record_set_props_progress_bar(
        &mut self,
        new_progress_bar: Option<ProgressBar>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_progress_bar = std::mem::replace(&mut props.progress_bar, new_progress_bar);
        ChangeIndexCell::SetProps(ChangeCellProps::SetProgressBar(old_progress_bar))
    }
    pub fn record_set_props_readonly(&mut self, new_readonly: bool) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_readonly = std::mem::replace(&mut props.readonly, new_readonly);
        ChangeIndexCell::SetProps(ChangeCellProps::SetReadonly(old_readonly))
    }
    // TableCell
    pub fn record_set_table_cell_val(
        &mut self,
        col: usize,
        new_val: Val,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let old_val = std::mem::replace(&mut table_cell.val, new_val);
        ChangeIndexCell::SetTableCellVal { col, val: old_val }
    }
    // TableCell CellProps
    pub fn record_set_table_cell_props_align(
        &mut self,
        col: usize,
        new_align: Option<String>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_align = std::mem::replace(&mut props.align, new_align);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetAlign(old_align),
        }
    }
    pub fn record_set_table_cell_props_borders(
        &mut self,
        col: usize,
        new_borders: Option<CellBorders>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_borders = std::mem::replace(&mut props.borders, new_borders);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetBorders(old_borders),
        }
    }
    pub fn record_set_table_cell_props_checkbox(
        &mut self,
        col: usize,
        new_checkbox: Option<Checkbox>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_checkbox = std::mem::replace(&mut props.checkbox, new_checkbox);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetCheckbox(old_checkbox),
        }
    }
    pub fn record_set_table_cell_props_data_format(
        &mut self,
        col: usize,
        new_data_format: Option<DataFormat>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_data_format = std::mem::replace(&mut props.data_format, new_data_format);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetDataFormat(old_data_format),
        }
    }
    pub fn record_set_table_cell_props_dropdown(
        &mut self,
        col: usize,
        new_dropdown: Option<Dropdown>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_dropdown = std::mem::replace(&mut props.dropdown, new_dropdown);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetDropdown(old_dropdown),
        }
    }
    pub fn record_set_table_cell_props_highlight(
        &mut self,
        col: usize,
        new_highlight: Option<Highlight>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_highlight = std::mem::replace(&mut props.highlight, new_highlight);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetHighlight(old_highlight),
        }
    }
    pub fn record_set_table_cell_props_note(
        &mut self,
        col: usize,
        new_note: Option<Note>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_note = std::mem::replace(&mut props.note, new_note);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetNote(old_note),
        }
    }
    pub fn record_set_table_cell_props_progress_bar(
        &mut self,
        col: usize,
        new_progress_bar: Option<ProgressBar>,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_progress_bar = std::mem::replace(&mut props.progress_bar, new_progress_bar);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetProgressBar(old_progress_bar),
        }
    }
    pub fn record_set_table_cell_props_readonly(
        &mut self,
        col: usize,
        new_readonly: bool,
        default_val: Val,
    ) -> ChangeIndexCell {
        let vals = &mut self.vals;
        let table_cell = vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.get_or_insert_with(CellProps::default);
        let old_readonly = std::mem::replace(&mut props.readonly, new_readonly);
        ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::SetReadonly(old_readonly),
        }
    }
}
