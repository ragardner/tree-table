use crate::CellKey;
use crate::SpanKey;
use crate::Val;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FillDataArgs {
    pub key: SpanKey,
    #[cfg_attr(feature = "serde", serde(default))]
    pub val: Option<Val>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub add_rows: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub add_cols: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub ignore_fmt_errs: bool,
}

impl Default for FillDataArgs {
    fn default() -> Self {
        FillDataArgs {
            key: SpanKey::None,
            val: None,
            add_rows: false,
            add_cols: false,
            undo: default_undo_for_api(),
            emit: default_emit_event_for_api(),
            save_selection: default_save_selection_for_api(),
            ignore_fmt_errs: true,
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDataArgs {
    pub data: Vec<(String, Vec<(String, Val)>)>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_invalid: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub ignore_fmt_errs: bool,
}

impl Default for SetDataArgs {
    fn default() -> Self {
        SetDataArgs {
            data: vec![],
            ignore_invalid: true,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
            ignore_fmt_errs: true,
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDataSpanArgs {
    pub key: SpanKey,
    pub index: bool,
    pub header: bool,
    pub transpose: bool,
    pub table: bool,
}

impl Default for GetDataSpanArgs {
    fn default() -> Self {
        GetDataSpanArgs {
            key: SpanKey::None,
            index: false,
            header: false,
            transpose: false,
            table: true,
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDataIidsArgs {
    pub rows: Vec<(String, Vec<String>)>,
    pub ignore_invalid: bool,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDataYxArgs {
    pub coords: Vec<CellKey>,
}
