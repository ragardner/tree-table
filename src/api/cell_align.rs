use crate::CellAlignArgs;
use crate::EventData;
use crate::Table;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn align(
        &mut self,
        kwargs: Option<CellAlignArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: CellAlignArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let align = kwargs.align; // None = clear alignment, Some = set it
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
                    Self::acr_event_data_set_talign(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        align,
                        empty_val,
                        &mut event_data,
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

    pub fn align_i(
        &mut self,
        kwargs: Option<CellAlignArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: CellAlignArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        }
        let align = kwargs.align;
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let mut error: Option<String> = None;

        for row in start_row..=end_row {
            match Self::acr_event_data_set_ialign(
                &mut self.grid.index.cells,
                row,
                align,
                &mut event_data,
            ) {
                Err(e) => {
                    error = Some(e);
                    break;
                }
                _ => {}
            }
        }
        if let Some(e) = error {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
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

    pub fn align_h(
        &mut self,
        kwargs: Option<CellAlignArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: CellAlignArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let align = kwargs.align;
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let mut error: Option<String> = None;

        for col in start_col..=end_col {
            match Self::acr_event_data_set_halign(
                &mut self.grid.header.cells,
                col,
                align,
                &mut event_data,
            ) {
                Err(e) => {
                    error = Some(e);
                    break;
                }
                _ => {}
            }
        }
        if let Some(e) = error {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
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
