use crate::EventData;
use crate::Table;
use crate::Val;
use crate::args::{ItemArgs, ItemResult};
use crate::args::{ItemColArgs, ItemColResult};
use crate::str_err;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    /// Modify a table item (row).
    pub fn item(&mut self, kwargs: ItemArgs) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let mut iid: String = kwargs.item;
        let row = Self::acr_pos(&self.grid.index.iid_to_row, &iid)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);
        // Deal with open parameter
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
        // Deal with val parameter
        if let Some(val) = kwargs.val {
            Self::acr_event_data_fmt_set_ival(
                &mut self.grid.index.cells,
                row,
                val,
                &mut event_data,
            )?;
        }

        // Deal with row values parameter
        if let Some(vals) = kwargs.vals {
            let empty_val = &self.opts.empty_val;
            let table_ncols = Self::acr_total_cols(&self.grid.header.cells);
            for (key, val) in vals.into_iter() {
                let col = Self::acr_pos(&self.grid.header.iid_to_col, &key)?;
                if col >= table_ncols {
                    return Err(str_err!(
                        "Col does not exist. Col: {}, Table total: {}",
                        col,
                        table_ncols,
                    ));
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
                return Err(str_err!(
                    "Cannot rename iid, new name already exists: {}",
                    new_name
                ));
            }
            iid = new_name.clone();
            Self::acr_tree_rename_iid(
                &mut self.grid.index,
                &iid,
                Rc::from(new_name),
                &mut event_data,
            )?;
        }

        self.cr_finalize_event_nsc(event_data, kwargs.undo, kwargs.emit, true)
    }

    /// Get information about an item (row).
    pub fn get_item(&mut self, iid: String) -> Result<ItemResult, String> {
        let row = Self::acr_pos(&self.grid.index.iid_to_row, &iid)?;
        let empty_val = &self.opts.empty_val;
        Ok(ItemResult {
            pos: row as u64,
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

    /// Modify a table column (header item).
    pub fn col(&mut self, kwargs: ItemColArgs) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let iid: String = kwargs.item;
        let col = Self::acr_pos(&self.grid.header.iid_to_col, &iid)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);

        // Deal with val parameter (header cell value / column label)
        if let Some(val) = kwargs.val {
            Self::acr_event_data_fmt_set_hval(
                &mut self.grid.header.cells,
                col,
                val,
                &mut event_data,
            )?;
        }

        // Deal with vals parameter — set table values *down this column*
        if let Some(vals) = kwargs.vals {
            let empty_val = &self.opts.empty_val;
            let table_nrows = Self::acr_total_rows(&self.grid.index.cells);
            for (key, val) in vals.into_iter() {
                let row = Self::acr_pos(&self.grid.index.iid_to_row, &key)?;
                if row >= table_nrows {
                    return Err(str_err!(
                        "Row does not exist. Row: {}, Table total: {}",
                        row,
                        table_nrows,
                    ));
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

        // Deal with iid parameter (rename)
        if let Some(new_name) = kwargs.iid {
            if self.grid.header.iid_to_col.get(new_name.as_str()).is_some() {
                return Err(str_err!(
                    "Cannot rename col, new name already exists: {}",
                    new_name
                ));
            }
            Self::acr_header_rename_iid(
                &mut self.grid.header,
                col,
                &iid,
                Rc::from(new_name.clone()),
                &mut event_data,
            )?;
        }

        self.cr_finalize_event_nsc(event_data, kwargs.undo, kwargs.emit, true)
    }

    /// Get information about a column.
    pub fn get_col(&mut self, iid: String) -> Result<ItemColResult, String> {
        let col = Self::acr_pos(&self.grid.header.iid_to_col, &iid)?;
        let empty_val = &self.opts.empty_val;
        Ok(ItemColResult {
            pos: col as u64,
            val: (*Self::acr_hcell_val(&self.grid.header.cells, col)?).clone(),
            values: (0..Self::acr_total_rows(&self.grid.index.cells))
                .map(|row| {
                    Self::acr_tcell_val(&self.grid.index.cells, row, col)
                        .map_or(empty_val.clone(), |r| (*r).clone())
                })
                .collect::<Vec<Val>>(),
        })
    }
}
