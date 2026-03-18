use crate::Change;
use crate::EventData;
use crate::Table;
use crate::args::TreeOpenArgs;
use crate::utils::consecutive_ranges::consecutive_ranges_borrowed;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    // Sets IndexCells to open/close and returns a Set of rows to show/hide.
    pub(crate) fn cr_tree_open_close<I>(
        &mut self,
        rows: I,
        open: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<Vec<usize>, String>
    where
        I: IntoIterator<Item = usize>,
    {
        // Rows to be shown after opening `rows`
        let mut to_show_or_hide: Vec<usize> = Vec::new();
        // Helper to determine if a row is visible
        let mut vis_check: HashSet<usize> = HashSet::new();

        match event_data {
            Some(event_data_mut) => {
                for rn in rows {
                    let cell = Self::acr_icell_mut(&mut self.grid.index.cells, rn)?;
                    event_data_mut.changes.push(Change::IndexCell {
                        row: rn,
                        change: cell.record_set_open(open),
                    });
                    if !cell.chn.is_empty() && cell.vis || vis_check.contains(&rn) {
                        for res in Self::acr_tree_descendants_rows(
                            &self.grid.index.cells,
                            &self.grid.index.iid_to_row,
                            rn,
                            true,
                        ) {
                            let did_row = res?;
                            if open {
                                vis_check.insert(did_row);
                            }
                            to_show_or_hide.push(did_row);
                        }
                    }
                }
            }
            None => {
                for rn in rows {
                    let cell = Self::acr_icell_mut(&mut self.grid.index.cells, rn)?;
                    cell.open = open;
                    if !cell.chn.is_empty() && cell.vis || vis_check.contains(&rn) {
                        for res in Self::acr_tree_descendants_rows(
                            &self.grid.index.cells,
                            &self.grid.index.iid_to_row,
                            rn,
                            true,
                        ) {
                            let did_row = res?;
                            if open {
                                vis_check.insert(did_row);
                            }
                            to_show_or_hide.push(did_row);
                        }
                    }
                }
            }
        }

        Ok(to_show_or_hide)
    }

    pub(crate) fn tree_set_open_or_closed(
        &mut self,
        kwargs: TreeOpenArgs,
        open: bool,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        // Adds selection boxes to event data
        let mut event_data: EventData = self.cr_new_event(true);

        // Adds rows arg to event data
        let to_show_or_hide = if kwargs.rows.is_empty() {
            let end_row = Self::acr_total_rows(&self.grid.index.cells);
            self.cr_tree_open_close((0..end_row).into_iter(), open, Some(&mut event_data))?
        } else {
            let mut rows = Vec::new();
            for iid in kwargs.rows.into_iter() {
                rows.push(Self::acr_pos(&self.grid.index.iid_to_row, &iid)?);
            }
            rows.sort_unstable();
            rows.dedup();
            self.cr_tree_open_close(rows.into_iter(), open, Some(&mut event_data))?
        };

        // Adds any descendants rows to event data
        if !to_show_or_hide.is_empty() {
            self.acr_adjust_selections_after_row_visibility_change(&to_show_or_hide, open);

            Self::acr_set_row_vis(
                &mut self.grid.index.cells,
                to_show_or_hide.into_iter(),
                open,
                Some(&mut event_data),
            )?;
            Self::acr_remake_rows(
                &mut self.grid.canvas_rows,
                &mut self.grid.disp_rows,
                &self.grid.index.cells,
                self.opts.horizontal_grid_line_width.clone(),
            );
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            false,
        )
    }

    pub(crate) fn acr_adjust_selections_after_row_visibility_change(
        &mut self,
        changed_data_rows: &[usize], // already sorted + deduped data indices
        becoming_visible: bool,
    ) {
        if changed_data_rows.is_empty() {
            return;
        }

        if becoming_visible {
            // Expand = multiple inserts into displayed list
            let mut cumulative_added = 0usize;
            for (start, end) in consecutive_ranges_borrowed(changed_data_rows.iter()) {
                let num = end - start + 1;
                let base_disp = Self::acr_compute_disp_insert_pos(&self.grid.disp_rows, start);
                let disp_insert_pos = base_disp + cumulative_added;
                self.acr_adjust_selections_after_row_insert(disp_insert_pos, num);
                cumulative_added += num;
            }
        } else {
            // Collapse
            self.cr_adjust_selections_after_row_delete(
                &changed_data_rows
                    .iter()
                    .filter_map(|&d| Self::acr_data_to_disp_row_opt(&self.grid.disp_rows, d))
                    .collect::<Vec<_>>(),
            );
        }
    }

    /// Sets items/rows to open. If used without args then all rows are opened.
    pub fn tree_open(
        &mut self,
        kwargs: Option<TreeOpenArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.tree_set_open_or_closed(kwargs.unwrap_or_default(), true)
    }

    /// Sets items/rows to closed. If used without args then all rows are closed.
    pub fn tree_close(
        &mut self,
        kwargs: Option<TreeOpenArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        self.tree_set_open_or_closed(kwargs.unwrap_or_default(), false)
    }
}
