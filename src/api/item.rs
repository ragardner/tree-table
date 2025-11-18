use crate::Table;
use crate::Val;
use crate::args::{ItemArgs, ItemResult};
use crate::str_err;
use std::rc::Rc;

impl Table {
    /// Modify a table item or get information about a table item.
    pub fn item(&mut self, kwargs: ItemArgs) -> Result<ItemResult, String> {
        let mut iid: String = kwargs.item;
        let row = Self::acr_pos(&self.grid.index.iid_to_row, &iid)?;
        let mut event_data = self.cr_new_event(true);
        // Deal with open parameter, edits event_data.index_cells
        if let Some(open) = kwargs.open {
            Self::acr_tree_iid_set_open(
                &mut self.grid.index.cells,
                &self.grid.index.iid_to_row,
                &iid,
                open,
                Some(&mut event_data),
            )?;
            if !Self::acr_tree_iid_chn(&self.grid.index.cells, &self.grid.index.iid_to_row, &iid)?
                .is_empty()
                && Self::acr_row_iid_is_vis(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    &iid,
                )?
            {
                Self::acr_tree_descendants_set_vis(
                    &mut self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    &iid,
                    open,
                    Some(&mut event_data),
                )?;
                Self::acr_remake_rows(
                    &mut self.grid.canvas_rows,
                    &mut self.grid.disp_rows,
                    &self.grid.index.cells,
                    self.opts.horizontal_grid_line_width.clone(),
                );
            }
        }
        // Deal with val parameter, edits event_data.index_cells
        if let Some(val) = kwargs.val {
            Self::acr_event_data_fmt_set_ival(
                &mut self.grid.index.cells,
                row,
                val,
                &mut event_data,
            )?;
        }

        // Deal with row values parameter, edits event_data.table_cells
        if let Some(vals) = kwargs.vals {
            let empty_val = &self.opts.empty_val;
            let table_ncols = Self::acr_total_cols(&self.grid.header.cells);
            for (key, val) in vals.into_iter() {
                let col = Self::acr_pos(&self.grid.header.iid_to_col, &key)?;
                if col >= table_ncols {
                    return Err(str_err!(format!(
                        "Col does not exist. Col: {}, Table total: {}",
                        col, table_ncols,
                    )));
                }
                Self::acr_event_data_fmt_set_tval(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    val,
                    empty_val,
                    &mut event_data,
                )?;
            }
        }
        // Deal with iid parameter, id rename
        if let Some(new_name) = kwargs.iid {
            if self.grid.index.iid_to_row.get(new_name.as_str()).is_some() {
                return Err(str_err!(format!(
                    "Cannot rename iid, new name already exists: {}",
                    new_name
                )));
            }
            iid = new_name.clone();
            Self::acr_tree_rename_iid(
                &mut self.grid.index,
                &iid,
                Rc::from(new_name),
                &mut event_data,
            )?;
        }
        // Deal with undo stack
        if !event_data.table_cells.is_empty() || !event_data.index_cells.is_empty() {
            self.cr_event_modified(&event_data, true, true)?;
            if self.opts.enabled_undo && kwargs.undo {
                self.grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }

        let empty_val = &self.opts.empty_val;
        Ok(ItemResult {
            val: (*Self::acr_icell_val(
                &self.grid.index.cells,
                Self::acr_pos(&self.grid.index.iid_to_row, &iid)?,
            )?)
            .clone(),
            values: (0..Self::acr_total_cols(&self.grid.header.cells))
                .map(|col| {
                    Self::acr_tcell_val(&self.grid.index.cells, row, col)
                        .map_or(empty_val.clone(), |r| (*r).clone())
                })
                .collect::<Vec<Val>>(),
            open: Self::acr_tree_iid_is_open(
                &self.grid.index.cells,
                &self.grid.index.iid_to_row,
                &iid,
            )?,
        })
    }
}
