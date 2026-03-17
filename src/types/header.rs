use crate::CellProps;
use crate::HeaderCell;
use crate::str_err;
use crate::types::CyclingCounter;
use crate::utils::utils_fns::n_to_base62;
use crate::utils::utils_fns::pull_n;
use crate::utils::utils_fns::push_n_add;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Header {
    pub iid_to_col: HashMap<Rc<str>, usize>,
    pub cells: Vec<HeaderCell>,
    pub iid_ctr: CyclingCounter,
}

impl Header {
    pub fn new() -> Self {
        Header {
            iid_to_col: HashMap::new(),
            cells: Vec::new(),
            iid_ctr: CyclingCounter::new(),
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
        self.cells.get(*col).and_then(|cell| cell.props.clone())
    }

    /// Get header cell properties reference at col.
    pub fn get_props(&self, col: &usize) -> Option<&CellProps> {
        self.cells.get(*col).and_then(|cell| cell.props.as_ref())
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
            self.cells.insert(pos, val);
        }
    }

    /// Remove and return multiple items, delete cols
    pub fn del_cols(&mut self, sorted_seq: &[usize]) -> Result<Vec<(usize, HeaderCell)>, String> {
        let largest_pos = *sorted_seq
            .last()
            .ok_or_else(|| str_err!("To delete is empty."))?;
        if largest_pos >= self.cells.len() {
            return Err(str_err!(
                "Deletion pos: {} out of bounds of header len: {}",
                largest_pos,
                self.cells.len()
            ));
        }
        let mut removed = Vec::with_capacity(sorted_seq.len());
        for (i, &pos) in sorted_seq.iter().enumerate() {
            let cell = self.cells.remove(pos - i);
            self.iid_to_col.remove(&cell.iid);
            removed.push((pos, cell));
        }
        // Adjust indices post removal
        for (_, pos) in self.iid_to_col.iter_mut() {
            *pos = pull_n(*pos, sorted_seq);
        }
        Ok(removed)
    }

    pub fn move_cols(&mut self, full_new_to_old: &[usize]) -> Result<(), String> {
        let n = self.cells.len();
        if n == 0 {
            return Ok(());
        }
        if full_new_to_old.len() != n {
            return Err(str_err!(
                "move_cols: length mismatch (got {}, expected {})",
                full_new_to_old.len(),
                n
            ));
        }

        #[cfg(any(debug_assertions, feature = "safety"))]
        {
            let mut seen = vec![false; n];
            for &old_idx in full_new_to_old {
                if old_idx >= n || seen[old_idx] {
                    return Err(str_err!("Mapping error: invalid old index {}", old_idx));
                }
                seen[old_idx] = true;
            }
        }

        let mut old_cells = std::mem::take(&mut self.cells)
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>();

        self.cells = Vec::with_capacity(n);

        for (new_idx, &old_idx) in full_new_to_old.iter().enumerate() {
            let cell = old_cells
                .get_mut(old_idx)
                .and_then(Option::take)
                .ok_or_else(|| str_err!("Mapping error: invalid or duplicate index {}", old_idx))?;

            if let Some(col) = self.iid_to_col.get_mut(&cell.iid) {
                *col = new_idx; // new column index
            }
            self.cells.push(cell);
        }

        Ok(())
    }

    #[inline]
    pub fn new_iid(&mut self) -> Result<String, String> {
        let mut iid = n_to_base62(
            self.iid_ctr
                .next()
                .ok_or_else(|| str_err!("Cycling counter should never end"))?,
        );
        if self.iid_to_col.contains_key(iid.as_str()) {
            let mut cycles: usize = 0;
            while self.iid_to_col.contains_key(iid.as_str()) {
                if cycles == usize::MAX {
                    return Err(str_err!("Column limit reached."));
                }
                iid = n_to_base62(
                    self.iid_ctr
                        .next()
                        .ok_or_else(|| str_err!("Cycling counter should never end"))?,
                );
                cycles = cycles.saturating_add(1);
            }
        }
        Ok(iid)
    }
}
