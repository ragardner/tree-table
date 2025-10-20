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
use crate::args::TreeExtendRow;
use crate::str_err;
use crate::utils::consecutive_ranges::consecutive_ranges_owned;
use std::collections::HashMap;

#[inline]
fn tree_extend_get_descendants<'a>(
    tree: &'a HashMap<String, TreeExtendRow>,
    initial_chn: &'a [String],
) -> impl Iterator<Item = Result<&'a String, String>> + 'a {
    // Define the iterator struct
    struct TreeDescendantIterator<'a> {
        tree: &'a HashMap<String, TreeExtendRow>,
        stack: Vec<std::slice::Iter<'a, String>>, // Stack of child iterators
    }

    impl<'a> Iterator for TreeDescendantIterator<'a> {
        type Item = Result<&'a String, String>;

        fn next(&mut self) -> Option<Self::Item> {
            // Process the stack with a while loop
            while let Some(top_iterator) = self.stack.last_mut() {
                match top_iterator.next() {
                    Some(child_iid) => {
                        // Look up the child's RowSpec in the tree
                        match self.tree.get(child_iid) {
                            Some(row_spec) => {
                                // If the child has children, push their iterator onto the stack
                                let chn_slice =
                                    row_spec.chn.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
                                if !chn_slice.is_empty() {
                                    self.stack.push(chn_slice.iter());
                                }
                                // Yield the current child iid
                                return Some(Ok(child_iid));
                            }
                            None => {
                                // Error if the child iid is not in the tree
                                return Some(Err(str_err!(format!(
                                    "iid not found in tree: {}",
                                    child_iid
                                ))));
                            }
                        }
                    }
                    None => {
                        // Pop the exhausted iterator
                        self.stack.pop();
                    }
                }
            }
            // No more descendants to yield
            None
        }
    }

    TreeDescendantIterator {
        tree,
        stack: if initial_chn.is_empty() {
            vec![]
        } else {
            vec![initial_chn.iter()]
        },
    }
}

#[inline]
fn tree_extend_check_inf_loop(
    tree_map: &HashMap<String, TreeExtendRow>,
    initial_chn: &[String],
    par: &str,
) -> Result<bool, String> {
    for did_res in tree_extend_get_descendants(tree_map, initial_chn) {
        if did_res? == par {
            return Ok(true);
        }
    }
    Ok(false)
}

impl Table {
    /// Builds table cells for a given row specification, handling both provided values and inherited properties.
    #[inline]
    fn build_row_cells(
        icells: &Vec<IndexCell>,
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
            let cn = Self::acr_resolve_col(&header, &key)?;
            let props = if do_inherit {
                Self::acr_tcell_props(&icells, &inherit_row, &cn).clone()
            } else {
                None
            };
            cols_map.set_cell(cn, TableCell { val, props });
        }

