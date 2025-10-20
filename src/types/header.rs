use super::cell_props::CellProps;
use super::header_cell::HeaderCell;
use super::val::Val;
use crate::str_err;
use crate::types::CyclingCounter;
use crate::utils::utils_fns::n_to_base62;
use crate::utils::utils_fns::pull_n;
use crate::utils::utils_fns::push_n_add;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Header {
    pub iid_to_col: HashMap<String, usize>,
    pub cells: Vec<HeaderCell>,
    pub iid_ctr: CyclingCounter,
    pub to_swap_cell: HeaderCell,
}

impl Header {
    pub fn new() -> Self {
        Header {
            iid_to_col: HashMap::new(),
            cells: Vec::new(),
            iid_ctr: CyclingCounter::new(),
            to_swap_cell: HeaderCell::new(Val::Nil(()), String::new(), 1, true),
        }
    }

    /// Clears all mappings and cells from the header and resets the iid counter.
    pub fn reset(&mut self) {
        self.iid_to_col.clear();
        self.cells.clear();
        self.iid_ctr = CyclingCounter::new();
    }

    /// Clone header cell properties at col.
    pub fn clone_props(&self, col: &usize) -> Option<CellProps> {
        self.cells
            .get(*col as usize)
            .and_then(|cell| cell.props.clone())
    }

    /// Get header cell properties reference at col.
    pub fn get_props(&self, col: &usize) -> Option<&CellProps> {
        self.cells
            .get(*col as usize)
            .and_then(|cell| cell.props.as_ref())
    }

    #[inline]
    pub fn clone_cell(&self, col: &usize) -> Result<HeaderCell, String> {
        self.cells
            .get(*col as usize)
            .ok_or(str_err!(format!("Col not found: {col}")))
            .cloned()
    }

    /// Insert multiple items, add cols
    pub fn add_cols(&mut self, to_add: Vec<(usize, HeaderCell)>) {
        // Adjust existing HashMap indices
        for (_, col) in self.iid_to_col.iter_mut() {
            *col = push_n_add(*col, &to_add);
        }
        // Add items
        for (pos, val) in to_add.into_iter() {
            self.iid_to_col.insert(val.iid.clone(), pos.clone());
            self.cells.insert(pos as usize, val);
        }
    }

    /// Remove and return multiple items, delete cols
    pub fn del_cols(
        &mut self,
        sorted_seq: &Vec<usize>,
    ) -> Result<Vec<(usize, HeaderCell)>, String> {
        let largest_pos = *sorted_seq.last().ok_or(str_err!("To delete is empty."))? as usize;
        if largest_pos >= self.cells.len() {
            return Err(str_err!(format!(
                "Deletion pos: {} out of bounds of header len: {}",
                largest_pos,
                self.cells.len()
            )));
        }
        let mut removed = Vec::with_capacity(sorted_seq.len());
        for (i, &pos) in sorted_seq.iter().enumerate() {
            let cell = self.cells.remove((pos as usize) - i);
            self.iid_to_col.remove(&cell.iid.to_string());
            removed.push((pos, cell));
        }
        // Adjust indices post removal
        for (_, pos) in self.iid_to_col.iter_mut() {
            *pos = pull_n(*pos, sorted_seq);
        }
        Ok(removed)
    }

    pub fn move_cols(&mut self, full_new_to_old: &HashMap<usize, usize>) -> Result<(), String> {
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
                    if let Some(col) = self.iid_to_col.get_mut(&cell.iid) {
                        *col = i;
                    };
                    self.cells.push(cell);
                }
            }
        }
        Ok(())
    }

    /// Get index using iid
    #[inline]
    pub fn col(&self, iid: &str) -> Option<&usize> {
        self.iid_to_col.get(iid)
    }

    /// Get iid using index
    #[inline]
    pub fn iid(&self, col: &usize) -> Option<&String> {
        self.cells.get(*col as usize).map(|cell| &cell.iid)
    }

    #[inline]
    pub fn new_iid(&mut self) -> Result<String, String> {
        let mut iid = n_to_base62(
            self.iid_ctr
                .next()
                .ok_or("Cycling counter should never end")?,
        );
        if self.iid_to_col.contains_key(&iid) {
            let mut cycles: usize = 0;
            while self.iid_to_col.contains_key(&iid) {
                if cycles == usize::MAX {
                    return Err(str_err!("Column limit reached."));
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

    /// Overwrite the cell at the given col with a new cell. If the new cell has a different IID,
    /// the mapping is updated, ensuring the new IID does not already exist.
    #[inline]
    pub fn set_cell(&mut self, col: usize, new_cell: HeaderCell) -> Result<(), String> {
        let col_usize = col as usize;
        if col_usize >= self.cells.len() {
            return Err(str_err!(format!(
                "Overwrite position {} out of bounds for header length {}",
                col,
                self.cells.len()
            )));
        }

        // Early validation of new IID uniqueness if different from old IID
        let old_iid = &self.cells[col_usize].iid;
        if old_iid != &new_cell.iid && self.iid_to_col.contains_key(&new_cell.iid) {
            return Err(str_err!(format!(
                "New IID '{}' already exists in the header",
                new_cell.iid
            )));
        }

        // Update HashMap: remove old IID and insert new IID
        if old_iid != &new_cell.iid {
            self.iid_to_col.remove(old_iid);
            self.iid_to_col.insert(new_cell.iid.clone(), col);
        }

        // Overwrite the cell
        self.cells[col_usize] = new_cell;
        Ok(())
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

    /// Overwrite the Val for the header cell at the given col.
    #[inline]
    pub fn set_val<T>(&mut self, col: usize, val: T) -> Result<(), String>
    where
        T: Into<Val>,
    {
        let col_usize = col as usize;
        if col_usize >= self.cells.len() {
            return Err(str_err!(format!(
                "Set val position {} out of bounds for header length {}",
                col,
                self.cells.len()
            )));
        }
        self.cells[col_usize].val = val.into();
        Ok(())
    }
}
