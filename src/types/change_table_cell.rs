use crate::Border;
use crate::CellBorders;
use crate::CellProps;
use crate::ChangeBorders;
use crate::ChangeCellProps;
use crate::ChangeCheckbox;
use crate::ChangeIndexCell;
use crate::ChangeNote;
use crate::ChangeProgressBar;
use crate::Checkbox;
use crate::Color3;
use crate::DataFormat;
use crate::Dropdown;
use crate::DropdownState;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::TableCell;
use crate::Val;
use crate::str_err;
use crate::types::change_index_cell::ChangeDropdown;

// Recording functions: These apply a new value (or change) to the IndexCell Vals and return a ChangeIndexCell
// encapsulating the old value. This is for the recording phase when making changes - apply the new,
// capture old in the change for the undo stack.
// The difference with invert is that these do not require a ChangeIndexCell to begin with.
impl IndexCell {
    // ───── TABLE CELL ─────

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

    // ───── TABLE CELL ALIGN ─────

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

    // ───── TABLE CELL BORDERS ─────

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
    pub fn record_set_table_cell_props_borders_top(
        &mut self,
        col: usize,
        new_top: Option<Border>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let borders = props.borders.as_mut().ok_or(str_err!(
            "No CellBorders exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_top = std::mem::replace(&mut borders.top, new_top);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateBorders(ChangeBorders::SetTop(old_top)),
        })
    }
    pub fn record_set_table_cell_props_borders_right(
        &mut self,
        col: usize,
        new_right: Option<Border>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let borders = props.borders.as_mut().ok_or(str_err!(
            "No CellBorders exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_right = std::mem::replace(&mut borders.right, new_right);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateBorders(ChangeBorders::SetRight(old_right)),
        })
    }
    pub fn record_set_table_cell_props_borders_bottom(
        &mut self,
        col: usize,
        new_bottom: Option<Border>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let borders = props.borders.as_mut().ok_or(str_err!(
            "No CellBorders exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_bottom = std::mem::replace(&mut borders.bottom, new_bottom);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateBorders(ChangeBorders::SetBottom(old_bottom)),
        })
    }
    pub fn record_set_table_cell_props_borders_left(
        &mut self,
        col: usize,
        new_left: Option<Border>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let borders = props.borders.as_mut().ok_or(str_err!(
            "No CellBorders exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_left = std::mem::replace(&mut borders.left, new_left);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateBorders(ChangeBorders::SetLeft(old_left)),
        })
    }

    // ───── TABLE CELL CHECKBOX ─────

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
    pub fn record_set_table_cell_props_checkbox_true_val(
        &mut self,
        col: usize,
        new_true_val: Val,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let cb = props.checkbox.as_mut().ok_or(str_err!(
            "No Checkbox exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_true_val = std::mem::replace(&mut cb.true_val, new_true_val);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateCheckbox(ChangeCheckbox::SetTrueVal(old_true_val)),
        })
    }
    pub fn record_set_table_cell_props_checkbox_false_val(
        &mut self,
        col: usize,
        new_false_val: Val,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let cb = props.checkbox.as_mut().ok_or(str_err!(
            "No Checkbox exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_false_val = std::mem::replace(&mut cb.false_val, new_false_val);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateCheckbox(ChangeCheckbox::SetFalseVal(old_false_val)),
        })
    }
    pub fn record_set_table_cell_props_checkbox_text(
        &mut self,
        col: usize,
        new_text: Option<String>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let cb = props.checkbox.as_mut().ok_or(str_err!(
            "No Checkbox exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_text = std::mem::replace(&mut cb.text, new_text);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateCheckbox(ChangeCheckbox::SetText(old_text)),
        })
    }
    pub fn record_set_table_cell_props_checkbox_state(
        &mut self,
        col: usize,
        new_state: bool,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let cb = props.checkbox.as_mut().ok_or(str_err!(
            "No Checkbox exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_state = std::mem::replace(&mut cb.state, new_state);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateCheckbox(ChangeCheckbox::SetState(old_state)),
        })
    }

    // ───── TABLE CELL DATA FORMAT ─────

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

