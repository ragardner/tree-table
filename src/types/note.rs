use crate::Color3;
use alloc::string::String;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub note: String,
    pub bg: Option<Color3>,
    pub fg: Option<Color3>,
    pub readonly: bool,
}
