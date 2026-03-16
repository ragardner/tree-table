use crate::Change;
use crate::EventData;
use crate::GetResultExt;
use crate::Index;
use crate::Table;
use crate::str_err;
use crate::types::TreeMove;
use crate::types::event_data::TreeChange;
use crate::utils::utils_fns::get_full_move_map;
use crate::utils::utils_fns::offset_move_indices;
use crate::utils::utils_fns::remove_duplicates_outside_section;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_move_rows(
        &mut self,
        event_data: &mut EventData,
        old: HashSet<usize>,
        mut old_to_new: Vec<(usize, usize)>,
        mut new_to_old: HashMap<usize, usize>,
        tree_undo_data: Option<&mut HashMap<Rc<str>, TreeChange>>,
        tree_move: Option<TreeMove>,
        mod_tree: bool,
    ) -> Result<(), String> {
        // Modify tree
        if mod_tree && self.opts.tree_mode {
            // potentially mutates old_to_new and new_to_old
            match self.cr_tree_move_rows_mod_cells(
                event_data,
                &mut old_to_new,
                &mut new_to_old,
                tree_undo_data,
                tree_move,
            ) {
                Err(e) => {
                    let _ = self.cr_undo_and_invert_event(
                        Rc::new(RefCell::new(std::mem::take(event_data))),
                        vec![],
                        false,
                    );
                    return Err(e);
                }
                _ => {}
            }
        }

        // Create full Vec of new positions
        let (full_old_to_new, full_new_to_old) = get_full_move_map(
            Self::acr_total_rows(&self.grid.index.cells),
            &old,
            &new_to_old,
        );

        // Grid move – if this fails, undo any tree changes we made
        if let Err(e) = self.grid.move_rows(&full_old_to_new, &full_new_to_old) {
            if mod_tree && self.opts.tree_mode {
                let _ = self.cr_undo_and_invert_event(
                    Rc::new(RefCell::new(std::mem::take(event_data))),
                    vec![],
                    false,
                );
            }
            return Err(e);
        }

        event_data.changes.push(Change::MoveRows(old_to_new));

        Self::acr_remake_rows(
            &mut self.grid.canvas_rows,
            &mut self.grid.disp_rows,
            &self.grid.index.cells,
            self.opts.horizontal_grid_line_width.clone(),
        );

        Ok(())
    }

    fn acr_tree_move_iid(
        index: &mut Index,
        mut event_data: &mut EventData,
        old_to_new: &mut Vec<(usize, usize)>,
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

        // Update row mappings in old_to_new
        let mut current_row_ctr = row_ctr;
        let item_rn = Self::acr_pos(&index.iid_to_row, &iid)?;

        old_to_new.push((item_rn, current_row_ctr));
        current_row_ctr += 1;

        for did_res in
            Self::acr_tree_descendants_rows(&index.cells, &index.iid_to_row, item_rn, false)
        {
            let did_rn = did_res?;
            old_to_new.push((did_rn, current_row_ctr));
            current_row_ctr += 1;
        }

        // Return the updated row counter
        Ok(current_row_ctr)
    }

    pub(crate) fn cr_tree_move_rows_mod_cells(
        &mut self,
        mut event_data: &mut EventData,
        mut old_to_new: &mut Vec<(usize, usize)>,
        mut new_to_old: &mut HashMap<usize, usize>,
        tree_undo_data: Option<&mut HashMap<Rc<str>, TreeChange>>,
        tree_move: Option<TreeMove>,
    ) -> Result<(), String> {
        // Caller was originally an undo
        if let Some(undo_data) = tree_undo_data {
            Self::acr_tree_restore_cells(&mut self.grid.index, undo_data, Some(&mut event_data))?;
            return Ok(());
        };

        // Nothing to move
        if old_to_new.is_empty() && tree_move.is_none() {
            return Ok(());
        }

        let to_move: Vec<usize>;
        let mut insert_row: usize;
        let mut move_to_pos: usize;
        let new_par: Rc<str>;

        // There is a TreeMove object
        if let Some(move_op) = tree_move {
            // Only removes descendants from this prospective list of ids
            // to be moved. Does not make changes to the tree
            to_move = self.cr_tree_remove_descendants(move_op.iids)?;
            new_par = move_op.new_par;

            // New parent exists
            if !new_par.is_empty() {
                let new_par_chn = Self::acr_tree_iid_chn(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
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
                    insert_row = Self::acr_pos(&self.grid.index.iid_to_row, &new_par)? + 1;

                // New parent has children, move_to_position is end
                } else if move_to_pos >= new_par_chn_len {
                    let last_chld = new_par_chn
                        .last()
                        .ok_or(str_err!("Could not get last child of: {}", new_par))?;
                    let last_chld_rn = Self::acr_pos(&self.grid.index.iid_to_row, last_chld)?;
                    let num_desc = Self::acr_tree_descendants_num(&self.grid, last_chld_rn, false)?;
                    insert_row = last_chld_rn + num_desc + 1;

                // New parent has children, move_to_position is not end
                } else {
                    // Set insert_row to the row of the child at move_to_pos
                    let cx = new_par_chn.get(move_to_pos).ok_or(str_err!(
                        "Vec index out of range: {} for vec: {:?}",
                        move_to_pos,
                        new_par_chn
                    ))?;
                    insert_row = Self::acr_pos(&self.grid.index.iid_to_row, cx)?;

                    // If any iids are merely moving index
                    // & the index (pos) is > than their current index
                    // then we have to adjust the insert_row and move_to_pos
                    // to accurately drop the rows being moved
                    for rn in to_move.iter() {
                        let iid = Self::acr_borrow_riid(&self.grid.index.cells, *rn)?;
                        if Self::acr_tree_iids_par_is_equal_to(
                            &self.grid.index.cells,
                            &self.grid.index.iid_to_row,
                            iid,
                            &new_par,
                        )? && move_to_pos > Self::acr_tree_pos(&self.grid.index, &iid)?
                        {
                            let cx = new_par_chn.get(move_to_pos).ok_or(str_err!(
                                "Vec index out of range: {} for iid: {} vec: {:?}",
                                move_to_pos,
                                iid,
                                new_par_chn
                            ))?;
                            let num_desc = Self::acr_tree_descendants_num(
                                &self.grid,
                                Self::acr_pos(&self.grid.index.iid_to_row, cx)?,
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
                let num_top = Self::acr_tree_top_iids(&self.grid.index.cells).count();
                move_to_pos = match move_op.pos {
                    None => num_top,
                    Some(pos) => num_top.min(pos as usize),
                };
                insert_row = self.cr_tree_get_iid_insert_row(Some(move_to_pos), "")?;
            };

        // There is NO TreeMove object, use event_data to determine move
        } else {
            let move_to_iid: Rc<str>;
            to_move = self.cr_tree_remove_descendants(
                old_to_new
                    .iter()
                    .filter_map(|(rn, _)| {
                        Self::acr_borrow_riid(&self.grid.index.cells, *rn)
                            .ok()
                            .map(|r| r.clone())
                    })
                    .collect::<HashSet<Rc<str>>>(),
            )?;

            // find the insert_row etc
            let min_from = old_to_new
                .iter()
                .map(|&(from, _)| from)
                .min()
                .ok_or(str_err!("old_to_new unexpectedly empty"))?;

            let min_to = old_to_new
                .iter()
                .map(|&(_, to)| to)
                .min()
                .ok_or(str_err!("old_to_new unexpectedly empty"))?;

            let max_to = old_to_new
                .iter()
                .map(|&(_, to)| to)
                .max()
                .ok_or(str_err!("old_to_new unexpectedly empty"))?;
            insert_row = if min_from <= min_to { max_to } else { min_to };
            move_to_iid = Self::acr_borrow_riid(&self.grid.index.cells, insert_row)?.clone();

            move_to_pos = Self::acr_tree_pos(&self.grid.index, &move_to_iid)?;
            new_par = Self::acr_tree_iid_par(
                &self.grid.index.cells,
                &self.grid.index.iid_to_row,
                &move_to_iid,
            )?;

            // Check for recursive loops
            let mut causes_recursive_loop: Option<(Rc<str>, Rc<str>)> = None;
            for &row in to_move.iter() {
                let iid = Self::acr_borrow_riid(&self.grid.index.cells, row)?;
                if Self::acr_tree_move_pid_causes_recursive_loop(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    &iid,
                    &new_par,
                )? {
                    causes_recursive_loop = Some((iid.clone(), new_par.clone()));
                    break;
                }
            }
            if let Some(causes_recursive_loop) = causes_recursive_loop {
                old_to_new.clear();
                return Err(str_err!(
                    "Moving '{}' to '{}' would cause a recursive loop",
                    causes_recursive_loop.0,
                    causes_recursive_loop.1,
                ));
            }
        }

        // Set up row counter to determine new row mapping
        let mut row_ctr = insert_row;
        old_to_new.clear();

        // Move the iids
        let mut iids: Vec<Rc<str>> = Vec::new();
        for rn in &to_move {
            let iid = Self::acr_copy_riid(&self.grid.index.cells, *rn)?;
            row_ctr = Self::acr_tree_move_iid(
                &mut self.grid.index,
                event_data,
                &mut old_to_new,
                &iid,
                &new_par,
                row_ctr,
            )?;
            iids.push(iid);
        }

        // If new parent, add the new iids to the new parent's children
        if !new_par.is_empty() {
            let parent = Self::acr_icell_mut(
                &mut self.grid.index.cells,
                *self.grid.index.iid_to_row.get_res(&new_par)?,
            )?;
            let mut chn = std::mem::take(&mut parent.chn);
            let section_size = iids.len();
            chn.splice(move_to_pos..move_to_pos, iids);
            // Remove any duplicates which are there if any of the iids only moved position
            // & restore the children to the cell
            let new_chn = remove_duplicates_outside_section(chn, move_to_pos, section_size);
            event_data.add_tree_change(&new_par, parent.record_set_chn(new_chn)?);
        }
        let old_rows: Vec<usize> = old_to_new.drain(..).map(|(from, _)| from).collect();
        old_to_new.extend(offset_move_indices(
            insert_row,
            &old_rows,
            &mut new_to_old,
            // new_to_old gets edited to be the ..
            // inverse of the updatedold_to_new
        ));

        if !old_to_new.is_empty() {
            // Determine if new location for all iids is visible
            let new_loc_displayed = if new_par.is_empty() {
                true
            } else {
                Self::acr_tree_iid_is_open(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    &new_par,
                )? && Self::acr_row_iid_is_vis(
                    &self.grid.index.cells,
                    &self.grid.index.iid_to_row,
                    &new_par,
                )?
            };
            if !new_loc_displayed {
                // New location is hidden, hide all moved rows
                Self::acr_set_row_vis(
                    &mut self.grid.index.cells,
                    old_to_new.iter().map(|&(from, _)| from),
                    false,
                    Some(event_data),
                )?;
            } else {
                // New location is not hidden, some rows may have been previously hidden
                // We have to check each one
                let mut rows_to_show = Vec::new();
                for k in old_to_new.iter().map(|&(from, _)| from) {
                    let row = k;
                    let iid = Self::acr_borrow_riid(&self.grid.index.cells, row)?;

                    // we're iterating over all moved rows, including ones not in iids
                    // so checking if ancestors are open is necessary
                    //
                    // we haven't moved the rows yet, but we have modified their ancestors/chn
                    // vis was originally set before rows were moved
                    // set vis -> move rows
                    // stored the row numbers prior to row move
                    //
                    // undo sets vis after moving rows
                    // set vis is inverted in undo... so vis IS recorded in undo
                    // undo -> rev() -> (restore cells) move rows -> set vis
                    // set vis is now recorded in undo after moving rows
                    //
                    // so it has the new row numbers so we have to set vis before moving
                    // which redo does do
                    // redo -> rev() -> set vis -> move rows

                    if self.cr_tree_ancestors_all_open(&iid, "")? {
                        rows_to_show.push(row);
                    }
                }
                Self::acr_set_row_vis(
                    &mut self.grid.index.cells,
                    rows_to_show.into_iter(),
                    true,
                    Some(event_data),
                )?;
            }
        };

        Ok(())
    }
}
