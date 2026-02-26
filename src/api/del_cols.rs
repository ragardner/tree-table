use crate::args::DelArgs;
use crate::args::args_del::DelPosArgs;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use crate::{EventData, Table};
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    /// Delete cols from the table using item ids.
    pub fn del_cols(
        &mut self,
        kwargs: Option<DelArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_cols = Self::acr_total_cols(&self.grid.header.cells);
        if num_cols == 0 {
            return Ok(None);
        }

        let kwargs = kwargs.unwrap_or(DelArgs {
            items: Some(vec![
                Self::acr_borrow_ciid(&self.grid.header.cells, num_cols.saturating_sub(1))?
                    .to_string(),
            ]),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        });

        let items = kwargs.items.unwrap_or(vec![
            Self::acr_borrow_ciid(&self.grid.header.cells, num_cols.saturating_sub(1))?.to_string(),
        ]);

        let mut event_data = self.cr_new_event(false);
        let mut positions: Vec<usize> = Vec::new();

        for iid in items.iter() {
            positions.push(Self::acr_pos(&self.grid.header.iid_to_col, iid)?);
        }

        if positions.is_empty() {
            return Ok(None);
        } else {
            positions.sort_unstable();
            positions.dedup();
        }

        let deleted_disp = Self::acr_compute_deleted_disp_cols(&self.grid.disp_cols, &positions);
        let boxes = kwargs
            .save_selection
            .then_some(Self::acr_new_change_with_old_boxes(&self.sel));

        self.cr_del_cols(positions, &mut event_data)?;

        self.acr_adjust_selections_after_col_delete(&deleted_disp);

        if let Some(mut boxes) = boxes {
            Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
            event_data.changes.push(boxes);
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    /// Delete cols from the table using data positions (indices).
    pub fn del_cols_pos(
        &mut self,
        kwargs: Option<DelPosArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_cols = Self::acr_total_cols(&self.grid.header.cells);
        if num_cols == 0 {
            return Ok(None);
        }

        let kwargs = kwargs.unwrap_or(DelPosArgs {
            items: Some(vec![num_cols.saturating_sub(1)]),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        });

        let mut positions: Vec<usize> = if let Some(items) = kwargs.items {
            items.into_iter().filter(|&p| p < num_cols).collect()
        } else {
            vec![num_cols.saturating_sub(1)]
        };

        if positions.is_empty() {
            return Ok(None);
        } else {
            positions.sort_unstable();
            positions.dedup();
        }

        let mut event_data = self.cr_new_event(false);

        let deleted_disp = Self::acr_compute_deleted_disp_cols(&self.grid.disp_cols, &positions);
        let boxes = kwargs
            .save_selection
            .then_some(Self::acr_new_change_with_old_boxes(&self.sel));

        self.cr_del_cols(positions, &mut event_data)?;

        self.acr_adjust_selections_after_col_delete(&deleted_disp);

        if let Some(mut boxes) = boxes {
            Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
            event_data.changes.push(boxes);
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
