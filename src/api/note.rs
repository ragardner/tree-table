use crate::EventData;
use crate::Note;
use crate::NoteArgs;
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

        self.cr_finalize_event_nsc(event_data, kwargs.undo, kwargs.emit, true)
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
        self.cr_finalize_event_nsc(event_data, kwargs.undo, kwargs.emit, true)
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
        self.cr_finalize_event_nsc(event_data, kwargs.undo, kwargs.emit, true)
    }
}
