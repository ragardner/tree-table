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
use crate::str_err;
use crate::{HeaderCell, IndexCell, TableCell};
use std::collections::HashMap;

impl Table {
    pub(crate) fn acr_tree_take_iid_chn(
        cells: &mut Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Vec<String>, String> {
        let row_num = Self::acr_copy_pos(&iid_to_row, iid)?;
        let cell = cells
            .get_mut(row_num as usize)
            .ok_or(str_err!(format!("Row not found: {}", row_num)))?;
        Ok(std::mem::take(&mut cell.chn))
    }

    pub(crate) fn acr_tree_iid_chn_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<&'v mut Vec<String>, String> {
        let row_num = Self::acr_copy_pos(&iid_to_row, iid)?;
        match cells.get_mut(row_num as usize) {
            Some(cell) => Ok(&mut cell.chn),
            None => Err(str_err!(format!("Row not found: {}", row_num))),
        }
    }

    pub(crate) fn acr_tree_iid_chn<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<&'v Vec<String>, String> {
        let row_num = Self::acr_copy_pos(&iid_to_row, iid)?;
        match cells.get(row_num as usize) {
            Some(cell) => Ok(&cell.chn),
            None => Err(str_err!(format!("Row not found: {}", row_num))),
        }
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Option<&'v mut IndexCell> {
        let par_row = {
            let child_row = iid_to_row.get(iid)?;
            let child_cell = cells.get(*child_row as usize)?;
            if child_cell.par.is_empty() {
                return None;
            }
            *iid_to_row.get(&child_cell.par)?
        };
        cells.get_mut(par_row as usize)
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Option<&'v IndexCell> {
        let child_row = iid_to_row.get(iid)?;
        let child_cell = cells.get(*child_row as usize)?;
        if child_cell.par.is_empty() {
            None
        } else {
            let par_row = iid_to_row.get(&child_cell.par)?;
            cells.get(*par_row as usize)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_iid_cell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<&'v mut IndexCell, String> {
        Ok(iid_to_row
            .get(iid)
            .and_then(|rn| cells.get_mut(*rn as usize))
            .ok_or(str_err!(format!("iid not found: {}", iid)))?)
    }

    #[inline]
    pub(crate) fn cr_tree_iid_cell<'v>(
        cells: &'v Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<&'v IndexCell, String> {
        Ok(iid_to_row
            .get(iid)
            .and_then(|rn| cells.get(*rn as usize))
            .ok_or(str_err!(format!("iid not found: {}", iid)))?)
    }

