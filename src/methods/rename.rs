use crate::{Change, EventData, Header, Index, Table, str_err};
use std::rc::Rc;

impl Table {
    /// Renames a column iid in the header.
    /// This function is not called during undo & redo but records the changes necessary
    /// for undo & redo.
    /// Much simpler than the tree version — header has no parent/child relationships or treeview.
    pub(crate) fn acr_header_rename_iid(
        header: &mut Header,
        col: usize,
        old: &str,
        new: Rc<str>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let old_rc: Rc<str> = Rc::from(old);

        // Update the HeaderCell's iid and record the change (for undo/redo)
        let cell = Self::acr_hcell_mut(&mut header.cells, col)?;
        let change = cell.record_set_iid(new.clone(), &mut header.iid_to_col)?;

        event_data.changes.push(Change::HeaderCell { col, change });

        // If a progress bar exists on this header cell, update only its name field (granular)
        if let Some(props) = cell.props.as_ref() {
            if props.progress_bar.is_some() {
                Self::acr_event_data_set_hprogressbar_name(
                    &mut header.cells,
                    col,
                    new.to_string(),
                    event_data,
                )?;
            }
        }

        // Update the lookup map
        header.iid_to_col.remove(&old_rc);
        header.iid_to_col.insert(new, col);

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
            event_data
                .changes
                .push(Change::IndexCell { row: c_row, change });
        }
        Self::acr_icell_mut(&mut index.cells, iid_row)?.chn = chn;

        // Update parent's chn list if parent exists
        let pos = Self::acr_tree_pos(&index, &old_rc)?;
        let par_cell_opt =
            Self::acr_tree_par_cell_mut(&mut index.cells, &index.iid_to_row, &old_rc)?;
        if let Some(par_cell) = par_cell_opt {
            let par_row = Self::acr_pos(&index.iid_to_row, &par_cell.iid)?;
            if pos >= par_cell.chn.len() {
                return Err(str_err!(
                    "Index {} is out of bounds for parent's children list of length {}",
                    pos,
                    par_cell.chn.len()
                ));
            }
            if par_cell.chn[pos] != old_rc {
                return Err(str_err!(
                    "Parent's child at index {} does not match old IID: expected {}, found {}",
                    pos,
                    old_rc,
                    &*par_cell.chn[pos]
                ));
            }
            let mut new_chn = par_cell.chn.clone();
            new_chn[pos] = new.clone();
            let change = par_cell.record_set_chn(new_chn)?;
            event_data.changes.push(Change::IndexCell {
                row: par_row,
                change,
            });
        }

        // If an index progress bar exists we update its name field
        let cell = Self::acr_icell_mut(&mut index.cells, iid_row)?;
        if cell
            .props
            .as_ref()
            .and_then(|p| p.progress_bar.as_ref())
            .is_some()
        {
            Self::acr_event_data_set_iprogressbar_name(
                &mut index.cells,
                iid_row,
                new.to_string(),
                event_data,
            )?;
        }

        let cell = Self::acr_icell_mut(&mut index.cells, iid_row)?;
        let change = cell.record_set_iid(new.clone(), &mut index.iid_to_row)?;
        event_data.changes.push(Change::IndexCell {
            row: Self::acr_pos(&index.iid_to_row, &old_rc)?,
            change,
        });
        index.iid_to_row.remove(&old_rc);
        index.iid_to_row.insert(new, iid_row);
        Ok(())
    }
}
