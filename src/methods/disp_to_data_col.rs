use crate::Table;
use crate::str_err;
use crate::utils::utils_fns::bin_index_usize;

impl Table {
    /// Convert a displayed col number to a data col number.
    #[inline]
    pub(crate) fn acr_disp_to_data_col(
        disp_cols: &Vec<usize>,
        disp_col: usize,
    ) -> Result<usize, String> {
        disp_cols
            .get(disp_col)
            .ok_or(str_err!(format!(
                "Failed to retrieve value at index '{}' len: {}",
                disp_col,
                disp_cols.len()
            )))
            .map(|n| *n)
    }

    /// Convert a data col number to a displayed col number.
    #[inline]
    pub(crate) fn acr_data_to_disp_col(
        disp_cols: &Vec<usize>,
        data_col: usize,
    ) -> Result<usize, String> {
        bin_index_usize(&disp_cols, data_col)
            .ok_or(str_err!(format!("Value '{}' not found", data_col)))
            .map(|n| n)
    }

    /// Convert a data col number to a displayed col number, return Option
    #[inline]
    pub(crate) fn acr_data_to_disp_col_opt(
        disp_cols: &Vec<usize>,
        data_col: usize,
    ) -> Option<usize> {
        bin_index_usize(&disp_cols, data_col).map(|n| n)
    }
}
