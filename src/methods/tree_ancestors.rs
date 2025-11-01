use crate::Index;
use crate::IndexCell;
use crate::Table;
use crate::TableGrid;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

// Custom iterator for traversing ancestor rows
// Custom iterator for traversing ancestor rows
struct AncestorRowIterator<'a> {
    iid_to_row: &'a HashMap<String, usize>,
    current: Option<Weak<RefCell<IndexCell>>>,
}

impl<'a> Iterator for AncestorRowIterator<'a> {
    type Item = Result<usize, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(weak) = self.current.take() {
            if let Some(rc) = weak.upgrade() {
                let cell = rc.borrow();
                let iid = &cell.iid;
                let row = match self.iid_to_row.get(iid) {
                    Some(&r) => r,
                    None => return Some(Err(format!("IID {} not found in iid_to_row", iid))),
                };
                self.current = cell.par.clone();
                Some(Ok(row))
            } else {
                Some(Err("Failed to upgrade weak reference to parent".to_string()))
            }
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
        let first_parent: Option<Weak<RefCell<IndexCell>>> = match self.iid_to_row.get(iid) {
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
                Some(rc) => {
                    let cell = rc.borrow();
                    cell.par.clone()
                }
            },
        };
        if let Some(first_par) = first_parent {
            Box::new(AncestorRowIterator {
                iid_to_row: &self.iid_to_row,
                current: Some(first_par),
            })
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl Table {
    /// Checks if all ancestors of `iid` are open, optionally stopping at `stop_at`.
    /// If `stop_at` is non-empty, returns `true` if all ancestors up to `stop_at` are open.
    /// If `stop_at` is empty, returns `true` if all ancestors are open.
    pub(crate) fn cr_tree_ancestors_all_open(
        &self,
        grid: &TableGrid,
        iid: &str,
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
            let cb = cell.borrow();
            if !cb.open {
                return Ok(false);
            }
            if stop_at_provided && cb.iid.as_str() == stop_at {
                return Ok(true);
            }
        }
        Ok(true)
    }
}
