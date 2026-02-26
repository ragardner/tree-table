use crate::Change;
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
use crate::str_err;
use crate::types::extensions::TrySetExt;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[inline]
fn tree_extend_get_descendants<'a>(
    tree: &'a HashMap<Rc<str>, IndexCell>,
    initial_chn: &[Rc<str>],
) -> impl Iterator<Item = Rc<str>> + 'a {
    struct TreeDescendantIterator<'a> {
        tree: &'a HashMap<Rc<str>, IndexCell>,
        stack: Vec<Rc<str>>,
    }

    impl Iterator for TreeDescendantIterator<'_> {
        type Item = Rc<str>;

        fn next(&mut self) -> Option<Self::Item> {
            self.stack.pop().map(|current| {
                if let Some(cell) = self.tree.get(&current) {
                    for child in cell.chn.iter().rev().cloned() {
                        self.stack.push(child);
                    }
                }
                current
            })
        }
    }

    let stack = initial_chn.iter().cloned().rev().collect::<Vec<_>>();
    TreeDescendantIterator { tree, stack }
}

#[inline]
fn tree_extend_check_inf_loop(
    tree_map: &HashMap<Rc<str>, IndexCell>,
    initial_chn: &[Rc<str>],
    par: &str,
) -> bool {
    tree_extend_get_descendants(tree_map, initial_chn).any(|did| &*did == par)
}

impl Table {
    /// Builds table cells for a given row specification, handling both provided values and inherited properties.
    #[inline]
    pub(crate) fn build_row_cells(
        icells: &Vec<IndexCell>,
        header: &Header,
        vals: Option<Vec<(String, Val)>>,
        inherit_row: usize,
        do_inherit: bool,
        empty_val: &Val,
        ncols: usize,
    ) -> Result<MaxMap<TableCell>, String> {
        let mut cols_map: MaxMap<TableCell> = MaxMap::new();
        if do_inherit {
            let mut provided: Vec<Option<Val>> = vec![None; ncols];
            if let Some(vals) = vals {
                for (key, val) in vals.into_iter() {
                    let cn = Self::acr_pos(&header.iid_to_col, &key)?;
                    // Last value wins if duplicate positions
                    provided.try_set(cn, Some(val))?;
                }
            }
            for (cn, opt_val) in provided.into_iter().enumerate() {
                let props = Self::acr_tcell_props(icells, inherit_row, cn).cloned();
                if let Some(val) = opt_val {
                    // Provided: set with inherited props (which may be None)
                    cols_map.set_cell(cn, TableCell { val, props });
                } else if props.is_some() {
                    // Missing but has props: set with empty val
                    cols_map.set_cell(
                        cn,
                        TableCell {
                            val: empty_val.clone(),
                            props,
                        },
                    );
                }
                // Otherwise: missing with no props -> skip
            }
        } else {
            // Process cells provided by row_spec.vals
            if let Some(vals) = vals {
                for (key, val) in vals.into_iter() {
                    let cn = Self::acr_pos(&header.iid_to_col, &key)?;
                    cols_map.set_cell(cn, TableCell { val, props: None });
                }
            }
        }
        Ok(cols_map)
    }

