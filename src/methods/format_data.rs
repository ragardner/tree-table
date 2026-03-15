use crate::DataFormat;
use crate::Table;
use crate::Val;

impl Table {
    #[inline(always)]
    pub(crate) fn acr_format_data(data_format: &DataFormat, val: Val) -> Val {
        val
    }
}
