use crate::CellKey;
use crate::SpanKey;
use crate::Val;
#[cfg(feature = "serde")]
use crate::statics::default_emit_event_for_api;
#[cfg(feature = "serde")]
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FillDataArgs {
    pub key: SpanKey,
    pub val: Val,
    #[cfg_attr(feature = "serde", serde(default))]
    pub add_rows: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub add_cols: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDataArgs {
    pub data: Vec<(String, Vec<(String, Val)>)>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_invalid: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDataSpanArgs {
    pub key: SpanKey,
    pub index: bool,
    pub header: bool,
    pub transpose: bool,
    pub table: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDataIidsArgs {
    pub rows: Vec<(String, Vec<String>)>,
    pub ignore_invalid: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDataYxArgs {
    pub coords: Vec<CellKey>,
}
