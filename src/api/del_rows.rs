use crate::args::DelArgs;
use crate::args::args_del::DelPosArgs;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use crate::{EventData, Table};
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    /// Delete rows from the table.
    pub fn del_rows(
        &mut self,
        kwargs: Option<DelArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells);
        if num_rows == 0 {
            return Ok(None);
        }

        let kwargs = kwargs.unwrap_or(DelArgs {
            items: Some(vec![
                Self::acr_borrow_riid(&self.grid.index.cells, num_rows.saturating_sub(1))?
                    .to_string(),
            ]),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        });

        let items = kwargs.items.unwrap_or(vec![
            Self::acr_borrow_riid(&self.grid.index.cells, num_rows.saturating_sub(1))?.to_string(),
        ]);

        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let mut positions: Vec<usize> = Vec::new();

        for iid in items.iter() {
            positions.push(Self::acr_pos(&self.grid.index.iid_to_row, iid)?);
        }

        if positions.is_empty() {
            return Ok(None);
        }
        positions.sort_unstable();
        positions.dedup();

        let deleted_disp = Self::acr_compute_deleted_disp_rows(&self.grid.disp_rows, &positions);
        self.cr_del_rows(positions, &mut event_data, true)?;
        self.cr_adjust_selections_after_row_delete(&deleted_disp);
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    /// Delete rows from the table using data positions (indices).
    pub fn del_rows_pos(
        &mut self,
        kwargs: Option<DelPosArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells);
        if num_rows == 0 {
            return Ok(None);
        }

        let kwargs = kwargs.unwrap_or(DelPosArgs {
            items: Some(vec![num_rows.saturating_sub(1)]),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        });

        let mut positions: Vec<usize> = if let Some(items) = kwargs.items {
            items.into_iter().filter(|&p| p < num_rows).collect()
        } else {
            vec![num_rows.saturating_sub(1)]
        };

        if positions.is_empty() {
            return Ok(None);
        }
        positions.sort_unstable();
        positions.dedup();

        let mut event_data = self.cr_new_event(kwargs.save_selection);
        let deleted_disp = Self::acr_compute_deleted_disp_rows(&self.grid.disp_rows, &positions);
        self.cr_del_rows(positions, &mut event_data, true)?;
        self.cr_adjust_selections_after_row_delete(&deleted_disp);
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
