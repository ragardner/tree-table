use crate::Color3;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ProgressBar {
    pub bg: Option<Color3>,
    pub fg: Option<Color3>,
    pub pc: f64,
    pub remove_on_complete: bool,
}
