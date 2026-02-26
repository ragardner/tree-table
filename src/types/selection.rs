use crate::statics::CELLS;
use crate::statics::COLS;
use crate::statics::ROWS;
use crate::str_err;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Coords {
    pub row: usize,
    pub end_row: Option<usize>,
    pub col: usize,
    pub end_col: Option<usize>,
}

impl Coords {
    pub fn new(row: usize, end_row: Option<usize>, col: usize, end_col: Option<usize>) -> Self {
        Coords {
            row,
            end_row,
            col,
            end_col,
        }
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
// #[serde(rename_all = "lowercase")]
pub enum SelectionKind {
    Rows,
    Cols,
    Cells,
}

impl SelectionKind {
    // Convert enum to &str
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectionKind::Rows => ROWS,
            SelectionKind::Cols => COLS,
            SelectionKind::Cells => CELLS,
        }
    }

    // Parse from string to enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rows" => Some(SelectionKind::Rows),
            "cols" => Some(SelectionKind::Cols),
            "cells" => Some(SelectionKind::Cells),
            _ => None,
        }
    }

    // Convert from string to enum with error handling
    pub fn from_str_res(s: &str) -> Result<Self, String> {
        Self::from_str(s).ok_or(str_err!("Cannot convert: {} to SelectionKind", s))
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
// #[serde(rename_all = "lowercase")]
pub enum SelectionState {
    Normal,
    Hidden,
}

impl SelectionState {
    // Convert enum to &str
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectionState::Normal => "normal",
            SelectionState::Hidden => "hidden",
        }
    }

    // Parse from string to enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(SelectionState::Normal),
            "hidden" => Some(SelectionState::Hidden),
            _ => None,
        }
    }

    // Convert from string to enum with error handling
    pub fn from_str_res(s: &str) -> Result<Self, String> {
        Self::from_str(s).ok_or(str_err!("Cannot convert: {} to SelectionState", s))
    }
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Selected {
    pub row: usize,
    pub col: usize,
    pub coords: Coords,
    pub kind: SelectionKind,
    pub pos: usize,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionBox {
    pub coords: Coords,
    pub kind: SelectionKind,
    pub state: SelectionState,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    pub selection_boxes: Vec<SelectionBox>,
    pub selected: Option<Selected>,
}

impl Selection {
    pub fn new() -> Self {
        Selection {
            selection_boxes: Vec::new(),
            selected: None,
        }
    }
}
