use crate::EventData;
use crate::SpanKey;
use crate::Table;
use crate::args::FillDataArgs;
use crate::args::SetDataArgs;

impl Table {
    // Sets table cell data using row ids/pos and col ids/pos.
    pub fn set_data(&self, kwargs: Option<SetDataArgs>) -> Result<(), String> {
        let kwargs = kwargs.unwrap_or(SetDataArgs {
            data: vec![],
            ignore_invalid: true,
            emit: false,
            undo: true,
        });
        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();
        let empty_val = &opts.empty_val;
        let mut event_data: EventData = self.cr_new_event(true);
        if kwargs.ignore_invalid {
            for (row_pos, cols) in kwargs.data.into_iter() {
                let Ok(rn) = Self::acr_resolve_row(&grid.index, &row_pos) else {
                    continue;
                };
                for (col_pos, val) in cols.into_iter() {
                    let Ok(cn) = Self::acr_resolve_col(&grid.header, &col_pos) else {
                        continue;
                    };
                    Self::acr_event_data_fmt_set_tval(
                        &mut grid.index.cells,
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
                let rn = Self::acr_resolve_row(&grid.index, &row_pos)?;
                for (col_pos, val) in cols.into_iter() {
                    let cn = Self::acr_resolve_col(&grid.header, &col_pos)?;
                    Self::acr_event_data_fmt_set_tval(
                        &mut grid.index.cells,
                        rn,
                        cn,
                        val,
                        empty_val,
                        &mut event_data,
                    )?;
                }
            }
        }
        if !event_data.table_cells.is_empty() {
            self.cr_event_modified(&event_data, kwargs.emit, true)?;
            if opts.enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }

    /// Sets table cell data using a key and a single value.
    pub fn fill_data(&self, kwargs: Option<FillDataArgs>) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();
        let empty_val = &opts.empty_val;
        let kwargs = kwargs.unwrap_or(FillDataArgs {
            key: SpanKey::None,
            val: empty_val.clone(),
            undo: true,
            emit: true,
            add_rows: false,
            add_cols: false,
        });
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(true);
        let val = kwargs.val;
        if kwargs.add_rows {
            self.cr_ensure_widget_nrows(span.max_row(&self), &mut event_data, true)?;
        }
        if kwargs.add_cols {
            self.cr_ensure_widget_ncols(span.max_col(&self), &mut event_data, true)?;
        }
        let row_start = span.row;
        let row_end = span.end_row.unwrap_or(self.cr_total_rows());
        let effective_row_end = if kwargs.add_rows {
            row_end
        } else {
            row_end.min(self.cr_total_rows())
        };
        let col_start = span.col;
        let col_end = span.end_col.unwrap_or(self.cr_total_cols());
        let effective_col_end = if kwargs.add_cols {
            col_end
        } else {
            col_end.min(self.cr_total_cols())
        };
        for row in row_start..effective_row_end {
            for col in col_start..effective_col_end {
                Self::acr_event_data_fmt_set_tval(
                    &mut grid.index.cells,
                    row,
                    col,
                    val.clone(),
                    empty_val,
                    &mut event_data,
                )?;
            }
        }
        if !event_data.table_cells.is_empty() {
            self.cr_event_modified(&event_data, kwargs.emit, true)?;
            if opts.enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
