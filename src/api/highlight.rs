use crate::EventData;
use crate::Highlight;
use crate::Table;
use crate::args::HighlightArgs;
use crate::try_hex_to_color3;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn highlight(
        &mut self,
        kwargs: Option<HighlightArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: HighlightArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
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

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
                self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
            }
            let empty_val = &self.opts.empty_val;

            for row in start_row..=end_row {
                for col in start_col..=end_col {
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

    pub fn highlight_i(
        &mut self,
        kwargs: Option<HighlightArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: HighlightArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        }
        let highlight = if kwargs.bg.is_none() && kwargs.fg.is_none() {
            None
        } else {
            Some(Highlight {
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
            })
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let mut error: Option<String> = None;

        for row in start_row..=end_row {
            if let Err(e) = Self::acr_event_data_set_ihighlight(
                &mut self.grid.index.cells,
                row,
                highlight.clone(),
                &mut event_data,
            ) {
                error = Some(e);
                break;
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

    pub fn highlight_h(
        &mut self,
        kwargs: Option<HighlightArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: HighlightArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        if kwargs.expand {
            self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
        }
        let highlight = if kwargs.bg.is_none() && kwargs.fg.is_none() {
            None
        } else {
            Some(Highlight {
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
            })
        };
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let mut error: Option<String> = None;

        for col in start_col..=end_col {
            if let Err(e) = Self::acr_event_data_set_hhighlight(
                &mut self.grid.header.cells,
                col,
                highlight.clone(),
                &mut event_data,
            ) {
                error = Some(e);
                break;
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