    // ───── TABLE CELL DROPDOWN ─────

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
    pub fn record_set_table_cell_props_dropdown_vals(
        &mut self,
        col: usize,
        new_vals: Vec<Val>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let dd = props.dropdown.as_mut().ok_or(str_err!(
            "No Dropdown exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_vals = std::mem::replace(&mut dd.vals, new_vals);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateDropdown(ChangeDropdown::SetVals(old_vals)),
        })
    }
    pub fn record_set_table_cell_props_dropdown_default_val(
        &mut self,
        col: usize,
        new_default_val: Option<Val>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let dd = props.dropdown.as_mut().ok_or(str_err!(
            "No Dropdown exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_default_val = std::mem::replace(&mut dd.default_val, new_default_val);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateDropdown(ChangeDropdown::SetDefaultVal(old_default_val)),
        })
    }
    pub fn record_set_table_cell_props_dropdown_validation(
        &mut self,
        col: usize,
        new_validation: bool,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let dd = props.dropdown.as_mut().ok_or(str_err!(
            "No Dropdown exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_validation = std::mem::replace(&mut dd.validation, new_validation);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateDropdown(ChangeDropdown::SetValidation(old_validation)),
        })
    }
    pub fn record_set_table_cell_props_dropdown_text(
        &mut self,
        col: usize,
        new_text: Option<String>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let dd = props.dropdown.as_mut().ok_or(str_err!(
            "No Dropdown exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_text = std::mem::replace(&mut dd.text, new_text);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateDropdown(ChangeDropdown::SetText(old_text)),
        })
    }
    pub fn record_set_table_cell_props_dropdown_state(
        &mut self,
        col: usize,
        new_state: DropdownState,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let dd = props.dropdown.as_mut().ok_or(str_err!(
            "No Dropdown exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_state = std::mem::replace(&mut dd.state, new_state);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateDropdown(ChangeDropdown::SetState(old_state)),
        })
    }

    // ───── TABLE CELL HIGHLIGHT ─────

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

    // ───── TABLE CELL NOTE ─────

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
    pub fn record_set_table_cell_props_note_text(
        &mut self,
        col: usize,
        new_text: String,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let note = props.note.as_mut().ok_or(str_err!(
            "No Note exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_text = std::mem::replace(&mut note.note, new_text);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateNote(ChangeNote::SetText(old_text)),
        })
    }
    pub fn record_set_table_cell_props_note_bg(
        &mut self,
        col: usize,
        new_bg: Option<Color3>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let note = props.note.as_mut().ok_or(str_err!(
            "No Note exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_bg = std::mem::replace(&mut note.bg, new_bg);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateNote(ChangeNote::SetBg(old_bg)),
        })
    }
    pub fn record_set_table_cell_props_note_fg(
        &mut self,
        col: usize,
        new_fg: Option<Color3>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let note = props.note.as_mut().ok_or(str_err!(
            "No Note exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_fg = std::mem::replace(&mut note.fg, new_fg);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateNote(ChangeNote::SetFg(old_fg)),
        })
    }
    pub fn record_set_table_cell_props_note_readonly(
        &mut self,
        col: usize,
        new_readonly: bool,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let note = props.note.as_mut().ok_or(str_err!(
            "No Note exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_readonly = std::mem::replace(&mut note.readonly, new_readonly);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateNote(ChangeNote::SetReadonly(old_readonly)),
        })
    }

    // ───── TABLE CELL PROGRESS BAR ─────

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
    pub fn record_set_table_cell_props_progress_bar_name(
        &mut self,
        col: usize,
        new_name: String,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let pb = props.progress_bar.as_mut().ok_or(str_err!(
            "No ProgressBar exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_name = std::mem::replace(&mut pb.name, new_name);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetName(old_name)),
        })
    }
    pub fn record_set_table_cell_props_progress_bar_bg(
        &mut self,
        col: usize,
        new_bg: Option<Color3>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let pb = props.progress_bar.as_mut().ok_or(str_err!(
            "No ProgressBar exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_bg = std::mem::replace(&mut pb.bg, new_bg);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetBg(old_bg)),
        })
    }
    pub fn record_set_table_cell_props_progress_bar_fg(
        &mut self,
        col: usize,
        new_fg: Option<Color3>,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let pb = props.progress_bar.as_mut().ok_or(str_err!(
            "No ProgressBar exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_fg = std::mem::replace(&mut pb.fg, new_fg);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetFg(old_fg)),
        })
    }
    pub fn record_set_table_cell_props_progress_bar_pc(
        &mut self,
        col: usize,
        new_pc: f64,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let pb = props.progress_bar.as_mut().ok_or(str_err!(
            "No ProgressBar exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_pc = std::mem::replace(&mut pb.pc, new_pc);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetPc(old_pc)),
        })
    }
    pub fn record_set_table_cell_props_progress_bar_remove_on_complete(
        &mut self,
        col: usize,
        new_flag: bool,
        default_val: Val,
    ) -> Result<ChangeIndexCell, String> {
        let table_cell = self
            .vals
            .entry(col)
            .or_insert_with(|| TableCell::new(default_val));
        let props = table_cell.props.as_mut().ok_or(str_err!(
            "No CellProps exist for table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let pb = props.progress_bar.as_mut().ok_or(str_err!(
            "No ProgressBar exists on table cell row '{}', col '{}'",
            self.iid,
            col
        ))?;
        let old_flag = std::mem::replace(&mut pb.remove_on_complete, new_flag);
        Ok(ChangeIndexCell::SetTableCellProps {
            col,
            change: ChangeCellProps::UpdateProgressBar(ChangeProgressBar::SetRemoveOnComplete(
                old_flag,
            )),
        })
    }

    // ───── TABLE CELL READONLY ─────

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
