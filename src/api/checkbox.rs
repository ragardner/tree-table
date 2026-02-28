use crate::Checkbox;
use crate::CheckboxArgs;
use crate::EventData;
use crate::SetCheckboxArgs;
use crate::Table;
use crate::Val;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn checkbox(
        &mut self,
        kwargs: Option<CheckboxArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: CheckboxArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        let checkbox = if kwargs.del {
            None
        } else {
            Some(Checkbox {
                true_val: kwargs.true_val.unwrap_or(Val::Bl(true)),
                false_val: kwargs.false_val.unwrap_or(Val::Bl(false)),
                text: kwargs.text,
                state: kwargs.state,
            })
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;
        for row in start_row..end_row {
            for col in start_col..end_col {
                Self::acr_event_data_set_tcheckbox(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    checkbox.clone(),
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

    pub fn checkbox_i(
        &mut self,
        kwargs: Option<CheckboxArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: CheckboxArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let checkbox = if kwargs.del {
            None
        } else {
            Some(Checkbox {
                true_val: kwargs.true_val.unwrap_or(Val::Bl(true)),
                false_val: kwargs.false_val.unwrap_or(Val::Bl(false)),
                text: kwargs.text,
                state: kwargs.state,
            })
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        for row in start_row..end_row {
            Self::acr_event_data_set_icheckbox(
                &mut self.grid.index.cells,
                row,
                checkbox.clone(),
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

    pub fn checkbox_h(
        &mut self,
        kwargs: Option<CheckboxArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: CheckboxArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let checkbox = if kwargs.del {
            None
        } else {
            Some(Checkbox {
                true_val: kwargs.true_val.unwrap_or(Val::Bl(true)),
                false_val: kwargs.false_val.unwrap_or(Val::Bl(false)),
                text: kwargs.text,
                state: kwargs.state,
            })
        };
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        for col in start_col..end_col {
            Self::acr_event_data_set_hcheckbox(
                &mut self.grid.header.cells,
                col,
                checkbox.clone(),
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

    pub fn set_checkbox(
        &mut self,
        kwargs: Option<SetCheckboxArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;

        for row in start_row..end_row {
            for col in start_col..end_col {
                if let Some(true_val) = &kwargs.true_val {
                    Self::acr_event_data_set_tcheckbox_true_val(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        true_val.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(false_val) = &kwargs.false_val {
                    Self::acr_event_data_set_tcheckbox_false_val(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        false_val.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(text) = &kwargs.text {
                    Self::acr_event_data_set_tcheckbox_text(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        text.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(state) = kwargs.state {
                    Self::acr_event_data_set_tcheckbox_state(
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

    pub fn set_checkbox_i(
        &mut self,
        kwargs: Option<SetCheckboxArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        for row in start_row..end_row {
            if let Some(true_val) = &kwargs.true_val {
                Self::acr_event_data_set_icheckbox_true_val(
                    &mut self.grid.index.cells,
                    row,
                    true_val.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(false_val) = &kwargs.false_val {
                Self::acr_event_data_set_icheckbox_false_val(
                    &mut self.grid.index.cells,
                    row,
                    false_val.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(text) = &kwargs.text {
                Self::acr_event_data_set_icheckbox_text(
                    &mut self.grid.index.cells,
                    row,
                    text.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(state) = kwargs.state {
                Self::acr_event_data_set_icheckbox_state(
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

    pub fn set_checkbox_h(
        &mut self,
        kwargs: Option<SetCheckboxArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        for col in start_col..end_col {
            if let Some(true_val) = &kwargs.true_val {
                Self::acr_event_data_set_hcheckbox_true_val(
                    &mut self.grid.header.cells,
                    col,
                    true_val.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(false_val) = &kwargs.false_val {
                Self::acr_event_data_set_hcheckbox_false_val(
                    &mut self.grid.header.cells,
                    col,
                    false_val.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(text) = &kwargs.text {
                Self::acr_event_data_set_hcheckbox_text(
                    &mut self.grid.header.cells,
                    col,
                    text.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(state) = kwargs.state {
                Self::acr_event_data_set_hcheckbox_state(
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
