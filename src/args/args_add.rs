use crate::statics::default_emit_event_for_api;
use crate::statics::default_inherit;
use crate::statics::default_undo_for_api;
#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct AddArgs {
    pub items: Option<Vec<String>>,
    pub pos: Option<u64>,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub vis: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_inherit"))]
    pub inherit: String,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

impl Default for AddArgs {
    fn default() -> Self {
        AddArgs {
            items: None,
            pos: None,
            vis: true,
            select: false,
            inherit: default_inherit(),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct AddNumArgs {
    pub num: Option<u64>,
    pub pos: Option<u64>,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub vis: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub select: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_inherit"))]
    pub inherit: String,
    #[cfg_attr(feature = "serde", serde(default = "default_emit_event_for_api"))]
    pub emit: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_undo_for_api"))]
    pub undo: bool,
}

impl Default for AddNumArgs {
    fn default() -> Self {
        AddNumArgs {
            num: None,
            pos: None,
            vis: true,
            select: false,
            inherit: default_inherit(),
            emit: default_emit_event_for_api(),
            undo: default_undo_for_api(),
        }
    }
}
