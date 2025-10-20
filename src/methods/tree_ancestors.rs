use crate::Index;
use crate::IndexCell;
use crate::Table;
use std::collections::HashMap;

// Custom iterator for traversing ancestor rows
struct AncestorRowIterator<'a> {
    cells: &'a Vec<IndexCell>,
    iid_to_row: &'a HashMap<String, usize>,
    current_row: Option<usize>,
}

impl<'a> Iterator for AncestorRowIterator<'a> {
    type Item = Result<usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.current_row.take() {
            let cell = match self.cells.get(row as usize) {
                Some(c) => c,
                None => return Some(Err(format!("Cell not found for row {}", row))),
            };
            let parent_iid = &cell.par;
            let next_row = if parent_iid.is_empty() {
                None
            } else {
                match self.iid_to_row.get(parent_iid) {
                    Some(&p_row) => Some(p_row),
                    None => {
                        return Some(Err(format!(
                            "Parent IID {} not found in iid_to_row",
                            parent_iid
                        )));
                    }
                }
            };
            self.current_row = next_row;
            Some(Ok(row))
        } else {
            None
        }
    }
}

impl Index {
    /// Returns an iterator over the row numbers of the ancestors of the given `iid`, from direct parent to root.
    pub(crate) fn ancestor_rows<'a>(
        &'a self,
        iid: &str,
    ) -> Box<dyn Iterator<Item = Result<usize, String>> + 'a> {
        let first_parent_row: Option<usize> = match self.iid_to_row.get(iid) {
            None => {
                return Box::new(std::iter::once(Err(format!(
                    "IID {} not found in iid_to_row",
                    iid
                ))));
            }
            Some(&row) => match self.cells.get(row) {
                None => {
                    return Box::new(std::iter::once(Err(format!(
                        "Cell not found for IID {} at row {}",
                        iid, row
                    ))));
                }
                Some(cell) => {
                    let parent_iid = &cell.par;
                    if parent_iid.is_empty() {
                        None
                    } else {
                        match self.iid_to_row.get(parent_iid) {
                            Some(&p_row) => Some(p_row),
                            None => {
                                return Box::new(std::iter::once(Err(format!(
                                    "Parent IID {} for IID {} not found in iid_to_row",
                                    parent_iid, iid
                                ))));
                            }
                        }
                    }
                }
            },
        };
        Box::new(AncestorRowIterator {
            cells: &self.cells,
            iid_to_row: &self.iid_to_row,
            current_row: first_parent_row,
        })
    }
}

impl Table {
    /// Checks if all ancestors of `iid` are open, optionally stopping at `stop_at`.
    /// If `stop_at` is non-empty, returns `true` if all ancestors up to `stop_at` are open.
    /// If `stop_at` is empty, returns `true` if all ancestors are open.
    pub(crate) fn cr_tree_ancestors_all_open(
        &self,
        iid: &str,
        stop_at: &str,
    ) -> Result<bool, String> {
        let grid = self.grid.borrow();
        let stop_at_provided = !stop_at.is_empty();
        for ancestor in grid.index.ancestor_rows(iid) {
            let ancestor_row = match ancestor {
                Ok(row) => row,
                Err(e) => return Err(e),
            };
            let cell = match grid.index.cells.get(ancestor_row as usize) {
                Some(c) => c,
                None => return Err(format!("Cell not found for ancestor row {}", ancestor_row)),
            };
            if !cell.open {
                return Ok(false);
            }
            if stop_at_provided && cell.iid == stop_at {
                return Ok(true);
            }
        }
        Ok(true)
    }

    #[inline]
    pub(crate) fn cr_tree_iid_lvl(&self, iid: &str) -> Result<Option<usize>, String> {
        if iid.is_empty() {
            return Ok(None);
        }
        let grid = self.grid.borrow();
        let count = grid
            .index
            .ancestor_rows(iid)
            .try_fold(0usize, |acc, res| res.map(|_| acc + 1))?;
        Ok(Some(count))
    }
}
