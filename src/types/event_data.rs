use crate::ChangeIndexCell;
use crate::types::Change;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeChange {
    pub par: Option<ChangeIndexCell>,
    pub chn: Option<ChangeIndexCell>,
}

pub enum TreeChangeKind {
    Par,
    Chn,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EventData {
    pub widget_name: String,
    pub widget_id: u32,
    pub changes: Vec<Change>,
    // FOR UNDO, cells that were modified by a change
    pub tree_changes: HashMap<Rc<str>, TreeChange>,
}

impl EventData {
    pub fn new() -> Self {
        Self {
            widget_name: "".into(),
            widget_id: 0,
            changes: Vec::new(),
            tree_changes: HashMap::new(),
        }
    }

    #[inline]
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

    #[inline]
    pub fn has_non_selection_change(&self, reverse: bool) -> bool {
        let is_selection_change = |c: &Change| {
            matches!(
                c,
                Change::UpdateSelection { .. } | Change::SaveSelection { .. }
            )
        };
        if reverse {
            // Start from the end → finds non-selection changes at the tail instantly
            self.changes.iter().rev().any(|c| !is_selection_change(c))
        } else {
            self.changes.iter().any(|c| !is_selection_change(c))
        }
    }
}
