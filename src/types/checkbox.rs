use crate::Val;
use alloc::string::String;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Checkbox {
    pub true_val: Val,
    pub false_val: Val,
    pub text: Option<String>,
    pub state: bool,
}
