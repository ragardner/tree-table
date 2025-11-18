use crate::EventData;
use crate::SpanKey;
use crate::Table;
use crate::args::DataFormatArgs;
use crate::string_to_data_format;

impl Table {
    pub fn data_format(&mut self, kwargs: Option<DataFormatArgs>) -> Result<(), String> {
        let kwargs: DataFormatArgs = kwargs.unwrap_or(DataFormatArgs {
            key: SpanKey::None,
            fmt: None,
            emit: true,
            undo: true,
        });
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(true);
        self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        let fmt = if let Some(fmt) = kwargs.fmt {
            if fmt.is_empty() {
                Some(string_to_data_format(&fmt)?)
            } else {
                None
            }
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;
        for row in start_row..end_row {
            for col in start_col..end_col {
                Self::acr_event_data_set_tdata_format(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    fmt.clone(),
                    empty_val,
                    &mut event_data,
                )?;
            }
        }
        self.cr_event_modified(&event_data, kwargs.emit, true)?;
        if !event_data.table_cells.is_empty() {
            if self.opts.enabled_undo && kwargs.undo {
                self.grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
