use alloc::{rc::Rc, string::String, vec, vec::Vec};
use core::cell::RefCell;
use hashbrown::HashSet;

use crate::args::{AddArgs, AddNumArgs};
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use crate::{Change, EventData, NO_END, Table, str_err};

impl Table {
    /// Add new rows with the provided item ids to the Table.
    pub fn add_rows(
        &mut self,
        kwargs: Option<AddArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: AddArgs = kwargs.unwrap_or_default();
        let event_data = self.cr_new_event(false);
        let pos = if let Some(pos) = kwargs.pos {
            pos as usize
        } else {
            Self::acr_total_rows(&self.grid.index.cells)
        };
        let iids = kwargs.items.unwrap_or(vec![self.grid.index.new_iid()?]);
        let mut iid_set = HashSet::new();
        for iid in &iids {
            if !iid_set.insert(iid.clone()) {
                return Err(str_err!("Duplicate row iid: {}", iid));
            }
            if self.grid.index.iid_to_row.get(iid.as_str()).is_some() {
                return Err(str_err!("Row iid already exists: {}", iid));
            }
        }
        self._add_rows(
            iids,
            pos,
            kwargs.inherit,
            kwargs.vis,
            kwargs.select,
            kwargs.undo,
            kwargs.emit,
            kwargs.save_selection,
            event_data,
        )
    }

    /// Add a number of new rows with auto-generated item ids to the Table.
    pub fn add_num_rows(
        &mut self,
        kwargs: Option<AddNumArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: AddNumArgs = kwargs.unwrap_or_default();
        let event_data = self.cr_new_event(false);
        let pos = if let Some(pos) = kwargs.pos {
            pos as usize
        } else {
            Self::acr_total_rows(&self.grid.index.cells)
        };
        let num = kwargs.num.unwrap_or(1);
        let iids: Vec<String> = (0..num)
            .map(|_| self.grid.index.new_iid())
            .collect::<Result<_, _>>()?;
        self._add_rows(
            iids,
            pos,
            kwargs.inherit,
            kwargs.vis,
            kwargs.select,
            kwargs.undo,
            kwargs.emit,
            kwargs.save_selection,
            event_data,
        )
    }

    #[inline]
    fn _add_rows(
        &mut self,
        iids: Vec<String>,
        pos: usize,
        inherit: String,
        vis: bool,
        select: bool,
        undo: bool,
        emit: bool,
        save_selection: bool,
        mut event_data: EventData,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let show_new_rows = self.acr_compute_new_rows_visibility(pos, vis);

        let (index, sorted_seq) =
            self.cr_add_rows_get_args(iids.clone(), pos, inherit.as_str(), show_new_rows)?;

        let num_visible_added = if show_new_rows { sorted_seq.len() } else { 0 };
        let disp_insert_pos = Self::acr_compute_disp_insert_pos(&self.grid.disp_rows, pos);

        self.cr_add_rows(index, sorted_seq, &mut event_data, true, true)?;
        self.acr_adjust_selections_after_row_insert(disp_insert_pos, num_visible_added);

        if select {
            Self::acr_deselect_all(
                &mut self.sel,
                if save_selection {
                    Some(&mut event_data)
                } else {
                    None
                },
            );
            for (start, end) in consecutive_ranges_owned(
                event_data
                    .changes
                    .iter()
                    .filter_map(|change| match change {
                        Change::AddRows(added) => Some(added.iter()),
                        _ => None,
                    })
                    .flatten()
                    .filter_map(|n| Self::acr_data_to_disp_opt(&self.grid.disp_rows, *n)),
            ) {
                Self::acr_add_selection_box(&mut self.sel, &self.grid, start, end, 0, NO_END, true);
            }
            if save_selection {
                event_data.changes.push(Change::add_selection_boxes(
                    self.sel.selection_boxes.len(),
                    None,
                ));
            }
        }

        self.cr_finalize_event(event_data, undo, emit)
    }
}
