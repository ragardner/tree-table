use crate::SpanKey;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_expand_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatArgs {
    pub key: SpanKey,
    pub fmt: Option<String>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_expand_for_api"))]
    pub expand: bool,
}

impl Default for DataFormatArgs {
    fn default() -> Self {
        DataFormatArgs {
            key: SpanKey::None,
            fmt: None,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
            expand: default_expand_for_api(),
        }
    }
}
