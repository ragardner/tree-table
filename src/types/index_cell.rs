use super::cell_props::CellProps;
use super::val::Val;
use crate::MaxMap;
use crate::TableCell;
use std::borrow::Borrow;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct IndexCell {
    pub iid: Rc<str>,
    pub val: Val,
    pub vals: MaxMap<TableCell>,
    pub dim: u64,
    pub vis: bool,
    pub par: Rc<str>,
    pub chn: Vec<Rc<str>>,
    pub open: bool,
    pub props: Option<CellProps>,
}

impl Borrow<str> for IndexCell {
    fn borrow(&self) -> &str {
        &self.iid
    }
}

impl PartialEq for IndexCell {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid
    }
}

impl Eq for IndexCell {}

impl Hash for IndexCell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iid.hash(state);
    }
}

impl IndexCell {
    pub fn new(val: Val, dim: u64, vis: bool, iid: Rc<str>) -> Self {
        IndexCell {
            iid,
            val,
            vals: MaxMap::new(),
            dim,
            vis,
            par: Rc::from(""),
            chn: Vec::new(),
            open: false,
            props: None,
        }
    }
}
