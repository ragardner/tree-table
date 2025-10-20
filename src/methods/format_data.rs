use crate::DataFormat;
use crate::Table;
use crate::Val;

impl Table {
    #[inline(always)]
    pub(crate) fn acr_format_data(data_format: &Option<DataFormat>, val: Val) -> Val {
        if let Some(data_format) = data_format {
            val
        } else {
            val
        }
    }
}
