use crate::Change;
use crate::Coords;
use crate::EventData;
use crate::IndexCell;
use crate::MaxMap;
use crate::SelectionBox;
use crate::SelectionKind;
use crate::Table;
use crate::TableCell;
use crate::TableGrid;
use crate::Val;
use crate::statics::DEFAULT_INHERIT;
use crate::str_err;
use crate::utils::utils_fns::binary_search_ins_pos;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_add_rows(
        &mut self,
        index: Vec<(usize, IndexCell)>,
        sorted_seq: Vec<usize>,
        event_data: &mut EventData,
        mod_tree: bool,
        remake_rows: bool,
    ) -> Result<(), String> {
        if mod_tree {
            Self::acr_tree_add_rows(&mut self.grid, &index, event_data)?;
        }

        // push outstanding row numbers
        self.grid.push_rows(&sorted_seq);

        // insert into index data
        if !index.is_empty() {
            self.grid.index.add_rows(index);
        }

        // remake canvas relevant vars
        if remake_rows {
            Self::acr_remake_rows(
                &mut self.grid.canvas_rows,
                &mut self.grid.disp_rows,
                &self.grid.index.cells,
                self.opts.horizontal_grid_line_width.clone(),
            );
        }

        // Update event_data, but not selections
        event_data.changes.push(Change::AddRows(sorted_seq));
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_compute_new_rows_visibility(
        &self,
        data_pos: usize,
        requested_vis: bool,
    ) -> bool {
        if !self.opts.tree_mode {
            return requested_vis;
        }

        // Tree mode: new rows are visible only if their parent is visible *and* open
        match Self::acr_tree_row_par_cell(
            &self.grid.index.cells,
            &self.grid.index.iid_to_row,
            data_pos,
        ) {
            Ok(Some(par_cell)) => par_cell.vis && par_cell.open,
            _ => true, // no parent or error → top-level → always visible
        }
    }

    pub(crate) fn cr_add_rows_get_args(
        &self,
        iids: Vec<String>,
        mut pos: usize,
        inherit: &str,
        show_new_rows: bool,
    ) -> Result<(Vec<(usize, IndexCell)>, Vec<usize>), String> {
        let ncols = Self::acr_total_cols(&self.grid.header.cells);
        if pos > Self::acr_total_rows(&self.grid.index.cells) {
            pos = Self::acr_total_rows(&self.grid.index.cells);
        }
        let end = pos.checked_add(iids.len()).ok_or(str_err!("Max. rows"))?;
        let mut index: Vec<(usize, IndexCell)> = Vec::new();
        let mut sorted_seq: Vec<usize> = Vec::new();
        let empty_val: &Val = &self.opts.empty_val;
        let empty_index_val: &Val = &self.opts.empty_val;
        let opt_inherit_row = self.cr_get_inherit_row(&self.grid.index.iid_to_row, pos, inherit)?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row: usize = opt_inherit_row.unwrap_or(0);
        let index_props = if do_inherit {
            self.grid.index.clone_props(&inherit_row)
        } else {
            None
        };
        let default_dim = self.cr_get_default_row_dim();
        let tree_mode = self.opts.tree_mode;

        // Create index
        for (rn, iid) in (pos..end).zip(iids.into_iter()) {
            sorted_seq.push(rn);
            let mut cols_map: MaxMap<TableCell> = MaxMap::new();
            // Table
            if do_inherit {
                // Only insert to table if there are cell props, maintain sparse data
                for cn in 0..ncols {
                    if let Some(props) = self.grid.index.clone_table_props(&inherit_row, &cn) {
                        cols_map.set_cell(
                            cn,
                            TableCell {
                                val: empty_val.clone(),
                                props: Some(props),
                            },
                        );
                    }
                }
            }
            let val = if tree_mode {
                Val::Str(iid.clone())
            } else {
                empty_index_val.clone()
            };
            let par =
                Self::acr_tree_iid_par(&self.grid.index.cells, &self.grid.index.iid_to_row, &iid)?;
            index.push((
                rn,
                IndexCell {
                    iid: Rc::from(iid),
                    val,
                    vals: cols_map,
                    dim: default_dim,
                    vis: show_new_rows,
                    par: par,
                    chn: Vec::new(),
                    open: false,
                    props: index_props.clone(),
                },
            ));
        }
        Ok((index, sorted_seq))
    }

    /// Restores cells when undo/redo OR modifies parents children if adding rows.
    pub(crate) fn acr_tree_add_rows(
        grid: &mut TableGrid,
        index_cells: &[(usize, IndexCell)],
        event_data: &mut EventData,
    ) -> Result<(), String> {
        // e.g. when undoing a del_rows change
        if !event_data.tree_changes.is_empty() {
            Self::acr_tree_restore_cells(&mut grid.index, &mut event_data.tree_changes, None)?;
            return Ok(());
        }
        // Don't record to event data because when
        // del_rows occurs it does record to event data AND
        // it also modifies the chn and pars
        for (row, cell) in index_cells.iter() {
            let par = &cell.par;
            if !par.is_empty() {
                // Parent's children
                let chn =
                    Self::acr_tree_iid_chn_mut(&mut grid.index.cells, &grid.index.iid_to_row, par)?;
                // Parent has children, find Vec position to add new
                if !chn.is_empty() {
                    let pos = binary_search_ins_pos(&chn, &grid.index.iid_to_row, *row)?;
                    chn.insert(pos, cell.iid.clone());

                // Parent has NO children, extend with new
                } else {
                    chn.push(cell.iid.clone());
                }
            }
        }
        Ok(())
    }

    pub(crate) fn cr_ensure_widget_nrows(
        &mut self,
        max_row: usize,
        event_data: &mut EventData,
        remake_rows: bool,
    ) -> Result<usize, String> {
        let table_nrows = Self::acr_total_rows(&self.grid.index.cells);
        let rows_to_add: usize = if max_row + 1 > table_nrows {
            (max_row + 1) - table_nrows
        } else {
            0
        };
        if rows_to_add > 0 {
            let iids: Vec<String> = (0..rows_to_add)
                .map(|_| self.grid.index.new_iid())
                .collect::<Result<Vec<String>, String>>()?;
            let cr_add_rows_pos = Self::acr_total_rows(&self.grid.index.cells);
            let (index, sorted_seq) =
                self.cr_add_rows_get_args(iids, cr_add_rows_pos, DEFAULT_INHERIT, true)?;
            self.cr_add_rows(index, sorted_seq, event_data, false, remake_rows)?;
        };
        Ok(rows_to_add)
    }

    pub(crate) fn acr_adjust_selections_after_row_insert(
        &mut self,
        disp_insert_pos: usize, // insertion point in DISPLAYED coordinates
        num_visible_added: usize,
    ) {
        if num_visible_added == 0 {
            return;
        }

        // We may append while iterating → only process the boxes that existed before this call
        let original_len = self.sel.selection_boxes.len();
        let mut i = 0;

        while i < original_len {
            let b = &mut self.sel.selection_boxes[i];

            if b.kind == SelectionKind::Cols {
                i += 1;
                continue;
            }

            let start = b.coords.row;
            let end = b.coords.end_row.unwrap_or(start); // inclusive

            if end < disp_insert_pos {
                // completely before insertion point → untouched
            } else if start >= disp_insert_pos {
                // completely at/after insertion point → shift whole box
                b.coords.row += num_visible_added;
                if let Some(er) = &mut b.coords.end_row {
                    *er += num_visible_added;
                }
            } else {
                // straddles the insertion point → split the box
                // (new rows are never selected)
                let left_end = disp_insert_pos.saturating_sub(1);

                if start <= left_end {
                    // shorten existing box to left part (inclusive)
                    b.coords.end_row = Some(left_end);

                    // append new box for right part (shifted)
                    let right_start = disp_insert_pos + num_visible_added;
                    let right_end = end + num_visible_added;

                    let new_coords = Coords::new(
                        right_start,
                        Some(right_end), // always inclusive — works for single or multi
                        b.coords.col,
                        b.coords.end_col,
                    );

                    let kind = b.kind.clone();
                    let state = b.state.clone();
                    self.sel.selection_boxes.push(SelectionBox {
                        coords: new_coords,
                        kind,
                        state,
                    });
                }
            }
            i += 1;
        }

        // Adjust the focused cell (the "cursor")
        // Shifts even on full-column selections
        if let Some(sel) = &mut self.sel.selected {
            if sel.row >= disp_insert_pos {
                sel.row += num_visible_added;
                sel.coords.row += num_visible_added;
                if let Some(er) = &mut sel.coords.end_row {
                    *er += num_visible_added;
                }
            }
        }
    }
}
