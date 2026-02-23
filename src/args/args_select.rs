use crate::SpanKey;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct AddSelectionArgs {
    pub key: SpanKey,
    pub current: bool,
    pub emit: bool,
    pub undo: bool,
}

impl Default for AddSelectionArgs {
    fn default() -> Self {
        AddSelectionArgs {
            key: SpanKey::None,
            current: true,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SelectRowsColsArgs {
    pub items: Vec<String>,
    pub replace: bool,
    pub current: bool,
    pub emit: bool,
    pub undo: bool,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionClearArgs {
    pub emit: bool,
    pub undo: bool,
}

impl Default for SelectionClearArgs {
    fn default() -> Self {
        SelectionClearArgs {
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}
