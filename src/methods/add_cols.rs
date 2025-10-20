use crate::CellProps;
use crate::EventData;
use crate::HeaderCell;
use crate::MaxMap;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::statics::DEFAULT_INHERIT;
use crate::str_err;
use std::collections::HashMap;

impl Table {
    pub(crate) fn cr_add_cols(
        &self,
        cols: HashMap<usize, MaxMap<TableCell>>,
        header: Vec<(usize, HeaderCell)>,
        sorted_seq: Vec<usize>,
        event_data: &mut EventData,
        mod_sel: bool,
        remake_cols: bool,
    ) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        // insert into table data
        if !cols.is_empty() {
            grid.index.add_cols(Some(cols), &sorted_seq);
        }

        // push outstanding col numbers
        grid.push_cols(&sorted_seq);

        // insert into header data
        if !header.is_empty() {
            grid.header.add_cols(header);
        }

        // remake canvas relevant vars
        if remake_cols {
            Self::acr_remake_cols(
                &mut grid.canvas_cols,
                &mut grid.disp_cols,
                &grid.header.cells,
            );
        }

        // Update event_data
        event_data.added_cols = sorted_seq;
        if mod_sel {
            let sel = self.sel.borrow();
            event_data.selection_boxes = sel.selection_boxes.clone();
            event_data.selected = sel.selected.clone();
        }
        Ok(())
    }

    pub(crate) fn cr_add_cols_get_args(
        &self,
        iids: Vec<String>,
        mut pos: usize,
        inherit: &str,
        vis: bool,
    ) -> Result<
        (
            HashMap<usize, MaxMap<TableCell>>,
            Vec<(usize, HeaderCell)>,
            Vec<usize>,
        ),
        String,
    > {
        let grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();
        let nrows = self.cr_total_rows();
        if pos as usize > self.cr_total_cols() as usize {
            pos = self.cr_total_cols();
        }
        let end = pos.checked_add(iids.len()).ok_or(str_err!("Max. cols"))?;
        let mut cols: HashMap<usize, MaxMap<TableCell>> = HashMap::new();
        let mut header: Vec<(usize, HeaderCell)> = Vec::new();
        let mut sorted_seq: Vec<usize> = Vec::new();
        let empty_val: &Val = &opts.empty_val;
        let opt_inherit_col = self.cr_get_inherit_col(pos, inherit)?;
        let do_inherit = opt_inherit_col.is_some();
        let inherit_col: usize = opt_inherit_col.unwrap_or(0);
        let header_props: Option<CellProps> = if do_inherit {
            grid.header.clone_props(&inherit_col)
        } else {
            None
        };
        let default_dim = opts.default_col_dim;

        // Create cols and header
        for (cn, iid) in (pos..end).zip(iids.into_iter()) {
            sorted_seq.push(cn);
            // Table
            if do_inherit {
                // Only insert to table if there are cell props, maintain sparse data
                let mut rows_map: MaxMap<TableCell> = MaxMap::new();
                for rn in 0..nrows {
                    if let Some(props) = grid.index.clone_table_props(&rn, &inherit_col) {
                        rows_map.set_cell(
                            rn,
                            TableCell {
                                val: empty_val.clone(),
                                props: Some(props),
                            },
                        );
                    }
                }
                if !rows_map.is_empty() {
                    cols.insert(cn, rows_map);
                }
            }
            // Header
            header.push((
                cn,
                HeaderCell {
                    val: Val::Str(iid.clone()),
                    iid,
                    dim: default_dim,
                    vis,
                    props: header_props.clone(),
                },
            ));
        }
        Ok((cols, header, sorted_seq))
    }

    pub(crate) fn cr_ensure_widget_ncols(
        &self,
        max_col: usize,
        event_data: &mut EventData,
        remake_cols: bool,
    ) -> Result<usize, String> {
        let grid = &mut *self.grid.borrow_mut();
        let table_ncols = self.cr_total_cols();
        let cols_to_add: usize = if max_col + 1 > table_ncols {
            (max_col + 1) - table_ncols
        } else {
            0
        };
        if cols_to_add > 0 {
            let iids = (0..cols_to_add)
                .map(|_| grid.header.new_iid())
                .collect::<Result<Vec<String>, String>>()?;
            let (cols, header, sorted_seq) =
                self.cr_add_cols_get_args(iids, self.cr_total_cols(), DEFAULT_INHERIT, true)?;
            self.cr_add_cols(cols, header, sorted_seq, event_data, false, remake_cols)?;
        };
        Ok(cols_to_add)
    }
}
