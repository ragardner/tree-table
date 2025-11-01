use crate::Table;
use crate::str_err;
use crate::utils::utils_fns::bin_index_usize;

impl Table {
    /// Convert a data row/col number to a displayed row/col number, return Option
    #[inline]
    pub(crate) fn acr_data_to_disp_opt(disp: &Vec<usize>, data_row: usize) -> Option<usize> {
        bin_index_usize(&disp, data_row).map(|n| n)
    }

    /// Convert a displayed row number to a data row number.
    #[inline]
    pub(crate) fn acr_disp_to_data_row(
        disp_rows: &Vec<usize>,
        disp_row: usize,
    ) -> Result<usize, String> {
        disp_rows
            .get(disp_row)
            .ok_or(str_err!(format!(
                "Failed to retrieve value at index '{}' len: {}",
                disp_row,
                disp_rows.len()
            )))
            .map(|n| *n)
    }

    /// Convert a data row number to a displayed row number.
    #[inline]
    pub(crate) fn acr_data_to_disp_row(
        disp_rows: &Vec<usize>,
        data_row: usize,
    ) -> Result<usize, String> {
        bin_index_usize(&disp_rows, data_row)
            .ok_or(str_err!(format!("Value '{}' not found", data_row)))
            .map(|n| n)
    }

    /// Convert a data row number to a displayed row number, return Option
    #[inline]
    pub(crate) fn acr_data_to_disp_row_opt(
        disp_rows: &Vec<usize>,
        data_row: usize,
    ) -> Option<usize> {
        bin_index_usize(&disp_rows, data_row).map(|n| n)
    }
}
