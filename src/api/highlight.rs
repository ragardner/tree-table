use crate::EventData;
use crate::Highlight;
use crate::SpanKey;
use crate::Table;
use crate::args::HighlightArgs;
use crate::try_hex_to_color3;

impl Table {
    pub fn highlight(&self, kwargs: Option<HighlightArgs>) -> Result<(), String> {
        let kwargs: HighlightArgs = kwargs.unwrap_or(HighlightArgs {
            key: SpanKey::None,
            bg: None,
            fg: None,
            emit: true,
            undo: true,
        });
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(true);
        self.cr_ensure_widget_nrows(span.max_row(&self), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self), &mut event_data, true)?;
        let highlight = if kwargs.bg.is_none() && kwargs.fg.is_none() {
            None
        } else {
            Some(Highlight {
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
            })
        };
        let grid = &mut *self.grid.borrow_mut();
        let empty_val = &self.opts.borrow().empty_val;
        for row in span.rows(self) {
            for col in span.cols(self) {
                Self::acr_event_data_set_thighlight(
                    &mut grid.index.cells,
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
            if self.opts.borrow().enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
