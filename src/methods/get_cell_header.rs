use crate::CellAlign;
use crate::CellBorders;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::HeaderCell;
use crate::Highlight;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;
use crate::str_err;
use std::collections::HashMap;
use std::rc::Rc;

impl Table {
    #[inline]
    pub(crate) fn acr_col_iid_is_vis<'v>(
        cells: &Vec<HeaderCell>,
        iid_to_col: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        let col = Self::acr_pos(&iid_to_col, &iid)?;
        let cell = Self::acr_hcell(&cells, col)?;
        Ok(cell.vis)
    }

    /// Get Result<borrowed iid, String> using col
    #[inline]
    pub(crate) fn acr_borrow_ciid<'v>(
        v: &'v Vec<HeaderCell>,
        pos: usize,
    ) -> Result<&'v Rc<str>, String> {
        v.get(pos)
            .map(|cell| &cell.iid)
            .ok_or(str_err!("pos out of bounds: {}", pos))
    }

    /// Get Result<iid, String> using col
    #[inline]
    pub(crate) fn acr_copy_ciid<'v>(v: &'v Vec<HeaderCell>, pos: usize) -> Result<Rc<str>, String> {
        v.get(pos)
            .map(|cell| cell.iid.clone())
            .ok_or(str_err!("pos out of bounds: {}", pos))
    }

    /// Get Result<borrowed HeaderCell, String> using col
    #[inline]
    pub(crate) fn acr_hcell<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<&'v HeaderCell, String> {
        cells.get(cn).ok_or(str_err!("Col not found: {}", cn))
    }

    #[inline]
    pub(crate) fn acr_hcell_mut<'v>(
        cells: &'v mut Vec<HeaderCell>,
        cn: usize,
    ) -> Result<&'v mut HeaderCell, String> {
        cells.get_mut(cn).ok_or(str_err!("Col not found: {}", cn))
    }

    /// Get borrowed val from HeaderCell.
    #[inline]
    pub(crate) fn acr_hcell_val<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<&'v Val, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(&cell.val)
    }

    /// Get borrowed align from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_align<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v CellAlign>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell.props.as_ref().and_then(|props| props.align.as_ref()))
    }

    /// Get borrowed borders from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_borders<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v CellBorders>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell.props.as_ref().and_then(|props| props.borders.as_ref()))
    }

    /// Get borrowed checkbox from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_checkbox<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v Checkbox>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell
            .props
            .as_ref()
            .and_then(|props| props.checkbox.as_ref()))
    }

    /// Get borrowed data_format from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_data_format<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v DataFormat>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell
            .props
            .as_ref()
            .and_then(|props| props.data_format.as_ref()))
    }

    /// Get borrowed dropdown from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_dropdown<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v Dropdown>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell
            .props
            .as_ref()
            .and_then(|props| props.dropdown.as_ref()))
    }

    /// Get borrowed highlight from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_highlight<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v Highlight>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell
            .props
            .as_ref()
            .and_then(|props| props.highlight.as_ref()))
    }

    /// Get borrowed note from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_note<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v Note>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell.props.as_ref().and_then(|props| props.note.as_ref()))
    }

    /// Get borrowed progress_bar from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_progress_bar<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<Option<&'v ProgressBar>, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell
            .props
            .as_ref()
            .and_then(|props| props.progress_bar.as_ref()))
    }

    /// Get borrowed readonly from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_readonly<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: usize,
    ) -> Result<bool, String> {
        let cell = cells.get(cn).ok_or(str_err!("Col not found: {}", cn))?;
        Ok(cell.props.as_ref().map_or(false, |props| props.readonly))
    }
}
