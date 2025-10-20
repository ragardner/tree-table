#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ResetArgs {
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub grid: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub index: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub header: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub selections: bool,
    #[cfg_attr(feature = "serde", serde(default = "get_true"))]
    pub undo_stack: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub opts: bool,
}

impl Default for ResetArgs {
    fn default() -> Self {
        Self {
            grid: true,
            index: true,
            header: true,
            selections: true,
            undo_stack: true,
            opts: false,
        }
    }
}
