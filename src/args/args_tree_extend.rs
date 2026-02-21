use crate::Val;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_inherit;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum EmptyIdHandling {
    #[default]
    New,
    Error,
    Ignore,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DuplicateIdHandling {
    #[default]
    Rename,
    Error,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CycleHandling {
    #[default]
    Ignore,
    Clear,
    Error,
    Skip,
    OnlyPrior,
}

/// For tree_extend functions.
/// Row containing:
/// - iid
/// - par
/// - vals of Vec<(String, Val)>
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeExtendRow {
    #[cfg_attr(feature = "serde", serde(default))]
    pub iid: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub par: String,
    pub val: Option<Val>,
    pub vals: Option<Vec<(String, Val)>>,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub open: bool,
    pub dim: Option<u64>,
}

/// For tree_extend functions.
/// Uses rows containing:
/// - iid
/// - par
/// - vals of Vec<(String, Val)>
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeExtendArgs {
    pub rows: Vec<TreeExtendRow>,
    #[cfg_attr(feature = "serde", serde(default = "default_inherit"))]
    pub inherit: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub empty_ids: EmptyIdHandling,
    #[cfg_attr(feature = "serde", serde(default))]
    pub duplicate_ids: DuplicateIdHandling,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cycles: CycleHandling,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for TreeExtendArgs {
    fn default() -> Self {
        TreeExtendArgs {
            rows: vec![],
            inherit: default_inherit(),
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            empty_ids: EmptyIdHandling::default(),
            duplicate_ids: DuplicateIdHandling::default(),
            cycles: CycleHandling::default(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
