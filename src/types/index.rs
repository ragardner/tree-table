use crate::CellProps;
use crate::IndexCell;
use crate::MaxMap;
use crate::TableCell;
use crate::str_err;
use crate::types::CyclingCounter;
use crate::utils::utils_fns::n_to_base62;
use crate::utils::utils_fns::pull_n;
use crate::utils::utils_fns::push_n;
use crate::utils::utils_fns::push_n_add;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Index {
    pub iid_to_row: HashMap<String, usize>,
    pub cells: Vec<Rc<RefCell<IndexCell>>>,
    pub iid_ctr: CyclingCounter,
}

impl Index {
    pub(crate) fn new() -> Self {
        Index {
            iid_to_row: HashMap::new(),
            cells: Vec::new(),
            iid_ctr: CyclingCounter::new(),
        }
    }

    /// Clears all mappings and cells from the index and resets the iid counter.
    pub(crate) fn reset(&mut self) {
        self.iid_to_row.clear();
        self.cells.clear();
        self.iid_ctr = CyclingCounter::new();
    }

    /// Clone index cell properties at row.
    pub(crate) fn clone_props(&self, row: &usize) -> Option<CellProps> {
        self.cells
            .get(*row)
            .and_then(|cell_rc| cell_rc.borrow().props.clone())
    }

    /// Clone table cell properties at row and col.
    pub(crate) fn clone_table_props(&self, row: &usize, col: &usize) -> Option<CellProps> {
        self.cells.get(*row).and_then(|icell_rc| {
            let borrow = icell_rc.borrow();
            borrow
                .vals
                .get(col)
                .and_then(|table_cell| table_cell.props.clone())
        })
    }

    /// Use a Vec of tuples of usize, Indexcell to push the existing
    /// indices of the Index forward.
    pub(crate) fn add_rows_just_push_hm(&mut self, sorted_seq: &Vec<usize>) {
        // Adjust existing HashMap indices
        for (_, row) in self.iid_to_row.iter_mut() {
            *row = push_n(*row, &sorted_seq);
        }
    }

    /// Add a single row, assumes iid_to_row has been adjusted before insertion.
    #[inline(always)]
    pub(crate) fn add_row(&mut self, pos: usize, iid: String, cell: Rc<RefCell<IndexCell>>) {
        self.iid_to_row.insert(iid, pos);
        self.cells.insert(pos, cell);
    }

    /// Add a single row, assumes iid_to_row has been adjusted before insertion.
    #[inline(always)]
    pub(crate) fn push_row(&mut self, pos: usize, iid: String, cell: Rc<RefCell<IndexCell>>) {
        self.iid_to_row.insert(iid, pos);
        self.cells.push(cell);
    }

    /// Insert multiple items, add rows
    pub(crate) fn add_rows(&mut self, to_add: Vec<(usize, Rc<RefCell<IndexCell>>)>) {
        // Adjust existing HashMap indices
        for (_, row) in self.iid_to_row.iter_mut() {
            *row = push_n_add(*row, &to_add);
        }
        // Add items
        for (pos, cell) in to_add.into_iter() {
            self.iid_to_row.insert(cell.borrow().iid.clone(), pos);
            self.cells.insert(pos, cell);
        }
    }

    /// Remove and return multiple items, delete rows
    pub(crate) fn del_rows(
        &mut self,
        sorted_seq: &Vec<usize>,
    ) -> Result<Vec<(usize, Rc<RefCell<IndexCell>>)>, String> {
        let largest_pos = *sorted_seq.last().ok_or(str_err!("To delete is empty."))?;
        if largest_pos >= self.cells.len() {
            return Err(str_err!(format!(
                "Deletion pos: {} out of bounds of index len: {}",
                largest_pos,
                self.cells.len()
            )));
        }
        let mut removed = Vec::with_capacity(sorted_seq.len());
        for (i, &pos) in sorted_seq.iter().enumerate() {
            let cell_rc = self.cells.remove(pos - i);
            self.iid_to_row.remove(&cell_rc.borrow().iid);
            removed.push((pos, cell_rc));
        }
        // Adjust indices post removal
        for (_, pos) in self.iid_to_row.iter_mut() {
            *pos = pull_n(*pos, sorted_seq);
        }
        Ok(removed)
    }

    pub(crate) fn move_rows(
        &mut self,
        full_new_to_old: &HashMap<usize, usize>,
    ) -> Result<(), String> {
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
                    if let Some(row) = self.iid_to_row.get_mut(&cell.borrow().iid) {
                        *row = i;
                    };
                    self.cells.push(cell);
                }
            }
        }
        Ok(())
    }

    // Insert multiple columns.
    pub(crate) fn add_cols(
        &mut self,
        to_add: Option<HashMap<usize, MaxMap<TableCell>>>,
        sorted_seq: &Vec<usize>,
    ) {
        // Adjust existing HashMap indices
        for icell in self.cells.iter_mut() {
            let index_row_vals = &mut icell.borrow_mut().vals;
            index_row_vals.hm = index_row_vals
                .hm
                .drain()
                .map(|(k, v)| (push_n(k, sorted_seq), v))
                .collect();
            index_row_vals.maxk = index_row_vals.maxk.map(|max| push_n(max, sorted_seq));
        }

        // Add the new columns if provided
        if let Some(to_add) = to_add {
            for (rowk, cols) in to_add {
                if let Some(row) = self.cells.get_mut(rowk) {
                    let index_row_vals = &mut row.borrow_mut().vals;
                    for (colk, val) in cols.hm {
                        index_row_vals.insert(colk, val);
                    }
                }
            }
        }
    }

    pub(crate) fn del_cols(
        &mut self,
        sorted_seq: &Vec<usize>,
    ) -> HashMap<usize, MaxMap<TableCell>> {
        // Remove items and create HashMap of removed items
        let mut removed: HashMap<usize, MaxMap<TableCell>> = HashMap::new();

        // Process each row
        for (rowk, icell) in self.cells.iter_mut().enumerate() {
            let mut row_removed: MaxMap<TableCell> = MaxMap::new();
            let mut redo_maxk = false;
            let row = &mut icell.borrow_mut().vals;

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

    pub(crate) fn move_cols(&mut self, full_new_idxs: &Vec<usize>) {
        for icell in self.cells.iter_mut() {
            let max_map = &mut icell.borrow_mut().vals;
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

    #[inline]
    pub(crate) fn new_iid(&mut self) -> Result<String, String> {
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
}
