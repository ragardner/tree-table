use crate::ChangeHeaderCell;
use crate::ChangeIndexCell;
use crate::HeaderCell;
use crate::IndexCell;
use crate::MaxMap;
use crate::Selected;
use crate::SelectionBox;
use crate::TableCell;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeChange {
    pub par: Option<ChangeIndexCell>,
    pub chn: Option<ChangeIndexCell>,
}

pub enum TreeChangeKind {
    Par,
    Chn,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EventData {
    pub widget_name: String,
    pub widget_id: u32,
    pub table_cells: HashMap<usize, Vec<ChangeIndexCell>>,
    pub index_cells: HashMap<usize, Vec<ChangeIndexCell>>,
    pub header_cells: HashMap<usize, Vec<ChangeHeaderCell>>,
    pub added_rows: Vec<usize>,
    pub added_cols: Vec<usize>,
    pub added_progress_bars: HashMap<String, (usize, usize)>,
    pub removed_index: Vec<(usize, IndexCell)>,
    pub removed_header: Vec<(usize, HeaderCell)>,
    pub removed_table_cells: HashMap<usize, MaxMap<TableCell>>,
    pub removed_progress_bars: HashMap<String, (usize, usize)>,
    pub moved_rows: HashMap<usize, usize>,
    pub moved_cols: HashMap<usize, usize>,
    pub selection_boxes: Vec<SelectionBox>,
    pub selected: Option<Selected>,
    pub key: Option<String>, // Keyboard / mouse events
    pub row: Option<usize>,
    pub col: Option<usize>,
    // Validation
    pub table_data: HashMap<usize, Vec<ChangeIndexCell>>,
    pub index_data: HashMap<usize, Vec<ChangeIndexCell>>,
    pub header_data: HashMap<usize, Vec<ChangeHeaderCell>>,
    // FOR UNDO, cells that were modified by a change
    pub tree_changes: HashMap<Rc<str>, TreeChange>,
}

impl EventData {
    #[inline]
    pub fn add_change_tcells(&mut self, key: usize, change: ChangeIndexCell) {
        if let Some(changes) = self.table_cells.get_mut(&key) {
            changes.push(change);
        } else {
            self.table_cells.insert(key, vec![change]);
        }
    }
    #[inline]
    pub fn add_change_icells(&mut self, key: usize, change: ChangeIndexCell) {
        if let Some(changes) = self.index_cells.get_mut(&key) {
            changes.push(change);
        } else {
            self.index_cells.insert(key, vec![change]);
        }
    }
    #[inline]
    pub fn add_change_hcells(&mut self, key: usize, change: ChangeHeaderCell) {
        if let Some(changes) = self.header_cells.get_mut(&key) {
            changes.push(change);
        } else {
            self.header_cells.insert(key, vec![change]);
        }
    }
    #[inline]
    pub fn add_change_tdata(&mut self, key: usize, change: ChangeIndexCell) {
        if let Some(changes) = self.table_data.get_mut(&key) {
            changes.push(change);
        } else {
            self.table_data.insert(key, vec![change]);
        }
    }
    #[inline]
    pub fn add_change_idata(&mut self, key: usize, change: ChangeIndexCell) {
        if let Some(changes) = self.index_data.get_mut(&key) {
            changes.push(change);
        } else {
            self.index_data.insert(key, vec![change]);
        }
    }
    #[inline]
    pub fn add_change_hdata(&mut self, key: usize, change: ChangeHeaderCell) {
        if let Some(changes) = self.header_data.get_mut(&key) {
            changes.push(change);
        } else {
            self.header_data.insert(key, vec![change]);
        }
    }
    pub fn tree_change_required(&self, iid: &Rc<str>, kind: TreeChangeKind) -> bool {
        if let Some(tree_change) = self.tree_changes.get(iid) {
            match kind {
                TreeChangeKind::Par => tree_change.par.is_none(),
                TreeChangeKind::Chn => tree_change.chn.is_none(),
            }
        } else {
            true
        }
    }
    #[inline]
    pub fn add_tree_change(&mut self, iid: &Rc<str>, change: ChangeIndexCell) {
        if let Some(tree_change) = self.tree_changes.get_mut(iid) {
            match change {
                ChangeIndexCell::SetPar(_) => {
                    if tree_change.par.is_none() {
                        tree_change.par = Some(change)
                    }
                }
                ChangeIndexCell::SetChn(_) => {
                    if tree_change.chn.is_none() {
                        tree_change.chn = Some(change)
                    }
                }
                _ => (),
            }
        } else {
            match change {
                ChangeIndexCell::SetPar(_) => {
                    self.tree_changes.insert(
                        iid.clone(),
                        TreeChange {
                            par: Some(change),
                            chn: None,
                        },
                    );
                }
                ChangeIndexCell::SetChn(_) => {
                    self.tree_changes.insert(
                        iid.clone(),
                        TreeChange {
                            par: None,
                            chn: Some(change),
                        },
                    );
                }
                _ => (),
            }
        }
    }
}
