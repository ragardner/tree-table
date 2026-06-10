use crate::statics::{
    default_emit_event_for_api, default_save_selection_for_api, default_undo_for_api,
};
use alloc::string::String;
use alloc::vec::Vec;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeOpenArgs {
    pub rows: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for TreeOpenArgs {
    fn default() -> Self {
        TreeOpenArgs {
            rows: Vec::new(),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
