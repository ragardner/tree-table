use crate::statics::default_emit_event_for_api;
use crate::statics::default_undo_for_api;
use std::collections::HashSet;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MoveItemsArgs {
    pub items: Option<HashSet<String>>,
    pub par: Option<String>,
    pub pos: Option<u64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

impl Default for MoveItemsArgs {
    fn default() -> Self {
        MoveItemsArgs {
            items: None,
            par: None,
            pos: None,
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}
