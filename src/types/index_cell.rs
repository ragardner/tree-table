use super::cell_props::CellProps;
use super::val::Val;
use crate::Change;
use crate::EventData;
use crate::MaxMap;
use crate::TableCell;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct IndexCell {
    pub iid: Rc<str>,
    pub val: Val,
    pub vals: MaxMap<TableCell>,
    pub dim: u64,
    pub vis: bool,
    pub par: Rc<str>,
    pub chn: Vec<Rc<str>>,
    pub open: bool,
    pub props: Option<CellProps>,
}

impl Borrow<str> for IndexCell {
    fn borrow(&self) -> &str {
        &self.iid
    }
}

impl PartialEq for IndexCell {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid
    }
}

impl Eq for IndexCell {}

impl Hash for IndexCell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iid.hash(state);
    }
}

impl IndexCell {
    pub fn new(val: Val, dim: u64, vis: bool, iid: Rc<str>) -> Self {
        IndexCell {
            iid,
            val,
            vals: MaxMap::new(),
            dim,
            vis,
            par: Rc::from(""),
            chn: Vec::new(),
            open: false,
            props: None,
        }
    }

    #[inline(always)]
    pub fn event_data_apply_own_iformat(
        &mut self,
        row: usize,
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
                    event_data.changes.push(Change::IndexCell { row, change });
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

    #[inline(always)]
    pub fn event_data_apply_own_tformat(
        &mut self,
        row: usize,
        col: usize,
        empty_val: &Val,
        event_data: &mut EventData,
        normalize: bool,
        ignore_fmt_errs: bool,
    ) -> Result<bool, String> {
        let Some(fmt) = self
            .vals
            .get(&col)
            .and_then(|tc| tc.props.as_ref())
            .and_then(|props| props.data_format.as_ref())
        else {
            return Ok(false);
        };

        match fmt.try_format_val(&self.val, normalize) {
            Ok(Cow::Owned(new_val)) => {
                if new_val != self.val {
                    let change = self.record_set_table_cell_val(col, new_val, empty_val.clone());
                    event_data.changes.push(Change::IndexCell { row, change });
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
