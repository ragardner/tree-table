use crate::EventData;
use crate::Header;
use crate::IndexCell;
use crate::MaxMap;
use crate::SelectionKind;
use crate::SelectionState;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::args::CycleHandling;
use crate::args::DuplicateIdHandling;
use crate::args::EmptyIdHandling;
use crate::args::TreeExtendArgs;
use crate::log;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

#[inline]
fn tree_extend_get_descendants(
    initial_chn: &[Rc<RefCell<IndexCell>>],
) -> impl Iterator<Item = Rc<RefCell<IndexCell>>> {
    struct TreeDescendantIterator {
        stack: Vec<Rc<RefCell<IndexCell>>>,
    }

    impl Iterator for TreeDescendantIterator {
        type Item = Rc<RefCell<IndexCell>>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.stack.is_empty() {
                return None;
            }

            let current = self.stack.pop().unwrap();
            let borrow = current.borrow();
            for child in borrow.chn.iter().rev().cloned() {
                self.stack.push(child);
            }
            drop(borrow);
            Some(current)
        }
    }
    let stack: Vec<_> = initial_chn.iter().rev().cloned().collect();
    TreeDescendantIterator { stack }
}

#[inline]
fn tree_extend_check_inf_loop(initial_chn: &[Rc<RefCell<IndexCell>>], par: &str) -> bool {
    for cell in tree_extend_get_descendants(initial_chn) {
        if cell.borrow().iid == par {
            return true;
        }
    }
    false
}

impl Table {
    /// Builds table cells for a given row specification, handling both provided values and inherited properties.
    #[inline]
    fn build_row_cells(
        icells: &Vec<Rc<RefCell<IndexCell>>>,
        header: &Header,
        vals: Vec<(String, Val)>,
        inherit_row: usize,
        do_inherit: bool,
        empty_val: &Val,
        ncols: usize,
    ) -> Result<MaxMap<TableCell>, String> {
        let mut cols_map: MaxMap<TableCell> = MaxMap::new();
        // Process cells provided by row_spec.vals
        for (key, val) in vals.into_iter() {
            let cn = Self::acr_pos(&header.iid_to_col, &key)?;
            let props = if do_inherit {
                Self::acr_tcell_props(&icells, inherit_row, cn)
            } else {
                None
            };
            cols_map.set_cell(
                cn,
                TableCell {
                    val,
                    props: props.map(|pr| (*pr).clone()),
                },
            );
        }
        // Handle inheritance for missing cells
        if do_inherit {
            for cn in 0..ncols {
                if !cols_map.contains_key(&cn) {
                    if let Some(props) = Self::acr_tcell_props(&icells, inherit_row, cn) {
                        cols_map.set_cell(
                            cn,
                            TableCell {
                                val: empty_val.clone(),
                                props: Some(props.clone()),
                            },
                        );
                    }
                }
            }
        }
        Ok(cols_map)
    }