    /// Extend the tree by adding new rows.
    pub fn tree_extend(
        &mut self,
        kwargs: Option<TreeExtendArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let empty_id_handling = kwargs.empty_ids;
        let duplicate_id_handling = kwargs.duplicate_ids;
        let cycle_handling = kwargs.cycles;
        let check_for_cycles: bool = cycle_handling != CycleHandling::Ignore;
        let mut tree_map: HashMap<Rc<str>, IndexCell> = HashMap::with_capacity(kwargs.rows.len());
        let mut tops: Vec<Rc<str>> = Vec::with_capacity(kwargs.rows.len() / 2);
        // parents added to the tree_map but which never have their own rows as iids
        let mut par_stubs: HashMap<Rc<str>, Vec<Rc<str>>> = HashMap::new();
        let empty_val: Val = self.opts.empty_val.clone();
        let empty_ival: Val = self.opts.empty_index_val.clone();
        let default_dim = self.cr_get_default_row_dim();
        let mut event_data: EventData = self.cr_new_event(true);
        let ncols = Self::acr_total_cols(&self.grid.header.cells);
        let mut row_ctr = Self::acr_total_rows(&self.grid.index.cells);

        // Determine which row to inherit cell props from
        let opt_inherit_row =
            self.cr_get_inherit_row(&self.grid.index.iid_to_row, row_ctr, &kwargs.inherit)?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row = opt_inherit_row.unwrap_or(0);
        let index_props = if do_inherit {
            self.grid.index.clone_props(&inherit_row)
        } else {
            None
        };

        // Iterate through the array elements
        for mut row_spec in kwargs.rows.into_iter() {
            // We're effectively taking row_spec.iid here
            let iid: Rc<str> = if row_spec.iid.is_empty() {
                match empty_id_handling {
                    EmptyIdHandling::Error => {
                        return Err(str_err!(
                            "Empty iid. Parent: {:?}, Text: {:?}, Values: {:?}",
                            row_spec.par,
                            row_spec.val,
                            row_spec.vals
                        ));
                    }
                    EmptyIdHandling::Ignore => {
                        continue;
                    }
                    EmptyIdHandling::New => {
                        // new_iid() guarantees a new id
                        let mut new = self.grid.index.new_iid()?;
                        while tree_map.contains_key(new.as_str()) {
                            new = self.grid.index.new_iid()?;
                        }
                        Rc::from(new)
                    }
                }
            // iid should not already be in Table
            } else if self
                .grid
                .index
                .iid_to_row
                .contains_key(row_spec.iid.as_str())
            {
                match duplicate_id_handling {
                    DuplicateIdHandling::Error => {
                        return Err(str_err!("iid already exists: {}", row_spec.iid));
                    }
                    DuplicateIdHandling::Rename => {
                        let mut n: u64 = 1;
                        let iid = row_spec.iid.as_str();
                        let mut new = format!("{iid}_{n}");
                        while self.grid.index.iid_to_row.contains_key(new.as_str()) {
                            n += 1;
                            new = format!("{iid}_{n}");
                        }
                        Rc::from(new)
                    }
                }
            } else {
                Rc::from(row_spec.iid)
            };

            let mut par: Rc<str> = Rc::from(row_spec.par);
            // par should not already be in Table
            if self.grid.index.iid_to_row.contains_key(&par) {
                match duplicate_id_handling {
                    DuplicateIdHandling::Error => {
                        return Err(str_err!("parent already exists: {}", par));
                    }
                    DuplicateIdHandling::Rename => {
                        let mut n: u64 = 1;
                        let mut new = format!("{par}_{n}");
                        while self.grid.index.iid_to_row.contains_key(new.as_str()) {
                            n += 1;
                            new = format!("{par}_{n}");
                        }
                        par = Rc::from(new);
                    }
                }
            }

            // Check for duplicate iids in rows
            let iid = if tree_map.contains_key(&iid) {
                match duplicate_id_handling {
                    DuplicateIdHandling::Error => {
                        return Err(str_err!("iid is a duplicate in rows: {}", iid));
                    }
                    DuplicateIdHandling::Rename => {
                        let mut n: u64 = 1;
                        let mut new = format!("{iid}_{n}");
                        // Have to check a 2nd time if iid_to_row contains newly renamed iid
                        while tree_map.contains_key(new.as_str())
                            || self.grid.index.iid_to_row.contains_key(new.as_str())
                        {
                            n += 1;
                            new = format!("{iid}_{n}");
                        }
                        Rc::from(new)
                    }
                }
            } else {
                iid
            };

            let mut new_icell = IndexCell {
                iid: iid.clone(),
                val: row_spec.val.unwrap_or(empty_ival.clone()),
                vals: Self::build_row_cells(
                    &self.grid.index.cells,
                    &self.grid.header,
                    row_spec.vals.take(),
                    inherit_row,
                    do_inherit,
                    &empty_val,
                    ncols,
                )?,
                dim: row_spec.dim.unwrap_or(default_dim),
                vis: false,
                par: self.grid.index.empty_par.clone(),
                chn: Vec::new(),
                open: row_spec.open,
                props: index_props.clone(),
            };

            // If it is an existing stub, remove it from stubs and
            // xfer the stubs chn to the icell
            // (No need to transfer pending children pars here; defer to DFS)
            if let Some(pending_chn) = par_stubs.remove(&iid) {
                new_icell.chn = pending_chn;
            }

            // Check for cycles (inf loops of children) and id-parent equality
            if iid == par {
                match cycle_handling {
                    CycleHandling::Error => {
                        return Err(str_err!("iid: {iid} equal to parent"));
                    }
                    CycleHandling::Skip => {
                        continue;
                    }
                    CycleHandling::OnlyPrior => {
                        break;
                    }
                    _ => par = self.grid.index.empty_par.clone(),
                }
            }
            if check_for_cycles {
                let inf_loop =
                    tree_extend_check_inf_loop(&tree_map, new_icell.chn.as_slice(), &par);
                if inf_loop {
                    match cycle_handling {
                        CycleHandling::Error => {
                            return Err(str_err!(
                                "Cycle detected for iid: {} with parent: {}",
                                iid,
                                par
                            ));
                        }
                        CycleHandling::Skip => {
                            continue;
                        }
                        CycleHandling::Clear => {
                            par = self.grid.index.empty_par.clone();
                        }
                        CycleHandling::OnlyPrior => {
                            break;
                        }
                        CycleHandling::Ignore => unreachable!(),
                    }
                }
            }

            // Finally, deal with parent and add icell
            if !par.is_empty() {
                if let Some(par_cell) = tree_map.get_mut(&par) {
                    // Parent is already an iid in tree
                    // - Set icell parent
                    // - Add icell to parents chn
                    new_icell.par = par;
                    par_cell.chn.push(iid.clone());
                } else {
                    // Parent is not already an iid in tree
                    // Append icell to par_stubs under par iid
                    // All chn under par iid stubs (including new_icell)
                    // will have to have .par updated later
                    par_stubs
                        .entry(par)
                        .or_insert_with(Vec::new)
                        .push(iid.clone());
                }
            } else {
                new_icell.vis = true;
                tops.push(iid.clone());
            }
            tree_map.insert(iid, new_icell);
        }

        // Drain remaining stub ids into tree_map as top-level with defaults
        for (iid, chn) in par_stubs.into_iter() {
            let cols_map = Self::build_row_cells(
                &self.grid.index.cells,
                &self.grid.header,
                None,
                inherit_row,
                do_inherit,
                &empty_val,
                ncols,
            )?;
            let stub = IndexCell {
                iid: iid.clone(),
                val: empty_ival.clone(),
                vals: cols_map,
                dim: default_dim,
                vis: true,
                par: self.grid.index.empty_par.clone(),
                chn,
                open: false,
                props: index_props.clone(),
            };
            // (No need to update child pars here; defer to DFS)
            tops.push(iid.clone());
            tree_map.insert(iid, stub);
        }

        // Return early without modifying existing table if no rows are to be added
        if tree_map.is_empty() {
            return Err(str_err!("No rows to be added"));
        }
        let mut sorted_seq: Vec<usize> = Vec::with_capacity(tree_map.len());

        // self.cr_remake_rows()
        let mut acc: u64 = self.grid.canvas_rows.last().cloned().unwrap_or(0);
        let mut stack: Vec<(Rc<str>, bool, Rc<str>)> = Vec::with_capacity(100);
        self.grid.index.cells.reserve(tree_map.len());
        self.grid.index.iid_to_row.reserve(tree_map.len());
        for top in tops.into_iter() {
            sorted_seq.push(row_ctr);
            let icell = tree_map
                .remove(&top)
                .ok_or(str_err!("IndexCell: {} not found in tree_map", top))?;
            let child_vis = icell.open;
            for child in icell.chn.iter().rev().cloned() {
                stack.push((child, child_vis, top.clone()));
            }
            self.grid.disp_rows.push(row_ctr);
            acc += icell.dim;
            self.grid.canvas_rows.push(acc);
            self.grid.index.push_row(row_ctr, icell.iid.clone(), icell);
            row_ctr += 1;

            while let Some((not_top, inherited_vis, par_iid)) = stack.pop() {
                sorted_seq.push(row_ctr);
                let mut icell = tree_map
                    .remove(&not_top)
                    .ok_or(str_err!("IndexCell: {} not found in tree_map", not_top))?;
                icell.par = par_iid;
                icell.vis = inherited_vis;
                let child_vis = inherited_vis && icell.open;
                for child in icell.chn.iter().rev().cloned() {
                    stack.push((child, child_vis, not_top.clone()));
                }
                // Row grid lines, self.cr_remake_rows()
                if inherited_vis {
                    self.grid.disp_rows.push(row_ctr);
                    acc += icell.dim;
                    self.grid.canvas_rows.push(acc);
                }
                // Push cell to index, self.grid.index.add_rows()
                self.grid.index.push_row(row_ctr, icell.iid.clone(), icell);
                row_ctr += 1;
            }
        }

        let boxes = kwargs
            .save_selection
            .then_some(Self::acr_new_change_with_old_boxes(&self.sel));
        // Select the added rows
        if kwargs.select {
            Self::acr_deselect_all(&mut self.sel);
            for (start, end) in consecutive_ranges_owned(
                sorted_seq
                    .iter()
                    .filter_map(|n| Self::acr_data_to_disp_opt(&self.grid.disp_rows, *n)),
            ) {
                Self::acr_add_selection_box(
                    &mut self.sel,
                    &self.grid,
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

        event_data.changes.push(Change::AddRows(sorted_seq));

        if let Some(mut boxes) = boxes {
            Self::acr_change_set_new_boxes(&self.sel, &mut boxes);
            event_data.changes.push(boxes);
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
