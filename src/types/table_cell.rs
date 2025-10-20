use super::cell_props::CellProps;
use super::val::Val;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub val: Val,
    pub props: Option<CellProps>,
}

impl TableCell {
    pub fn new(val: Val) -> Self {
        Self { val, props: None }
    }
}

impl Default for TableCell {
    fn default() -> Self {
        TableCell::new(Val::Nil(()))
    }
}
