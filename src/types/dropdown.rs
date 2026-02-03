use crate::Val;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Dropdown {
    pub vals: Vec<Val>,
    pub default_val: Option<Val>,
    pub validation: bool,
    pub text: String,
    pub state: bool,
}
