use super::{cell_props::CellProps, index_cell::IndexCell, val::Val};
use crate::MaxMap;
use crate::TableCell;
use crate::str_err;
use crate::types::CyclingCounter;
use crate::utils::utils_fns::n_to_base62;
use crate::utils::utils_fns::pull_n;
use crate::utils::utils_fns::push_n;
use crate::utils::utils_fns::push_n_add;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Index {
    pub iid_to_row: HashMap<String, usize>,
    pub cells: Vec<IndexCell>,
    pub iid_ctr: CyclingCounter,
    pub to_swap_cell: IndexCell,
}

impl Index {
    pub fn new() -> Self {
        Index {
            iid_to_row: HashMap::new(),
            cells: Vec::new(),
            iid_ctr: CyclingCounter::new(),
            to_swap_cell: IndexCell::new(Val::Nil(()), 1, true, String::new()),
        }
    }

    /// Clears all mappings and cells from the index and resets the iid counter.
    pub fn reset(&mut self) {
        self.iid_to_row.clear();
        self.cells.clear();
        self.iid_ctr = CyclingCounter::new();
    }

    /// Clone index cell properties at row.
    pub fn clone_props(&self, row: &usize) -> Option<CellProps> {
        self.cells
            .get(*row as usize)
            .and_then(|cell| cell.props.clone())
    }

    /// Get index cell properties reference at row.
    pub fn get_props(&self, row: &usize) -> Option<&CellProps> {
        self.cells
            .get(*row as usize)
            .and_then(|cell| cell.props.as_ref())
    }

    /// Clone table cell properties at row and col.
    pub fn clone_table_props(&self, row: &usize, col: &usize) -> Option<CellProps> {
        self.cells
            .get(*row as usize)
            .and_then(|index_cell| index_cell.vals.get(col))
            .and_then(|table_cell| table_cell.props.clone())
    }

    #[inline]
    pub fn clone_cell(&self, row: &usize) -> Result<IndexCell, String> {
        self.cells
            .get(*row as usize)
            .ok_or(str_err!(format!("Row not found: {row}")))
            .cloned()
    }

    /// Use a Vec of tuples of usize, Indexcell to push the existing
    /// indices of the Index forward.
    pub fn add_rows_just_push_hm(&mut self, sorted_seq: &Vec<usize>) {
        // Adjust existing HashMap indices
        for (_, row) in self.iid_to_row.iter_mut() {
            *row = push_n(*row, &sorted_seq);
        }
    }

    /// Add a single row, assumes iid_to_row has been adjusted before insertion.
    #[inline]
    pub fn add_row(&mut self, pos: usize, cell: IndexCell) {
        self.iid_to_row.insert(cell.iid.clone(), pos.clone());
        self.cells.insert(pos as usize, cell);
    }

    /// Insert multiple items, add rows
    pub fn add_rows(&mut self, to_add: Vec<(usize, IndexCell)>) {
        // Adjust existing HashMap indices
        for (_, row) in self.iid_to_row.iter_mut() {
            *row = push_n_add(*row, &to_add);
        }
        // Add items
        for (pos, val) in to_add.into_iter() {
            self.iid_to_row.insert(val.iid.clone(), pos.clone());
            self.cells.insert(pos as usize, val);
        }
    }

    /// Remove and return multiple items, delete rows
    pub fn del_rows(&mut self, sorted_seq: &Vec<usize>) -> Result<Vec<(usize, IndexCell)>, String> {
        let largest_pos = *sorted_seq.last().ok_or(str_err!("To delete is empty."))? as usize;
        if largest_pos >= self.cells.len() {
            return Err(str_err!(format!(
                "Deletion pos: {} out of bounds of index len: {}",
                largest_pos,
                self.cells.len()
            )));
        }
        let mut removed = Vec::with_capacity(sorted_seq.len());
        for (i, &pos) in sorted_seq.iter().enumerate() {
            let cell = self.cells.remove((pos as usize) - i);
            self.iid_to_row.remove(&cell.iid.to_string());
            removed.push((pos, cell));
        }
        // Adjust indices post removal
        for (_, pos) in self.iid_to_row.iter_mut() {
            *pos = pull_n(*pos, sorted_seq);
        }
        Ok(removed)
    }