    #[inline]
    pub(crate) fn acr_tree_has_par(
        cells: &Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Option<bool> {
        Self::acr_tree_par_cell(&cells, &iid_to_row, iid).map(|cell| !cell.par.is_empty())
    }

    #[inline]
    pub(crate) fn acr_tree_iid_is_open(
        cells: &Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        let rn = Self::acr_borrow_pos(&iid_to_row, iid)?;
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("iid not found: {}", rn)))?;
        Ok(cell.open)
    }

    #[inline]
    pub(crate) fn acr_tree_iid_displayed(
        cells: &Vec<IndexCell>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        if let Some(row) = iid_to_row.get(iid) {
            if let Some(cell) = cells.get(*row as usize) {
                Ok(cell.vis)
            } else {
                Err(str_err!(format!("No cell found for row: {}", row)))
            }
        } else {
            Err(str_err!(format!("No row found for iid: {}", iid)))
        }
    }

    #[inline]
    pub(crate) fn acr_tree_row_is_open(
        cells: &Vec<IndexCell>,
        row: &usize,
    ) -> Result<bool, String> {
        let cell = cells
            .get(*row as usize)
            .ok_or(str_err!(format!("Row not found: {}", row)))?;
        Ok(cell.open)
    }

    /// Get Result<copied row number, String> using iid
    #[inline]
    pub(crate) fn acr_copy_pos(m: &HashMap<String, usize>, iid: &str) -> Result<usize, String> {
        m.get(iid)
            .copied()
            .ok_or(str_err!(format!("iid not found: {}", iid)))
    }

    /// Get Result<borrowed row number, String> using iid
    #[inline]
    pub(crate) fn acr_borrow_pos<'hm>(
        m: &'hm HashMap<String, usize>,
        iid: &str,
    ) -> Result<&'hm usize, String> {
        m.get(iid)
            .ok_or(str_err!(format!("iid not found: {}", iid)))
    }

    /// Get Result<borrowed iid, String> using row
    #[inline]
    pub(crate) fn acr_borrow_riid<'v>(
        v: &'v Vec<IndexCell>,
        pos: &usize,
    ) -> Result<&'v String, String> {
        v.get(*pos as usize)
            .map(|cell| &cell.iid)
            .ok_or(str_err!(format!("pos out of bounds: {}", pos)))
    }

    /// Get Result<borrowed iid, String> using col
    #[inline]
    pub(crate) fn acr_borrow_ciid<'v>(
        v: &'v Vec<HeaderCell>,
        pos: &usize,
    ) -> Result<&'v String, String> {
        v.get(*pos as usize)
            .map(|cell| &cell.iid)
            .ok_or(str_err!(format!("pos out of bounds: {}", pos)))
    }

    /// Get Option<borrowed TableCell, String> using row, col
    #[inline]
    pub(crate) fn acr_tcell<'v>(
        v: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> Option<&'v TableCell> {
        if let Some(icell) = v.get(*rn as usize) {
            icell.vals.get(cn)
        } else {
            None
        }
    }

    /// Get Option<borrowed TableCell, String> using row, col
    #[inline]
    pub(crate) fn acr_tcell_props<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<CellProps> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .map(|cell| &cell.props)
            .unwrap_or(&None)
    }

    /// Get Option<borrowed TableCell, String> using row, col
    #[inline]
    pub(crate) fn acr_tcell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> Option<&'v mut TableCell> {
        cells
            .get_mut(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get_mut(cn))
    }

    /// Get Result<borrowed IndexCell, String> using row
    #[inline]
    pub(crate) fn acr_icell<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v IndexCell, String> {
        cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))
    }

    #[inline]
    pub(crate) fn acr_icell_mut<'v>(
        cells: &'v mut Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v mut IndexCell, String> {
        cells
            .get_mut(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))
    }

    /// Get Result<owned IndexCell, String> using row, replacing with provided new cell
    #[inline]
    pub(crate) fn acr_icell_replace(
        cells: &mut Vec<IndexCell>,
        rn: &usize,
        repl: IndexCell,
    ) -> Result<IndexCell, String> {
        cells
            .get_mut(*rn as usize)
            .map(|c| std::mem::replace(c, repl))
            .ok_or(str_err!(format!("Row not found: {}", rn)))
    }

    #[inline]
    pub(crate) fn acr_icell_swap(
        cells: &mut Vec<IndexCell>,
        pos: &usize,
        swap_cell: &mut IndexCell,
    ) -> Result<(), String> {
        cells
            .get_mut(*pos as usize)
            .map(|c| std::mem::swap(c, swap_cell))
            .ok_or(str_err!(format!("Row not found: {}", pos)))
    }

    /// Get Result<borrowed HeaderCell, String> using col
    #[inline]
    pub(crate) fn acr_hcell<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v HeaderCell, String> {
        cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))
    }

    #[inline]
    pub(crate) fn acr_hcell_mut<'v>(
        cells: &'v mut Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v mut HeaderCell, String> {
        cells
            .get_mut(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))
    }

    /// Get Result<owned HeaderCell, String> using col, replacing with provided new cell
    #[inline]
    pub(crate) fn acr_hcell_replace(
        cells: &mut Vec<HeaderCell>,
        cn: &usize,
        repl: HeaderCell,
    ) -> Result<HeaderCell, String> {
        cells
            .get_mut(*cn as usize)
            .map(|c| std::mem::replace(c, repl))
            .ok_or(str_err!(format!("Col not found: {}", cn)))
    }

    #[inline]
    pub(crate) fn acr_hcell_swap(
        cells: &mut Vec<HeaderCell>,
        pos: &usize,
        swap_cell: &mut HeaderCell,
    ) -> Result<(), String> {
        cells
            .get_mut(*pos as usize)
            .map(|c| std::mem::swap(c, swap_cell))
            .ok_or(str_err!(format!("Col not found: {}", pos)))
    }

    #[inline]
    pub(crate) fn acr_tcell_replace(
        cells: &mut Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
        repl: TableCell,
    ) -> Option<TableCell> {
        cells
            .get_mut(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get_mut(cn))
            .map(|cell| std::mem::replace(cell, repl))
    }

    /// Get borrowed val from TableCell.
    #[inline]
    pub(crate) fn acr_tcell_val<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> Option<&'v Val> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .map(|cell| &cell.val)
    }

    /// Get borrowed align from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_align<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<String> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.align)
            .unwrap_or(&None)
    }

    /// Get borrowed borders from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_borders<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<CellBorders> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.borders)
            .unwrap_or(&None)
    }

    /// Get borrowed checkbox from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_checkbox<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<Checkbox> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.checkbox)
            .unwrap_or(&None)
    }

    /// Get borrowed data_format from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_data_format<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<DataFormat> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.data_format)
            .unwrap_or(&None)
    }

    /// Get borrowed dropdown from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_dropdown<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<Dropdown> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.dropdown)
            .unwrap_or(&None)
    }

    /// Get borrowed highlight from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_highlight<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<Highlight> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.highlight)
            .unwrap_or(&None)
    }

    /// Get borrowed note from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_note<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<Note> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.note)
            .unwrap_or(&None)
    }

    /// Get borrowed progress_bar from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_progress_bar<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v Option<ProgressBar> {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.progress_bar)
            .unwrap_or(&None)
    }

    /// Get borrowed readonly from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_readonly<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
        cn: &usize,
    ) -> &'v bool {
        cells
            .get(*rn as usize)
            .and_then(|index_cell| index_cell.vals.get(cn))
            .and_then(|cell| cell.props.as_ref())
            .map(|props| &props.readonly)
            .unwrap_or(&false)
    }

    /// Get borrowed val from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_val<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Val, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(&cell.val)
    }

    /// Get borrowed open from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_open(cells: &Vec<IndexCell>, rn: &usize) -> Result<bool, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.open)
    }

    /// Get borrowed align from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_align<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<String>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.align))
    }

    /// Get borrowed borders from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_borders<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<CellBorders>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.borders))
    }

    /// Get borrowed checkbox from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_checkbox<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<Checkbox>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.checkbox))
    }

    /// Get borrowed data_format from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_data_format<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<DataFormat>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell
            .props
            .as_ref()
            .map_or(&None, |props| &props.data_format))
    }

    /// Get borrowed dropdown from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_dropdown<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<Dropdown>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.dropdown))
    }

    /// Get borrowed highlight from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_highlight<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<Highlight>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.highlight))
    }

    /// Get borrowed note from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_note<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<Note>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.note))
    }

    /// Get borrowed progress_bar from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_progress_bar<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v Option<ProgressBar>, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell
            .props
            .as_ref()
            .map_or(&None, |props| &props.progress_bar))
    }

    /// Get borrowed readonly from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_readonly<'v>(
        cells: &'v Vec<IndexCell>,
        rn: &usize,
    ) -> Result<&'v bool, String> {
        let cell = cells
            .get(*rn as usize)
            .ok_or(str_err!(format!("Row not found: {}", rn)))?;
        Ok(cell.props.as_ref().map_or(&false, |props| &props.readonly))
    }

    /// Get borrowed val from HeaderCell.
    #[inline]
    pub(crate) fn acr_hcell_val<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Val, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(&cell.val)
    }

    /// Get borrowed align from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_align<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<String>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.align))
    }

    /// Get borrowed borders from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_borders<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<CellBorders>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.borders))
    }

    /// Get borrowed checkbox from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_checkbox<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<Checkbox>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.checkbox))
    }

    /// Get borrowed data_format from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_data_format<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<DataFormat>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell
            .props
            .as_ref()
            .map_or(&None, |props| &props.data_format))
    }

    /// Get borrowed dropdown from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_dropdown<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<Dropdown>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.dropdown))
    }

    /// Get borrowed highlight from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_highlight<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<Highlight>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.highlight))
    }

    /// Get borrowed note from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_note<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<Note>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&None, |props| &props.note))
    }

    /// Get borrowed progress_bar from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_progress_bar<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v Option<ProgressBar>, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell
            .props
            .as_ref()
            .map_or(&None, |props| &props.progress_bar))
    }

    /// Get borrowed readonly from HeaderCell props.
    #[inline]
    pub(crate) fn acr_hcell_readonly<'v>(
        cells: &'v Vec<HeaderCell>,
        cn: &usize,
    ) -> Result<&'v bool, String> {
        let cell = cells
            .get(*cn as usize)
            .ok_or(str_err!(format!("Col not found: {}", cn)))?;
        Ok(cell.props.as_ref().map_or(&false, |props| &props.readonly))
    }
}
