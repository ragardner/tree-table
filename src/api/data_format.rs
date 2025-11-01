use crate::EventData;
use crate::SpanKey;
use crate::Table;
use crate::args::DataFormatArgs;
use crate::string_to_data_format;

impl Table {
    pub fn data_format(&self, kwargs: Option<DataFormatArgs>) -> Result<(), String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or(DataFormatArgs {
            key: SpanKey::None,
            fmt: None,
            emit: true,
            undo: true,
        });
        let span = self.cr_key_to_span(kwargs.key)?;
        let (mut grid, opts, sel) = self.cr_mut_borrows();
        let empty_val = &opts.empty_val;
        let mut event_data: EventData = self.cr_new_event(true, &opts, &sel);
        let span_max_row = span.max_row(&grid);
        let span_max_col = span.max_col(&grid);
        self.cr_ensure_widget_nrows(&mut grid, &opts, &sel, span_max_row, &mut event_data, true)?;
        self.cr_ensure_widget_ncols(&mut grid, &opts, &sel, span_max_col, &mut event_data, true)?;
        let fmt = if let Some(fmt) = kwargs.fmt {
            if fmt.is_empty() {
                Some(string_to_data_format(&fmt)?)
            } else {
                None
            }
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&grid);
        let (start_col, end_col) = span.cols_start_end(&grid);
        for row in start_row..end_row {
            for col in start_col..end_col {
                Self::acr_event_data_set_tdata_format(
                    &mut grid.index.cells,
                    row,
                    col,
                    fmt.clone(),
                    empty_val,
                    &mut event_data,
                )?;
            }
        }
        self.cr_event_modified(
            &mut grid,
            &self.events.borrow(),
            &event_data,
            kwargs.emit,
            true,
        )?;
        if !event_data.table_cells.is_empty() {
            if opts.enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
