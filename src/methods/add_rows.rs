use crate::EventData;
use crate::IndexCell;
use crate::MaxMap;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::statics::DEFAULT_INHERIT;
use crate::str_err;

impl Table {
    pub(crate) fn cr_add_rows(
        &self,
        index: Vec<(usize, IndexCell)>,
        sorted_seq: Vec<usize>,
        event_data: &mut EventData,
        mod_sel: bool,
        mod_tree: bool,
        remake_rows: bool,
    ) -> Result<(), String> {
        let mut grid = &mut *self.grid.borrow_mut();
        if mod_tree {
            Self::acr_tree_add_rows(&mut grid, &index, event_data)?;
        }

        // push outstanding row numbers
        grid.push_rows(&sorted_seq);

        // insert into index data
        if !index.is_empty() {
            grid.index.add_rows(index);
        }

        // remake canvas relevant vars
        if remake_rows {
            Self::acr_remake_rows(
                &mut grid.canvas_rows,
                &mut grid.disp_rows,
                &grid.index.cells,
            );
        }

        // Update event_data
        event_data.added_rows = sorted_seq;
        if mod_sel {
            let sel = self.sel.borrow();
            event_data.selection_boxes = sel.selection_boxes.clone();
            event_data.selected = sel.selected.clone();
        }
        Ok(())
    }

    pub(crate) fn cr_add_rows_get_args(
        &self,
        iids: Vec<String>,
        mut pos: usize,
        inherit: &str,
        vis: bool,
    ) -> Result<(Vec<(usize, IndexCell)>, Vec<usize>), String> {
        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();
        let ncols = self.cr_total_cols();
        if pos as usize > self.cr_total_rows() as usize {
            pos = self.cr_total_rows();
        }
        let end = pos.checked_add(iids.len()).ok_or(str_err!("Max. rows"))?;
        let mut index: Vec<(usize, IndexCell)> = Vec::new();
        let mut sorted_seq: Vec<usize> = Vec::new();
        let empty_val: &Val = &opts.empty_val;
        let empty_index_val: &Val = &opts.empty_val;
        let opt_inherit_row = self.cr_get_inherit_row(pos, inherit)?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row: usize = opt_inherit_row.unwrap_or(0);
        let index_props = if do_inherit {
            grid.index.clone_props(&inherit_row)
        } else {
            None
        };
        let default_dim = Self::acr_get_default_row_dim(&opts);

        // Tree mode
        let tree_mode = opts.tree_mode;
        let mut par: String = "".to_string();
        let mut show_new_rows: bool = vis.clone();
        // Determine parent of insert pos & whether new rows will be visible
        if tree_mode {
            par = Self::acr_tree_row_par(&grid, &pos)?;
            if par.is_empty() {
                show_new_rows = true;
            } else {
                if let Some(par_rn) = grid.index.iid_to_row.get(&par) {
                    show_new_rows = Self::acr_tree_iid_displayed(
                        &grid.index.cells,
                        &grid.index.iid_to_row,
                        &par,
                    )? && Self::acr_tree_row_is_open(&grid.index.cells, par_rn)?;
                } else {
                    show_new_rows = false;
                }
            }
        }

        // Create index
        for (rn, iid) in (pos..end).zip(iids.into_iter()) {
            sorted_seq.push(rn);
            let mut cols_map: MaxMap<TableCell> = MaxMap::new();
            // Table
            if do_inherit {
                // Only insert to table if there are cell props, maintain sparse data
                for cn in 0..ncols {
                    if let Some(props) = grid.index.clone_table_props(&inherit_row, &cn) {
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
            index.push((
                rn,
                IndexCell {
                    iid,
                    val,
                    vals: cols_map,
                    dim: default_dim,
                    vis: show_new_rows,
                    par: par.clone(),
                    chn: Vec::new(),
                    open: false,
                    props: index_props.clone(),
                },
            ));
        }
        Ok((index, sorted_seq))
    }

    pub(crate) fn cr_ensure_widget_nrows(
        &self,
        max_row: usize,
        event_data: &mut EventData,
        remake_rows: bool,
    ) -> Result<usize, String> {
        let grid = &mut *self.grid.borrow_mut();
        let table_nrows = self.cr_total_rows();
        let rows_to_add: usize = if max_row + 1 > table_nrows {
            (max_row + 1) - table_nrows
        } else {
            0
        };
        if rows_to_add > 0 {
            let iids: Vec<String> = (0..rows_to_add)
                .map(|_| grid.index.new_iid())
                .collect::<Result<Vec<String>, String>>()?;
            let (index, sorted_seq) =
                self.cr_add_rows_get_args(iids, self.cr_total_rows(), DEFAULT_INHERIT, true)?;
            self.cr_add_rows(index, sorted_seq, event_data, false, false, remake_rows)?;
        };
        Ok(rows_to_add)
    }
}
