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
