use crate::Index;
use crate::IndexCell;
use crate::Table;
use crate::TableGrid;
use std::collections::HashMap;
use std::rc::Rc;

// Custom iterator for traversing ancestor rows
struct AncestorRowIterator<'a> {
    iid_to_row: &'a HashMap<Rc<str>, usize>,
    cells: &'a Vec<IndexCell>,
    current: Option<Rc<str>>,
}

impl<'a> Iterator for AncestorRowIterator<'a> {
    type Item = Result<usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_iid = if let Some(iid) = self.current.take() {
            iid
        } else {
            return None;
        };

        // Look up row for current IID
        let row = match self.iid_to_row.get(&current_iid) {
            Some(&r) => r,
            None => {
                return Some(Err(format!(
                    "IID {} not found in iid_to_row",
                    &*current_iid
                )));
            }
        };

        // Get cell and extract next parent IID
        let cell = match self.cells.get(row) {
            Some(c) => c,
            None => return Some(Err(format!("Cell not found at row {}", row))),
        };
        self.current = if cell.par.is_empty() {
            None
        } else {
            Some(cell.par.clone())
        }; // Cheap Rc clone if present

        Some(Ok(row))
    }
}

impl Index {
    /// Returns an iterator over the row numbers of the ancestors of the given `iid`, from direct parent to root.
    pub(crate) fn ancestor_rows<'a>(
        &'a self,
        iid: &Rc<str>, // Accept &Rc<str> for efficiency (avoids conversion); can add &str overload if needed
    ) -> Box<dyn Iterator<Item = Result<usize, String>> + 'a> {
        // Look up row for the given IID
        let row = match self.iid_to_row.get(iid) {
            None => {
                return Box::new(std::iter::once(Err(format!(
                    "IID {} not found in iid_to_row",
                    &*iid
                ))));
            }
            Some(&r) => r,
        };

        // Get cell and extract first parent IID
        let first_parent = match self.cells.get(row) {
            None => {
                return Box::new(std::iter::once(Err(format!(
                    "Cell not found for IID {} at row {}",
                    &*iid, row
                ))));
            }
            Some(cell) => {
                if cell.par.is_empty() {
                    return Box::new(std::iter::empty());
                }
                cell.par.clone() // Cheap Rc clone
            }
        };

        Box::new(AncestorRowIterator {
            iid_to_row: &self.iid_to_row,
            cells: &self.cells,
            current: Some(first_parent),
        })
    }
}

impl Table {
    /// Checks if all ancestors of `iid` are open, optionally stopping at `stop_at`.
    /// If `stop_at` is non-empty, returns `true` if all ancestors up to `stop_at` are open.
    /// If `stop_at` is empty, returns `true` if all ancestors are open.
    pub(crate) fn cr_tree_ancestors_all_open(
        &self,
        grid: &TableGrid,
        iid: &Rc<str>, // Updated to match; adjust caller if needed
        stop_at: &str,
    ) -> Result<bool, String> {
        let stop_at_provided = !stop_at.is_empty();
        for ancestor in grid.index.ancestor_rows(iid) {
            let ancestor_row = match ancestor {
                Ok(row) => row,
                Err(e) => return Err(e),
            };
            let cell = match grid.index.cells.get(ancestor_row) {
                Some(c) => c,
                None => return Err(format!("Cell not found for ancestor row {}", ancestor_row)),
            };
            // No borrow needed; direct access
            if !cell.open {
                return Ok(false);
            }
            if stop_at_provided && &*cell.iid == stop_at {
                return Ok(true);
            }
        }
        Ok(true)
    }
}
