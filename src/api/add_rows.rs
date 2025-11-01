use crate::args::{AddArgs, AddNumArgs};
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use crate::{EventData, log, str_err};
use crate::{Options, Selection, Table, TableGrid};
use crate::{SelectionKind, SelectionState};
use std::collections::HashSet;

impl Table {
    /// Add new rows with the provided item ids to the Table.
    pub fn add_rows(&self, kwargs: Option<AddArgs>) -> Result<Vec<String>, String> {
        let (mut grid, opts, mut sel) = self.cr_mut_borrows();
        let kwargs: AddArgs = kwargs.unwrap_or_default();
        let event_data = self.cr_new_event(false, &opts, &sel);
        let pos = if let Some(pos) = kwargs.pos {
            pos as usize
        } else {
            Self::acr_total_rows(&grid.index.cells)
        };
        let iids = kwargs.items.unwrap_or(vec![grid.index.new_iid()?]);
        let mut iid_set = HashSet::new();
        for iid in &iids {
            if !iid_set.insert(iid.clone()) {
                return Err(str_err!(format!("Duplicate row iid: {}", iid)));
            }
            if grid.index.iid_to_row.get(iid).is_some() {
                return Err(str_err!(format!("Row iid already exists: {}", iid)));
            }
        }
        self._add_rows(
            &mut grid,
            &opts,
            &mut sel,
            iids,
            pos,
            kwargs.inherit,
            kwargs.vis,
            kwargs.select,
            kwargs.undo,
            kwargs.emit,
            event_data,
        )
    }

    /// Add a number of new rows with auto-generated item ids to the Table.
    pub fn add_num_rows(&self, kwargs: Option<AddNumArgs>) -> Result<Vec<String>, String> {
        let (mut grid, opts, mut sel) = self.cr_mut_borrows();
        let kwargs: AddNumArgs = kwargs.unwrap_or_default();
        let event_data = self.cr_new_event(false, &opts, &sel);
        let pos = if let Some(pos) = kwargs.pos {
            pos as usize
        } else {
            Self::acr_total_rows(&grid.index.cells)
        };
        let num = kwargs.num.unwrap_or(1);
        let iids: Vec<String> = (0..num)
            .map(|_| grid.index.new_iid())
            .collect::<Result<_, _>>()?;
        self._add_rows(
            &mut grid,
            &opts,
            &mut sel,
            iids,
            pos,
            kwargs.inherit,
            kwargs.vis,
            kwargs.select,
            kwargs.undo,
            kwargs.emit,
            event_data,
        )
    }

    #[inline]
    fn _add_rows(
        &self,
        mut grid: &mut TableGrid,
        opts: &Options,
        mut sel: &mut Selection,
        iids: Vec<String>,
        pos: usize,
        inherit: String,
        vis: bool,
        select: bool,
        undo: bool,
        emit: bool,
        mut event_data: EventData,
    ) -> Result<Vec<String>, String> {
        let (index, sorted_seq) =
            self.cr_add_rows_get_args(&mut grid, &opts, iids.clone(), pos, inherit.as_str(), vis)?;

        self.cr_add_rows(
            &mut grid,
            &sel,
            index,
            sorted_seq,
            &mut event_data,
            true,
            true,
            true,
        )?;

        if select {
            Self::acr_deselect_all(&mut sel);
            for (start, end) in consecutive_ranges_owned(
                event_data
                    .added_rows
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
                    true,
                );
            }
        }
        self.cr_event_modified(&mut grid, &self.events.borrow(), &event_data, emit, true)?;
        if opts.enabled_undo && undo {
            grid.undo_stack.push_back(event_data);
        }
        Ok(iids)
    }
}
