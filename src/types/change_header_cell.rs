use crate::Border;
use crate::CellBorders;
use crate::CellProps;
use crate::ChangeCellProps;
use crate::ChangeCheckbox;
use crate::ChangeDropdown;
use crate::ChangeNote;
use crate::ChangeProgressBar;
use crate::Checkbox;
use crate::Color3;
use crate::DataFormat;
use crate::Dropdown;
use crate::DropdownState;
use crate::HeaderCell;
use crate::Highlight;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;
use crate::str_err;
use crate::types::change_index_cell::ChangeBorders;
use std::collections::HashMap;
use std::rc::Rc;

// Enum for changes to HeaderCell fields. Each variant stores the *old* value,
// since the new value is already in the Table. Inversion will fetch current (new) as old for the inverse.
// Simpler than IndexCell: No par, chn, open, or vals map.
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
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
                    Err(str_err!("Column not found for inversion: {}", col))
                }
            }
            ChangeHeaderCell::SetDim(old_dim) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    let current_dim = std::mem::replace(&mut cell.dim, *old_dim);
                    Ok(ChangeHeaderCell::SetDim(current_dim))
                } else {
                    Err(str_err!("Column not found for inversion: {}", col))
                }
            }
            ChangeHeaderCell::SetVis(old_vis) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    let current_vis = std::mem::replace(&mut cell.vis, *old_vis);
                    Ok(ChangeHeaderCell::SetVis(current_vis))
                } else {
                    Err(str_err!("Column not found for inversion: {}", col))
                }
            }
            ChangeHeaderCell::SetIid(old_iid) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    let current_iid = std::mem::replace(&mut cell.iid, old_iid.clone());
                    let removed_pos = iid_to_col
                        .remove(&current_iid)
                        .ok_or(str_err!("iid not found: {}", current_iid))?;
                    iid_to_col.insert(old_iid.clone(), removed_pos);
                    Ok(ChangeHeaderCell::SetIid(current_iid))
                } else {
                    Err(str_err!("Column not found for inversion: {}", col))
                }
            }
            ChangeHeaderCell::SetProps(change) => {
                if let Ok(cell) = Table::acr_hcell_mut(&mut cells, col) {
                    if let Some(props) = cell.props.as_mut() {
                        let inverted = change.invert(props)?;
                        Ok(ChangeHeaderCell::SetProps(inverted))
                    } else {
                        Err(str_err!("No props for inversion in column: {}", col))
                    }
                } else {
                    Err(str_err!("Column not found for inversion: {}", col))
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
            .ok_or(str_err!("iid not found: {}", current_iid))?;
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

    // ───── HEADERCELL CELLPROPS ─────

    // ───── ALIGN ─────

    pub fn record_set_props_align(&mut self, new_align: Option<String>) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_align = std::mem::replace(&mut props.align, new_align);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetAlign(old_align))
    }

    // ───── BORDERS ─────

    pub fn record_set_props_borders(
        &mut self,
        new_borders: Option<CellBorders>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_borders = std::mem::replace(&mut props.borders, new_borders);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetBorders(old_borders))
    }
    pub fn record_set_props_borders_top(
        &mut self,
        new_top: Option<Border>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or(str_err!("No CellBorders exists on col '{}'", self.iid))?;
        let old_top = std::mem::replace(&mut borders.top, new_top);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetTop(old_top),
        )))
    }
    pub fn record_set_props_borders_right(
        &mut self,
        new_right: Option<Border>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or(str_err!("No CellBorders exists on col '{}'", self.iid))?;
        let old_right = std::mem::replace(&mut borders.right, new_right);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetRight(old_right),
        )))
    }
    pub fn record_set_props_borders_bottom(
        &mut self,
        new_bottom: Option<Border>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or(str_err!("No CellBorders exists on col '{}'", self.iid))?;
        let old_bottom = std::mem::replace(&mut borders.bottom, new_bottom);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetBottom(old_bottom),
        )))
    }
    pub fn record_set_props_borders_left(
        &mut self,
        new_left: Option<Border>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or(str_err!("No CellBorders exists on col '{}'", self.iid))?;
        let old_left = std::mem::replace(&mut borders.left, new_left);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetLeft(old_left),
        )))
    }

    // ───── CHECKBOX ─────

    pub fn record_set_props_checkbox(
        &mut self,
        new_checkbox: Option<Checkbox>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_checkbox = std::mem::replace(&mut props.checkbox, new_checkbox);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetCheckbox(old_checkbox))
    }
    pub fn record_set_props_checkbox_true_val(
        &mut self,
        new_true_val: Val,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or(str_err!("No Checkbox exists on col '{}'", self.iid))?;
        let old_true_val = std::mem::replace(&mut cb.true_val, new_true_val);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetTrueVal(old_true_val),
        )))
    }
    pub fn record_set_props_checkbox_false_val(
        &mut self,
        new_false_val: Val,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or(str_err!("No Checkbox exists on col '{}'", self.iid))?;
        let old_false_val = std::mem::replace(&mut cb.false_val, new_false_val);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetFalseVal(old_false_val),
        )))
    }
    pub fn record_set_props_checkbox_text(
        &mut self,
        new_text: Option<String>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or(str_err!("No Checkbox exists on col '{}'", self.iid))?;
        let old_text = std::mem::replace(&mut cb.text, new_text);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetText(old_text),
        )))
    }
    pub fn record_set_props_checkbox_state(
        &mut self,
        new_state: bool,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or(str_err!("No Checkbox exists on col '{}'", self.iid))?;
        let old_state = std::mem::replace(&mut cb.state, new_state);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetState(old_state),
        )))
    }

    // ───── DATA FORMAT ─────

    pub fn record_set_props_data_format(
        &mut self,
        new_data_format: Option<DataFormat>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_data_format = std::mem::replace(&mut props.data_format, new_data_format);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetDataFormat(old_data_format))
    }

    // ───── DROPDOWN ─────

    pub fn record_set_props_dropdown(
        &mut self,
        new_dropdown: Option<Dropdown>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_dropdown = std::mem::replace(&mut props.dropdown, new_dropdown);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetDropdown(old_dropdown))
    }
    pub fn record_set_props_dropdown_vals(
        &mut self,
        new_vals: Vec<Val>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or(str_err!("No Dropdown exists on col '{}'", self.iid))?;
        let old_vals = std::mem::replace(&mut dd.vals, new_vals);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetVals(old_vals),
        )))
    }
    pub fn record_set_props_dropdown_default_val(
        &mut self,
        new_default_val: Option<Val>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or(str_err!("No Dropdown exists on col '{}'", self.iid))?;
        let old_default_val = std::mem::replace(&mut dd.default_val, new_default_val);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetDefaultVal(old_default_val),
        )))
    }
    pub fn record_set_props_dropdown_validation(
        &mut self,
        new_validation: bool,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or(str_err!("No Dropdown exists on col '{}'", self.iid))?;
        let old_validation = std::mem::replace(&mut dd.validation, new_validation);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetValidation(old_validation),
        )))
    }
    pub fn record_set_props_dropdown_text(
        &mut self,
        new_text: Option<String>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or(str_err!("No Dropdown exists on col '{}'", self.iid))?;
        let old_text = std::mem::replace(&mut dd.text, new_text);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetText(old_text),
        )))
    }
    pub fn record_set_props_dropdown_state(
        &mut self,
        new_state: DropdownState,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or(str_err!("No Dropdown exists on col '{}'", self.iid))?;
        let old_state = std::mem::replace(&mut dd.state, new_state);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetState(old_state),
        )))
    }

    // ───── HIGHLIGHT ─────

    pub fn record_set_props_highlight(
        &mut self,
        new_highlight: Option<Highlight>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_highlight = std::mem::replace(&mut props.highlight, new_highlight);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetHighlight(old_highlight))
    }

    // ───── NOTE ─────

    pub fn record_set_props_note(&mut self, new_note: Option<Note>) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_note = std::mem::replace(&mut props.note, new_note);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetNote(old_note))
    }
    pub fn record_set_props_note_text(
        &mut self,
        new_text: String,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or(str_err!("No Note exists on col '{}'", self.iid))?;
        let old_text = std::mem::replace(&mut note.note, new_text);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetText(old_text),
        )))
    }
    pub fn record_set_props_note_bg(
        &mut self,
        new_bg: Option<Color3>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or(str_err!("No Note exists on col '{}'", self.iid))?;
        let old_bg = std::mem::replace(&mut note.bg, new_bg);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetBg(old_bg),
        )))
    }
    pub fn record_set_props_note_fg(
        &mut self,
        new_fg: Option<Color3>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or(str_err!("No Note exists on col '{}'", self.iid))?;
        let old_fg = std::mem::replace(&mut note.fg, new_fg);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetFg(old_fg),
        )))
    }
    pub fn record_set_props_note_readonly(
        &mut self,
        new_readonly: bool,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or(str_err!("No Note exists on col '{}'", self.iid))?;
        let old_readonly = std::mem::replace(&mut note.readonly, new_readonly);
        Ok(ChangeHeaderCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetReadonly(old_readonly),
        )))
    }

    // ───── PROGRESS BAR ─────

    pub fn record_set_props_progress_bar(
        &mut self,
        new_progress_bar: Option<ProgressBar>,
    ) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_progress_bar = std::mem::replace(&mut props.progress_bar, new_progress_bar);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetProgressBar(old_progress_bar))
    }
    pub fn record_set_props_progress_bar_name(
        &mut self,
        new_name: String,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or(str_err!("No ProgressBar exists on col '{}'", self.iid))?;
        let old_name = std::mem::replace(&mut pb.name, new_name);
        Ok(ChangeHeaderCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetName(old_name)),
        ))
    }
    pub fn record_set_props_progress_bar_bg(
        &mut self,
        new_bg: Option<Color3>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or(str_err!("No ProgressBar exists on col '{}'", self.iid))?;
        let old_bg = std::mem::replace(&mut pb.bg, new_bg);
        Ok(ChangeHeaderCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetBg(old_bg)),
        ))
    }
    pub fn record_set_props_progress_bar_fg(
        &mut self,
        new_fg: Option<Color3>,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or(str_err!("No ProgressBar exists on col '{}'", self.iid))?;
        let old_fg = std::mem::replace(&mut pb.fg, new_fg);
        Ok(ChangeHeaderCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetFg(old_fg)),
        ))
    }
    pub fn record_set_props_progress_bar_pc(
        &mut self,
        new_pc: f64,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or(str_err!("No ProgressBar exists on col '{}'", self.iid))?;
        let old_pc = std::mem::replace(&mut pb.pc, new_pc);
        Ok(ChangeHeaderCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetPc(old_pc)),
        ))
    }
    pub fn record_set_props_progress_bar_remove_on_complete(
        &mut self,
        new_flag: bool,
    ) -> Result<ChangeHeaderCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or(str_err!("No CellProps exist for col '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or(str_err!("No ProgressBar exists on col '{}'", self.iid))?;
        let old_flag = std::mem::replace(&mut pb.remove_on_complete, new_flag);
        Ok(ChangeHeaderCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetRemoveOnComplete(old_flag)),
        ))
    }

    // ───── READONLY ─────

    pub fn record_set_props_readonly(&mut self, new_readonly: bool) -> ChangeHeaderCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_readonly = std::mem::replace(&mut props.readonly, new_readonly);
        ChangeHeaderCell::SetProps(ChangeCellProps::SetReadonly(old_readonly))
    }
}
