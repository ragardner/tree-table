use crate::{Change, EventData};

use super::cell_props::CellProps;
use super::val::Val;
use std::borrow::Cow;
use std::rc::Rc;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderCell {
    pub iid: Rc<str>,
    pub val: Val,
    pub dim: u64,
    pub vis: bool,
    pub props: Option<CellProps>,
}

impl HeaderCell {
    pub fn new(val: Val, iid: Rc<str>, dim: u64, vis: bool) -> Self {
        HeaderCell {
            iid,
            val,
            dim,
            vis,
            props: None,
        }
    }

    #[inline(always)]
    pub fn event_data_apply_own_hformat(
        &mut self,
        col: usize,
        event_data: &mut EventData,
        normalize: bool,
        ignore_fmt_errs: bool,
    ) -> Result<bool, String> {
        let Some(fmt) = self
            .props
            .as_ref()
            .and_then(|props| props.data_format.as_ref())
        else {
            return Ok(false);
        };

        match fmt.try_format_val(&self.val, normalize) {
            Ok(Cow::Owned(new_val)) => {
                if new_val != self.val {
                    let change = self.record_set_val(new_val);
                    event_data.changes.push(Change::HeaderCell { col, change });
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(Cow::Borrowed(_)) => Ok(false), // same-type + !normalize = no-op
            Err(e) => {
                if ignore_fmt_errs {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }
}
