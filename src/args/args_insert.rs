use crate::Val;
#[cfg(feature = "serde")]
use crate::statics::default_emit_event_for_api;
#[cfg(feature = "serde")]
use crate::statics::default_inherit;
#[cfg(feature = "serde")]
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InsertRow {
    pub iid: Option<String>,
    pub val: Option<Val>,
    pub vals: Option<Vec<(String, Val)>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub open: bool,
    pub dim: Option<u64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InsertArgs {
    pub rows: Option<Vec<InsertRow>>,
    pub par: Option<String>,
    pub pos: Option<u64>,
    #[cfg_attr(feature = "serde", serde(default = "default_inherit"))]
    pub inherit: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}
