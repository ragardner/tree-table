use crate::{
    SpanKey,
    statics::{default_emit_event_for_api, default_save_selection_for_api, default_undo_for_api},
};

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BorderArgs {
    pub size: Option<u8>,
    pub bg: Option<String>, // hex color string (None = use default color)
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BordersArgs {
    pub key: SpanKey,
    pub top: Option<BorderArgs>, // Some = set border, None = remove border on this side
    pub right: Option<BorderArgs>,
    pub bottom: Option<BorderArgs>,
    pub left: Option<BorderArgs>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for BordersArgs {
    fn default() -> Self {
        BordersArgs {
            key: SpanKey::None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}

// ───── SetBordersArgs – Partial / multi-side update ─────
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SetBordersArgs {
    pub key: SpanKey,
    /// Some(Some(b)) = set top border, Some(None) = remove top border, None = leave unchanged
    pub top: Option<Option<BorderArgs>>,
    pub right: Option<Option<BorderArgs>>,
    pub bottom: Option<Option<BorderArgs>>,
    pub left: Option<Option<BorderArgs>>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for SetBordersArgs {
    fn default() -> Self {
        SetBordersArgs {
            key: SpanKey::None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
