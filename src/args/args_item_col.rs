use crate::Val;
#[cfg(feature = "serde")]
use crate::statics::default_emit_event_for_api;
#[cfg(feature = "serde")]
use crate::statics::default_save_selection_for_api;
#[cfg(feature = "serde")]
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ItemColArgs {
    pub item: String,
    pub iid: Option<String>,
    pub val: Option<Val>,
    pub vals: Option<Vec<(String, Val)>>,
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
pub struct ItemColResult {
    pub pos: u64,
    pub val: Val,
    pub values: Vec<Val>,
}
