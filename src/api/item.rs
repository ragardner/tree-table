use std::ops::DerefMut;

use crate::Table;
use crate::Val;
use crate::args::{ItemArgs, ItemResult};
use crate::str_err;

impl Table {
    /// Modify a table item or get information about a table item.
    pub fn item(&self, kwargs: ItemArgs) -> Result<ItemResult, String> {
        let mut iid: String = kwargs.item;
        let (mut grid, opts, sel) = self.cr_mut_borrows();
        let mut grid = grid.deref_mut();
        let row = Self::acr_pos(&grid.index.iid_to_row, &iid)?;
        let mut event_data = self.cr_new_event(true, &opts, &sel);
        // Deal with open parameter, edits event_data.index_cells
        if let Some(open) = kwargs.open {
            Self::acr_tree_iid_set_open(
                &mut grid.index.cells,
                &grid.index.iid_to_row,
                &iid,
                open,
                Some(&mut event_data),
            )?;
            if !Self::acr_tree_iid_chn(&grid.index.cells, &grid.index.iid_to_row, &iid)?.is_empty()
                && Self::acr_row_iid_is_vis(&grid.index.cells, &grid.index.iid_to_row, &iid)?
            {
                Self::acr_tree_descendants_set_vis(
                    &mut grid.index.cells,
                    &grid.index.iid_to_row,
                    &iid,
                    open,
                    Some(&mut event_data),
                )?;
                Self::acr_remake_rows(
                    &mut grid.canvas_rows,
                    &mut grid.disp_rows,
                    &grid.index.cells,
                );
            }
        }
        // Deal with val parameter, edits event_data.index_cells
        if let Some(val) = kwargs.val {
            Self::acr_event_data_fmt_set_ival(&mut grid.index.cells, row, val, &mut event_data)?;
        }

        let empty_val = &opts.empty_val;

        // Deal with row values parameter, edits event_data.table_cells
        if let Some(vals) = kwargs.vals {
            let table_ncols = Self::acr_total_cols(&grid.header.cells);
            for (key, val) in vals.into_iter() {
                let col = Self::acr_pos(&grid.header.iid_to_col, &key)?;
                if col >= table_ncols {
                    return Err(str_err!(format!(
                        "Col does not exist. Col: {}, Table total: {}",
                        col, table_ncols,
                    )));
                }
                Self::acr_event_data_fmt_set_tval(
                    &mut grid.index.cells,
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
            if grid.index.iid_to_row.get(&new_name).is_some() {
                return Err(str_err!(format!(
                    "Cannot rename iid, new name already exists: {}",
                    new_name
                )));
            }
            Self::acr_tree_rename_iid(&mut grid.index, &iid, new_name.clone(), &mut event_data)?;
            iid = new_name;
        }
        // Deal with undo stack
        if !event_data.table_cells.is_empty() || !event_data.index_cells.is_empty() {
            self.cr_event_modified(&mut grid, &self.events.borrow(), &event_data, true, true)?;
            if opts.enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }

        Ok(ItemResult {
            val: (*Self::acr_icell_val(
                &grid.index.cells,
                Self::acr_pos(&grid.index.iid_to_row, &iid)?,
            )?)
            .clone(),
            values: (0..Self::acr_total_cols(&grid.header.cells))
                .map(|col| {
                    Self::acr_tcell_val(&grid.index.cells, row, col)
                        .map_or(empty_val.clone(), |r| (*r).clone())
                })
                .collect::<Vec<Val>>(),
            open: Self::acr_tree_iid_is_open(&grid.index.cells, &grid.index.iid_to_row, &iid)?,
        })
    }
}
