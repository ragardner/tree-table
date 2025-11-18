use crate::EventData;
use crate::Highlight;
use crate::SpanKey;
use crate::Table;
use crate::args::HighlightArgs;
use crate::try_hex_to_color3;

impl Table {
    pub fn highlight(&mut self, kwargs: Option<HighlightArgs>) -> Result<(), String> {
        let kwargs: HighlightArgs = kwargs.unwrap_or(HighlightArgs {
            key: SpanKey::None,
            bg: None,
            fg: None,
            emit: true,
            undo: true,
        });
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(true);
        self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        let highlight = if kwargs.bg.is_none() && kwargs.fg.is_none() {
            None
        } else {
            Some(Highlight {
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
            })
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;
        for row in start_row..end_row {
            for col in start_col..end_col {
                Self::acr_event_data_set_thighlight(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    highlight.clone(),
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