    /// Extend the tree by adding new rows.
    pub fn tree_extend(&self, kwargs: Option<TreeExtendArgs>) -> Result<(), String> {
        let kwargs = kwargs.unwrap_or_default();
        let empty_id_handling = kwargs.empty_ids;
        let duplicate_id_handling = kwargs.duplicate_ids;
        let cycle_handling = kwargs.cycles;
        let check_for_cycles: bool = cycle_handling != CycleHandling::Ignore;
        let mut tree_map: HashMap<String, Rc<RefCell<IndexCell>>> =
            HashMap::with_capacity(kwargs.rows.len());
        let mut tops: Vec<Rc<RefCell<IndexCell>>> = Vec::with_capacity(kwargs.rows.len() / 2);
        // parents added to the tree_map but which never have their own rows as iids
        let mut par_stubs: HashMap<String, Vec<Rc<RefCell<IndexCell>>>> = HashMap::new();
        let mut grid = &mut *self.grid.borrow_mut();
        let opts = self.opts.borrow();
        let sel_ref = self.sel.borrow();
        let empty_val: Val = opts.empty_val.clone();
        let empty_ival: Val = opts.empty_index_val.clone();
        let default_dim = Self::acr_get_default_row_dim(&opts);
        let mut event_data: EventData = self.cr_new_event(true, &opts, &sel_ref);
        let ncols = Self::acr_total_cols(&grid.header.cells);
        let mut row_ctr = Self::acr_total_rows(&grid.index.cells);

        // Determine which row to inherit cell props from
        let opt_inherit_row =
            self.cr_get_inherit_row(&grid.index.iid_to_row, row_ctr, &kwargs.inherit)?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row = opt_inherit_row.unwrap_or(0);
        let index_props = if do_inherit {
            grid.index.clone_props(&inherit_row)
        } else {
            None
        };

        let start = Instant::now();
        // Iterate through the array elements
        for mut row_spec in kwargs.rows.into_iter() {
            // If iid is empty
            let iid = if row_spec.iid.is_empty() {
                match empty_id_handling {
                    EmptyIdHandling::Error => {
                        return Err(str_err!(format!(
                            "Empty iid. Parent: {:?}, Text: {:?}, Values: {:?}",
                            row_spec.par, row_spec.val, row_spec.vals
                        )));
                    }
                    EmptyIdHandling::Ignore => {
                        continue;
                    }
                    EmptyIdHandling::New => {
                        // new_iid() guarantees a new id
                        let mut new = grid.index.new_iid()?;
                        while tree_map.contains_key(&new) {
                            new = grid.index.new_iid()?;
                        }
                        row_spec.iid = new.clone();
                        new
                    }
                }
            // iid should not already be in Table
            } else if grid.index.iid_to_row.contains_key(&row_spec.iid) {
                match duplicate_id_handling {
                    DuplicateIdHandling::Error => {
                        return Err(str_err!(format!("iid already exists: {}", row_spec.iid)));
                    }
                    DuplicateIdHandling::Rename => {
                        let mut n: u64 = 1;
                        let iid = row_spec.iid.as_str();
                        let mut new = format!("{iid}_{n}");
                        while grid.index.iid_to_row.contains_key(&new) {
                            n += 1;
                            new = format!("{iid}_{n}");
                        }
                        row_spec.iid = new.clone();
                        new
                    }
                }
            } else {
                row_spec.iid.clone()
            };

            // par should not already be in Table
            if grid.index.iid_to_row.contains_key(&row_spec.par) {
                let par = row_spec.par;
                match duplicate_id_handling {
                    DuplicateIdHandling::Error => {
                        return Err(str_err!(format!("parent already exists: {}", par)));
                    }
                    DuplicateIdHandling::Rename => {
                        let mut n: u64 = 1;
                        let mut new = format!("{par}_{n}");
                        while grid.index.iid_to_row.contains_key(&new) {
                            n += 1;
                            new = format!("{par}_{n}");
                        }
                        row_spec.par = new;
                    }
                }
            }

            // Check for duplicate iids in rows
            if tree_map.contains_key(&iid) {
                match duplicate_id_handling {
                    DuplicateIdHandling::Error => {
                        return Err(str_err!(format!("iid is a duplicate in rows: {}", iid)));
                    }
                    DuplicateIdHandling::Rename => {
                        let mut n: u64 = 1;
                        let mut new = format!("{iid}_{n}");
                        // Have to check a 2nd time if iid_to_row contains newly renamed iid
                        while tree_map.contains_key(&new)
                            || grid.index.iid_to_row.contains_key(&new)
                        {
                            n += 1;
                            new = format!("{iid}_{n}");
                        }
                        row_spec.iid = new;
                    }
                }
            }

            let cols_map = Self::build_row_cells(
                &grid.index.cells,
                &grid.header,
                row_spec.vals.take().unwrap_or_default(),
                inherit_row,
                do_inherit,
                &empty_val,
                ncols,
            )?;
            let new_icell = Rc::new(RefCell::new(IndexCell {
                iid: row_spec.iid,
                val: row_spec.val.unwrap_or(empty_ival.clone()),
                vals: cols_map,
                dim: row_spec.dim.unwrap_or(default_dim),
                vis: false,
                par: None,
                chn: Vec::new(),
                open: row_spec.open,
                props: index_props.clone(),
            }));
            let mut new_icell_mut = new_icell.borrow_mut();

            // If it is an existing stub, remove it from stubs and
            // xfer the stubs chn and iid to the icell
            // Transfer pending children (assign since input chn is empty)
            if let Some(pending_chn) = par_stubs.remove(&iid) {
                new_icell_mut.chn = pending_chn;
                for child in &new_icell_mut.chn {
                    child.borrow_mut().par = Some(Rc::downgrade(&new_icell));
                }
            }
            // Check for cycles (inf loops of children) and id-parent equality
            if iid == row_spec.par {
                match cycle_handling {
                    CycleHandling::Error => {
                        return Err(str_err!(format!("iid: {iid} equal to parent")));
                    }
                    CycleHandling::Skip => {
                        continue;
                    }
                    CycleHandling::OnlyPrior => {
                        break;
                    }
                    _ => row_spec.par = String::new(),
                }
            }
            if check_for_cycles
                && tree_extend_check_inf_loop(new_icell_mut.chn.as_slice(), &row_spec.par)
            {
                match cycle_handling {
                    CycleHandling::Error => {
                        return Err(str_err!(format!(
                            "Cycle detected for iid: {} with parent: {}",
                            iid, row_spec.par
                        )));
                    }
                    CycleHandling::Skip => {
                        continue;
                    }
                    CycleHandling::Clear => {
                        row_spec.par = String::new();
                    }
                    CycleHandling::OnlyPrior => {
                        break;
                    }
                    CycleHandling::Ignore => unreachable!(),
                }
            }

            // Finally, deal with parent and add icell
            if !row_spec.par.is_empty() {
                if let Some(par_cell_rc) = tree_map.get(&row_spec.par) {
                    // Parent is already an iid in tree
                    // - Set icell parent
                    // - Add icell to parents chn
                    new_icell_mut.par = Some(Rc::downgrade(&par_cell_rc));
                    par_cell_rc.borrow_mut().chn.push(new_icell.clone());
                } else {
                    // Parent is not already an iid in tree
                    // Append icell to par_stubs under par iid
                    // All chn under par iid stubs (including new_icell)
                    // will have to have .par updated later
                    par_stubs
                        .entry(row_spec.par)
                        .or_insert_with(Vec::new)
                        .push(new_icell.clone());
                }
            } else {
                new_icell_mut.vis = true;
                tops.push(new_icell.clone());
            }
            drop(new_icell_mut);
            tree_map.insert(iid, new_icell);
        }
        let duration = start.elapsed();
        log!("First traversal time taken: {:?}", duration);

        let start = Instant::now();
        // Drain remaining stub ids into tree_map as top-level with defaults
        for (iid, chn) in par_stubs.into_iter() {
            let cols_map = Self::build_row_cells(
                &grid.index.cells,
                &grid.header,
                vec![],
                inherit_row,
                do_inherit,
                &empty_val,
                ncols,
            )?;
            let stub = Rc::new(RefCell::new(IndexCell {
                iid: iid.clone(),
                val: empty_ival.clone(),
                vals: cols_map,
                dim: default_dim.clone(),
                vis: true,
                par: None,
                chn,
                open: false,
                props: index_props.clone(),
            }));
            let stub_mut = stub.borrow_mut();
            for c in &stub_mut.chn {
                c.borrow_mut().par = Some(Rc::downgrade(&stub));
            }
            drop(stub_mut);
            tops.push(stub.clone());
            tree_map.insert(iid, stub);
        }

        // Return early without modifying existing table if no rows are to be added
        if tree_map.is_empty() {
            return Err(str_err!("No rows to be added"));
        }
        let mut sorted_seq: Vec<usize> = Vec::with_capacity(tree_map.len());

        // self.cr_remake_rows()
        let mut acc: u64 = grid.canvas_rows.last().cloned().unwrap_or(0);
        let mut stack: Vec<(Rc<RefCell<IndexCell>>, bool)> = Vec::new();
        grid.index.cells.reserve(tree_map.len());
        grid.index.iid_to_row.reserve(tree_map.len());
        for top in tops.into_iter() {
            sorted_seq.push(row_ctr);
            let cb_mut = top.borrow();
            let iid = cb_mut.iid.clone();
            let child_vis = cb_mut.open;
            for child in cb_mut.chn.iter().rev().cloned() {
                stack.push((child, child_vis));
            }
            grid.disp_rows.push(row_ctr);
            acc += cb_mut.dim;
            grid.canvas_rows.push(acc);
            drop(cb_mut);
            grid.index.push_row(row_ctr, iid, top);
            row_ctr += 1;

            while let Some((icell, inherited_vis)) = stack.pop() {
                sorted_seq.push(row_ctr);
                let mut cb_mut = icell.borrow_mut();
                let iid = cb_mut.iid.clone();
                cb_mut.vis = inherited_vis;
                let child_vis = inherited_vis && cb_mut.open;
                for child in cb_mut.chn.iter().rev().cloned() {
                    stack.push((child, child_vis));
                }
                // Row grid lines, self.cr_remake_rows()
                if inherited_vis {
                    grid.disp_rows.push(row_ctr);
                    acc += cb_mut.dim;
                    grid.canvas_rows.push(acc);
                }
                // Push cell to index, self.grid.index.add_rows()
                // Only clones a reference
                drop(cb_mut);
                grid.index.push_row(row_ctr, iid, icell);
                row_ctr += 1;
            }
        }
        event_data.added_rows = sorted_seq;
        event_data.selection_boxes = sel_ref.selection_boxes.clone();
        event_data.selected = sel_ref.selected.clone();
        drop(sel_ref);

        // Select the added rows
        if kwargs.select {
            let mut sel = self.sel.borrow_mut();
            Self::acr_deselect_all(&mut sel);
            for (start, end) in consecutive_ranges_owned(
                event_data
                    .added_rows
                    .iter()
                    .filter_map(|n| Self::acr_data_to_disp_opt(&grid.disp_rows, *n)),
            ) {
                Self::acr_add_selection_box(
                    &mut sel,
                    &grid,
                    start,
                    Some(end),
                    0,
                    None,
                    SelectionKind::Rows,
                    SelectionState::Normal,
                    true,
                );
            }
        }
        let duration = start.elapsed();
        log!("Second traversal time taken: {:?}", duration);

        // Sheet has been modified
        self.cr_event_modified(
            &mut grid,
            &self.events.borrow(),
            &event_data,
            kwargs.emit,
            true,
        )?;

        // Add to undo stack if undo is enabled and undo is true in kwargs
        if opts.enabled_undo && kwargs.undo {
            grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
