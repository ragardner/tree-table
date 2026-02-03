use crate::CellProps;
use crate::EventData;
use crate::IndexCell;
use crate::SelectionKind;
use crate::SelectionState;
use crate::Table;
use crate::Val;
use crate::args::{InsertArgs, InsertRow};
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use crate::utils::utils_fns::binary_search_ins_pos;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    /// Insert multiple new rows into the table.
    /// If using tree mode then they can only be inserted under a single
    /// existing parent.
    pub fn insert(&mut self, kwargs: Option<InsertArgs>) -> Result<(), String> {
        let kwargs = kwargs.unwrap_or_default();
        let rows = kwargs.rows.unwrap_or(vec![InsertRow {
            iid: None,
            val: None,
            vals: None,
            open: false,
            dim: None,
        }]);

        if rows.is_empty() {
            return Ok(());
        }
        let empty_val: Val = self.opts.empty_val.clone();
        let empty_index_val: Val = self.opts.empty_index_val.clone();
        let tree_mode = self.opts.tree_mode;

        let mut event_data: EventData = self.cr_new_event(true);
        let mut new_iids: HashSet<Rc<str>> = HashSet::new();
        let dim = self.cr_get_default_row_dim();
        let ncols = Self::acr_total_cols(&self.grid.header.cells);
        // If parent is None set to root level
        let par = kwargs.par.unwrap_or_default();

        let pos = match kwargs.pos {
            Some(pos) => Some(pos as usize),
            None => None,
        };
        // Determine data insert row number, Also checks if par exists
        let start_rn = self.cr_tree_get_iid_insert_row(pos, &par)?;

        // Determine which row to inherit cell props from
        let opt_inherit_row = self.cr_get_inherit_row(
            &self.grid.index.iid_to_row,
            start_rn,
            kwargs.inherit.as_str(),
        )?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row = opt_inherit_row.unwrap_or(0);

        let chn_splice_pos: Option<usize> = if !par.is_empty() {
            let chn =
                Self::acr_tree_iid_chn(&self.grid.index.cells, &self.grid.index.iid_to_row, &par)?;
            let pos = binary_search_ins_pos(&chn, &self.grid.index.iid_to_row, start_rn)?;
            Some(pos)
        } else {
            None
        };
        // variable set up
        let splice_chn: bool = chn_splice_pos.is_some();
        let mut chn_to_splice: Vec<Rc<str>> = Vec::with_capacity(rows.len());
        let sorted_seq: Vec<usize> = (start_rn..start_rn + rows.len()).collect();

        // push existing hashmap row numbers
        self.grid.push_rows(&sorted_seq);
        self.grid.index.add_rows_just_push_hm(&sorted_seq);

        // get parent and determine new rows visibility
        let (vis, par_rc) = if tree_mode {
            if par.is_empty() {
                (true, self.grid.index.empty_par.clone())
            } else {
                let par_row = Self::acr_pos(&self.grid.index.iid_to_row, par.as_str())?;
                let par_cell = Self::acr_icell(&self.grid.index.cells, par_row)?;
                (par_cell.vis && par_cell.open, par_cell.iid.clone())
            }
        } else {
            (true, self.grid.index.empty_par.clone())
        };

        // Assess and create rows, index
        for (i, row_spec) in rows.into_iter().enumerate() {
            // Row number for new row
            let rn = start_rn + i;

            // If iid is None generate a new one
            let iid: Rc<str> = if let Some(iid) = row_spec.iid {
                // Check if the provided iid already exists
                if self.grid.index.iid_to_row.contains_key(iid.as_str()) {
                    return Err(str_err!(format!("iid already exists: {}", iid)));
                }
                Rc::from(iid)
            } else {
                Rc::from(self.grid.index.new_iid()?)
            };
            // Check for repetitions of iids
            if !new_iids.insert(iid.clone()) {
                return Err(str_err!(format!("Repeated iid in rows: {}", iid)));
            };

            // Index
            // If val is None use the iid
            let val = if let Some(val) = row_spec.val {
                val
            } else if tree_mode {
                Val::Str(iid.to_string())
            } else {
                empty_index_val.clone()
            };
            let index_props: Option<CellProps> = if do_inherit {
                self.grid.index.clone_props(&inherit_row)
            } else {
                None
            };

            // Table
            let new_icell = IndexCell {
                iid: iid.clone(),
                val,
                vals: Self::build_row_cells(
                    &self.grid.index.cells,
                    &self.grid.header,
                    row_spec.vals,
                    inherit_row,
                    do_inherit,
                    &empty_val,
                    ncols,
                )?,
                dim: row_spec.dim.unwrap_or(dim),
                vis,
                par: par_rc.clone(),
                chn: Vec::new(),
                open: row_spec.open,
                props: index_props,
            };

            if splice_chn {
                chn_to_splice.push(iid.clone());
            }

            self.grid.index.add_row(rn, iid, new_icell);
        }

        // We don't need to record a tree change because undo -> del_rows() removes the spliced chn
        if let Some(chn_splice_pos) = chn_splice_pos {
            let chn = Self::acr_tree_iid_chn_mut(
                &mut self.grid.index.cells,
                &self.grid.index.iid_to_row,
                &par,
            )?;
            chn.splice(chn_splice_pos..chn_splice_pos, chn_to_splice.into_iter());
        }

        // remake canvas relevant vars
        Self::acr_remake_rows(
            &mut self.grid.canvas_rows,
            &mut self.grid.disp_rows,
            &self.grid.index.cells,
            self.opts.horizontal_grid_line_width.clone(),
        );
        // Update event_data
        event_data.added_rows = sorted_seq;
        event_data.selection_boxes = self.sel.selection_boxes.clone();
        event_data.selected = self.sel.selected.clone();

        // Select the added rows
        if kwargs.select {
            Self::acr_deselect_all(&mut self.sel);
            for (start, end) in consecutive_ranges_owned(
                event_data
                    .added_rows
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
                    true,
                );
            }
        }

        // Sheet has been modified
        self.cr_event_modified(&event_data, kwargs.emit, true)?;

        // Add to undo stack if undo is enabled and undo is true in kwargs
        if self.opts.enabled_undo && kwargs.undo {
            self.grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
