use crate::CellAlign;
use crate::CellBorders;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::GetCellMutResultExt;
use crate::GetCellResultExt;
use crate::GetResultExt;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;
use std::collections::HashMap;
use std::rc::Rc;

impl Table {
    #[inline]
    pub(crate) fn acr_tree_iid_chn_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<&'v mut Vec<Rc<str>>, String> {
        let row_num = Self::acr_pos(iid_to_row, iid)?;
        let cell = cells.get_res_mut(row_num)?;
        Ok(&mut cell.chn)
    }

    #[inline]
    pub(crate) fn acr_tree_iid_chn<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<&'v Vec<Rc<str>>, String> {
        let row_num = Self::acr_pos(iid_to_row, iid)?;
        let cell = cells.get_res(row_num)?;
        Ok(&cell.chn)
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<Option<&'v mut IndexCell>, String> {
        let row = Self::acr_pos(iid_to_row, iid)?;
        let cell = cells.get_res(row)?;
        if cell.par.is_empty() {
            Ok(None)
        } else {
            let par_row = *iid_to_row.get_res(&*cell.par)?;
            let par_cell = cells.get_res_mut(par_row)?;
            Ok(Some(par_cell))
        }
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<Option<&'v IndexCell>, String> {
        let row = Self::acr_pos(iid_to_row, iid)?;
        let cell = cells.get_res(row)?;
        if cell.par.is_empty() {
            Ok(None)
        } else {
            let par_row = *iid_to_row.get_res(&*cell.par)?;
            let par_cell = cells.get_res(par_row)?;
            Ok(Some(par_cell))
        }
    }

    #[inline]
    pub(crate) fn acr_tree_row_par_cell<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        row: usize,
    ) -> Result<Option<&'v IndexCell>, String> {
        let child = cells.get_res(row)?;
        if child.par.is_empty() {
            Ok(None)
        } else {
            let par_row = *iid_to_row.get_res(&*child.par)?;
            let par_cell = cells.get_res(par_row)?;
            Ok(Some(par_cell))
        }
    }

    #[inline]
    pub(crate) fn acr_tree_row_par(
        cells: &Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        row: usize,
    ) -> Result<Rc<str>, String> {
        if row >= Self::acr_total_rows(&cells) {
            Ok(Rc::from(""))
        } else {
            let par_cell = Self::acr_tree_row_par_cell(cells, iid_to_row, row)?;
            Ok(par_cell
                .map(|c| c.iid.clone())
                .unwrap_or_else(|| Rc::from("")))
        }
    }

    #[inline]
    pub(crate) fn acr_tree_iid_par(
        cells: &Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<Rc<str>, String> {
        let par_cell = Self::acr_tree_par_cell(cells, iid_to_row, iid)?;
        Ok(par_cell
            .map(|c| c.iid.clone())
            .unwrap_or_else(|| Rc::from("")))
    }

    #[inline]
    pub(crate) fn acr_tree_pars_equal(
        cells: &Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid_to_check: &str,
        to_compare: &str,
    ) -> Result<bool, String> {
        let par_cell = Self::acr_tree_par_cell(cells, iid_to_row, iid_to_check)?;
        if let Some(par_cell) = par_cell {
            let par_iid = par_cell.iid.as_ref();
            Ok(par_iid == to_compare)
        } else {
            Ok("" == to_compare)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_iid_cell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<&'v mut IndexCell, String> {
        let row = Self::acr_pos(iid_to_row, iid)?;
        cells.get_res_mut(row)
    }

    #[inline]
    pub(crate) fn acr_tree_iid_cell<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<&'v IndexCell, String> {
        let row = Self::acr_pos(iid_to_row, iid)?;
        cells.get_res(row)
    }

    #[inline]
    pub(crate) fn acr_tree_has_par<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        Self::acr_tree_iid_cell(cells, iid_to_row, iid).map(|cell| !cell.par.is_empty())
    }

    #[inline]
    pub(crate) fn acr_row_has_par<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<bool, String> {
        Self::acr_icell(cells, rn).map(|cell| !cell.par.is_empty())
    }

    #[inline]
    pub(crate) fn acr_tree_iid_is_open<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        let cell = Self::acr_tree_iid_cell(cells, iid_to_row, iid)?;
        Ok(cell.open)
    }

    #[inline]
    pub(crate) fn acr_row_iid_is_vis<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<Rc<str>, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        let cell = Self::acr_tree_iid_cell(cells, iid_to_row, iid)?;
        Ok(cell.vis)
    }

    #[inline]
    pub(crate) fn acr_row_is_vis<'v>(
        cells: &'v Vec<IndexCell>,
        row: usize,
    ) -> Result<bool, String> {
        let cell = Self::acr_icell(cells, row)?;
        Ok(cell.vis)
    }

    #[inline]
    pub(crate) fn acr_tree_row_is_open<'v>(
        cells: &'v Vec<IndexCell>,
        row: usize,
    ) -> Result<bool, String> {
        let cell = cells.get_res(row)?;
        Ok(cell.open)
    }

    /// Get Result<copied position, String> using iid.
    #[inline]
    pub(crate) fn acr_pos(m: &HashMap<Rc<str>, usize>, iid: &str) -> Result<usize, String> {
        m.get_res(iid).copied()
    }

    /// Get Result<borrowed iid, String> using row
    #[inline]
    pub(crate) fn acr_borrow_riid<'v>(
        cells: &'v Vec<IndexCell>,
        pos: usize,
    ) -> Result<&'v Rc<str>, String> {
        let cell = cells.get_res(pos)?;
        Ok(&cell.iid)
    }

    /// Get Result<iid, String> using row
    #[inline]
    pub(crate) fn acr_copy_riid<'v>(
        cells: &'v Vec<IndexCell>,
        pos: usize,
    ) -> Result<Rc<str>, String> {
        let cell = cells.get_res(pos)?;
        Ok(cell.iid.clone())
    }

    /// Get Result<borrowed IndexCell, String> using row
    #[inline]
    pub(crate) fn acr_icell<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<&'v IndexCell, String> {
        cells.get_res(rn)
    }

    #[inline]
    pub(crate) fn acr_icell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        rn: usize,
    ) -> Result<&'v mut IndexCell, String> {
        cells.get_res_mut(rn)
    }

    /// Get borrowed val from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_val<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<&'v Val, String> {
        let ic = Self::acr_icell(cells, rn)?;
        Ok(&ic.val)
    }

    /// Get borrowed val from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_val_cloned<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Val, String> {
        let ic = Self::acr_icell(cells, rn)?;
        Ok(ic.val.clone())
    }

    /// Get borrowed open from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_open<'v>(cells: &'v Vec<IndexCell>, rn: usize) -> Result<bool, String> {
        let ic = Self::acr_icell(cells, rn)?;
        Ok(ic.open)
    }

    /// Get borrowed align from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_align<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v CellAlign>, String> {
        Self::acr_icell(cells, rn)
            .map(|cell| cell.props.as_ref().and_then(|props| props.align.as_ref()))
    }

    /// Get borrowed borders from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_borders<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v CellBorders>, String> {
        Self::acr_icell(cells, rn)
            .map(|cell| cell.props.as_ref().and_then(|props| props.borders.as_ref()))
    }

    /// Get borrowed checkbox from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_checkbox<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v Checkbox>, String> {
        Self::acr_icell(cells, rn).map(|cell| {
            cell.props
                .as_ref()
                .and_then(|props| props.checkbox.as_ref())
        })
    }

    /// Get borrowed data_format from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_data_format<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v DataFormat>, String> {
        Self::acr_icell(cells, rn).map(|cell| {
            cell.props
                .as_ref()
                .and_then(|props| props.data_format.as_ref())
        })
    }

    /// Get borrowed dropdown from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_dropdown<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v Dropdown>, String> {
        Self::acr_icell(cells, rn).map(|cell| {
            cell.props
                .as_ref()
                .and_then(|props| props.dropdown.as_ref())
        })
    }

    /// Get borrowed highlight from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_highlight<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v Highlight>, String> {
        Self::acr_icell(cells, rn).map(|cell| {
            cell.props
                .as_ref()
                .and_then(|props| props.highlight.as_ref())
        })
    }

    /// Get borrowed note from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_note<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v Note>, String> {
        Self::acr_icell(cells, rn)
            .map(|cell| cell.props.as_ref().and_then(|props| props.note.as_ref()))
    }

    /// Get borrowed progress_bar from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_progress_bar<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<Option<&'v ProgressBar>, String> {
        Self::acr_icell(cells, rn).map(|cell| {
            cell.props
                .as_ref()
                .and_then(|props| props.progress_bar.as_ref())
        })
    }

    /// Get borrowed readonly from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_readonly<'v>(
        cells: &'v Vec<IndexCell>,
        rn: usize,
    ) -> Result<bool, String> {
        Ok(Self::acr_icell(cells, rn)?
            .props
            .as_ref()
            .map_or(false, |props| props.readonly))
    }
}
