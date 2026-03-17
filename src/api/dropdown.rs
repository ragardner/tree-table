use crate::Dropdown;
use crate::EventData;
use crate::SetDropdownArgs;
use crate::Table;
use crate::args::args_dropdown::DropdownArgs;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn dropdown(
        &mut self,
        kwargs: Option<DropdownArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DropdownArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let dropdown = if let Some(vals) = kwargs.vals {
            Some(Dropdown {
                vals: vals,
                default_val: kwargs.default_val,
                validation: kwargs.validation,
                text: kwargs.text,
                state: kwargs.state,
            })
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;
        for row in start_row..=end_row {
            for col in start_col..=end_col {
                Self::acr_event_data_set_tdropdown(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    dropdown.clone(),
                    empty_val,
                    &mut event_data,
                )?;
            }
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn dropdown_i(
        &mut self,
        kwargs: Option<DropdownArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DropdownArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        }
        let dropdown = if let Some(vals) = kwargs.vals {
            Some(Dropdown {
                vals: vals,
                default_val: kwargs.default_val,
                validation: kwargs.validation,
                text: kwargs.text,
                state: kwargs.state,
            })
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        for row in start_row..=end_row {
            Self::acr_event_data_set_idropdown(
                &mut self.grid.index.cells,
                row,
                dropdown.clone(),
                &mut event_data,
            )?;
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn dropdown_h(
        &mut self,
        kwargs: Option<DropdownArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DropdownArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let dropdown = if let Some(vals) = kwargs.vals {
            Some(Dropdown {
                vals: vals,
                default_val: kwargs.default_val,
                validation: kwargs.validation,
                text: kwargs.text,
                state: kwargs.state,
            })
        } else {
            None
        };
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        for col in start_col..=end_col {
            Self::acr_event_data_set_hdropdown(
                &mut self.grid.header.cells,
                col,
                dropdown.clone(),
                &mut event_data,
            )?;
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn set_dropdown(
        &mut self,
        kwargs: Option<SetDropdownArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                if let Some(vals) = &kwargs.vals {
                    Self::acr_event_data_set_tdropdown_vals(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        vals.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(default_val) = &kwargs.default_val {
                    Self::acr_event_data_set_tdropdown_default_val(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        default_val.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(validation) = kwargs.validation {
                    Self::acr_event_data_set_tdropdown_validation(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        validation,
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(text) = &kwargs.text {
                    Self::acr_event_data_set_tdropdown_text(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        text.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(state) = kwargs.state {
                    Self::acr_event_data_set_tdropdown_state(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        state,
                        empty_val,
                        &mut event_data,
                    )?;
                }
            }
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn set_dropdown_i(
        &mut self,
        kwargs: Option<SetDropdownArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let (start_row, end_row) = span.rows_start_end(&self.grid);

        for row in start_row..=end_row {
            if let Some(vals) = &kwargs.vals {
                Self::acr_event_data_set_idropdown_vals(
                    &mut self.grid.index.cells,
                    row,
                    vals.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(default_val) = &kwargs.default_val {
                Self::acr_event_data_set_idropdown_default_val(
                    &mut self.grid.index.cells,
                    row,
                    default_val.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(validation) = kwargs.validation {
                Self::acr_event_data_set_idropdown_validation(
                    &mut self.grid.index.cells,
                    row,
                    validation,
                    &mut event_data,
                )?;
            }
            if let Some(text) = &kwargs.text {
                Self::acr_event_data_set_idropdown_text(
                    &mut self.grid.index.cells,
                    row,
                    text.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(state) = kwargs.state {
                Self::acr_event_data_set_idropdown_state(
                    &mut self.grid.index.cells,
                    row,
                    state,
                    &mut event_data,
                )?;
            }
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn set_dropdown_h(
        &mut self,
        kwargs: Option<SetDropdownArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let (start_col, end_col) = span.cols_start_end(&self.grid);

        for col in start_col..=end_col {
            if let Some(vals) = &kwargs.vals {
                Self::acr_event_data_set_hdropdown_vals(
                    &mut self.grid.header.cells,
                    col,
                    vals.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(default_val) = &kwargs.default_val {
                Self::acr_event_data_set_hdropdown_default_val(
                    &mut self.grid.header.cells,
                    col,
                    default_val.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(validation) = kwargs.validation {
                Self::acr_event_data_set_hdropdown_validation(
                    &mut self.grid.header.cells,
                    col,
                    validation,
                    &mut event_data,
                )?;
            }
            if let Some(text) = &kwargs.text {
                Self::acr_event_data_set_hdropdown_text(
                    &mut self.grid.header.cells,
                    col,
                    text.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(state) = kwargs.state {
                Self::acr_event_data_set_hdropdown_state(
                    &mut self.grid.header.cells,
                    col,
                    state,
                    &mut event_data,
                )?;
            }
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }
}
