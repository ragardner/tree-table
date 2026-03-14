use crate::EventData;
use crate::Table;
use crate::args::FillDataArgs;
use crate::args::SetDataArgs;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    // Sets table cell data using row ids/pos and col ids/pos.
    pub fn set_data(
        &mut self,
        kwargs: Option<SetDataArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();

        let empty_val = &self.opts.empty_val;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);

        if kwargs.ignore_invalid {
            for (row_pos, cols) in kwargs.data.into_iter() {
                let Ok(rn) = Self::acr_pos(&self.grid.index.iid_to_row, &row_pos) else {
                    continue;
                };
                for (col_pos, val) in cols.into_iter() {
                    let Ok(cn) = Self::acr_pos(&self.grid.header.iid_to_col, &col_pos) else {
                        continue;
                    };
                    Self::acr_event_data_fmt_set_tval(
                        &mut self.grid.index.cells,
                        rn,
                        cn,
                        val,
                        empty_val,
                        &mut event_data,
                    )?;
                }
            }
        } else {
            for (row_pos, cols) in kwargs.data.into_iter() {
                let rn = Self::acr_pos(&self.grid.index.iid_to_row, &row_pos)?;
                for (col_pos, val) in cols.into_iter() {
                    let cn = Self::acr_pos(&self.grid.header.iid_to_col, &col_pos)?;
                    Self::acr_event_data_fmt_set_tval(
                        &mut self.grid.index.cells,
                        rn,
                        cn,
                        val,
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

    /// Sets table cell data using a key and a single value.
    pub fn fill_data(
        &mut self,
        kwargs: Option<FillDataArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();

        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let val = kwargs.val.unwrap_or(self.opts.empty_val.clone());

        if kwargs.add_rows {
            let span_max_row = span.max_row(&self.grid);
            self.cr_ensure_widget_nrows(span_max_row, &mut event_data, true)?;
        }
        if kwargs.add_cols {
            let span_max_col = span.max_col(&self.grid);
            self.cr_ensure_widget_ncols(span_max_col, &mut event_data, true)?;
        }

        let row_start = span.row;
        let row_end = span.max_row(&self.grid);
        let effective_row_end = if kwargs.add_rows {
            row_end
        } else {
            row_end.min(Self::acr_total_rows(&self.grid.index.cells))
        };

        let col_start = span.col;
        let col_end = span.max_col(&self.grid);
        let effective_col_end = if kwargs.add_cols {
            col_end
        } else {
            col_end.min(Self::acr_total_cols(&self.grid.header.cells))
        };

        let empty_val = &self.opts.empty_val;

        for row in row_start..=effective_row_end {
            for col in col_start..=effective_col_end {
                Self::acr_event_data_fmt_set_tval(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    val.clone(),
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
}
