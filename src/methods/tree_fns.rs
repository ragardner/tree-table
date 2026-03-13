use crate::Change;
use crate::EventData;
use crate::GetCellResultExt;
use crate::Index;
use crate::IndexCell;
use crate::Table;
use crate::str_err;
use crate::types::event_data::TreeChange;
use crate::types::event_data::TreeChangeKind;
use crate::utils::utils_fns::binary_search_exact_pos;
use std::collections::HashMap;
use std::rc::Rc;

impl Table {
    pub(crate) fn acr_tree_restore_cells(
        index: &mut Index,
        restore_from: &mut HashMap<Rc<str>, TreeChange>,
        record_to: Option<&mut EventData>,
    ) -> Result<(), String> {
        let mut error: Option<String> = None;
        if let Some(record_to) = record_to {
            for (iid, tree_change) in restore_from.drain() {
                let rn = match Self::acr_pos(&index.iid_to_row, &iid) {
                    Ok(rn) => rn,
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                };
                if let Some(change) = tree_change.par {
                    let inv = match change.invert(&mut index.cells, &mut index.iid_to_row, rn) {
                        Ok(inv) => inv,
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                    };
                    record_to.add_tree_change(&iid, inv);
                }
                if let Some(change) = tree_change.chn {
                    let inv = match change.invert(&mut index.cells, &mut index.iid_to_row, rn) {
                        Ok(inv) => inv,
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                    };
                    record_to.add_tree_change(&iid, inv);
                }
            }
            if let Some(error) = error {
                for (iid, tree_change) in record_to.tree_changes.drain() {
                    if let Ok(rn) = Self::acr_pos(&index.iid_to_row, &iid) {
                        if let Some(change) = tree_change.par {
                            let _ = change.invert(&mut index.cells, &mut index.iid_to_row, rn);
                        }
                        if let Some(change) = tree_change.chn {
                            let _ = change.invert(&mut index.cells, &mut index.iid_to_row, rn);
                        }
                    }
                }
                return Err(error);
            }
        } else {
            let mut temp: EventData = EventData::new();
            for (iid, tree_change) in restore_from.drain() {
                let rn = match Self::acr_pos(&index.iid_to_row, &iid) {
                    Ok(rn) => rn,
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                };
                if let Some(change) = tree_change.par {
                    let inv = match change.invert(&mut index.cells, &mut index.iid_to_row, rn) {
                        Ok(inv) => inv,
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                    };
                    temp.add_tree_change(&iid, inv);
                }
                if let Some(change) = tree_change.chn {
                    let inv = match change.invert(&mut index.cells, &mut index.iid_to_row, rn) {
                        Ok(inv) => inv,
                        Err(e) => {
                            error = Some(e);
                            break;
                        }
                    };
                    temp.add_tree_change(&iid, inv);
                }
            }
            if let Some(error) = error {
                for (iid, tree_change) in temp.tree_changes.drain() {
                    if let Ok(rn) = Self::acr_pos(&index.iid_to_row, &iid) {
                        if let Some(change) = tree_change.par {
                            let _ = change.invert(&mut index.cells, &mut index.iid_to_row, rn);
                        }
                        if let Some(change) = tree_change.chn {
                            let _ = change.invert(&mut index.cells, &mut index.iid_to_row, rn);
                        }
                    }
                }
                return Err(error);
            }
        }
        Ok(())
    }

    pub(crate) fn cr_tree_get_iid_insert_row(
        &self,
        pos: Option<usize>,
        par: &str,
    ) -> Result<usize, String> {
        if !par.is_empty() {
            let chn =
                Self::acr_tree_iid_chn(&self.grid.index.cells, &self.grid.index.iid_to_row, par)?;
            if let Some(idx) = pos {
                let idx = idx.min(chn.len());
                if idx == 0 {
                    let parent_rn = Self::acr_pos(&self.grid.index.iid_to_row, par)?;
                    return Ok(parent_rn + 1);
                } else {
                    let prev_chld = chn.get(idx - 1).ok_or(str_err!(
                        "Could not get child at pos: {} of parent: {}",
                        idx - 1,
                        par
                    ))?;
                    let prev_chld_rn = Self::acr_pos(&self.grid.index.iid_to_row, prev_chld)?;
                    let num_desc = Self::acr_tree_descendants_num(&self.grid, prev_chld_rn, false)?;
                    return Ok(prev_chld_rn + num_desc + 1);
                }
            } else {
                if !chn.is_empty() {
                    let last_chld = chn
                        .last()
                        .ok_or(str_err!("Could not get last child of: {}", par))?;
                    let last_chld_rn = Self::acr_pos(&self.grid.index.iid_to_row, last_chld)?;
                    let num_desc = Self::acr_tree_descendants_num(&self.grid, last_chld_rn, false)?;
                    return Ok(last_chld_rn + num_desc + 1);
                } else {
                    let parent_rn = Self::acr_pos(&self.grid.index.iid_to_row, par)?;
                    return Ok(parent_rn + 1);
                }
            }
        } else {
            if let Some(idx) = pos {
                if idx == 0 {
                    return Ok(0);
                } else {
                    if let Some(datarn) = Self::acr_tree_top_iid_rn_at_pos(
                        &self.grid.index.cells,
                        &self.grid.index.iid_to_row,
                        idx,
                    ) {
                        return Ok(datarn);
                    } else {
                        return Ok(Self::acr_total_rows(&self.grid.index.cells));
                    }
                }
            } else {
                return Ok(Self::acr_total_rows(&self.grid.index.cells));
            }
        }
    }

