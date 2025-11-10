use crate::CellBorders;
use crate::CellProps;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::Highlight;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;
use crate::{IndexCell, TableCell};

impl Table {
    /// Get Option<borrowed TableCell> using row, col
    #[inline]
    pub(crate) fn acr_tcell<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v TableCell> {
        let cell = cells.get(rn)?;
        cell.vals.get(&cn)
    }

    /// Get Option<borrowed TableCell.props> using row, col
    #[inline]
    pub(crate) fn acr_tcell_props<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v CellProps> {
        cells
            .get(rn)
            .and_then(|cell| cell.vals.get(&cn).and_then(|tc| tc.props.as_ref()))
    }

    /// Get Option<borrowed TableCell> using row, col
    #[inline]
    pub(crate) fn acr_tcell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v mut TableCell> {
        let cell = cells.get_mut(rn)?;
        cell.vals.get_mut(&cn)
    }

    /// Get borrowed val from TableCell.
    #[inline]
    pub(crate) fn acr_tcell_val<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v Val> {
        Self::acr_tcell(&cells, rn, cn).map(|tc| &tc.val)
    }

    /// Get cloned val from TableCell.
    #[inline]
    pub(crate) fn acr_tcell_val_cloned(
        cells: &Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<Val> {
        Self::acr_tcell(cells, rn, cn).map(|tc| tc.val.clone())
    }

    /// Get borrowed align from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_align<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v String> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.align.as_ref())
        })
    }

    /// Get borrowed borders from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_borders<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v CellBorders> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.borders.as_ref())
        })
    }

    /// Get borrowed checkbox from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_checkbox<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v Checkbox> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.checkbox.as_ref())
        })
    }

    /// Get borrowed data_format from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_data_format<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v DataFormat> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.data_format.as_ref())
        })
    }

    /// Get borrowed dropdown from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_dropdown<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v Dropdown> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.dropdown.as_ref())
        })
    }

    /// Get borrowed highlight from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_highlight<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v Highlight> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.highlight.as_ref())
        })
    }

    /// Get borrowed note from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_note<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v Note> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.note.as_ref())
        })
    }

    /// Get borrowed progress_bar from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_progress_bar<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<&'v ProgressBar> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|props| props.progress_bar.as_ref())
        })
    }

    /// Get readonly from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_readonly<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
        cn: usize,
    ) -> Option<bool> {
        cells.get(rn).and_then(|cell| {
            cell.vals
                .get(&cn)
                .and_then(|tc| tc.props.as_ref())
                .map(|p| p.readonly)
        })
    }
}
