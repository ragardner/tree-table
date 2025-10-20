use crate::Header;
use crate::Index;
use crate::Table;
use crate::str_err;

impl Table {
    // Helper to resolve column from PosKey based on pos_mode
    #[inline]
    pub(crate) fn acr_resolve_col(header: &Header, key: &str) -> Result<usize, String> {
        header
            .col(key)
            .map(|col| *col)
            .ok_or(str_err!(format!("Could not resolve col: {}", key)))
    }

    // Helper to resolve row from PosKey based on pos_mode
    #[inline]
    pub(crate) fn acr_resolve_row(index: &Index, key: &str) -> Result<usize, String> {
        index
            .row(key)
            .map(|row| *row)
            .ok_or(str_err!(format!("Could not resolve row: {}", key)))
    }
}
