// use serde::{Deserialize, Serialize};
use alloc::rc::Rc;
use hashbrown::HashSet;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeMove {
    pub iids: HashSet<Rc<str>>,
    pub new_par: Rc<str>,
    pub pos: Option<u64>,
}
