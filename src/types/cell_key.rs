#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CellKey(pub u64, pub u64);
