use crate::{
    Border, CellAlign, CellBorders, Change, Checkbox, Color3, DataFormat, Dropdown, DropdownState,
    EventData, Highlight, IndexCell, Note, ProgressBar, Table, Val,
};
use alloc::string::String;
use alloc::vec::Vec;

impl Table {
    #[inline]
    pub(crate) fn acr_event_data_fmt_set_ival(
        cells: &mut Vec<IndexCell>,
        row: usize,
        val: Val,
        normalize: bool,
        ignore_fmt_errs: bool,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        let fmt_opt = Self::acr_icell_data_format(cells, row)?;
        if fmt_opt.is_some() || Self::acr_val_valid_i(cells, &row, &val, false) {
            let val = Self::acr_apply_format(fmt_opt, val, normalize, ignore_fmt_errs)?;
            let cell = Self::acr_icell_mut(cells, row)?;
            event_data.changes.push(Change::IndexCell {
                row,
                change: cell.record_set_val(val),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // #[inline]
    // pub(crate) fn acr_event_data_set_ival(
    //     cells: &mut Vec<IndexCell>,
    //     row: usize,
    //     val: Val,
    //     event_data: &mut EventData,
    // ) -> Result<bool, String> {
    //     if Self::acr_val_valid_i(cells, &row, &val, false) {
    //         let cell = Self::acr_icell_mut(cells, row)?;
    //         event_data.changes.push(Change::IndexCell {
    //             row,
    //             change: cell.record_set_val(val),
    //         });
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }

    #[inline]
    pub(crate) fn acr_event_data_set_ivis(
        cells: &mut Vec<IndexCell>,
        row: usize,
        vis: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_vis(vis),
        });
        Ok(())
    }

    // #[inline]
    // pub(crate) fn acr_event_data_set_open(
    //     cells: &mut Vec<IndexCell>,
    //     row: usize,
    //     open: bool,
    //     event_data: &mut EventData,
    // ) -> Result<(), String> {
    //     let cell = Self::acr_icell_mut(cells, row)?;
    //     event_data.changes.push(Change::IndexCell {
    //         row,
    //         change: cell.record_set_open(open),
    //     });
    //     Ok(())
    // }

    #[inline]
    pub(crate) fn acr_event_data_set_ialign(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<CellAlign>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_align(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iborders(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<CellBorders>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_borders(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_icheckbox(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Checkbox>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_checkbox(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idata_format(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<DataFormat>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_data_format(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idropdown(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Dropdown>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_dropdown(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_ihighlight(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Highlight>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_highlight(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_inote(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Note>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_note(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iprogressbar(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<ProgressBar>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_progress_bar(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_ireadonly(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_readonly(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iprogressbar_bg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_progress_bar_bg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iprogressbar_fg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_progress_bar_fg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iprogressbar_pc(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: f64,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_progress_bar_pc(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iprogressbar_remove_on_complete(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_progress_bar_remove_on_complete(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_inote_text(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: String,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_note_text(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_inote_bg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_note_bg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_inote_fg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_note_fg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_inote_readonly(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_note_readonly(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idropdown_vals(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Vec<Val>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_dropdown_vals(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idropdown_default_val(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Val>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_dropdown_default_val(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idropdown_validation(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_dropdown_validation(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idropdown_text(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<String>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_dropdown_text(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_idropdown_state(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: DropdownState,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_dropdown_state(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_icheckbox_true_val(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_checkbox_true_val(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_icheckbox_false_val(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_checkbox_false_val(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_icheckbox_text(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<String>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_checkbox_text(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_icheckbox_state(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_checkbox_state(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iborders_top(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_borders_top(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iborders_right(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_borders_right(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iborders_bottom(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_borders_bottom(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_iborders_left(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_borders_left(prop)?,
        });
        Ok(())
    }
}
