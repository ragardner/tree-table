use crate::Val;
#[cfg(feature = "serde")]
use crate::statics::default_emit_event_for_api;
#[cfg(feature = "serde")]
use crate::statics::default_save_selection_for_api;
#[cfg(feature = "serde")]
use crate::statics::default_undo_for_api;
#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ItemArgs {
    pub item: String,
    pub iid: Option<String>,
    pub val: Option<Val>,
    pub vals: Option<Vec<(String, Val)>>,
    pub open: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub ignore_fmt_errs: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fmt_normalize: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ItemResult {
    pub pos: u64,
    pub val: Val,
    pub values: Vec<Val>,
    pub open: bool,
}
