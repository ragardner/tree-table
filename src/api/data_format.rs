use crate::DataFormat;
use crate::EventData;
use crate::Table;
use crate::args::DataFormatArgs;
use crate::string_to_data_format;
use std::cell::RefCell;
use std::rc::Rc;

fn parse_data_format(fmt: Option<String>) -> Result<Option<DataFormat>, String> {
    fmt.filter(|s| !s.is_empty())
        .map(|s| string_to_data_format(&s))
        .transpose()
}

impl Table {
    pub fn data_format(
        &mut self,
        kwargs: Option<DataFormatArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let normalize = kwargs.fmt_normalize;
        let ignore_fmt_errs = kwargs.ignore_fmt_errs;
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let fmt = parse_data_format(kwargs.fmt)?;
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;
        let set_vals = kwargs.set_vals;
        let mut error: Option<String> = None;

        for row in start_row..end_row {
            for col in start_col..end_col {
                match Self::acr_event_data_set_tdata_format(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    fmt.clone(),
                    empty_val,
                    &mut event_data,
                ) {
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                    _ => {}
                };
                if set_vals {
                    let cell = match Self::acr_icell_mut(&mut self.grid.index.cells, row) {
                        Ok(c) => c,
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                    };
                    match cell.event_data_apply_own_tformat(
                        row,
                        col,
                        empty_val,
                        &mut event_data,
                        normalize,
                        ignore_fmt_errs,
                    ) {
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        if let Some(e) = error {
            let _ = self.cr_undo_and_invert_event(Rc::new(RefCell::new(event_data)), vec![], false);
            return Err(e);
        } else {
            self.cr_finalize_event_if_any_non_selection_change(
                event_data,
                kwargs.undo,
                kwargs.emit,
                true,
            )
        }
    }

    pub fn data_format_i(
        &mut self,
        kwargs: Option<DataFormatArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let normalize = kwargs.fmt_normalize;
        let ignore_fmt_errs = kwargs.ignore_fmt_errs;
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        }
        let fmt = parse_data_format(kwargs.fmt)?;
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let set_vals = kwargs.set_vals;
        let mut error: Option<String> = None;

        for row in start_row..end_row {
            match Self::acr_event_data_set_idata_format(
                &mut self.grid.index.cells,
                row,
                fmt.clone(),
                &mut event_data,
            ) {
                Err(e) => {
                    error = Some(e);
                    break;
                }
                _ => {}
            };
            if set_vals {
                let cell = match Self::acr_icell_mut(&mut self.grid.index.cells, row) {
                    Ok(c) => c,
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                };
                match cell.event_data_apply_own_iformat(
                    row,
                    &mut event_data,
                    normalize,
                    ignore_fmt_errs,
                ) {
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                    _ => {}
                }
            }
        }

        if let Some(e) = error {
            let _ = self.cr_undo_and_invert_event(Rc::new(RefCell::new(event_data)), vec![], false);
            return Err(e);
        } else {
            self.cr_finalize_event_if_any_non_selection_change(
                event_data,
                kwargs.undo,
                kwargs.emit,
                true,
            )
        }
    }

    pub fn data_format_h(
        &mut self,
        kwargs: Option<DataFormatArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let normalize = kwargs.fmt_normalize;
        let ignore_fmt_errs = kwargs.ignore_fmt_errs;
        if kwargs.expand {
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let fmt = parse_data_format(kwargs.fmt)?;
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let set_vals = kwargs.set_vals;
        let mut error: Option<String> = None;

        for col in start_col..end_col {
            match Self::acr_event_data_set_hdata_format(
                &mut self.grid.header.cells,
                col,
                fmt.clone(),
                &mut event_data,
            ) {
                Err(e) => {
                    error = Some(e);
                    break;
                }
                _ => {}
            };
            if set_vals {
                let cell = match Self::acr_hcell_mut(&mut self.grid.header.cells, col) {
                    Ok(c) => c,
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                };
                match cell.event_data_apply_own_hformat(
                    col,
                    &mut event_data,
                    normalize,
                    ignore_fmt_errs,
                ) {
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                    _ => {}
                }
            }
        }

        if let Some(e) = error {
            let _ = self.cr_undo_and_invert_event(Rc::new(RefCell::new(event_data)), vec![], false);
            return Err(e);
        } else {
            self.cr_finalize_event_if_any_non_selection_change(
                event_data,
                kwargs.undo,
                kwargs.emit,
                true,
            )
        }
    }
}
