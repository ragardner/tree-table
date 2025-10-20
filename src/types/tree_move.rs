// use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeMove {
    pub iids: HashSet<String>,
    pub new_par: String,
    pub pos: Option<u64>,
}
