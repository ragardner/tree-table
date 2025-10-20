use crate::CellBorders;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::Highlight;
use crate::Note;
use crate::ProgressBar;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CellProps {
    pub align: Option<String>,
    pub borders: Option<CellBorders>,
    pub checkbox: Option<Checkbox>,
    pub data_format: Option<DataFormat>,
    pub dropdown: Option<Dropdown>,
    pub highlight: Option<Highlight>,
    pub note: Option<Note>,
    pub progress_bar: Option<ProgressBar>,
    pub readonly: bool,
}

impl Default for CellProps {
    fn default() -> Self {
        CellProps::new()
    }
}

impl CellProps {
    pub fn new() -> Self {
        CellProps {
            align: None,
            borders: None,
            checkbox: None,
            data_format: None,
            dropdown: None,
            highlight: None,
            note: None,
            progress_bar: None,
            readonly: false,
        }
    }
}
