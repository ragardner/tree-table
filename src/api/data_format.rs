use crate::DataFormat;
use crate::EventData;
use crate::FormatSpec;
use crate::Table;
use crate::args::DataFormatArgs;
use crate::string_to_data_format;
use std::cell::RefCell;
use std::rc::Rc;

fn parse_format_spec(spec: Option<FormatSpec>) -> Result<Option<DataFormat>, String> {
    match spec {
        None => Ok(None),
        Some(FormatSpec::Simple(s)) if !s.is_empty() => string_to_data_format(&s).map(Some),
        Some(FormatSpec::Simple(_)) => Ok(None),
        Some(FormatSpec::Rich(df)) => Ok(Some(df)),
    }
}

impl Table {
    pub fn data_format(
        &mut self,
        kwargs: Option<DataFormatArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let fmt = parse_format_spec(kwargs.fmt)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let normalize = kwargs.fmt_normalize;
        let ignore_fmt_errs = kwargs.ignore_fmt_errs;
        let set_vals = kwargs.set_vals;
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
                self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
            }
            let empty_val = &self.opts.empty_val;

            for row in start_row..=end_row {
                for col in start_col..=end_col {
                    Self::acr_event_data_set_tdata_format(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        fmt.clone(),
                        empty_val,
                        &mut event_data,
                    )?;

                    if set_vals {
                        let cell = Self::acr_icell_mut(&mut self.grid.index.cells, row)?;
                        cell.event_data_apply_own_tformat(
                            row,
                            col,
                            empty_val,
                            &mut event_data,
                            normalize,
                            ignore_fmt_errs,
                        )?;
                    }
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn data_format_i(
        &mut self,
        kwargs: Option<DataFormatArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let fmt = parse_format_spec(kwargs.fmt)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let normalize = kwargs.fmt_normalize;
        let ignore_fmt_errs = kwargs.ignore_fmt_errs;
        let set_vals = kwargs.set_vals;
        let (start_row, end_row) = span.rows_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
            }
            for row in start_row..=end_row {
                Self::acr_event_data_set_idata_format(
                    &mut self.grid.index.cells,
                    row,
                    fmt.clone(),
                    &mut event_data,
                )?;
                if set_vals {
                    let cell = Self::acr_icell_mut(&mut self.grid.index.cells, row)?;
                    cell.event_data_apply_own_iformat(
                        row,
                        &mut event_data,
                        normalize,
                        ignore_fmt_errs,
                    )?;
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn data_format_h(
        &mut self,
        kwargs: Option<DataFormatArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let fmt = parse_format_spec(kwargs.fmt)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let normalize = kwargs.fmt_normalize;
        let ignore_fmt_errs = kwargs.ignore_fmt_errs;
        let set_vals = kwargs.set_vals;
        let (start_col, end_col) = span.cols_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
            }
            for col in start_col..=end_col {
                Self::acr_event_data_set_hdata_format(
                    &mut self.grid.header.cells,
                    col,
                    fmt.clone(),
                    &mut event_data,
                )?;
                if set_vals {
                    let cell = Self::acr_hcell_mut(&mut self.grid.header.cells, col)?;
                    cell.event_data_apply_own_hformat(
                        col,
                        &mut event_data,
                        normalize,
                        ignore_fmt_errs,
                    )?;
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }
}
