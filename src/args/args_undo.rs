use crate::statics::default_emit_event_for_api;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct UndoRedoArgs {
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
}

impl Default for UndoRedoArgs {
    fn default() -> Self {
        UndoRedoArgs {
            emit: default_emit_event_for_api(),
        }
    }
}
