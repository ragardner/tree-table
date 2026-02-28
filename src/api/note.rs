use crate::Color3;
use crate::EventData;
use crate::Note;
use crate::NoteArgs;
use crate::SetNoteArgs;
use crate::Table;
use crate::try_hex_to_color3;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn note(
        &mut self,
        kwargs: Option<NoteArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: NoteArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;

        let note = if let Some(note_text) = kwargs.note {
            Some(Note {
                note: note_text,
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
                readonly: kwargs.readonly,
            })
        } else {
            None
        };

        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;

        for row in start_row..end_row {
            for col in start_col..end_col {
                Self::acr_event_data_set_tnote(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    note.clone(),
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

    pub fn note_i(
        &mut self,
        kwargs: Option<NoteArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: NoteArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let note = if let Some(note_text) = kwargs.note {
            Some(Note {
                note: note_text,
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
                readonly: kwargs.readonly,
            })
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        for row in start_row..end_row {
            Self::acr_event_data_set_inote(
                &mut self.grid.index.cells,
                row,
                note.clone(),
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

    pub fn note_h(
        &mut self,
        kwargs: Option<NoteArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: NoteArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let note = if let Some(note_text) = kwargs.note {
            Some(Note {
                note: note_text,
                bg: try_hex_to_color3(kwargs.bg)?,
                fg: try_hex_to_color3(kwargs.fg)?,
                readonly: kwargs.readonly,
            })
        } else {
            None
        };
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        for col in start_col..end_col {
            Self::acr_event_data_set_hnote(
                &mut self.grid.header.cells,
                col,
                note.clone(),
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

    pub fn set_note(
        &mut self,
        kwargs: Option<SetNoteArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: SetNoteArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
        self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;

        let bg_color: Option<Option<Color3>> = if let Some(maybe_hex) = &kwargs.bg {
            Some(try_hex_to_color3(maybe_hex.clone())?)
        } else {
            None
        };
        let fg_color: Option<Option<Color3>> = if let Some(maybe_hex) = &kwargs.fg {
            Some(try_hex_to_color3(maybe_hex.clone())?)
        } else {
            None
        };

        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;

        for row in start_row..end_row {
            for col in start_col..end_col {
                if let Some(text) = &kwargs.note {
                    Self::acr_event_data_set_tnote_text(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        text.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(color) = &bg_color {
                    Self::acr_event_data_set_tnote_bg(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        color.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(color) = &fg_color {
                    Self::acr_event_data_set_tnote_fg(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        color.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
                if let Some(readonly) = kwargs.readonly {
                    Self::acr_event_data_set_tnote_readonly(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        readonly,
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

    pub fn set_note_i(
        &mut self,
        kwargs: Option<SetNoteArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: SetNoteArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;

        // Pre-compute colors once
        let bg_color: Option<Option<Color3>> = if let Some(maybe_hex) = &kwargs.bg {
            Some(try_hex_to_color3(maybe_hex.clone())?)
        } else {
            None
        };
        let fg_color: Option<Option<Color3>> = if let Some(maybe_hex) = &kwargs.fg {
            Some(try_hex_to_color3(maybe_hex.clone())?)
        } else {
            None
        };

        let (start_row, end_row) = span.rows_start_end(&self.grid);

        for row in start_row..end_row {
            if let Some(text) = &kwargs.note {
                Self::acr_event_data_set_inote_text(
                    &mut self.grid.index.cells,
                    row,
                    text.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(color) = &bg_color {
                Self::acr_event_data_set_inote_bg(
                    &mut self.grid.index.cells,
                    row,
                    color.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(color) = &fg_color {
                Self::acr_event_data_set_inote_fg(
                    &mut self.grid.index.cells,
                    row,
                    color.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(readonly) = kwargs.readonly {
                Self::acr_event_data_set_inote_readonly(
                    &mut self.grid.index.cells,
                    row,
                    readonly,
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

    pub fn set_note_h(
        &mut self,
        kwargs: Option<SetNoteArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: SetNoteArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;

        // Pre-compute colors once
        let bg_color: Option<Option<Color3>> = if let Some(maybe_hex) = &kwargs.bg {
            Some(try_hex_to_color3(maybe_hex.clone())?)
        } else {
            None
        };
        let fg_color: Option<Option<Color3>> = if let Some(maybe_hex) = &kwargs.fg {
            Some(try_hex_to_color3(maybe_hex.clone())?)
        } else {
            None
        };

        let (start_col, end_col) = span.cols_start_end(&self.grid);

        for col in start_col..end_col {
            if let Some(text) = &kwargs.note {
                Self::acr_event_data_set_hnote_text(
                    &mut self.grid.header.cells,
                    col,
                    text.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(color) = &bg_color {
                Self::acr_event_data_set_hnote_bg(
                    &mut self.grid.header.cells,
                    col,
                    color.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(color) = &fg_color {
                Self::acr_event_data_set_hnote_fg(
                    &mut self.grid.header.cells,
                    col,
                    color.clone(),
                    &mut event_data,
                )?;
            }
            if let Some(readonly) = kwargs.readonly {
                Self::acr_event_data_set_hnote_readonly(
                    &mut self.grid.header.cells,
                    col,
                    readonly,
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