    pub fn move_rows(&mut self, full_new_to_old: &HashMap<usize, usize>) -> Result<(), String> {
        if !self.cells.is_empty() {
            let mut seq_opt = std::mem::take(&mut self.cells)
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            self.cells = Vec::with_capacity(seq_opt.len());
            for i in 0..seq_opt.len() {
                if let Some(&old_idx) = full_new_to_old.get(&i) {
                    let cell = seq_opt[old_idx]
                        .take()
                        .ok_or(str_err!(format!("Mapping error: {}", old_idx)))?;
                    if let Some(row) = self.iid_to_row.get_mut(&cell.iid) {
                        *row = i;
                    };
                    self.cells.push(cell);
                }
            }
        }
        Ok(())
    }

    // Insert multiple columns.
    pub fn add_cols(
        &mut self,
        to_add: Option<HashMap<usize, MaxMap<TableCell>>>,
        sorted_seq: &Vec<usize>,
    ) {
        // Adjust existing HashMap indices
        for icell in self.cells.iter_mut() {
            let row = &mut icell.vals;
            row.hm = row
                .hm
                .drain()
                .map(|(k, v)| (push_n(k, sorted_seq), v))
                .collect();
            row.maxk = row.maxk.map(|max| push_n(max, sorted_seq));
        }

        // Add the new columns if provided
        if let Some(to_add) = to_add {
            for (rowk, cols) in to_add {
                if let Some(row) = self.cells.get_mut(rowk as usize) {
                    for (colk, val) in cols.hm {
                        row.vals.insert(colk, val);
                    }
                }
            }
        }
    }

    pub fn del_cols(&mut self, sorted_seq: &Vec<usize>) -> HashMap<usize, MaxMap<TableCell>> {
        // Remove items and create HashMap of removed items
        let mut removed: HashMap<usize, MaxMap<TableCell>> = HashMap::new();

        // Process each row
        for (rowk, icell) in self.cells.iter_mut().enumerate() {
            let mut row_removed: MaxMap<TableCell> = MaxMap::new();
            let mut redo_maxk = false;
            let row = &mut icell.vals;

            // Remove specified columns and collect them
            for &colk in sorted_seq {
                if let Some(val) = row.hm.remove(&colk) {
                    row_removed.insert(colk, val);
                    if row.maxk == Some(colk) {
                        redo_maxk = true;
                    }
                }
            }

            // If we removed anything, store it in the removed map
            if !row_removed.is_empty() {
                removed.insert(rowk, row_removed);
            }

            // Shift remaining keys down using pull_n
            row.hm = row
                .hm
                .drain()
                .map(|(k, v)| (pull_n(k, sorted_seq), v))
                .collect();

            if redo_maxk {
                let mut new_maxk = None;
                for k in (0..row.maxk.unwrap_or(0)).rev() {
                    if row.hm.contains_key(&k) {
                        new_maxk = Some(k);
                        break;
                    }
                }
                row.maxk = new_maxk;
            } else {
                row.maxk = row.maxk.map(|k| pull_n(k, &sorted_seq));
            }
        }

        removed
    }

    pub fn move_cols(&mut self, full_new_idxs: &Vec<usize>) {
        for icell in self.cells.iter_mut() {
            let max_map = &mut icell.vals;
            if max_map.is_empty() {
                continue;
            }
            let mut new_hm: HashMap<usize, TableCell> = HashMap::new();
            let mut max: usize = 0;
            for (old_pos, new_pos) in full_new_idxs.iter().enumerate() {
                if let Some(cell) = max_map.hm.remove(&old_pos) {
                    new_hm.insert(*new_pos, cell);
                    if *new_pos > max {
                        max = *new_pos;
                    }
                }
            }
            max_map.hm = new_hm;
            max_map.maxk = Some(max);
        }
    }

