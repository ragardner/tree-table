use crate::DataFormat;
use crate::SpanKey;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_expand_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum FormatSpec {
    /// Strings for default options.
    Simple(String), // "dt", "fl64", "un32", etc.
    /// Supports decimals, display_format, max_length, etc.
    Rich(DataFormat),
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatArgs {
    pub key: SpanKey,
    pub fmt: Option<FormatSpec>,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub set_vals: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub ignore_fmt_errs: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fmt_normalize: bool,
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
            set_vals: true,
            ignore_fmt_errs: true,
            fmt_normalize: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
            expand: default_expand_for_api(),
        }
    }
}
