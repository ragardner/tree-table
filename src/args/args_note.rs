use crate::SpanKey;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_expand_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use alloc::string::String;
use alloc::string::ToString;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct NoteArgs {
    pub key: SpanKey,
    pub note: Option<String>,
    pub bg: Option<String>,
    pub fg: Option<String>,
    pub readonly: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_expand_for_api"))]
    pub expand: bool,
}

impl Default for NoteArgs {
    fn default() -> Self {
        NoteArgs {
            key: SpanKey::None,
            note: None,
            bg: None,
            fg: None,
            readonly: true,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
            expand: default_expand_for_api(),
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SetNoteArgs {
    pub key: SpanKey,
    pub note: Option<String>,
    pub bg: Option<Option<String>>,
    pub fg: Option<Option<String>>,
    pub readonly: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for SetNoteArgs {
    fn default() -> Self {
        SetNoteArgs {
            key: SpanKey::None,
            note: Some("".to_string()),
            bg: None,
            fg: None,
            readonly: None,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
