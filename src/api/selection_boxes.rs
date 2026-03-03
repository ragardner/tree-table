use crate::EventData;
use crate::SelectionKind;
use crate::SelectionState;
use crate::Table;
use crate::args::AddSelectionArgs;
use crate::args::SelectRowsColsArgs;
use crate::args::SelectionClearArgs;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    /// Clear all selections.
    pub fn selection_clear(
        &mut self,
        kwargs: Option<SelectionClearArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        if self.sel.selected.is_none() && self.sel.selection_boxes.is_empty() {
            return Ok(None);
        }
        let kwargs: SelectionClearArgs = kwargs.unwrap_or_default();
        let mut event_data: EventData = self.cr_new_event(false);

        let mut boxes = Self::acr_new_change_with_old_boxes(&self.sel);
        Self::acr_deselect_all(&mut self.sel);
        Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
        event_data.changes.push(boxes);

        if !event_data.changes.is_empty() {
            self.cr_event_modified(true)?;
            let shared = Rc::new(RefCell::new(event_data));
            if self.opts.enabled_undo && kwargs.undo && self.grid.undo_stack.get_maxlen() > 0 {
                self.grid.undo_stack.push_back(shared.clone());
            }
            Ok(kwargs.emit.then_some(shared))
        } else {
            Ok(None)
        }
    }

    /// Create a selection box using a span. Uses displayed indices.
    pub fn select(
        &mut self,
        kwargs: Option<AddSelectionArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: AddSelectionArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(false);

        let mut boxes = Self::acr_new_change_with_old_boxes(&self.sel);
        Self::acr_add_selection_box(
            &mut self.sel,
            &self.grid,
            span.row,
            span.end_row,
            span.col,
            span.end_col,
            span.kind(),
            SelectionState::Normal,
            kwargs.current,
        );
        Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
        event_data.changes.push(boxes);

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    /// Select rows using either item ids or row numbers. Uses data indices.
    pub fn select_rows(
        &mut self,
        kwargs: Option<SelectRowsColsArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: SelectRowsColsArgs = kwargs.unwrap_or(SelectRowsColsArgs {
            items: self
                .grid
                .index
                .cells
                .iter()
                .map(|c| c.iid.to_string())
                .collect(),
            replace: false,
            current: true,
            emit: true,
            undo: true,
        });
        if kwargs.items.is_empty() {
            return Ok(None);
        }

        // Convert iids / numbers to numbers
        let mut row_nums = Vec::new();
        for iid in kwargs.items.into_iter() {
            row_nums.push(Self::acr_pos(&self.grid.index.iid_to_row, &iid)?);
        }
        row_nums.sort_unstable();
        row_nums.dedup();

        let mut event_data: EventData = self.cr_new_event(false);
        let mut boxes = Self::acr_new_change_with_old_boxes(&self.sel);

        // If we're using tree mode then we need to expand rows
        if self.opts.tree_mode {
            let mut to_open: HashSet<usize> = HashSet::new();
            for rn in row_nums.iter() {
                // Is not visible and has a parent then add ancestors to to_open
                if !Self::acr_row_is_vis(&self.grid.index.cells, *rn)?
                    && Self::acr_row_has_par(&self.grid.index.cells, *rn)?
                {
                    let iid = Self::acr_borrow_riid(&self.grid.index.cells, *rn)?;
                    for anc_res in self.grid.index.ancestor_rows(&iid) {
                        to_open.insert(anc_res?);
                    }
                }
            }
            if !to_open.is_empty() {
                let to_show =
                    self.cr_tree_open_close(to_open.into_iter(), true, Some(&mut event_data))?;
                Self::acr_set_row_vis(
                    &mut self.grid.index.cells,
                    to_show.into_iter(),
                    true,
                    Some(&mut event_data),
                )?;
            }
        // If we're not using tree mode we just need to show items
        } else {
            Self::acr_set_row_vis(
                &mut self.grid.index.cells,
                row_nums.iter().map(|n| *n),
                true,
                Some(&mut event_data),
            )?;
        }

        // We have to remake canvas positions because items may have been unhidden
        Self::acr_remake_rows(
            &mut self.grid.canvas_rows,
            &mut self.grid.disp_rows,
            &self.grid.index.cells,
            self.opts.horizontal_grid_line_width.clone(),
        );

        // Clear current selections if kwargs.replace
        if kwargs.replace {
            Self::acr_deselect_all(&mut self.sel);
        }

        // Select the items, converting data item indices to displayed
        for (start, end) in consecutive_ranges_owned(
            row_nums
                .iter()
                .filter_map(|n| Self::acr_data_to_disp_opt(&self.grid.disp_rows, *n)),
        ) {
            Self::acr_add_selection_box(
                &mut self.sel,
                &self.grid,
                start,
                Some(end),
                0,
                None,
                SelectionKind::Rows,
                SelectionState::Normal,
                kwargs.current,
            );
        }

        // Update event_data with new selections
        Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
        event_data.changes.push(boxes);

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    /// Select cols using either item ids or col numbers. Uses data indices.
    pub fn select_cols(
        &mut self,
        kwargs: Option<SelectRowsColsArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        // Deserialize kwargs
        let kwargs: SelectRowsColsArgs = kwargs.unwrap_or(SelectRowsColsArgs {
            items: self
                .grid
                .header
                .cells
                .iter()
                .map(|c| c.iid.to_string())
                .collect(),
            replace: false,
            current: true,
            emit: true,
            undo: true,
        });
        if kwargs.items.is_empty() {
            return Ok(None);
        }

        // Convert iids / numbers to numbers
        let mut items = Vec::new();
        for iid in kwargs.items.into_iter() {
            items.push(Self::acr_pos(&self.grid.header.iid_to_col, &iid)?);
        }
        items.sort_unstable();
        items.dedup();

        let mut event_data: EventData = self.cr_new_event(false);
        let mut boxes = Self::acr_new_change_with_old_boxes(&self.sel);

        // Unhide any currently hidden items
        Self::acr_set_col_vis(
            &mut self.grid.header.cells,
            items.iter().map(|n| *n),
            true,
            Some(&mut event_data),
        )?;

        // We have to remake canvas positions because items may have been unhidden
        Self::acr_remake_cols(
            &mut self.grid.canvas_cols,
            &mut self.grid.disp_cols,
            &self.grid.header.cells,
            self.opts.vertical_grid_line_width.clone(),
        );

        // Clear current selections if kwargs.replace
        if kwargs.replace {
            Self::acr_deselect_all(&mut self.sel);
        }

        // Select the items, converting data item indices to displayed
        for (start, end) in consecutive_ranges_owned(
            items
                .iter()
                .filter_map(|n| Self::acr_data_to_disp_opt(&self.grid.disp_cols, *n)),
        ) {
            Self::acr_add_selection_box(
                &mut self.sel,
                &self.grid,
                0,
                None,
                start,
                Some(end),
                SelectionKind::Cols,
                SelectionState::Normal,
                kwargs.current,
            );
        }

        // Update event_data with new selections
        Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
        event_data.changes.push(boxes);

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