    #[inline]
    pub(crate) fn acr_tree_remove_iid_from_pars_chn(
        cells: &mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        if let Some(par_cell) = Self::acr_tree_par_cell_mut(cells, iid_to_row, iid)? {
            let par_iid = par_cell.iid.clone(); // Get the parent's iid for correct keying
            let event_data_required =
                event_data.tree_change_required(&par_iid, TreeChangeKind::Chn);
            if event_data_required {
                let new_chn_iids: Vec<Rc<str>> = par_cell
                    .chn
                    .iter()
                    .filter(|ciid| &***ciid != iid)
                    .map(|rc| rc.clone())
                    .collect();
                let change = par_cell.record_set_chn(new_chn_iids)?;
                event_data.add_tree_change(&par_iid, change);
            } else {
                // Find the index of the child to remove
                if let Some(index) = par_cell.chn.iter().position(|ciid| &**ciid == iid) {
                    par_cell.chn.remove(index);
                }
            }
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_tree_iid_set_open(
        mut cells: &mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
        open: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<(), String> {
        let change =
            Self::acr_tree_iid_cell_mut(&mut cells, &iid_to_row, iid)?.record_set_open(open);
        if let Some(event_data) = event_data {
            let pos = Self::acr_pos(&iid_to_row, iid)?;
            event_data
                .changes
                .push(Change::IndexCell { row: pos, change });
        }
        Ok(())
    }

    pub(crate) fn acr_tree_top_iids<'v>(
        cells: &'v Vec<IndexCell>,
    ) -> impl Iterator<Item = Result<&'v Rc<str>, String>> + 'v {
        struct TopIidsIter<'v> {
            cells: &'v Vec<IndexCell>,
            index: usize,
        }

        impl<'v> Iterator for TopIidsIter<'v> {
            type Item = Result<&'v Rc<str>, String>;

            fn next(&mut self) -> Option<Self::Item> {
                while self.index < self.cells.len() {
                    let res = self.cells.get_res(self.index);
                    self.index += 1;
                    match res {
                        Ok(icell) => {
                            if icell.par.is_empty() {
                                return Some(Ok(&icell.iid));
                            }
                            // Continue to the next cell if not a top-level
                        }
                        Err(e) => return Some(Err(e)),
                    }
                }
                None
            }
        }

        TopIidsIter { cells, index: 0 }
    }

    pub(crate) fn acr_tree_top_iid_rn_at_pos<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        pos: usize,
    ) -> Option<usize> {
        let iid_res = Self::acr_tree_top_iids(cells).nth(pos)?;
        match iid_res {
            Ok(iid) => iid_to_row.get(&*iid).copied(),
            Err(_) => None,
        }
    }

    /// Determines the index of an iid amongst its siblings.
    pub(crate) fn acr_tree_pos(index: &Index, iid: &Rc<str>) -> Result<usize, String> {
        // Get the target row for binary search
        let target_row = index
            .iid_to_row
            .get(iid)
            .ok_or(format!("Missing row mapping for IID: {}", &*iid))?
            .to_owned();

        // Get the parent item ID; returns Result<Rc<str>, String>
        let par = Self::acr_tree_iid_par(&index.cells, &index.iid_to_row, iid)?;

        if !par.is_empty() {
            // Item has a parent
            let par_cell = Self::acr_tree_par_cell(&index.cells, &index.iid_to_row, &par)?;
            if let Some(par_cell) = par_cell {
                // Binary search for exact pos by row
                if let Some(pos) =
                    binary_search_exact_pos(&par_cell.chn, &index.iid_to_row, target_row)?
                {
                    // Verify it's the correct IID (extra safety, though rows should be unique)
                    if &par_cell.chn[pos] == iid {
                        Ok(pos)
                    } else {
                        Err(str_err!("Row match found but IID mismatch for: {}", &*iid))
                    }
                } else {
                    Err(str_err!("iid: {} not in parent's children", &*iid))
                }
            } else {
                Err(str_err!("Par cell unexpectedly empty."))
            }
        } else {
            // Item is a top-level item
            let mut pos = 0usize;
            for res in Self::acr_tree_top_iids(&index.cells) {
                if res? == iid {
                    return Ok(pos);
                }
                pos += 1;
            }
            Err(str_err!("iid: {} not in top-level iids", &*iid))
        }
    }

    /// Checks if moving an item under a new parent would cause a recursive loop.
    /// Returns `true` if `move_to_parent` is a descendant of `to_move_iid`, `false` otherwise.
    pub(crate) fn acr_tree_move_pid_causes_recursive_loop<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        to_move_iid: &str,
        move_to_par: &str,
    ) -> Result<bool, String> {
        let start_rn = Self::acr_pos(iid_to_row, to_move_iid)?;
        for did_res in Self::acr_tree_descendants_rows(cells, iid_to_row, start_rn, false) {
            if &**Self::acr_borrow_riid(cells, did_res?)? == move_to_par {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[inline]
    pub(crate) fn acr_tree_exists(iid_to_row: &HashMap<Rc<str>, usize>, iid: &str) -> bool {
        iid_to_row.get(iid).is_some()
    }
}
