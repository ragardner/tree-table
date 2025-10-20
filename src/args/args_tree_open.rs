use crate::statics::default_emit_event_for_api;
use crate::statics::default_undo_for_api;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeOpenArgs {
    pub rows: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

impl Default for TreeOpenArgs {
    fn default() -> Self {
        TreeOpenArgs {
            rows: Vec::new(),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}
