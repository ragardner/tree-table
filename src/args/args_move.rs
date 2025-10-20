use crate::statics::default_emit_event_for_api;
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MoveArgs {
    pub old_to_new: Option<Vec<(u64, u64)>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

impl Default for MoveArgs {
    fn default() -> Self {
        MoveArgs {
            old_to_new: None,
            select: false,
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}
