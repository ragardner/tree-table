use crate::SpanKey;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;

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
        }
    }
}
