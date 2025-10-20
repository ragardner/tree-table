use crate::Options;
use crate::Table;

impl Table {
    #[inline]
    pub(crate) fn acr_get_default_row_dim(opts: &Options) -> u64 {
        opts.default_row_dim
    }
}
