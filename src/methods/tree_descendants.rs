use crate::EventData;
use crate::GetCellResultExt;
use crate::GetResultExt;
use crate::IndexCell;
use crate::Table;
use crate::TableGrid;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

impl Table {
    pub(crate) fn acr_tree_descendants_rows<'a>(
        cells: &'a Vec<IndexCell>,
        iid_to_row: &'a HashMap<Rc<str>, usize>,
        rn: usize,
        check_open: bool,
    ) -> impl Iterator<Item = Result<usize, String>> + 'a {
        // Define the custom iterator struct
        struct DescendantIterator<'a> {
            cells: &'a Vec<IndexCell>,
            iid_to_row: &'a HashMap<Rc<str>, usize>,
            stack: Vec<usize>, // Stack of row numbers
            check_open: bool,
            error: Option<String>, // Holds an initialization error, if any
            rn: usize,
            initialized: bool,
        }

        impl<'a> Iterator for DescendantIterator<'a> {
            type Item = Result<usize, String>;

            fn next(&mut self) -> Option<Self::Item> {
                // Yield the initialization error first, if it exists
                if let Some(err) = self.error.take() {
                    return Some(Err(err));
                }

                if !self.initialized {
                    self.initialized = true;

                    let icell = match self.cells.get(self.rn) {
                        Some(r) => r,
                        None => {
                            return Some(Err(format!("Row not found: {}", self.rn)));
                        }
                    };

                    // Push initial children in reverse order
                    for ciid in icell.chn.iter().rev() {
                        let child_rn = match self.iid_to_row.get(ciid) {
                            Some(&r) => r,
                            None => {
                                return Some(Err(format!("iid not found: {}", ciid)));
                            }
                        };
                        self.stack.push(child_rn);
                    }
                }

                // Iterate through the stack
                while let Some(current) = self.stack.pop() {
                    // Get the cell for the current row
                    let icell = match self.cells.get(current) {
                        Some(r) => r,
                        None => {
                            return Some(Err(format!("Row not found: {}", current)));
                        }
                    };

                    // Determine if we should expand children
                    let should_expand = if icell.chn.is_empty() {
                        false
                    } else if !self.check_open {
                        true
                    } else {
                        icell.open
                    };

                    if should_expand {
                        // Push children in reverse order
                        for ciid in icell.chn.iter().rev() {
                            let child_rn = match self.iid_to_row.get(ciid) {
                                Some(&r) => r,
                                None => {
                                    return Some(Err(format!("iid not found: {}", ciid)));
                                }
                            };
                            self.stack.push(child_rn);
                        }
                    }

                    return Some(Ok(current));
                }
                None // No more items to yield
            }
        }

        DescendantIterator {
            cells,
            iid_to_row,
            stack: vec![],
            check_open,
            error: None,
            rn,
            initialized: false,
        }
    }

    pub(crate) fn acr_tree_descendants_num(
        grid: &TableGrid,
        rn: usize,
        check_open: bool,
    ) -> Result<usize, String> {
        let cells = &grid.index.cells;
        let iid_to_row = &grid.index.iid_to_row;
        let mut ctr: usize = 0;
        for did_res in Table::acr_tree_descendants_rows(cells, iid_to_row, rn, check_open) {
            match did_res {
                Ok(_) => ctr += 1,
                Err(e) => return Err(e),
            }
        }
        Ok(ctr)
    }

    pub(crate) fn acr_tree_descendants_set_vis(
        mut cells: &mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
        vis: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<(), String> {
        let mut stack: VecDeque<usize> = VecDeque::new();
        let start_row = Self::acr_pos(&iid_to_row, iid)?;
        let start_cell = Self::acr_icell_mut(&mut cells, start_row)?;

        // If the starting cell has children, add their row numbers to the stack
        if !start_cell.chn.is_empty() && start_cell.open {
            for ciid in &start_cell.chn {
                let c_row = *iid_to_row.get_res(ciid)?;
                stack.push_back(c_row);
            }
        }
        if let Some(event_data) = event_data {
            while let Some(rn) = stack.pop_front() {
                Self::acr_event_data_set_ivis(&mut cells, rn, vis, event_data)?;
                let cell = Self::acr_icell_mut(&mut cells, rn)?;
                // If the cell has children and is open, add their row numbers to the stack
                if !cell.chn.is_empty() && cell.open {
                    for ciid in &cell.chn {
                        stack.push_back(*iid_to_row.get_res(ciid)?);
                    }
                }
            }
        } else {
            // No event data, just set vis
            while let Some(rn) = stack.pop_front() {
                let cell = Self::acr_icell_mut(&mut cells, rn)?;
                cell.vis = vis;
                if !cell.chn.is_empty() && cell.open {
                    for ciid in &cell.chn {
                        stack.push_back(*iid_to_row.get_res(ciid)?);
                    }
                }
            }
        }
        Ok(())
    }

    /// Remove any descendants from a set of item ids so that only
    /// the highest level items remain.
    pub(crate) fn cr_tree_remove_descendants(
        &self,
        iids: HashSet<Rc<str>>,
    ) -> Result<Vec<usize>, String> {
        let mut res: Vec<usize> = Vec::new();
        for iid in iids.iter() {
            let mut has_ancestor_in_set = false;
            for ancestor in self.grid.index.ancestor_rows(iid) {
                let anc_rn = ancestor?;
                let anc_cell = self.grid.index.cells.get_res(anc_rn)?;
                if iids.contains(&anc_cell.iid) {
                    has_ancestor_in_set = true;
                    break;
                }
            }
            if !has_ancestor_in_set {
                res.push(Self::acr_pos(&self.grid.index.iid_to_row, iid)?);
            }
        }
        Ok(res)
    }

    /// Remove any existing descendants & then
    /// add all descendant row numbers
    /// Return Result Vec of row numbers
    /// Effectively validates rows to be deleted
    pub(crate) fn cr_tree_remove_and_add_descendants(
        &self,
        iids: HashSet<Rc<str>>,
    ) -> Result<Vec<usize>, String> {
        let mut res: Vec<usize> = Vec::new();
        for iid in iids.iter() {
            let mut has_ancestor_in_set = false;
            for ancestor in self.grid.index.ancestor_rows(iid) {
                let anc_rn = ancestor?;
                let anc_cell = self.grid.index.cells.get_res(anc_rn)?;
                if iids.contains(&anc_cell.iid) {
                    has_ancestor_in_set = true;
                    break;
                }
            }
            if !has_ancestor_in_set {
                let rn = Self::acr_pos(&self.grid.index.iid_to_row, iid)?;
                res.push(rn);
                let desc: Result<Vec<usize>, String> = Self::acr_tree_descendants_rows(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    rn,
                    false,
                )
                .map(|desc_res| desc_res)
                .collect();
                res.extend(desc?);
            }
        }
        // TODO: is sorting necessary?
        res.sort_unstable();
        Ok(res)
    }
}
