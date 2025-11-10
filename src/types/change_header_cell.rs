use crate::CellBorders;
use crate::CellProps;
use crate::ChangeCellProps;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::HeaderCell;
use crate::Highlight;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;
use crate::str_err;
use std::collections::HashMap;
use std::rc::Rc;

// Enum for changes to HeaderCell fields. Each variant stores the *old* value,
// since the new value is already in the Table. Inversion will fetch current (new) as old for the inverse.
// Simpler than IndexCell: No par, chn, open, or vals map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeHeaderCell {
    SetVal(Val),               // Old val
    SetDim(u64),               // Old dim
    SetVis(bool),              // Old vis
    SetIid(Rc<str>),           // Old iid
    SetProps(ChangeCellProps), // Imported sub-enum for props changes
}

// Inversion function: Takes a mutable Table (to apply old and fetch current), the change (with old value),
// applies the old value, fetches the current value (which becomes old for inverse),
// and returns the inverted ChangeHeaderCell.
impl ChangeHeaderCell {
    pub fn invert(
        &self,
        mut cells: &mut Vec<HeaderCell>,
        iid_to_col: &mut HashMap<Rc<str>, usize>,
        col: usize,
    ) -> Result<ChangeHeaderCell, String> {
        match self {
            ChangeHeaderCell::SetVal(old_val) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    // Assuming header is HashMap<usize, HeaderCell> or similar
                    let current_val = std::mem::replace(&mut cell.val, old_val.clone());
                    Ok(ChangeHeaderCell::SetVal(current_val))
                } else {
                    Err(str_err!(format!("Column not found for inversion: {}", col)))
                }
            }
            ChangeHeaderCell::SetDim(old_dim) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    let current_dim = std::mem::replace(&mut cell.dim, *old_dim);
                    Ok(ChangeHeaderCell::SetDim(current_dim))
                } else {
                    Err(str_err!(format!("Column not found for inversion: {}", col)))
                }
            }
            ChangeHeaderCell::SetVis(old_vis) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    let current_vis = std::mem::replace(&mut cell.vis, *old_vis);
                    Ok(ChangeHeaderCell::SetVis(current_vis))
                } else {
                    Err(str_err!(format!("Column not found for inversion: {}", col)))
                }
            }
            ChangeHeaderCell::SetIid(old_iid) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    let current_iid = std::mem::replace(&mut cell.iid, old_iid.clone());
                    let removed_pos = iid_to_col
                        .remove(&current_iid)
                        .ok_or(str_err!(format!("iid not found: {}", current_iid)))?;
                    iid_to_col.insert(old_iid.clone(), removed_pos);
                    Ok(ChangeHeaderCell::SetIid(current_iid))
                } else {
                    Err(str_err!(format!("Column not found for inversion: {}", col)))
                }
            }
            ChangeHeaderCell::SetProps(change) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    if let Some(props) = cell.props.as_mut() {
                        let inverted = change.invert(props);
                        Ok(ChangeHeaderCell::SetProps(inverted))
                    } else {
                        Err(str_err!(format!(
                            "No props for inversion in column: {}",
                            col
                        )))
                    }
                } else {
                    Err(str_err!(format!("Column not found for inversion: {}", col)))
                }
            }
        }
    }
}

// Recording functions: These apply a new value to the HeaderCell and return a ChangeHeaderCell
// encapsulating the old value. This is for the recording phase when making changes - apply the new,
// capture old in the change for the undo stack.
impl HeaderCell {
    pub fn record_set_iid(
        &mut self,
        new_iid: Rc<str>,
        iid_to_col: &mut HashMap<Rc<str>, usize>,
    ) -> Result<ChangeHeaderCell, String> {
        let current_iid = std::mem::replace(&mut self.iid, new_iid.clone());
        let removed_pos = iid_to_col
            .remove(&current_iid)
            .ok_or(str_err!(format!("iid not found: {}", current_iid)))?;
        iid_to_col.insert(new_iid, removed_pos);
        Ok(ChangeHeaderCell::SetIid(current_iid))
    }
    pub fn record_set_val(&mut self, new_val: Val) -> ChangeHeaderCell {
        ChangeHeaderCell::SetVal(std::mem::replace(&mut self.val, new_val))
    }
    pub fn record_set_dim(&mut self, new_dim: u64) -> ChangeHeaderCell {
        ChangeHeaderCell::SetDim(std::mem::replace(&mut self.dim, new_dim))
    }
    pub fn record_set_vis(&mut self, new_vis: bool) -> ChangeHeaderCell {
        ChangeHeaderCell::SetVis(std::mem::replace(&mut self.vis, new_vis))
    }
    // HeaderCell CellProps
    pub fn record_set_props_align(&mut self, new_align: Option<String>) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_align = std::mem::replace(&mut props.align, new_align);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetAlign(old_align))
    }
    pub fn record_set_props_borders(
        &mut self,
        new_borders: Option<CellBorders>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_borders = std::mem::replace(&mut props.borders, new_borders);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetBorders(old_borders))
    }
    pub fn record_set_props_checkbox(
        &mut self,
        new_checkbox: Option<Checkbox>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_checkbox = std::mem::replace(&mut props.checkbox, new_checkbox);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetCheckbox(old_checkbox))
    }
    pub fn record_set_props_data_format(
        &mut self,
        new_data_format: Option<DataFormat>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_data_format = std::mem::replace(&mut props.data_format, new_data_format);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetDataFormat(old_data_format))
    }
    pub fn record_set_props_dropdown(
        &mut self,
        new_dropdown: Option<Dropdown>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_dropdown = std::mem::replace(&mut props.dropdown, new_dropdown);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetDropdown(old_dropdown))
    }
    pub fn record_set_props_highlight(
        &mut self,
        new_highlight: Option<Highlight>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_highlight = std::mem::replace(&mut props.highlight, new_highlight);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetHighlight(old_highlight))
    }
    pub fn record_set_props_note(&mut self, new_note: Option<Note>) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_note = std::mem::replace(&mut props.note, new_note);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetNote(old_note))
    }
    pub fn record_set_props_progress_bar(
        &mut self,
        new_progress_bar: Option<ProgressBar>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_progress_bar = std::mem::replace(&mut props.progress_bar, new_progress_bar);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetProgressBar(old_progress_bar))
    }
    pub fn record_set_props_readonly(&mut self, new_readonly: bool) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_readonly = std::mem::replace(&mut props.readonly, new_readonly);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetReadonly(old_readonly))
    }
}
