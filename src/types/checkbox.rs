use crate::Val;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Checkbox {
    pub true_val: Val,
    pub false_val: Val,
    pub text: String,
    pub state: bool,
}
