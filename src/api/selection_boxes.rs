use crate::EventData;
use crate::SelectionKind;
use crate::SelectionState;
use crate::SpanKey;
use crate::Table;
use crate::args::AddSelectionArgs;
use crate::args::SelectRowsColsArgs;
use crate::args::SelectionClearArgs;
use crate::statics::get_true;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use std::collections::HashSet;

impl Table {
    /// Clear all selections.
    pub fn selection_clear(&self, kwargs: Option<SelectionClearArgs>) -> Result<(), String> {
        let kwargs: SelectionClearArgs = kwargs.unwrap_or(SelectionClearArgs {
            emit: true,
            undo: get_true(),
        });
        let event_data: EventData = self.cr_new_event(true);
        Self::acr_deselect_all(&mut self.sel.borrow_mut());
        self.cr_event_select(&self.cr_new_event(true), kwargs.emit, true)?;
        if self.opts.borrow().enabled_undo && kwargs.undo {
            self.grid.borrow_mut().undo_stack.push_back(event_data);
        }
        // TODO: set refresh timer
        Ok(())
    }

    /// Create a selection box using a span. Uses displayed indices.
    pub fn select(&self, kwargs: Option<AddSelectionArgs>) -> Result<(), String> {
        let kwargs: AddSelectionArgs = kwargs.unwrap_or(AddSelectionArgs {
            key: SpanKey::None,
            current: true,
            emit: true,
            undo: true,
        });
        let span = self.cr_key_to_span(kwargs.key)?;
        let event_data: EventData = self.cr_new_event(true);
        Self::acr_add_selection_box(
            &mut self.sel.borrow_mut(),
            &self.grid.borrow(),
            span.row,
            span.end_row,
            span.col,
            span.end_col,
            span.kind(),
            SelectionState::Normal,
            kwargs.current,
        );
        self.cr_event_select(&self.cr_new_event(true), kwargs.emit, true)?;
        if self.opts.borrow().enabled_undo && kwargs.undo {
            self.grid.borrow_mut().undo_stack.push_back(event_data);
        }
        // TODO: set refresh timer
        Ok(())
    }