    /// Get row number using iid
    #[inline]
    pub fn row(&self, iid: &str) -> Option<&usize> {
        self.iid_to_row.get(iid)
    }

    /// Get iid using row number
    #[inline]
    pub fn iid(&self, row: &usize) -> Option<&String> {
        self.cells.get(*row as usize).map(|cell| &cell.iid)
    }

    #[inline]
    pub fn new_iid(&mut self) -> Result<String, String> {
        let mut iid = n_to_base62(
            self.iid_ctr
                .next()
                .ok_or("Cycling counter should never end")?,
        );
        if self.iid_to_row.contains_key(&iid) {
            let mut cycles: usize = 0;
            while self.iid_to_row.contains_key(&iid) {
                if cycles == usize::MAX {
                    return Err(str_err!("Row limit reached."));
                }
                iid = n_to_base62(
                    self.iid_ctr
                        .next()
                        .ok_or("Cycling counter should never end")?,
                );
                cycles = cycles.saturating_add(1);
            }
        }
        Ok(iid)
    }

    /// Overwrite the cell at the given row with a new cell. If the new cell has a different IID,
    /// the mapping is updated, ensuring the new IID does not already exist.
    #[inline]
    pub fn set_cell(&mut self, row: usize, new_cell: IndexCell) -> Result<(), String> {
        let row_usize = row as usize;
        if row_usize >= self.cells.len() {
            return Err(str_err!(format!(
                "Overwrite position {} out of bounds for index length {}",
                row,
                self.cells.len()
            )));
        }

        // Early validation of new IID uniqueness if different from old IID
        let old_iid = &self.cells[row_usize].iid;
        if old_iid != &new_cell.iid && self.iid_to_row.contains_key(&new_cell.iid) {
            return Err(str_err!(format!(
                "New IID '{}' already exists in the index",
                new_cell.iid
            )));
        }

        // Update HashMap: remove old IID and insert new IID
        if old_iid != &new_cell.iid {
            self.iid_to_row.remove(old_iid);
            self.iid_to_row.insert(new_cell.iid.clone(), row);
        }

        // Overwrite the cell
        self.cells[row_usize] = new_cell;
        Ok(())
    }

    /// Clears a table cell.
    pub fn clear_tcell(&mut self, row: usize, col: usize) {
        if let Some(icell) = self.cells.get_mut(row as usize) {
            let cols_map = &mut icell.vals;
            cols_map.hm.remove(&col);
            if cols_map.maxk == Some(col) {
                cols_map.max_removed();
            }
        }
    }

    /// Overwrite the cell at the given pos with a new cell.
    /// IMPORTANT: ASSUMES IID REMAINS THE SAME
    #[inline]
    pub fn swap_cell(&mut self, pos: usize) -> Result<(), String> {
        let pos_usize = pos as usize;
        if pos_usize >= self.cells.len() {
            return Err(str_err!(format!(
                "Overwrite position {} out of bounds for index length {}",
                pos,
                self.cells.len()
            )));
        }
        // Swap the cell, dummy should be in cells, while updated cell should be in to_swap_cell
        std::mem::swap(&mut self.to_swap_cell, &mut self.cells[pos_usize]);
        Ok(())
    }

    /// Overwrite the Val for the index cell at the given row.
    #[inline]
    pub fn set_val<T>(&mut self, row: usize, val: T) -> Result<(), String>
    where
        T: Into<Val>,
    {
        let row_usize = row as usize;
        if row_usize >= self.cells.len() {
            return Err(str_err!(format!(
                "Set val position {} out of bounds for index length {}",
                row,
                self.cells.len()
            )));
        }
        self.cells[row_usize].val = val.into();
        Ok(())
    }
}