        // Handle inheritance for missing cells
        if do_inherit {
            for cn in 0..ncols {
                if !cols_map.contains_key(&cn) {
                    if let Some(props) = Self::acr_tcell_props(&icells, &inherit_row, &cn) {
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
        let mut tree_map: HashMap<String, TreeExtendRow> =
            HashMap::with_capacity(kwargs.rows.len());
        let mut tops: Vec<String> = Vec::new();
        // parents added to the tree_map but which never have their own rows as iids
        let mut par_stubs: HashMap<String, Vec<String>> = HashMap::new();
        let grid = &mut *self.grid.borrow_mut();

        // Iterate through the array elements
        for mut row_spec in kwargs.rows.into_iter() {
            // Ignore any provided chn by overwriting to None
            row_spec.chn = None;

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
                // We have to clone to add to a parents children Vec
                row_spec.iid.clone()
            };
            // par should not already be in Table

            if grid.index.iid_to_row.contains_key(&row_spec.par) {
                let par = row_spec.par.clone();
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
            // If it is an existing stub, remove it from stubs and
            // xfer the stubs chn and iid to the row_spec
            // Transfer pending children (assign since input chn is empty)
            if let Some(pending_chn) = par_stubs.remove(&iid) {
                row_spec.chn = Some(pending_chn);
            }
            // Check for cycles (inf loops of children) and id-parent equality
            let chn_slice = row_spec.chn.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
            if iid == row_spec.par
                || (check_for_cycles
                    && tree_extend_check_inf_loop(&tree_map, chn_slice, &row_spec.par)?)
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

            // Finally if par is not empty
            if !row_spec.par.is_empty() {
                if let Some(par_row_spec) = tree_map.get_mut(&row_spec.par) {
                    // Parent is already an iid in tree
                    if let Some(chn) = &mut par_row_spec.chn {
                        chn.push(iid.clone());
                    } else {
                        par_row_spec.chn = Some(vec![iid.clone()]);
                    }
                } else {
                    // Parent is not already an iid in tree
                    // Placeholder/stub potentially to be updated later
                    par_stubs
                        .entry(row_spec.par.clone())
                        .or_insert_with(Vec::new)
                        .push(iid.clone());
                }
            } else {
                tops.push(iid.clone());
            }
            tree_map.insert(iid, row_spec);
        }

        // Drain remaining stubs into tree_map as top-level with defaults
        for (iid, chn) in par_stubs.into_iter() {
            tops.push(iid.clone());
            tree_map.insert(
                iid.clone(),
                TreeExtendRow {
                    iid,
                    par: String::new(),
                    val: None,
                    vals: None,
                    open: false,
                    dim: None,
                    chn: Some(chn),
                },
            );
        }

        // Return early without modifying existing table if no rows are to be added
        if tree_map.is_empty() {
            return Err(str_err!("No rows to be added"));
        }

        // Setup variables
        let mut event_data: EventData = self.cr_new_event(true);
        let opts = self.opts.borrow();
        let empty_val: Val = opts.empty_val.clone();
        let default_dim = Self::acr_get_default_row_dim(&opts);
        let mut sorted_seq: Vec<usize> = Vec::with_capacity(tree_map.len());
        let ncols = Self::acr_total_cols(&grid.header.cells);
        let mut rn = Self::acr_total_rows(&grid.index.cells);

        // Determine which row to inherit cell props from
        let opt_inherit_row = self.cr_get_inherit_row(rn, &kwargs.inherit)?;
        let do_inherit = opt_inherit_row.is_some();
        let inherit_row = opt_inherit_row.unwrap_or(0);
        let index_props = if do_inherit {
            grid.index.clone_props(&inherit_row)
        } else {
            None
        };

        // self.cr_remake_rows()
        let mut acc: u64 = grid.canvas_rows.last().cloned().unwrap_or(0);
        let mut stack: Vec<(String, bool)> = Vec::new();
        for top in tops.into_iter() {
            stack.push((top, true));
            while let Some((iid, vis)) = stack.pop() {
                sorted_seq.push(rn);

                let row_spec = tree_map
                    .get_mut(&iid)
                    .ok_or(format!("Row not found: {}", iid))?;

                let cols_map = Self::build_row_cells(
                    &grid.index.cells,
                    &grid.header,
                    row_spec.vals.take().unwrap_or_default(),
                    inherit_row,
                    do_inherit,
                    &empty_val,
                    ncols,
                )?;

                let open = row_spec.open;
                let dim = row_spec.dim.unwrap_or(default_dim);
                let child_vis = vis && open;
                let chn_slice = row_spec.chn.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
                stack.extend(
                    chn_slice
                        .iter()
                        .rev()
                        .map(|child| (child.clone(), child_vis)),
                );

                // Row grid lines, self.cr_remake_rows()
                if vis {
                    grid.disp_rows.push(rn as usize);
                    acc += dim;
                    grid.canvas_rows.push(acc);
                }

                // Push cell to index, self.grid.index.add_rows()
                grid.index.add_row(
                    rn,
                    IndexCell {
                        val: row_spec.val.take().map_or(Val::Str(iid.clone()), Val::from),
                        vals: cols_map,
                        dim,
                        vis,
                        iid,
                        par: row_spec.par.clone(),
                        chn: row_spec.chn.take().unwrap_or_default(),
                        open,
                        props: index_props.clone(),
                    },
                );

                rn += 1;
            }
        }
        let sel_ref = self.sel.borrow();
        event_data.added_rows = sorted_seq;
        event_data.selection_boxes = sel_ref.selection_boxes.clone();
        event_data.selected = sel_ref.selected.clone();

        // Select the added rows
        if kwargs.select {
            let mut sel = self.sel.borrow_mut();
            Self::acr_deselect_all(&mut sel);
            for (start, end) in
                consecutive_ranges_owned(event_data.added_rows.iter().filter_map(|n| {
                    if Self::acr_row_is_vis_opt(&grid.index.cells, n).is_some() {
                        Self::acr_data_to_disp_opt(&grid.disp_rows, *n)
                    } else {
                        None
                    }
                }))
            {
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

        // Sheet has been modified
        self.cr_event_modified(&event_data, kwargs.emit, true)?;

        // Add to undo stack if undo is enabled and undo is true in kwargs
        let opts = self.opts.borrow();
        if opts.enabled_undo && kwargs.undo {
            grid.undo_stack.push_back(event_data);
        }
        Ok(())
    }
}
