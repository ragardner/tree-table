use crate::args::{AddArgs, AddNumArgs};
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use crate::{Change, Table};
use crate::{EventData, str_err};
use crate::{SelectionKind, SelectionState};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    /// Add new cols with the provided item ids to the Table.
    pub fn add_cols(
        &mut self,
        kwargs: Option<AddArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: AddArgs = kwargs.unwrap_or_default();
        let event_data = self.cr_new_event(false);
        let pos = if let Some(pos) = kwargs.pos {
            pos as usize
        } else {
            Self::acr_total_cols(&self.grid.header.cells)
        };
        let iids = kwargs.items.unwrap_or(vec![self.grid.header.new_iid()?]);
        let mut iid_set = HashSet::new();
        for iid in &iids {
            if !iid_set.insert(iid.clone()) {
                return Err(str_err!("Duplicate col iid: {}", iid));
            }
            if self.grid.header.iid_to_col.get(iid.as_str()).is_some() {
                return Err(str_err!("Col iid already exists: {}", iid));
            }
        }
        self._add_cols(
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

    /// Add a number of new cols with auto-generated item ids to the Table.
    pub fn add_num_cols(
        &mut self,
        kwargs: Option<AddNumArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: AddNumArgs = kwargs.unwrap_or_default();
        let event_data = self.cr_new_event(false);
        let pos = if let Some(pos) = kwargs.pos {
            pos as usize
        } else {
            Self::acr_total_cols(&self.grid.header.cells)
        };
        let num = kwargs.num.unwrap_or(1);
        let iids: Vec<String> = (0..num)
            .map(|_| self.grid.header.new_iid())
            .collect::<Result<_, _>>()?;
        self._add_cols(
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
    fn _add_cols(
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
        let (cols, header, sorted_seq) =
            self.cr_add_cols_get_args(iids.clone(), pos, inherit.as_str(), vis)?;

        let num_visible_added = if vis { sorted_seq.len() } else { 0 };
        let disp_insert_pos = Self::acr_compute_disp_insert_pos(&self.grid.disp_cols, pos);
        self.cr_add_cols(cols, header, sorted_seq, &mut event_data, true)?;

        let boxes = save_selection.then_some(Self::acr_new_change_with_old_boxes(&self.sel));
        self.acr_adjust_selections_after_col_insert(disp_insert_pos, num_visible_added);

        if select {
            Self::acr_deselect_all(&mut self.sel);
            for (start, end) in consecutive_ranges_owned(
                event_data
                    .changes
                    .iter()
                    .filter_map(|change| match change {
                        Change::AddCols(added) => Some(added.iter()),
                        _ => None,
                    })
                    .flatten()
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
                    true,
                );
            }
        }

        if let Some(mut boxes) = boxes {
            Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
            event_data.changes.push(boxes);
        }

        self.cr_finalize_event(event_data, undo, emit)
    }
}
