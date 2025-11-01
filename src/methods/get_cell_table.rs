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
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

impl Table {
    /// Get Option<borrowed TableCell> using row, col
    #[inline]
    pub(crate) fn acr_tcell<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, TableCell>> {
        let rc = cells.get(rn)?;
        let cell_ref = rc.borrow();
        if cell_ref.vals.get(&cn).is_some() {
            Some(Ref::map(cell_ref, |cell| cell.vals.get(&cn).unwrap()))
        } else {
            None
        }
    }

    /// Get Option<borrowed TableCell.props> using row, col
    #[inline]
    pub(crate) fn acr_tcell_props<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, CellProps>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals.get(&cn).and_then(|tc| tc.props.as_ref())
            })
            .ok()
        })
    }

    /// Get Option<borrowed TableCell> using row, col
    #[inline]
    pub(crate) fn acr_tcell_mut<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<RefMut<'v, TableCell>> {
        let rc = cells.get(rn)?;
        let cell_mut = rc.borrow_mut();
        if cell_mut.vals.get(&cn).is_some() {
            Some(RefMut::map(cell_mut, |cell| {
                cell.vals.get_mut(&cn).unwrap()
            }))
        } else {
            None
        }
    }

    /// Get borrowed val from TableCell.
    #[inline]
    pub(crate) fn acr_tcell_val<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, Val>> {
        Self::acr_tcell(&cells, rn, cn).and_then(|rtc| Some(Ref::map(rtc, |tc| &tc.val)))
    }

    /// Get cloned val from TableCell.
    #[inline]
    pub(crate) fn acr_tcell_val_cloned(
        cells: &Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Val> {
        Self::acr_tcell(cells, rn, cn).map(|rtc| rtc.val.clone())
    }

    /// Get borrowed align from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_align<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, String>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.align.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed borders from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_borders<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, CellBorders>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.borders.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed checkbox from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_checkbox<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, Checkbox>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.checkbox.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed data_format from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_data_format<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, DataFormat>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.data_format.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed dropdown from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_dropdown<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, Dropdown>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.dropdown.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed highlight from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_highlight<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, Highlight>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.highlight.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed note from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_note<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, Note>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.note.as_ref())
            })
            .ok()
        })
    }

    /// Get borrowed progress_bar from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_progress_bar<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<Ref<'v, ProgressBar>> {
        cells.get(rn).and_then(|rc| {
            Ref::filter_map(rc.borrow(), |cell| {
                cell.vals
                    .get(&cn)
                    .and_then(|tc| tc.props.as_ref())
                    .and_then(|props| props.progress_bar.as_ref())
            })
            .ok()
        })
    }

    /// Get readonly from TableCell props.
    #[inline]
    pub(crate) fn acr_tcell_readonly<'v>(
        cells: &'v Vec<Rc<RefCell<IndexCell>>>,
        rn: usize,
        cn: usize,
    ) -> Option<bool> {
        let rc = cells.get(rn)?;
        let cell_ref = rc.borrow();
        cell_ref
            .vals
            .get(&cn)
            .and_then(|tc| tc.props.as_ref())
            .map(|p| p.readonly)
    }
}
