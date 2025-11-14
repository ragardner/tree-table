use crate::EventData;
use crate::IndexCell;
use crate::MaxMap;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::statics::DEFAULT_INHERIT;
use crate::str_err;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_add_rows(
        &mut self,
        index: Vec<(usize, IndexCell)>,
        sorted_seq: Vec<usize>,
        event_data: &mut EventData,
        mod_sel: bool,
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

        // Update event_data
        event_data.added_rows = sorted_seq;
        if mod_sel {
            event_data.selection_boxes = self.sel.selection_boxes.clone();
            event_data.selected = self.sel.selected.clone();
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

        // Tree mode
        let tree_mode = self.opts.tree_mode;
        let mut show_new_rows: bool = vis.clone();
        // Determine parent of insert pos & whether new rows will be visible
        if tree_mode {
            let par_cell_res = Self::acr_tree_row_par_cell(
                &self.grid.index.cells,
                &self.grid.index.iid_to_row,
                pos,
            );
            if let Ok(par_cell_opt) = par_cell_res {
                if let Some(par_cell) = par_cell_opt {
                    show_new_rows = par_cell.vis && par_cell.open;
                } else {
                    show_new_rows = true
                }
            } else {
                show_new_rows = true
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
            self.cr_add_rows(index, sorted_seq, event_data, false, false, remake_rows)?;
        };
        Ok(rows_to_add)
    }
}