    /// Select rows using either item ids or row numbers. Uses data indices.
    pub fn select_rows(&self, kwargs: Option<SelectRowsColsArgs>) -> Result<(), String> {
        let kwargs: SelectRowsColsArgs = kwargs.unwrap_or(SelectRowsColsArgs {
            items: self
                .grid
                .borrow()
                .index
                .cells
                .iter()
                .map(|c| c.iid.clone())
                .collect(),
            replace: false,
            current: true,
            emit: true,
            undo: true,
        });
        if kwargs.items.is_empty() {
            return Ok(());
        }

        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();

        // Convert iids / numbers to numbers
        let mut row_nums = Vec::new();
        for iid in kwargs.items.into_iter() {
            row_nums.push(Self::acr_copy_pos(&grid.index.iid_to_row, &iid)?);
        }
        row_nums.sort_unstable();
        row_nums.dedup();

        let mut event_data: EventData = self.cr_new_event(false);

        // If we're using tree mode then we need to expand rows
        if opts.tree_mode {
            let mut to_open: HashSet<usize> = HashSet::new();
            for rn in row_nums.iter() {
                // Is not visible and has a parent then add ancestors to to_open
                if !Self::acr_row_is_vis(&grid.index.cells, rn)?
                    && !Self::acr_icell(&grid.index.cells, rn)?.par.is_empty()
                {
                    let iid = Self::acr_borrow_riid(&grid.index.cells, rn)?;
                    for anc_res in grid.index.ancestor_rows(iid) {
                        to_open.insert(anc_res?);
                    }
                }
            }
            if !to_open.is_empty() {
                let to_show =
                    self.cr_tree_open_close(to_open.into_iter(), true, Some(&mut event_data))?;
                Self::acr_set_row_vis(
                    &mut grid.index.cells,
                    to_show.into_iter(),
                    true,
                    Some(&mut event_data),
                )?;
            }
        // If we're not using tree mode we just need to show items
        } else {
            Self::acr_set_row_vis(
                &mut grid.index.cells,
                row_nums.iter().map(|n| *n),
                true,
                Some(&mut event_data),
            )?;
        }

        // We have to remake canvas positions because items may have been unhidden
        Self::acr_remake_rows(
            &mut grid.canvas_rows,
            &mut grid.disp_rows,
            &grid.index.cells,
        );

        // Save prior selection boxes for potential undo
        let mut sel = self.sel.borrow_mut();
        let prior_boxes = sel.selection_boxes.clone();
        let prior_sel = sel.selected.clone();

        // Clear current selections if kwargs.replace
        if kwargs.replace {
            Self::acr_deselect_all(&mut sel);
        }

        // Select the items, converting data item indices to displayed
        for (start, end) in consecutive_ranges_owned(
            row_nums
                .iter()
                .filter_map(|n| Self::acr_data_to_disp_opt(&grid.disp_rows, *n)),
        ) {
            Self::acr_add_selection_box(
                &mut sel,
                &grid,
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
        event_data.selection_boxes = sel.selection_boxes.clone();
        event_data.selected = sel.selected.clone();

        // Emit an event with the saved cells and current selection boxes
        self.cr_event_select(&event_data, kwargs.emit, true)?;

        // Add saved cells and prior boxes to undo stack
        if opts.enabled_undo && kwargs.undo {
            // Update event_data with prior selection boxes for the undo stack
            event_data.selection_boxes = prior_boxes;
            event_data.selected = prior_sel;
            grid.undo_stack.push_back(event_data);
        }
        // TODO: set refresh timer
        Ok(())
    }

    /// Select cols using either item ids or col numbers. Uses data indices.
    pub fn select_cols(&self, kwargs: Option<SelectRowsColsArgs>) -> Result<(), String> {
        // Deserialize kwargs
        let kwargs: SelectRowsColsArgs = kwargs.unwrap_or(SelectRowsColsArgs {
            items: self
                .grid
                .borrow()
                .header
                .cells
                .iter()
                .map(|c| c.iid.clone())
                .collect(),
            replace: false,
            current: true,
            emit: true,
            undo: true,
        });
        if kwargs.items.is_empty() {
            return Ok(());
        }

        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();

        // Convert iids / numbers to numbers
        let mut items = Vec::new();
        for iid in kwargs.items.into_iter() {
            items.push(Self::acr_copy_pos(&grid.header.iid_to_col, &iid)?);
        }
        items.sort_unstable();
        items.dedup();

        let mut event_data: EventData = self.cr_new_event(false);

        // Unhide any currently hidden items
        Self::acr_set_col_vis(
            &mut grid.header.cells,
            items.iter().map(|n| *n),
            true,
            Some(&mut event_data),
        )?;

        // We have to remake canvas positions because items may have been unhidden
        Self::acr_remake_cols(
            &mut grid.canvas_cols,
            &mut grid.disp_cols,
            &grid.header.cells,
        );

        // Save prior selection boxes for potential undo
        let mut sel = self.sel.borrow_mut();
        let prior_boxes = sel.selection_boxes.clone();
        let prior_sel = sel.selected.clone();

        // Clear current selections if kwargs.replace
        if kwargs.replace {
            Self::acr_deselect_all(&mut sel);
        }

        // Select the items, converting data item indices to displayed
        for (start, end) in consecutive_ranges_owned(
            items
                .iter()
                .filter_map(|n| Self::acr_data_to_disp_opt(&grid.disp_cols, *n)),
        ) {
            Self::acr_add_selection_box(
                &mut sel,
                &grid,
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
        event_data.selection_boxes = sel.selection_boxes.clone();
        event_data.selected = sel.selected.clone();

        // Emit an event with the saved cells and current selection boxes
        self.cr_event_select(&event_data, kwargs.emit, true)?;

        // Add saved cells and prior boxes to undo stack
        if opts.enabled_undo && kwargs.undo {
            // Update event_data with prior selection boxes for the undo stack
            event_data.selection_boxes = prior_boxes;
            event_data.selected = prior_sel;
            grid.undo_stack.push_back(event_data);
        }
        // TODO: set refresh timer
        Ok(())
    }
}
