use crate::SpanKey;
use crate::Val;
use crate::statics::default_emit_event_for_api;
use crate::statics::default_save_selection_for_api;
use crate::statics::default_undo_for_api;
use alloc::vec;
use alloc::vec::Vec;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SortKey {
    #[default]
    Natural,
    X,
    Date,
    Version,
    Fast,
    Lexical,
    LexicalCS,
    NaturalCS,
}

impl SortKey {
    pub fn get_cmp(&self) -> fn(&Val, &Val) -> core::cmp::Ordering {
        match self {
            SortKey::Natural => Val::natural_cmp,
            SortKey::X => Val::cmp_x,
            SortKey::Date => Val::date_cmp,
            SortKey::Version => Val::version_cmp,
            SortKey::Fast => Val::fast_cmp,
            SortKey::Lexical => Val::lexical_cmp,
            SortKey::LexicalCS => Val::lexical_cs_cmp,
            SortKey::NaturalCS => Val::natural_cs_cmp,
        }
    }

    pub fn cmp(&self, a: &Val, b: &Val) -> core::cmp::Ordering {
        match self {
            SortKey::Natural => a.natural_cmp(b),
            SortKey::X => a.cmp_x(b),
            SortKey::Date => a.date_cmp(b),
            SortKey::Version => a.version_cmp(b),
            SortKey::Fast => a.fast_cmp(b),
            SortKey::Lexical => a.lexical_cmp(b),
            SortKey::LexicalCS => a.lexical_cs_cmp(b),
            SortKey::NaturalCS => a.natural_cs_cmp(b),
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SortRowsArgs {
    pub span: SpanKey,
    pub keys: Vec<(u64, bool)>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sort_key: SortKey,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_save_selection_for_api"))]
    pub save_selection: bool,
}

impl Default for SortRowsArgs {
    fn default() -> Self {
        SortRowsArgs {
            span: SpanKey::default(),
            keys: vec![],
            sort_key: SortKey::default(),
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
            save_selection: default_save_selection_for_api(),
        }
    }
}
