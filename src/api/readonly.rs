use crate::EventData;
use crate::ReadonlyArgs;
use crate::Table;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn readonly(
        &mut self,
        kwargs: Option<ReadonlyArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: ReadonlyArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let readonly = kwargs.readonly;
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;
        for row in start_row..=end_row {
            for col in start_col..=end_col {
                Self::acr_event_data_set_treadonly(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    readonly,
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

    pub fn readonly_i(
        &mut self,
        kwargs: Option<ReadonlyArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: ReadonlyArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        }
        let readonly = kwargs.readonly;
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        for row in start_row..=end_row {
            Self::acr_event_data_set_ireadonly(
                &mut self.grid.index.cells,
                row,
                readonly,
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

    pub fn readonly_h(
        &mut self,
        kwargs: Option<ReadonlyArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: ReadonlyArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let readonly = kwargs.readonly;
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        for col in start_col..=end_col {
            Self::acr_event_data_set_hreadonly(
                &mut self.grid.header.cells,
                col,
                readonly,
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
}
