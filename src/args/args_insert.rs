use crate::statics::default_emit_event_for_api;
use crate::statics::default_inherit;
use crate::statics::default_undo_for_api;
use crate::{Val, statics::default_save_selection_for_api};

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
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

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
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
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for InsertArgs {
    fn default() -> Self {
        InsertArgs {
            rows: None,
            par: None,
            pos: None,
            inherit: default_inherit(),
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
