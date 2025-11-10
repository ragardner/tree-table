use crate::EventData;
use crate::GetCellResultExt;
use crate::GetResultExt;
use crate::Index;
use crate::IndexCell;
use crate::Table;
use crate::TableGrid;
use crate::str_err;
use crate::types::TreeMove;
use crate::types::event_data::TreeChangeKind;
use crate::utils::utils_fns::binary_search_ins_pos;
use crate::utils::utils_fns::{offset_move_indices, remove_duplicates_outside_section};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Table {
    pub(crate) fn acr_tree_restore_cells(
        index: &mut Index,
        restore_from: &mut EventData,
        record_to: Option<&mut EventData>,
    ) -> Result<(), String> {
        if let Some(record_to) = record_to {
            for (iid, tree_change) in restore_from.tree_changes.drain() {
                let rn = Self::acr_pos(&index.iid_to_row, &iid)?;
                if let Some(change) = tree_change.par {
                    let inv = change.invert(&mut index.cells, &mut index.iid_to_row, rn)?;
                    record_to.add_tree_change(&iid, inv);
                }
                if let Some(change) = tree_change.chn {
                    let inv = change.invert(&mut index.cells, &mut index.iid_to_row, rn)?;
                    record_to.add_tree_change(&iid, inv);
                }
            }
        } else {
            for (iid, tree_change) in restore_from.tree_changes.drain() {
                let rn = Self::acr_pos(&index.iid_to_row, &iid)?;
                if let Some(change) = tree_change.par {
                    change.invert(&mut index.cells, &mut index.iid_to_row, rn)?;
                }
                if let Some(change) = tree_change.chn {
                    change.invert(&mut index.cells, &mut index.iid_to_row, rn)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn cr_tree_get_iid_insert_row(
        &self,
        grid: &TableGrid,
        pos: Option<usize>,
        par: &str,
    ) -> Result<usize, String> {
        if !par.is_empty() {
            let chn = Self::acr_tree_iid_chn(&grid.index.cells, &grid.index.iid_to_row, par)?;
            if let Some(idx) = pos {
                let idx = idx.min(chn.len());
                if idx == 0 {
                    let parent_rn = Self::acr_pos(&grid.index.iid_to_row, par)?;
                    return Ok(parent_rn + 1);
                } else {
                    let prev_chld = chn.get(idx - 1).ok_or(str_err!(format!(
                        "Could not get child at pos: {} of parent: {}",
                        idx - 1,
                        par
                    )))?;
                    let prev_chld_rn = Self::acr_pos(&grid.index.iid_to_row, prev_chld)?;
                    let num_desc = Self::acr_tree_descendants_num(&grid, prev_chld_rn, false)?;
                    return Ok(prev_chld_rn + num_desc + 1);
                }
            } else {
                if !chn.is_empty() {
                    let last_chld = chn
                        .last()
                        .ok_or(str_err!(format!("Could not get last child of: {}", par)))?;
                    let last_chld_rn = Self::acr_pos(&grid.index.iid_to_row, last_chld)?;
                    let num_desc = Self::acr_tree_descendants_num(&grid, last_chld_rn, false)?;
                    return Ok(last_chld_rn + num_desc + 1);
                } else {
                    let parent_rn = Self::acr_pos(&grid.index.iid_to_row, par)?;
                    return Ok(parent_rn + 1);
                }
            }
        } else {
            if let Some(idx) = pos {
                if idx == 0 {
                    return Ok(0);
                } else {
                    if let Some(datarn) = Self::acr_tree_top_iid_rn_at_pos(
                        &grid.index.cells,
                        &grid.index.iid_to_row,
                        idx,
                    ) {
                        return Ok(datarn);
                    } else {
                        return Ok(Self::acr_total_rows(&grid.index.cells));
                    }
                }
            } else {
                return Ok(Self::acr_total_rows(&grid.index.cells));
            }
        }
    }

    fn acr_tree_move_iid(
        index: &mut Index,
        mut event_data: &mut EventData,
        iid: &Rc<str>,
        new_par: &Rc<str>,
        row_ctr: usize,
    ) -> Result<usize, String> {
        // Get the current parent of the item
        let current_par = Self::acr_tree_iid_par(&index.cells, &index.iid_to_row, iid)?;

        // Handle the move operation
        if new_par != &current_par {
            if !current_par.is_empty() {
                // Remove iid from its current parent's chn
                Self::acr_tree_remove_iid_from_pars_chn(
                    &mut index.cells,
                    &index.iid_to_row,
                    iid,
                    &mut event_data,
                )?;
            }
            let item_cell = Self::acr_tree_iid_cell_mut(&mut index.cells, &index.iid_to_row, &iid)?;
            let change = item_cell.record_set_par(new_par.clone())?;
            event_data.add_tree_change(iid, change);
        }

        // Update row mappings in event_data.moved_rows
        let mut current_row_ctr = row_ctr;
        let item_rn = Self::acr_pos(&index.iid_to_row, &iid)?;
        if !event_data.moved_rows.contains_key(&(item_rn)) {
            event_data.moved_rows.insert(item_rn, current_row_ctr);
            current_row_ctr += 1;
        }
        for did_res in
            Self::acr_tree_descendants_rows(&index.cells, &index.iid_to_row, item_rn, false)
        {
            let did_rn = did_res?;
            if !event_data.moved_rows.contains_key(&did_rn) {
                event_data.moved_rows.insert(did_rn, current_row_ctr);
                current_row_ctr += 1;
            }
        }

        // Return the updated row counter
        Ok(current_row_ctr)
    }

    pub(crate) fn cr_tree_move_rows_mod_cells(
        &self,
        grid: &mut TableGrid,
        mut event_data: &mut EventData,
        new_to_old: &mut HashMap<usize, usize>,
        tree_undo_data: Option<&mut EventData>,
        tree_move: Option<TreeMove>,
    ) -> Result<(), String> {
        let new_par: Rc<str>;
        let mut new_loc_displayed: bool = false;
        let mut from_undo: bool = false;

        // Caller was originally an undo
        if let Some(undo_data) = tree_undo_data {
            from_undo = true;
            Self::acr_tree_restore_cells(&mut grid.index, undo_data, Some(&mut event_data))?;

        // Caller was not originally an undo
        } else if !event_data.moved_rows.is_empty() || tree_move.is_some() {
            let to_move: Vec<usize>;
            let mut insert_row: usize;
            let mut move_to_pos: usize;

            // There is a TreeMove object
            if let Some(move_op) = tree_move {
                // Only removes descendants from this prospective list of ids
                // to be moved. Does not make changes to the tree
                to_move = self.cr_tree_remove_descendants(move_op.iids)?;
                new_par = move_op.new_par;

                // New parent exists
                if !new_par.is_empty() {
                    let new_par_chn = Self::acr_tree_iid_chn(
                        &grid.index.cells,
                        &grid.index.iid_to_row,
                        &new_par,
                    )?;
                    let new_par_chn_len = new_par_chn.len();
                    move_to_pos = match move_op.pos {
                        None => new_par_chn.len(),
                        Some(pos) => new_par_chn.len().min(pos as usize),
                    };

                    // Determine insert row & move_to_pos
                    // New parent has no children
                    if new_par_chn_len == 0 {
                        insert_row = Self::acr_pos(&grid.index.iid_to_row, &new_par)? + 1;

                    // New parent has children, move_to_position is end
                    } else if move_to_pos >= new_par_chn_len {
                        let last_chld = new_par_chn.last().ok_or(str_err!(format!(
                            "Could not get last child of: {}",
                            new_par
                        )))?;
                        let last_chld_rn = Self::acr_pos(&grid.index.iid_to_row, last_chld)?;
                        let num_desc = Self::acr_tree_descendants_num(&grid, last_chld_rn, false)?;
                        insert_row = last_chld_rn + num_desc + 1;

                    // New parent has children, move_to_position is not end
                    } else {
                        // Set insert_row to the row of the child at move_to_pos
                        let cx = new_par_chn.get(move_to_pos).ok_or(str_err!(format!(
                            "Vec index out of range: {} for vec: {:?}",
                            move_to_pos, new_par_chn
                        )))?;
                        insert_row = Self::acr_pos(&grid.index.iid_to_row, cx)?;

                        // If any iids are merely moving index
                        // & the index (pos) is > than their current index
                        // then we have to adjust the insert_row and move_to_pos
                        // to accurately drop the rows being moved
                        for rn in to_move.iter() {
                            let iid = Self::acr_borrow_riid(&grid.index.cells, *rn)?;
                            if Self::acr_tree_pars_equal(
                                &grid.index.cells,
                                &grid.index.iid_to_row,
                                iid,
                                &new_par,
                            )? && move_to_pos > Self::acr_tree_pos(&grid.index, &iid, false)?
                            {
                                let cx = new_par_chn.get(move_to_pos).ok_or(str_err!(format!(
                                    "Vec index out of range: {} for iid: {} vec: {:?}",
                                    move_to_pos, iid, new_par_chn
                                )))?;
                                let num_desc = Self::acr_tree_descendants_num(
                                    &grid,
                                    Self::acr_pos(&grid.index.iid_to_row, cx)?,
                                    false,
                                )?;
                                insert_row += 1 + num_desc;
                                move_to_pos += 1;
                                if move_to_pos >= new_par_chn_len {
                                    break;
                                }
                            }
                        }
                    }

                // NO new parent
                } else {
                    let num_top = Self::acr_tree_top_iids(&grid.index.cells).count();
                    move_to_pos = match move_op.pos {
                        None => num_top,
                        Some(pos) => num_top.min(pos as usize),
                    };
                    insert_row = self.cr_tree_get_iid_insert_row(&grid, Some(move_to_pos), "")?;
                };

            // There is NO TreeMove object, use event_data to determine move
            } else {
                let move_to_iid: Rc<str>;
                to_move = self.cr_tree_remove_descendants(
                    event_data
                        .moved_rows
                        .iter()
                        .filter_map(|(rn, _)| {
                            Self::acr_borrow_riid(&grid.index.cells, *rn)
                                .ok()
                                .map(|r| r.clone())
                        })
                        .collect::<HashSet<Rc<str>>>(),
                )?;

                // event_data.row is Some, it's a displayed row number
                if let Some(drop_row) = event_data.row {
                    if drop_row >= grid.disp_rows.len() {
                        insert_row = Self::acr_total_rows(&grid.index.cells);
                    } else {
                        insert_row = Self::acr_disp_to_data_row(&&grid.disp_rows, drop_row.clone())?
                    }
                    let _get_rn = insert_row.min(Self::acr_total_rows(&grid.index.cells) - 1);
                    move_to_iid = Self::acr_borrow_riid(&grid.index.cells, _get_rn)?.clone();

                // event_data.row is None, we'll have to use event_data.moved_rows to
                // find the insert_row etc
                } else {
                    let min_from = event_data
                        .moved_rows
                        .keys()
                        .min()
                        .ok_or(str_err!("event_data.moved_rows unexpectedly empty"))?
                        .clone();
                    let min_to = event_data
                        .moved_rows
                        .values()
                        .min()
                        .ok_or(str_err!("event_data.moved_rows unexpectedly empty"))?
                        .clone();
                    let max_to = event_data
                        .moved_rows
                        .values()
                        .max()
                        .ok_or(str_err!("event_data.moved_rows unexpectedly empty"))?
                        .clone();
                    insert_row = if min_from <= min_to { max_to } else { min_to };
                    move_to_iid = Self::acr_borrow_riid(&grid.index.cells, insert_row)?.clone();
                }
                move_to_pos = Self::acr_tree_pos(&grid.index, &move_to_iid, false)?;
                new_par = Self::acr_tree_iid_par(
                    &grid.index.cells,
                    &grid.index.iid_to_row,
                    &move_to_iid,
                )?;

                // Check for recursive loops
                let mut causes_recursive_loop: Option<(Rc<str>, Rc<str>)> = None;
                for &row in to_move.iter() {
                    let iid = Self::acr_borrow_riid(&grid.index.cells, row)?;
                    if Self::acr_tree_move_pid_causes_recursive_loop(
                        &grid.index.cells,
                        &grid.index.iid_to_row,
                        &iid,
                        &new_par,
                    )? {
                        causes_recursive_loop = Some((iid.clone(), new_par.clone()));
                        break;
                    }
                }
                if let Some(causes_recursive_loop) = causes_recursive_loop {
                    event_data.moved_rows.clear();
                    return Err(str_err!(format!(
                        "Moving '{}' to '{}' would cause a recursive loop",
                        causes_recursive_loop.0, causes_recursive_loop.1,
                    )));
                }
            }

            // Set up row counter to determine new row mapping
            let mut row_ctr: usize = to_move.get(0).ok_or(str_err!("to_move is empty"))?.clone();
            event_data.moved_rows = HashMap::new();
            event_data.moved_rows.insert(row_ctr.clone(), insert_row);

            // Determine if new location for all iids is visible
            new_loc_displayed = if new_par.is_empty() {
                true
            } else {
                Self::acr_tree_iid_is_open(&grid.index.cells, &grid.index.iid_to_row, &new_par)?
                    && Self::acr_row_iid_is_vis(
                        &grid.index.cells,
                        &grid.index.iid_to_row,
                        &new_par,
                    )?
            };

            // Move the iids
            let mut iids: Vec<Rc<str>> = Vec::new();
            for rn in to_move.iter() {
                let iid = Self::acr_copy_riid(&grid.index.cells, *rn)?;
                row_ctr =
                    Self::acr_tree_move_iid(&mut grid.index, event_data, &iid, &new_par, row_ctr)?;
                iids.push(iid);
            }

            // If new parent, add the new iids to the new parent's children
            if !new_par.is_empty() {
                let parent = Self::acr_icell_mut(
                    &mut grid.index.cells,
                    *grid.index.iid_to_row.get_res(&new_par)?,
                )?;
                let mut chn = std::mem::take(&mut parent.chn);
                let iids_len = iids.len();
                chn.splice(move_to_pos..move_to_pos, iids);
                // Remove any duplicates which are there if any of the iids only moved position
                // & restore the children to the cell
                parent.chn = remove_duplicates_outside_section(chn, move_to_pos, iids_len);
            }

            event_data.moved_rows = offset_move_indices(
                insert_row,
                &event_data
                    .moved_rows
                    .keys()
                    .copied()
                    .collect::<Vec<usize>>(),
                // new_to_old gets edited to be the inverse of moved_rows
                new_to_old,
            )
        }
        if !from_undo && !event_data.moved_rows.is_empty() {
            if !new_loc_displayed {
                // New location is hidden, hide all moved rows
                Self::acr_set_row_vis(
                    &mut grid.index.cells,
                    event_data
                        .moved_rows
                        .keys()
                        .map(|&k| k)
                        .collect::<Vec<usize>>(),
                    false,
                    None,
                )?;
            } else {
                // New location is not hidden, some rows may have been previously hidden
                // We have to check each one
                let mut rows_to_show = Vec::new();
                for &k in event_data.moved_rows.keys() {
                    let row = k;
                    let iid = Self::acr_borrow_riid(&grid.index.cells, row)?;
                    if self.cr_tree_ancestors_all_open(&grid, &iid, "")? {
                        rows_to_show.push(row);
                    }
                }
                Self::acr_set_row_vis(&mut grid.index.cells, rows_to_show.into_iter(), true, None)?;
            }
        };
        Ok(())
    }

    /// Restores cells when undo/redo OR modifies parents children if adding rows.
    pub(crate) fn acr_tree_add_rows(
        grid: &mut TableGrid,
        index_cells: &Vec<(usize, IndexCell)>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        if !event_data.tree_changes.is_empty() {
            Self::acr_tree_restore_cells(&mut grid.index, event_data, None)?;
            return Ok(());
        }
        for (row, cell) in index_cells.iter() {
            let par = &cell.par;
            if !par.is_empty() {
                // Parent's children
                let chn =
                    Self::acr_tree_iid_chn_mut(&mut grid.index.cells, &grid.index.iid_to_row, par)?;
                // Parent has children, find Vec position to add new
                if !chn.is_empty() {
                    let pos = binary_search_ins_pos(&chn, &grid.index.iid_to_row, *row)?;
                    chn.insert(pos, cell.iid.clone());

                // Parent has NO children, extend with new
                } else {
                    chn.push(cell.iid.clone());
                }
            }
        }
        Ok(())
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

    /// Stores index cells prior to change
    pub(crate) fn acr_tree_del_rows(
        mut cells: &mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        positions: &Vec<usize>,
        mut event_data: &mut EventData,
    ) -> Result<(), String> {
        // positons should be sorted and unique
        for pos in positions.iter() {
            let cell = Self::acr_icell_mut(&mut cells, *pos)?;
            let change = cell.record_set_open(false);
            event_data.add_tree_change(&cell.iid, change);
            let iid = cell.iid.clone();
            Self::acr_tree_remove_iid_from_pars_chn(
                &mut cells,
                &iid_to_row,
                &iid,
                &mut event_data,
            )?;
        }
        Ok(())
    }

    /// Renames a row iid in the tree.
    /// This function is not called during undo & redo but records the changes necessary
    /// for undo & redo.
    pub(crate) fn acr_tree_rename_iid(
        index: &mut Index,
        old: &str,
        new: Rc<str>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let old_rc: Rc<str> = Rc::from(old);
        let iid_row = Self::acr_pos(&index.iid_to_row, &old_rc)?;

        // Update children's par fields to point to new iid
        let chn = std::mem::take(&mut Self::acr_icell_mut(&mut index.cells, iid_row)?.chn);
        for ciid in chn.iter() {
            let c_row = Self::acr_pos(&index.iid_to_row, &ciid)?;
            let c_cell = Self::acr_icell_mut(&mut index.cells, c_row)?;
            let change = c_cell.record_set_par(new.clone())?;
            event_data.add_change_icells(c_row, change);
        }
        Self::acr_icell_mut(&mut index.cells, iid_row)?.chn = chn;

        // Update parent's chn list if parent exists
        let pos = Self::acr_tree_pos(&index, &old_rc, false)?;
        let par_cell_opt =
            Self::acr_tree_par_cell_mut(&mut index.cells, &index.iid_to_row, &old_rc)?;
        if let Some(par_cell) = par_cell_opt {
            let par_row = Self::acr_pos(&index.iid_to_row, &par_cell.iid)?;
            if pos >= par_cell.chn.len() {
                return Err(str_err!(format!(
                    "Index {} is out of bounds for parent's children list of length {}",
                    pos,
                    par_cell.chn.len()
                )));
            }
            if par_cell.chn[pos] != old_rc {
                return Err(str_err!(format!(
                    "Parent's child at index {} does not match old IID: expected {}, found {}",
                    pos, old_rc, &*par_cell.chn[pos]
                )));
            }
            let mut new_chn = par_cell.chn.clone();
            new_chn[pos] = new.clone();
            let change = par_cell.record_set_chn(new_chn)?;
            event_data.add_change_icells(par_row, change);
        }

        let cell = Self::acr_icell_mut(&mut index.cells, iid_row)?;
        let change = cell.record_set_iid(new.clone(), &mut index.iid_to_row)?;
        event_data.add_change_icells(Self::acr_pos(&index.iid_to_row, &old_rc)?, change);
        index.iid_to_row.remove(&old_rc);
        index.iid_to_row.insert(new, iid_row);
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
            event_data.add_change_icells(pos, change);
        }
        Ok(())
    }

    pub(crate) fn acr_tree_top_iids<'v>(
        cells: &'v Vec<IndexCell>,
    ) -> impl Iterator<Item = &'v Rc<str>> + 'v {
        struct TopIidsIter<'v> {
            cells: &'v Vec<IndexCell>,
            index: usize,
        }

        impl<'v> Iterator for TopIidsIter<'v> {
            type Item = &'v Rc<str>;

            fn next(&mut self) -> Option<Self::Item> {
                while self.index < self.cells.len() {
                    // Assuming `get_res` returns `Result<&'v IndexCell, _>`; adjust if it's custom
                    // If it's actually `self.cells.get(self.index)`, change to `let opt_icell = self.cells.get(self.index);`
                    let res = self.cells.get_res(self.index);
                    self.index += 1;
                    let icell = match res {
                        Ok(cell) => cell,
                        Err(_) => continue, // Skip invalid cells
                    };
                    if icell.par.is_empty() {
                        // Check for no parent (empty Rc<str>)
                        return Some(&icell.iid);
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
        let iid_ref = Self::acr_tree_top_iids(cells).nth(pos)?;
        iid_to_row.get(&*iid_ref).copied()
    }

    /// Determines the index of an iid amongst its siblings.
    pub(crate) fn acr_tree_pos(
        index: &Index,
        iid: &Rc<str>,
        safety: bool,
    ) -> Result<usize, String> {
        // Get the parent item ID; returns Result<Rc<str>, String>
        let par = Self::acr_tree_iid_par(&index.cells, &index.iid_to_row, iid)?;

        if !par.is_empty() {
            // Item has a parent
            if !safety {
                // Fast path using row numbers
                let iid_row = Self::acr_pos(&index.iid_to_row, iid)?;
                let par_row = Self::acr_pos(&index.iid_to_row, &par)?;
                let pos = iid_row - par_row - 1;
                Ok(pos)
            } else {
                // Safe path using the parent's children list
                let par_cell = Self::acr_tree_par_cell(&index.cells, &index.iid_to_row, &par)?;
                if let Some(par_cell) = par_cell {
                    let pos = par_cell
                        .chn
                        .iter()
                        .position(|c| c == iid)
                        .ok_or(str_err!(format!("iid: {} not in parent's children", &*iid)))?;
                    Ok(pos)
                } else {
                    Err(str_err!("Par cell unexpectedly empty."))
                }
            }
        } else {
            // Item is a top-level item
            let pos = Self::acr_tree_top_iids(&index.cells)
                .position(|i| i == iid)
                .ok_or(str_err!(format!("iid: {} not in top-level iids", &*iid)))?;
            Ok(pos)
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

    // Sets IndexCells to open/close and returns a Set of rows to show/hide.
    pub(crate) fn cr_tree_open_close<I>(
        &self,
        grid: &mut TableGrid,
        rows: I,
        open: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<HashSet<usize>, String>
    where
        I: IntoIterator<Item = usize>,
    {
        // Rows to be shown after opening `rows`
        let mut to_show_or_hide: HashSet<usize> = HashSet::new();
        // Helper to determine if a row is visible
        let mut vis_check: HashSet<usize> = HashSet::new();

        match event_data {
            Some(event_data_mut) => {
                for rn in rows {
                    let cell = Self::acr_icell_mut(&mut grid.index.cells, rn)?;
                    event_data_mut.add_change_icells(rn, cell.record_set_open(open));
                    if !cell.chn.is_empty() && cell.vis || vis_check.contains(&rn) {
                        for res in Self::acr_tree_descendants_rows(
                            &grid.index.cells,
                            &grid.index.iid_to_row,
                            rn,
                            true,
                        ) {
                            let did_row = res?;
                            if open {
                                vis_check.insert(did_row);
                            }
                            to_show_or_hide.insert(did_row);
                        }
                    }
                }
            }
            None => {
                for rn in rows {
                    let cell = Self::acr_icell_mut(&mut grid.index.cells, rn)?;
                    cell.open = open;
                    if !cell.chn.is_empty() && cell.vis || vis_check.contains(&rn) {
                        for res in Self::acr_tree_descendants_rows(
                            &grid.index.cells,
                            &grid.index.iid_to_row,
                            rn,
                            true,
                        ) {
                            let did_row = res?;
                            if open {
                                vis_check.insert(did_row);
                            }
                            to_show_or_hide.insert(did_row);
                        }
                    }
                }
            }
        }

        Ok(to_show_or_hide)
    }
}
