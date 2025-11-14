use crate::Table;

impl Table {
    #[inline]
    pub(crate) fn cr_get_default_row_dim(&self) -> u64 {
        self.opts.default_row_dim
    }
}
