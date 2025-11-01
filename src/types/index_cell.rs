use super::cell_props::CellProps;
use super::val::Val;
use crate::MaxMap;
use crate::TableCell;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::{Rc, Weak};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct IndexCell {
    pub iid: String,
    pub val: Val,
    pub vals: MaxMap<TableCell>,
    pub dim: u64,
    pub vis: bool,
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serialize_par", deserialize_with = "deserialize_par")
    )]
    pub par: Option<Weak<RefCell<IndexCell>>>,
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serialize_chn", deserialize_with = "deserialize_chn")
    )]
    pub chn: Vec<Rc<RefCell<IndexCell>>>,
    pub open: bool,
    pub props: Option<CellProps>,
}

impl IndexCell {
    /// Returns Ok(Some(parent Rc)) if the parent exists and upgrades successfully,
    /// Ok(None) if no parent (par is None),
    /// or Err if upgrade fails (e.g., parent dropped unexpectedly).
    pub fn par_rc(&self) -> Result<Option<Rc<RefCell<IndexCell>>>, String> {
        match &self.par {
            Some(w) => w
                .upgrade()
                .ok_or_else(|| {
                    format!(
                        "Failed to upgrade parent weak reference for iid: {}",
                        &self.iid
                    )
                })
                .map(Some),
            None => Ok(None),
        }
    }
}

#[cfg(feature = "serde")]
fn serialize_par<S>(
    par: &Option<Weak<RefCell<IndexCell>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let iid = par
        .as_ref()
        .and_then(|w| w.upgrade())
        .map(|rc| rc.borrow().iid.clone())
        .unwrap_or_else(|| "".to_string());
    iid.serialize(serializer)
}

#[cfg(feature = "serde")]
fn serialize_chn<S>(chn: &Vec<Rc<RefCell<IndexCell>>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let iids: Vec<String> = chn.iter().map(|rc| rc.borrow().iid.clone()).collect();
    iids.serialize(serializer)
}

#[cfg(feature = "serde")]
fn deserialize_par<'de, D>(deserializer: D) -> Result<Option<Weak<RefCell<IndexCell>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

#[cfg(feature = "serde")]
fn deserialize_chn<'de, D>(deserializer: D) -> Result<Vec<Rc<RefCell<IndexCell>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Vec::new())
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

impl Borrow<str> for IndexCell {
    fn borrow(&self) -> &str {
        self.iid.as_str()
    }
}

impl Borrow<String> for IndexCell {
    fn borrow(&self) -> &String {
        &self.iid
    }
}

impl IndexCell {
    pub fn new(val: Val, dim: u64, vis: bool, iid: String) -> Self {
        IndexCell {
            iid,
            val,
            vals: MaxMap::new(),
            dim,
            vis,
            par: None,
            chn: Vec::new(),
            open: false,
            props: None,
        }
    }
}
