use crate::EventData;
use crate::IndexCell;
use crate::MaxMap;
use crate::Options;
use crate::Selection;
use crate::Table;
use crate::TableCell;
use crate::TableGrid;
use crate::Val;
use crate::statics::DEFAULT_INHERIT;
use crate::str_err;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_add_rows(
        &self,
        mut grid: &mut TableGrid,
        sel: &Selection,
        index: Vec<(usize, Rc<RefCell<IndexCell>>)>,
        sorted_seq: Vec<usize>,
        event_data: &mut EventData,
        mod_sel: bool,
        mod_tree: bool,
        remake_rows: bool,
    ) -> Result<(), String> {
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
            event_data.selection_boxes = sel.selection_boxes.clone();
            event_data.selected = sel.selected.clone();
        }
        Ok(())
    }

    pub(crate) fn cr_add_rows_get_args(
        &self,
        grid: &mut TableGrid,
        opts: &Options,
        iids: Vec<String>,
        mut pos: usize,
        inherit: &str,
        vis: bool,
    ) -> Result<(Vec<(usize, Rc<RefCell<IndexCell>>)>, Vec<usize>), String> {
        let ncols = Self::acr_total_cols(&grid.header.cells);
        if pos > Self::acr_total_rows(&grid.index.cells) {
            pos = Self::acr_total_rows(&grid.index.cells);
        }
        let end = pos.checked_add(iids.len()).ok_or(str_err!("Max. rows"))?;
        let mut index: Vec<(usize, Rc<RefCell<IndexCell>>)> = Vec::new();
        let mut sorted_seq: Vec<usize> = Vec::new();
        let empty_val: &Val = &opts.empty_val;
        let empty_index_val: &Val = &opts.empty_val;
        let opt_inherit_row = self.cr_get_inherit_row(&grid.index.iid_to_row, pos, inherit)?;
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
        let mut show_new_rows: bool = vis.clone();
        // Determine parent of insert pos & whether new rows will be visible
        if tree_mode {
            let par_cell_res =
                Self::acr_tree_row_par_cell(&grid.index.cells, &grid.index.iid_to_row, pos);
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
            let par_cell = {
                let res =
                    Self::acr_tree_par_cell_weak(&grid.index.cells, &grid.index.iid_to_row, &iid);
                if let Ok(op) = res { op } else { None }
            };
            index.push((
                rn,
                Rc::new(RefCell::new(IndexCell {
                    iid,
                    val,
                    vals: cols_map,
                    dim: default_dim,
                    vis: show_new_rows,
                    par: par_cell,
                    chn: Vec::new(),
                    open: false,
                    props: index_props.clone(),
                })),
            ));
        }
        Ok((index, sorted_seq))
    }

    pub(crate) fn cr_ensure_widget_nrows(
        &self,
        mut grid: &mut TableGrid,
        opts: &Options,
        sel: &Selection,
        max_row: usize,
        event_data: &mut EventData,
        remake_rows: bool,
    ) -> Result<usize, String> {
        let table_nrows = Self::acr_total_rows(&grid.index.cells);
        let rows_to_add: usize = if max_row + 1 > table_nrows {
            (max_row + 1) - table_nrows
        } else {
            0
        };
        if rows_to_add > 0 {
            let iids: Vec<String> = (0..rows_to_add)
                .map(|_| grid.index.new_iid())
                .collect::<Result<Vec<String>, String>>()?;
            let cr_add_rows_pos = Self::acr_total_rows(&grid.index.cells);
            let (index, sorted_seq) = self.cr_add_rows_get_args(
                &mut grid,
                &opts,
                iids,
                cr_add_rows_pos,
                DEFAULT_INHERIT,
                true,
            )?;
            self.cr_add_rows(
                &mut grid,
                &sel,
                index,
                sorted_seq,
                event_data,
                false,
                false,
                remake_rows,
            )?;
        };
        Ok(rows_to_add)
    }
}
