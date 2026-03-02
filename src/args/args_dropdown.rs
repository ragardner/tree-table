use crate::DropdownState;
use crate::SpanKey;
use crate::Val;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_expand_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DropdownArgs {
    pub key: SpanKey,
    pub vals: Option<Vec<Val>>,
    pub default_val: Option<Val>,
    pub validation: bool,
    pub text: Option<String>,
    pub state: DropdownState,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_expand_for_api"))]
    pub expand: bool,
}

impl Default for DropdownArgs {
    fn default() -> Self {
        DropdownArgs {
            key: SpanKey::None,
            vals: None,
            default_val: None,
            validation: true,
            text: None,
            state: DropdownState::Normal,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
            expand: default_expand_for_api(),
        }
    }
}

// ───── SetDropdownArgs – Partial / multi-field update (add after DropdownArgs) ─────
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDropdownArgs {
    pub key: SpanKey,
    pub vals: Option<Vec<Val>>,
    pub default_val: Option<Option<Val>>,
    pub validation: Option<bool>,
    pub text: Option<Option<String>>,
    pub state: Option<DropdownState>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for SetDropdownArgs {
    fn default() -> Self {
        SetDropdownArgs {
            key: SpanKey::None,
            vals: Some(vec![]),
            default_val: None,
            validation: None,
            text: None,
            state: None,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
