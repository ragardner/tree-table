use crate::Border;
use crate::CellAlign;
use crate::CellBorders;
use crate::CellProps;
use crate::Checkbox;
use crate::Color3;
use crate::DataFormat;
use crate::Dropdown;
use crate::DropdownState;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;
use crate::str_err;
use std::collections::HashMap;
use std::rc::Rc;

// Enum for changes to IndexCell fields. Each variant stores the *old* value,
// since the new value is already in the Table. Inversion will fetch current (new) as old for the inverse.
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeIndexCell {
    SetVal(Val),                                               // Old val
    SetDim(u64),                                               // Old dim
    SetVis(bool),                                              // Old vis
    SetIid(Rc<str>),                                           // Old iid (for renames, etc.)
    SetPar(Rc<str>),                                           // Old par
    SetChn(Vec<Rc<str>>),      // Old chn (clone is fine assuming small)
    SetOpen(bool),             // Old open
    SetProps(ChangeCellProps), // Sub-enum for props changes
    SetTableCellVal { col: usize, val: Val }, // For vals map: Old val per col
    SetTableCellProps { col: usize, change: ChangeCellProps }, // For table cell props changes
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeBorders {
    SetTop(Option<Border>),
    SetRight(Option<Border>),
    SetBottom(Option<Border>),
    SetLeft(Option<Border>),
}

impl ChangeBorders {
    pub fn invert(&self, borders: &mut CellBorders) -> ChangeBorders {
        match self {
            ChangeBorders::SetTop(old) => {
                ChangeBorders::SetTop(std::mem::replace(&mut borders.top, old.clone()))
            }
            ChangeBorders::SetRight(old) => {
                ChangeBorders::SetRight(std::mem::replace(&mut borders.right, old.clone()))
            }
            ChangeBorders::SetBottom(old) => {
                ChangeBorders::SetBottom(std::mem::replace(&mut borders.bottom, old.clone()))
            }
            ChangeBorders::SetLeft(old) => {
                ChangeBorders::SetLeft(std::mem::replace(&mut borders.left, old.clone()))
            }
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeCheckbox {
    SetTrueVal(Val),
    SetFalseVal(Val),
    SetText(Option<String>),
    SetState(bool),
}

impl ChangeCheckbox {
    pub fn invert(&self, cb: &mut Checkbox) -> ChangeCheckbox {
        match self {
            ChangeCheckbox::SetTrueVal(old) => {
                ChangeCheckbox::SetTrueVal(std::mem::replace(&mut cb.true_val, old.clone()))
            }
            ChangeCheckbox::SetFalseVal(old) => {
                ChangeCheckbox::SetFalseVal(std::mem::replace(&mut cb.false_val, old.clone()))
            }
            ChangeCheckbox::SetText(old) => {
                ChangeCheckbox::SetText(std::mem::replace(&mut cb.text, old.clone()))
            }
            ChangeCheckbox::SetState(old) => {
                ChangeCheckbox::SetState(std::mem::replace(&mut cb.state, *old))
            }
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeDropdown {
    SetVals(Vec<Val>),
    SetDefaultVal(Option<Val>),
    SetValidation(bool),
    SetText(Option<String>),
    SetState(DropdownState),
}

impl ChangeDropdown {
    pub fn invert(&self, dd: &mut Dropdown) -> ChangeDropdown {
        match self {
            ChangeDropdown::SetVals(old) => {
                ChangeDropdown::SetVals(std::mem::replace(&mut dd.vals, old.clone()))
            }
            ChangeDropdown::SetDefaultVal(old) => {
                ChangeDropdown::SetDefaultVal(std::mem::replace(&mut dd.default_val, old.clone()))
            }
            ChangeDropdown::SetValidation(old) => {
                ChangeDropdown::SetValidation(std::mem::replace(&mut dd.validation, *old))
            }
            ChangeDropdown::SetText(old) => {
                ChangeDropdown::SetText(std::mem::replace(&mut dd.text, old.clone()))
            }
            ChangeDropdown::SetState(old) => {
                ChangeDropdown::SetState(std::mem::replace(&mut dd.state, old.clone()))
            }
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeProgressBar {
    SetBg(Option<Color3>),
    SetFg(Option<Color3>),
    SetPc(f64),
    SetRemoveOnComplete(bool),
}

impl ChangeProgressBar {
    pub fn invert(&self, pb: &mut ProgressBar) -> ChangeProgressBar {
        match self {
            ChangeProgressBar::SetBg(old) => {
                ChangeProgressBar::SetBg(std::mem::replace(&mut pb.bg, old.clone()))
            }
            ChangeProgressBar::SetFg(old) => {
                ChangeProgressBar::SetFg(std::mem::replace(&mut pb.fg, old.clone()))
            }
            ChangeProgressBar::SetPc(old) => {
                ChangeProgressBar::SetPc(std::mem::replace(&mut pb.pc, *old))
            }
            ChangeProgressBar::SetRemoveOnComplete(old) => ChangeProgressBar::SetRemoveOnComplete(
                std::mem::replace(&mut pb.remove_on_complete, *old),
            ),
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeNote {
    SetText(String),       // Old note text (the .note field)
    SetBg(Option<Color3>), // Old bg
    SetFg(Option<Color3>), // Old fg
    SetReadonly(bool),     // Old readonly
}

impl ChangeNote {
    pub fn invert(&self, note: &mut Note) -> ChangeNote {
        match self {
            ChangeNote::SetText(old) => {
                ChangeNote::SetText(std::mem::replace(&mut note.note, old.clone()))
            }
            ChangeNote::SetBg(old) => {
                ChangeNote::SetBg(std::mem::replace(&mut note.bg, old.clone()))
            }
            ChangeNote::SetFg(old) => {
                ChangeNote::SetFg(std::mem::replace(&mut note.fg, old.clone()))
            }
            ChangeNote::SetReadonly(old) => {
                ChangeNote::SetReadonly(std::mem::replace(&mut note.readonly, *old))
            }
        }
    }
}

// Sub-enum for CellProps changes.
// storing old values.
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeCellProps {
    SetAlign(Option<CellAlign>),
    SetBorders(Option<CellBorders>),
    UpdateBorders(ChangeBorders),
    SetCheckbox(Option<Checkbox>),
    UpdateCheckbox(ChangeCheckbox),
    SetDataFormat(Option<DataFormat>),
    SetDropdown(Option<Dropdown>),
    UpdateDropdown(ChangeDropdown),
    SetHighlight(Option<Highlight>),
    SetNote(Option<Note>),
    UpdateNote(ChangeNote),
    SetProgressBar(Option<ProgressBar>),
    UpdateProgressBar(ChangeProgressBar),
    SetReadonly(bool),
}

// Inversion function: Takes a mutable Table (to apply old and fetch current), the change (with old value),
// applies the old value (performs the undo), fetches the current value (which becomes old for inverse),
// and returns the inverted ChangeIndexCell (or ChangeCellProps for props).
impl ChangeIndexCell {
    pub fn invert(
        &self,
        cells: &mut Vec<IndexCell>,
        iid_to_row: &mut HashMap<Rc<str>, usize>,
        row: usize,
    ) -> Result<ChangeIndexCell, String> {
        match self {
            ChangeIndexCell::SetPar(old_par_iid) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_par_iid = std::mem::replace(&mut cell.par, old_par_iid.clone());
                Ok(ChangeIndexCell::SetPar(current_par_iid))
            }
            ChangeIndexCell::SetChn(old_chn_iids) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_chn_iids = std::mem::replace(&mut cell.chn, old_chn_iids.clone());
                Ok(ChangeIndexCell::SetChn(current_chn_iids))
            }
            ChangeIndexCell::SetVal(old_val) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_val = std::mem::replace(&mut cell.val, old_val.clone());
                Ok(ChangeIndexCell::SetVal(current_val))
            }
            ChangeIndexCell::SetDim(old_dim) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_dim = std::mem::replace(&mut cell.dim, *old_dim);
                Ok(ChangeIndexCell::SetDim(current_dim))
            }
            ChangeIndexCell::SetVis(old_vis) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_vis = std::mem::replace(&mut cell.vis, *old_vis);
                Ok(ChangeIndexCell::SetVis(current_vis))
            }
            ChangeIndexCell::SetIid(old_iid) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_iid = std::mem::replace(&mut cell.iid, old_iid.clone());
                let removed_pos = iid_to_row
                    .remove(&current_iid)
                    .ok_or_else(|| str_err!("iid not found: {}", current_iid))?;
                iid_to_row.insert(old_iid.clone(), removed_pos);
                Ok(ChangeIndexCell::SetIid(current_iid))
            }
            ChangeIndexCell::SetOpen(old_open) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                let current_open = std::mem::replace(&mut cell.open, *old_open);
                Ok(ChangeIndexCell::SetOpen(current_open))
            }
            ChangeIndexCell::SetProps(change) => {
                let cell = Table::acr_icell_mut(cells, row)?;
                if let Some(props) = cell.props.as_mut() {
                    let inverted = change.invert(props)?;
                    Ok(ChangeIndexCell::SetProps(inverted))
                } else {
                    Err(str_err!("No props for inversion in row: {}", row))
                }
            }
            ChangeIndexCell::SetTableCellVal { col, val: old_val } => {
                let cell = Table::acr_icell_mut(cells, row)?;
                if let Some(table_cell) = cell.vals.get_mut(col) {
                    let current_val = std::mem::replace(&mut table_cell.val, old_val.clone());
                    Ok(ChangeIndexCell::SetTableCellVal {
                        col: *col,
                        val: current_val,
                    })
                } else {
                    Err(str_err!(
                        "Table cell not found for col {} in row: {}",
                        col,
                        row
                    ))
                }
            }
            ChangeIndexCell::SetTableCellProps { col, change } => {
                let cell = Table::acr_icell_mut(cells, row)?;
                if let Some(table_cell) = cell.vals.get_mut(col) {
                    if let Some(props) = table_cell.props.as_mut() {
                        let inverted = change.invert(props)?;
                        Ok(ChangeIndexCell::SetTableCellProps {
                            col: *col,
                            change: inverted,
                        })
                    } else {
                        Err(str_err!(
                            "No props for table cell col {} in row: {}",
                            col,
                            row
                        ))
                    }
                } else {
                    Err(str_err!(
                        "Table cell not found for col {} in row: {}",
                        col,
                        row
                    ))
                }
            }
        }
    }
}

// Similar inversion for ChangeCellProps: Applies old, returns inverted with current as old.
impl ChangeCellProps {
    pub fn invert(&self, props: &mut CellProps) -> Result<ChangeCellProps, String> {
        match self {
            ChangeCellProps::SetAlign(old_align) => {
                let current_align = std::mem::replace(&mut props.align, old_align.clone());
                Ok(ChangeCellProps::SetAlign(current_align))
            }
            ChangeCellProps::SetBorders(old_borders) => {
                let current_borders = std::mem::replace(&mut props.borders, old_borders.clone());
                Ok(ChangeCellProps::SetBorders(current_borders))
            }
            ChangeCellProps::UpdateBorders(change) => {
                let borders = props
                    .borders
                    .as_mut()
                    .ok_or_else(|| str_err!("CellBorders missing during inversion"))?;
                let inverted = change.invert(borders);
                Ok(ChangeCellProps::UpdateBorders(inverted))
            }
            ChangeCellProps::SetCheckbox(old_checkbox) => {
                let current_checkbox = std::mem::replace(&mut props.checkbox, old_checkbox.clone());
                Ok(ChangeCellProps::SetCheckbox(current_checkbox))
            }
            ChangeCellProps::UpdateCheckbox(change) => {
                let checkbox = props
                    .checkbox
                    .as_mut()
                    .ok_or_else(|| str_err!("Checkbox missing during inversion"))?;
                let inverted = change.invert(checkbox);
                Ok(ChangeCellProps::UpdateCheckbox(inverted))
            }
            ChangeCellProps::SetDataFormat(old_data_format) => {
                let current_data_format =
                    std::mem::replace(&mut props.data_format, old_data_format.clone());
                Ok(ChangeCellProps::SetDataFormat(current_data_format))
            }
            ChangeCellProps::SetDropdown(old_dropdown) => {
                let current_dropdown = std::mem::replace(&mut props.dropdown, old_dropdown.clone());
                Ok(ChangeCellProps::SetDropdown(current_dropdown))
            }
            ChangeCellProps::UpdateDropdown(change) => {
                let dropdown = props
                    .dropdown
                    .as_mut()
                    .ok_or_else(|| str_err!("Dropdown missing during inversion"))?;
                let inverted = change.invert(dropdown);
                Ok(ChangeCellProps::UpdateDropdown(inverted))
            }
            ChangeCellProps::SetHighlight(old_highlight) => {
                let current_highlight =
                    std::mem::replace(&mut props.highlight, old_highlight.clone());
                Ok(ChangeCellProps::SetHighlight(current_highlight))
            }
            ChangeCellProps::SetNote(old_note) => {
                let current_note = std::mem::replace(&mut props.note, old_note.clone());
                Ok(ChangeCellProps::SetNote(current_note))
            }
            ChangeCellProps::UpdateNote(change) => {
                let note = props
                    .note
                    .as_mut()
                    .ok_or_else(|| str_err!("Note missing during inversion"))?;
                let inverted = change.invert(note);
                Ok(ChangeCellProps::UpdateNote(inverted))
            }
            ChangeCellProps::SetProgressBar(old_progress_bar) => {
                let current_progress_bar =
                    std::mem::replace(&mut props.progress_bar, old_progress_bar.clone());
                Ok(ChangeCellProps::SetProgressBar(current_progress_bar))
            }
            ChangeCellProps::UpdateProgressBar(change) => {
                let pb = props
                    .progress_bar
                    .as_mut()
                    .ok_or_else(|| str_err!("ProgressBar missing during inversion"))?;
                let inverted = change.invert(pb);
                Ok(ChangeCellProps::UpdateProgressBar(inverted))
            }
            ChangeCellProps::SetReadonly(old_readonly) => {
                let current_readonly = std::mem::replace(&mut props.readonly, *old_readonly);
                Ok(ChangeCellProps::SetReadonly(current_readonly))
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
        new_iid: Rc<str>,
        iid_to_row: &mut HashMap<Rc<str>, usize>,
    ) -> Result<ChangeIndexCell, String> {
        let current_iid = std::mem::replace(&mut self.iid, new_iid.clone());
        let removed_pos = iid_to_row
            .remove(&current_iid)
            .ok_or_else(|| str_err!("iid not found: {}", current_iid))?;
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
    pub fn record_set_par(&mut self, new_par: Rc<str>) -> Result<ChangeIndexCell, String> {
        let current_par_iid = std::mem::replace(&mut self.par, new_par);
        Ok(ChangeIndexCell::SetPar(current_par_iid))
    }
    pub fn record_set_chn(&mut self, new_chn: Vec<Rc<str>>) -> Result<ChangeIndexCell, String> {
        let current_chn_iids = std::mem::replace(&mut self.chn, new_chn);
        return Ok(ChangeIndexCell::SetChn(current_chn_iids));
    }
    pub fn record_set_open(&mut self, new_open: bool) -> ChangeIndexCell {
        ChangeIndexCell::SetOpen(std::mem::replace(&mut self.open, new_open))
    }

    // ───── INDEXCELL CELLPROPS ─────

    // ───── ALIGN ─────

    pub fn record_set_props_align(&mut self, new_align: Option<CellAlign>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_align = std::mem::replace(&mut props.align, new_align);
        ChangeIndexCell::SetProps(ChangeCellProps::SetAlign(old_align))
    }

    // ───── BORDERS ─────

    pub fn record_set_props_borders(
        &mut self,
        new_borders: Option<CellBorders>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_borders = std::mem::replace(&mut props.borders, new_borders);
        ChangeIndexCell::SetProps(ChangeCellProps::SetBorders(old_borders))
    }
    pub fn record_set_props_borders_top(
        &mut self,
        new_top: Option<Border>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or_else(|| str_err!("No CellBorders exists for row '{}'", self.iid))?;
        let old_top = std::mem::replace(&mut borders.top, new_top);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetTop(old_top),
        )))
    }
    pub fn record_set_props_borders_right(
        &mut self,
        new_right: Option<Border>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or_else(|| str_err!("No CellBorders exists for row '{}'", self.iid))?;
        let old_right = std::mem::replace(&mut borders.right, new_right);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetRight(old_right),
        )))
    }

    pub fn record_set_props_borders_bottom(
        &mut self,
        new_bottom: Option<Border>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or_else(|| str_err!("No CellBorders exists for row '{}'", self.iid))?;
        let old_bottom = std::mem::replace(&mut borders.bottom, new_bottom);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetBottom(old_bottom),
        )))
    }
    pub fn record_set_props_borders_left(
        &mut self,
        new_left: Option<Border>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let borders = props
            .borders
            .as_mut()
            .ok_or_else(|| str_err!("No CellBorders exists for row '{}'", self.iid))?;
        let old_left = std::mem::replace(&mut borders.left, new_left);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateBorders(
            ChangeBorders::SetLeft(old_left),
        )))
    }

    // ───── CHECKBOX ─────

    pub fn record_set_props_checkbox(&mut self, new_checkbox: Option<Checkbox>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_checkbox = std::mem::replace(&mut props.checkbox, new_checkbox);
        ChangeIndexCell::SetProps(ChangeCellProps::SetCheckbox(old_checkbox))
    }
    pub fn record_set_props_checkbox_true_val(
        &mut self,
        new_true_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or_else(|| str_err!("No Checkbox exists for row '{}'", self.iid))?;
        let old_true_val = std::mem::replace(&mut cb.true_val, new_true_val);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetTrueVal(old_true_val),
        )))
    }
    pub fn record_set_props_checkbox_false_val(
        &mut self,
        new_false_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or_else(|| str_err!("No Checkbox exists for row '{}'", self.iid))?;
        let old_false_val = std::mem::replace(&mut cb.false_val, new_false_val);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetFalseVal(old_false_val),
        )))
    }
    pub fn record_set_props_checkbox_text(
        &mut self,
        new_text: Option<String>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or_else(|| str_err!("No Checkbox exists for row '{}'", self.iid))?;
        let old_text = std::mem::replace(&mut cb.text, new_text);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetText(old_text),
        )))
    }
    pub fn record_set_props_checkbox_state(
        &mut self,
        new_state: bool,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let cb = props
            .checkbox
            .as_mut()
            .ok_or_else(|| str_err!("No Checkbox exists for row '{}'", self.iid))?;
        let old_state = std::mem::replace(&mut cb.state, new_state);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateCheckbox(
            ChangeCheckbox::SetState(old_state),
        )))
    }

    // ───── DATA FORMAT ─────

    pub fn record_set_props_data_format(
        &mut self,
        new_data_format: Option<DataFormat>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_data_format = std::mem::replace(&mut props.data_format, new_data_format);
        ChangeIndexCell::SetProps(ChangeCellProps::SetDataFormat(old_data_format))
    }

    // ───── DROPDOWN ─────

    pub fn record_set_props_dropdown(&mut self, new_dropdown: Option<Dropdown>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_dropdown = std::mem::replace(&mut props.dropdown, new_dropdown);
        ChangeIndexCell::SetProps(ChangeCellProps::SetDropdown(old_dropdown))
    }
    pub fn record_set_props_dropdown_vals(
        &mut self,
        new_vals: Vec<Val>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or_else(|| str_err!("No Dropdown exists for row '{}'", self.iid))?;
        let old_vals = std::mem::replace(&mut dd.vals, new_vals);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetVals(old_vals),
        )))
    }
    pub fn record_set_props_dropdown_default_val(
        &mut self,
        new_default_val: Option<Val>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or_else(|| str_err!("No Dropdown exists for row '{}'", self.iid))?;
        let old_default_val = std::mem::replace(&mut dd.default_val, new_default_val);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetDefaultVal(old_default_val),
        )))
    }
    pub fn record_set_props_dropdown_validation(
        &mut self,
        new_validation: bool,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or_else(|| str_err!("No Dropdown exists for row '{}'", self.iid))?;
        let old_validation = std::mem::replace(&mut dd.validation, new_validation);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetValidation(old_validation),
        )))
    }
    pub fn record_set_props_dropdown_text(
        &mut self,
        new_text: Option<String>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or_else(|| str_err!("No Dropdown exists for row '{}'", self.iid))?;
        let old_text = std::mem::replace(&mut dd.text, new_text);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetText(old_text),
        )))
    }
    pub fn record_set_props_dropdown_state(
        &mut self,
        new_state: DropdownState,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let dd = props
            .dropdown
            .as_mut()
            .ok_or_else(|| str_err!("No Dropdown exists for row '{}'", self.iid))?;
        let old_state = std::mem::replace(&mut dd.state, new_state);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateDropdown(
            ChangeDropdown::SetState(old_state),
        )))
    }

    // ───── HIGHLIGHT ─────

    pub fn record_set_props_highlight(
        &mut self,
        new_highlight: Option<Highlight>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_highlight = std::mem::replace(&mut props.highlight, new_highlight);
        ChangeIndexCell::SetProps(ChangeCellProps::SetHighlight(old_highlight))
    }

    // ───── NOTE ─────

    pub fn record_set_props_note(&mut self, new_note: Option<Note>) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_note = std::mem::replace(&mut props.note, new_note);
        ChangeIndexCell::SetProps(ChangeCellProps::SetNote(old_note))
    }
    pub fn record_set_props_note_text(
        &mut self,
        new_text: String,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or_else(|| str_err!("No Note exists for row '{}'", self.iid))?;
        let old_text = std::mem::replace(&mut note.note, new_text);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetText(old_text),
        )))
    }
    pub fn record_set_props_note_bg(
        &mut self,
        new_bg: Option<Color3>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or_else(|| str_err!("No Note exists for row '{}'", self.iid))?;
        let old_bg = std::mem::replace(&mut note.bg, new_bg);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetBg(old_bg),
        )))
    }
    pub fn record_set_props_note_fg(
        &mut self,
        new_fg: Option<Color3>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or_else(|| str_err!("No Note exists for row '{}'", self.iid))?;
        let old_fg = std::mem::replace(&mut note.fg, new_fg);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetFg(old_fg),
        )))
    }
    pub fn record_set_props_note_readonly(
        &mut self,
        new_readonly: bool,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let note = props
            .note
            .as_mut()
            .ok_or_else(|| str_err!("No Note exists for row '{}'", self.iid))?;
        let old_readonly = std::mem::replace(&mut note.readonly, new_readonly);
        Ok(ChangeIndexCell::SetProps(ChangeCellProps::UpdateNote(
            ChangeNote::SetReadonly(old_readonly),
        )))
    }

    // ───── PROGRESS BAR ─────

    pub fn record_set_props_progress_bar(
        &mut self,
        new_progress_bar: Option<ProgressBar>,
    ) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_progress_bar = std::mem::replace(&mut props.progress_bar, new_progress_bar);
        ChangeIndexCell::SetProps(ChangeCellProps::SetProgressBar(old_progress_bar))
    }
    pub fn record_set_props_progress_bar_bg(
        &mut self,
        new_bg: Option<Color3>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or_else(|| str_err!("No ProgressBar exists for row '{}'", self.iid))?;
        let old_bg = std::mem::replace(&mut pb.bg, new_bg);
        Ok(ChangeIndexCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetBg(old_bg)),
        ))
    }
    pub fn record_set_props_progress_bar_fg(
        &mut self,
        new_fg: Option<Color3>,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or_else(|| str_err!("No ProgressBar exists for row '{}'", self.iid))?;
        let old_fg = std::mem::replace(&mut pb.fg, new_fg);
        Ok(ChangeIndexCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetFg(old_fg)),
        ))
    }
    pub fn record_set_props_progress_bar_pc(
        &mut self,
        new_pc: f64,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or_else(|| str_err!("No ProgressBar exists for row '{}'", self.iid))?;
        let old_pc = std::mem::replace(&mut pb.pc, new_pc);
        Ok(ChangeIndexCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetPc(old_pc)),
        ))
    }
    pub fn record_set_props_progress_bar_remove_on_complete(
        &mut self,
        new_flag: bool,
    ) -> Result<ChangeIndexCell, String> {
        let props = self
            .props
            .as_mut()
            .ok_or_else(|| str_err!("No CellProps exist for row '{}'", self.iid))?;
        let pb = props
            .progress_bar
            .as_mut()
            .ok_or_else(|| str_err!("No ProgressBar exists for row '{}'", self.iid))?;
        let old_flag = std::mem::replace(&mut pb.remove_on_complete, new_flag);
        Ok(ChangeIndexCell::SetProps(
            ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetRemoveOnComplete(old_flag)),
        ))
    }

    // ───── READONLY ─────

    pub fn record_set_props_readonly(&mut self, new_readonly: bool) -> ChangeIndexCell {
        let props = self.props.get_or_insert_with(CellProps::default);
        let old_readonly = std::mem::replace(&mut props.readonly, new_readonly);
        ChangeIndexCell::SetProps(ChangeCellProps::SetReadonly(old_readonly))
    }
}
