use std::rc::Rc;

use super::cell_props::CellProps;
use super::val::Val;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderCell {
    pub iid: Rc<str>,
    pub val: Val,
    pub dim: u64,
    pub vis: bool,
    pub props: Option<CellProps>,
}

impl HeaderCell {
    pub fn new(val: Val, iid: Rc<str>, dim: u64, vis: bool) -> Self {
        HeaderCell {
            iid,
            val,
            dim,
            vis,
            props: None,
        }
    }
}
