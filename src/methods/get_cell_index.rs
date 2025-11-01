use crate::CellBorders;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::GetCellResultExt;
use crate::GetResultExt;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::UpgradeResultExt;
use crate::Val;
use crate::str_err;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

impl Table {
    #[inline]
    pub(crate) fn acr_tree_iid_cell_weak(
        cells: &Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Option<Weak<RefCell<IndexCell>>>, String> {
        if let Some(&row) = iid_to_row.get(iid) {
            let rc = cells.get_res(row)?;
            Ok(Some(Rc::downgrade(rc)))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_iid_chn_mut<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<RefMut<'v, Vec<Rc<RefCell<IndexCell>>>>, String> {
        let row_num = Self::acr_pos(iid_to_row, iid)?;
        let rc = cells.get_res(row_num)?;
        let cell_mut = rc.borrow_mut();
        Ok(RefMut::map(cell_mut, |cell| &mut cell.chn))
    }

    #[inline]
    pub(crate) fn acr_tree_iid_chn<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Ref<'v, Vec<Rc<RefCell<IndexCell>>>>, String> {
        let row_num = Self::acr_pos(iid_to_row, iid)?;
        let rc = cells.get_res(row_num)?;
        let cell_ref = rc.borrow();
        Ok(Ref::map(cell_ref, |cell| &cell.chn))
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell_mut<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Option<RefMut<'v, IndexCell>>, String> {
        let iid_rc = cells.get_res(*iid_to_row.get_res(iid)?)?;
        if let Some(par_weak) = iid_rc.borrow().par.as_ref() {
            let par_rc_temp = par_weak.upgrade_res()?;
            let par_row = {
                let par_ref_temp = par_rc_temp.borrow();
                *iid_to_row.get_res(&par_ref_temp.iid)?
            };
            let par_rc = cells.get_res(par_row)?;
            Ok(Some(par_rc.borrow_mut()))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Option<Ref<'v, IndexCell>>, String> {
        let iid_rc = cells.get_res(*iid_to_row.get_res(iid)?)?;
        let iid_ref = iid_rc.borrow();
        let par_weak = iid_ref.par.as_ref();
        if let Some(par_weak) = par_weak {
            let par_rc_temp = par_weak.upgrade_res()?;
            let par_ref_temp = par_rc_temp.borrow();
            let par_iid = &par_ref_temp.iid;
            let par_row = *iid_to_row.get_res(par_iid)?;
            let par_rc = cells.get_res(par_row)?;
            Ok(Some(par_rc.borrow()))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_par_cell_weak<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Option<Weak<RefCell<IndexCell>>>, String> {
        let iid_rc = cells.get_res(*iid_to_row.get_res(iid)?)?;
        let iid_ref = iid_rc.borrow();
        let par_weak = iid_ref.par.as_ref();
        if let Some(par_weak) = par_weak {
            let par_rc_temp = par_weak.upgrade_res()?;
            let par_ref_temp = par_rc_temp.borrow();
            let par_iid = &par_ref_temp.iid;
            let par_row = *iid_to_row.get_res(par_iid)?;
            let par_rc = cells.get_res(par_row)?;
            Ok(Some(Rc::downgrade(&par_rc)))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_row_par_cell<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        row: usize,
    ) -> Result<Option<Ref<'v, IndexCell>>, String> {
        let child_rc = cells.get_res(row)?;
        let child_ref = child_rc.borrow();
        let par_weak = child_ref.par.as_ref();
        if let Some(par_weak) = par_weak {
            let par_rc_temp = par_weak.upgrade_res()?;
            let par_ref_temp = par_rc_temp.borrow();
            let par_iid = &par_ref_temp.iid;
            let par_row = *iid_to_row.get_res(par_iid)?;
            let par_rc = cells.get_res(par_row)?;
            Ok(Some(par_rc.borrow()))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_row_par(
        cells: &Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        row: usize,
    ) -> Result<String, String> {
        if row >= Self::acr_total_rows(&cells) {
            Ok("".to_string())
        } else {
            let par_cell = Self::acr_tree_row_par_cell(cells, iid_to_row, row)?;
            Ok(par_cell.map(|c| c.iid.clone()).unwrap_or_default())
        }
    }

    #[inline]
    pub(crate) fn acr_tree_iid_par(
        cells: &Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<String, String> {
        let par_cell = Self::acr_tree_par_cell(&cells, &iid_to_row, iid)?;
        Ok(par_cell.map(|c| c.iid.clone()).unwrap_or_default())
    }

    #[inline]
    pub(crate) fn acr_tree_pars_equal(
        cells: &Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid_to_check: &str,
        to_compare: &str,
    ) -> Result<bool, String> {
        let par_cell = Self::acr_tree_par_cell(&cells, &iid_to_row, iid_to_check)?;
        if let Some(par_cell) = par_cell {
            let par_iid = par_cell.iid.as_str();
            Ok(par_iid == to_compare)
        } else {
            Ok("" == to_compare)
        }
    }

    #[inline]
    pub(crate) fn acr_tree_iid_cell_mut<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<RefMut<'v, IndexCell>, String> {
        iid_to_row
            .get(iid)
            .and_then(|rn| cells.get(*rn))
            .map(|rc| rc.borrow_mut())
            .ok_or_else(|| format!("iid not found: {}", iid).into())
    }

    #[inline]
    pub(crate) fn acr_tree_iid_cell<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<Ref<'v, IndexCell>, String> {
        iid_to_row
            .get(iid)
            .and_then(|rn| cells.get(*rn))
            .map(|rc| rc.borrow())
            .ok_or_else(|| format!("iid not found: {}", iid).into())
    }

    #[inline]
    pub(crate) fn acr_tree_has_par<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        Self::acr_tree_iid_cell(cells, iid_to_row, iid).map(|cell| cell.par.is_some())
    }

    #[inline]
    pub(crate) fn acr_row_has_par<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<bool, String> {
        Self::acr_icell(cells, rn).map(|cell| cell.par.is_some())
    }

    #[inline]
    pub(crate) fn acr_tree_iid_is_open<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        let cell = Self::acr_tree_iid_cell(cells, iid_to_row, iid)?;
        Ok(cell.open)
    }

    #[inline]
    pub(crate) fn acr_row_iid_is_vis<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        iid_to_row: &HashMap<String, usize>,
        iid: &str,
    ) -> Result<bool, String> {
        let cell = Self::acr_tree_iid_cell(cells, iid_to_row, iid)?;
        Ok(cell.vis)
    }

    #[inline]
    pub(crate) fn acr_row_is_vis<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        row: usize,
    ) -> Result<bool, String> {
        let cell = Self::acr_icell(cells, row)?;
        Ok(cell.vis)
    }

    #[inline]
    pub(crate) fn acr_tree_row_is_open<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        row: usize,
    ) -> Result<bool, String> {
        let rc = cells.get_res(row)?;
        let cell = rc.borrow();
        Ok(cell.open)
    }

