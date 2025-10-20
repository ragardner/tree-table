use crate::CellProps;
use crate::EventData;
use crate::IndexCell;
use crate::MaxMap;
use crate::SelectionKind;
use crate::SelectionState;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::args::{InsertArgs, InsertRow};
use crate::statics::default_inherit;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use std::collections::HashSet;

impl Default for InsertArgs {
    fn default() -> Self {
        InsertArgs {
            rows: None,
            par: None,
            pos: None,
            inherit: default_inherit(),
            select: false,
            emit: true,
            undo: true,
        }
    }
}

impl Table {
    /// Insert a single new row into the table.
    pub fn insert(&self, kwargs: Option<InsertArgs>) -> Result<(), String> {
        let kwargs = kwargs.unwrap_or_default();
        let rows = kwargs.rows.unwrap_or(vec![InsertRow {
            iid: None,
            val: None,
            vals: None,
            open: false,
            dim: None,
        }]);

        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();
        let empty_val: Val = opts.empty_val.clone();
        let empty_index_val: Val = opts.empty_val.clone();
        let tree_mode = opts.tree_mode;

        let mut event_data: EventData = self.cr_new_event(true);
        let mut new_iids: HashSet<String> = HashSet::new();
        let dim = Self::acr_get_default_row_dim(&opts);
        let ncols = Self::acr_total_cols(&grid.header.cells);
        // If parent is None set to root level
        let par = kwargs.par.unwrap_or_default();

        let pos = match kwargs.pos {
            Some(pos) => Some(pos as usize),
            None => None,
        };
        // Determine data insert row number, Also checks if par exists
        let start_rn = self.cr_tree_get_iid_insert_row(pos, &par)?;

        // Determine which row to inherit cell props from
        let opt_inherit_row = self.cr_get_inherit_row(start_rn, kwargs.inherit.as_str())?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row = opt_inherit_row.unwrap_or(0);

        let chn_splice_pos: Option<usize> = if !par.is_empty() {
            let chn = Self::acr_tree_iid_chn(&grid.index.cells, &grid.index.iid_to_row, &par)?;
            // Parent has children, find Vec position to add new
            if !chn.is_empty() {
                let pos = chn
                    .iter()
                    .enumerate()
                    .find(|(_, cid)| {
                        grid.index
                            .iid_to_row
                            .get(*cid)
                            .map_or(false, |r| r >= &start_rn)
                    })
                    .map(|(i, _)| i)
                    .unwrap_or(chn.len());
                Some(pos)
            // Parent has NO children, extend with new
            } else {
                Some(Self::acr_tree_iid_chn(&grid.index.cells, &grid.index.iid_to_row, &par)?.len())
            }
        } else {
            None
        };
        let splice_chn: bool = chn_splice_pos.is_some();
        let mut chn_to_splice: Vec<String> = Vec::with_capacity(rows.len());
        let sorted_seq: Vec<usize> = (start_rn..start_rn + (rows.len())).into_iter().collect();
        // push existing hashmap row numbers
        grid.push_rows(&sorted_seq);
        grid.index.add_rows_just_push_hm(&sorted_seq);

        // Assess and create rows, index
        for (i, row_spec) in rows.into_iter().enumerate() {
            // Row number for new row
            let rn = start_rn + i;

            // If iid is None generate a new one
            let iid = if let Some(iid) = row_spec.iid {
                // Check if the provided iid already exists
                if grid.index.row(&iid).is_some() {
                    return Err(str_err!(format!("iid already exists: {}", iid)));
                }
                iid
            } else {
                grid.index.new_iid()?
            };
            // Check for repetitions of iids
            if !new_iids.insert(iid.clone()) {
                return Err(str_err!(format!("Repeated iid in rows: {}", iid)));
            };

            // Determine whether new rows will be visible
            let vis = if tree_mode {
                if par.is_empty() {
                    true
                } else {
                    if let Some(par_rn) = grid.index.iid_to_row.get(&par) {
                        Self::acr_tree_iid_displayed(
                            &grid.index.cells,
                            &grid.index.iid_to_row,
                            &par,
                        )? && Self::acr_tree_row_is_open(&grid.index.cells, par_rn)?
                    } else {
                        false
                    }
                }
            } else {
                true
            };

            // Index
            // If val is None use the iid
            let val = if let Some(val) = row_spec.val {
                val
            } else if tree_mode {
                Val::Str(iid.clone())
            } else {
                empty_index_val.clone()
            };
            let index_props: Option<CellProps> = if do_inherit {
                grid.index.clone_props(&inherit_row)
            } else {
                None
            };
            if splice_chn {
                chn_to_splice.push(iid.clone());
            }

            // Table
            let mut cols_map: MaxMap<TableCell> = MaxMap::new();
            // Do the cells provided by kwargs first
            // Then if inherit do the ones we haven't visited
            // Which can be identified by the cols not in cols_map
            if let Some(col_vals) = row_spec.vals {
                for (key, val) in col_vals.into_iter() {
                    let cn = Self::acr_resolve_col(&grid.header, &key)?;
                    let props = if do_inherit {
                        Self::acr_tcell_props(&grid.index.cells, &inherit_row, &cn).clone()
                    } else {
                        None
                    };
                    cols_map.set_cell(cn, TableCell { val, props });
                }
            }
            // Lastly if we're inheriting for this row we need to check for cells we've
            // missed that have props, don't overwrite
            if do_inherit {
                for cn in 0..ncols {
                    if !cols_map.contains_key(&cn) {
                        if let Some(props) =
                            Self::acr_tcell_props(&grid.index.cells, &inherit_row, &cn)
                        {
                            cols_map.set_cell(
                                cn,
                                TableCell {
                                    val: empty_val.clone(),
                                    props: Some(props.clone()),
                                },
                            );
                        }
                    }
                }
            }

            grid.index.add_row(
                rn,
                IndexCell {
                    val,
                    vals: cols_map,
                    dim: if let Some(rs_dim) = row_spec.dim {
                        rs_dim
                    } else {
                        dim
                    },
                    vis,
                    iid,
                    par: par.clone(),
                    chn: Vec::new(),
                    open: row_spec.open,
                    props: index_props,
                },
            );
        }

        // We don't need to record a tree change because undo -> del_rows() removes the spliced chn
        if let Some(chn_splice_pos) = chn_splice_pos {
            let chn =
                Self::acr_tree_iid_chn_mut(&mut grid.index.cells, &grid.index.iid_to_row, &par)?;
            chn.splice(chn_splice_pos..chn_splice_pos, chn_to_splice.into_iter());
        }

        // remake canvas relevant vars
        Self::acr_remake_rows(
            &mut grid.canvas_rows,
            &mut grid.disp_rows,
            &grid.index.cells,
        );
        // Update event_data
        let mut sel = self.sel.borrow_mut();
        event_data.added_rows = sorted_seq;
        event_data.selection_boxes = sel.selection_boxes.clone();
        event_data.selected = sel.selected.clone();

        // Select the added rows
        if kwargs.select {
            Self::acr_deselect_all(&mut sel);
            for (start, end) in
                consecutive_ranges_owned(event_data.added_rows.iter().filter_map(|n| {
                    if Self::acr_row_is_vis_opt(&grid.index.cells, n).is_some() {
                        Self::acr_data_to_disp_opt(&grid.disp_rows, *n)
                    } else {
                        None
                    }
                }))
            {
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

        // Sheet has been modified
        self.cr_event_modified(&event_data, kwargs.emit, true)?;

        // Add to undo stack if undo is enabled and undo is true in kwargs
        if opts.enabled_undo && kwargs.undo {
            grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
