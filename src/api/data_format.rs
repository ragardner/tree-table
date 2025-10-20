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
        let mut event_data: EventData = self.cr_new_event(true);
        self.cr_ensure_widget_nrows(span.max_row(&self), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self), &mut event_data, true)?;
        let grid = &mut *self.grid.borrow_mut();
        let empty_val = &self.opts.borrow().empty_val;
        let fmt = if let Some(fmt) = kwargs.fmt {
            if fmt.is_empty() {
                Some(string_to_data_format(&fmt)?)
            } else {
                None
            }
        } else {
            None
        };
        for row in span.rows(self) {
            for col in span.cols(self) {
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
        self.cr_event_modified(&event_data, kwargs.emit, true)?;
        if !event_data.table_cells.is_empty() {
            if self.opts.borrow().enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