    /// Get Result<copied position, String> using iid.
    #[inline]
    pub(crate) fn acr_pos(m: &HashMap<String, usize>, iid: &str) -> Result<usize, String> {
        m.get(iid)
            .copied()
            .ok_or(str_err!(format!("iid not found: {}", iid)))
    }

    /// Get Result<borrowed iid, String> using row
    #[inline]
    pub(crate) fn acr_borrow_riid<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        pos: usize,
    ) -> Result<Ref<'v, String>, String> {
        let rc = cells.get_res(pos)?;
        let cell_ref = rc.borrow();
        Ok(Ref::map(cell_ref, |cell| &cell.iid))
    }

    /// Get Result<iid, String> using row
    #[inline]
    pub(crate) fn acr_copy_riid<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        pos: usize,
    ) -> Result<String, String> {
        let rc = cells.get_res(pos)?;
        let cell_ref = rc.borrow();
        Ok(cell_ref.iid.clone())
    }

    /// Get Result<borrowed IndexCell, String> using row
    #[inline]
    pub(crate) fn acr_icell<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Ref<'v, IndexCell>, String> {
        cells
            .get(rn)
            .map(|rc| rc.borrow())
            .ok_or_else(|| format!("Row not found: {}", rn))
    }

    #[inline]
    pub(crate) fn acr_icell_mut<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<RefMut<'v, IndexCell>, String> {
        cells
            .get(rn)
            .map(|rc| rc.borrow_mut())
            .ok_or_else(|| format!("Row not found: {}", rn))
    }

    /// Get borrowed val from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_val<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Ref<'v, Val>, String> {
        Ok(Ref::map(Self::acr_icell(&cells, rn)?, |ic| &ic.val))
    }

    /// Get borrowed val from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_val_cloned<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Val, String> {
        Ok(Self::acr_icell(&cells, rn)?.val.clone())
    }

    /// Get borrowed open from IndexCell.
    #[inline]
    pub(crate) fn acr_icell_open<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<bool, String> {
        Ok(Self::acr_icell(cells, rn)?.open)
    }

    /// Get borrowed align from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_align<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, String>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props.as_ref().and_then(|props| props.align.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed borders from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_borders<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, CellBorders>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props.as_ref().and_then(|props| props.borders.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed checkbox from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_checkbox<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, Checkbox>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props
                    .as_ref()
                    .and_then(|props| props.checkbox.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed data_format from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_data_format<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, DataFormat>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props
                    .as_ref()
                    .and_then(|props| props.data_format.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed dropdown from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_dropdown<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, Dropdown>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props
                    .as_ref()
                    .and_then(|props| props.dropdown.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed highlight from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_highlight<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, Highlight>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props
                    .as_ref()
                    .and_then(|props| props.highlight.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed note from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_note<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, Note>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props.as_ref().and_then(|props| props.note.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed progress_bar from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_progress_bar<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<Option<Ref<'v, ProgressBar>>, String> {
        Self::acr_icell(cells, rn).map(|cell_ref| {
            Ref::filter_map(cell_ref, |cell| {
                cell.props
                    .as_ref()
                    .and_then(|props| props.progress_bar.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed readonly from IndexCell props.
    #[inline]
    pub(crate) fn acr_icell_readonly<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
    ) -> Result<bool, String> {
        Ok(Self::acr_icell(cells, rn)?
            .props
            .as_ref()
            .map_or(false, |props| props.readonly))
    }
}
