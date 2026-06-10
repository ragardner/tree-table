use crate::{
    Border, CellAlign, CellBorders, Change, Checkbox, Color3, DataFormat, Dropdown, DropdownState,
    EventData, HeaderCell, Highlight, Note, ProgressBar, Table, Val,
};
use alloc::string::String;
use alloc::vec::Vec;

impl Table {
    #[inline]
    pub(crate) fn acr_event_data_fmt_set_hval(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        val: Val,
        normalize: bool,
        ignore_fmt_errs: bool,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        let fmt_opt = Self::acr_hcell_data_format(cells, col)?;
        if fmt_opt.is_some() || Self::acr_val_valid_h(cells, &col, &val, false) {
            let val = Self::acr_apply_format(fmt_opt, val, normalize, ignore_fmt_errs)?;
            let cell = Self::acr_hcell_mut(cells, col)?;
            event_data.changes.push(Change::HeaderCell {
                col,
                change: cell.record_set_val(val),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // #[inline]
    // pub(crate) fn acr_event_data_set_hval(
    //     cells: &mut Vec<HeaderCell>,
    //     col: usize,
    //     val: Val,
    //     event_data: &mut EventData,
    // ) -> Result<bool, String> {
    //     if Self::acr_val_valid_h(cells, &col, &val, false) {
    //         let cell = Self::acr_hcell_mut(cells, col)?;
    //         event_data.changes.push(Change::HeaderCell {
    //             col,
    //             change: cell.record_set_val(val),
    //         });
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }

    #[inline]
    pub(crate) fn acr_event_data_set_hvis(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        vis: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_vis(vis),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_halign(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<CellAlign>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_align(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hborders(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<CellBorders>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_borders(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hcheckbox(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Checkbox>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_checkbox(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdata_format(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<DataFormat>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_data_format(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdropdown(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Dropdown>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_dropdown(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hhighlight(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Highlight>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_highlight(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hnote(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Note>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_note(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hprogressbar(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<ProgressBar>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_progress_bar(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hreadonly(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_readonly(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hprogressbar_bg(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_progress_bar_bg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hprogressbar_fg(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_progress_bar_fg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hprogressbar_pc(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: f64,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_progress_bar_pc(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hprogressbar_remove_on_complete(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_progress_bar_remove_on_complete(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hnote_text(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: String,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_note_text(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hnote_bg(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_note_bg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hnote_fg(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Color3>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_note_fg(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hnote_readonly(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_note_readonly(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdropdown_vals(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Vec<Val>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_dropdown_vals(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdropdown_default_val(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Val>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_dropdown_default_val(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdropdown_validation(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_dropdown_validation(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdropdown_text(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<String>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_dropdown_text(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdropdown_state(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: DropdownState,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_dropdown_state(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hcheckbox_true_val(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_checkbox_true_val(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hcheckbox_false_val(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_checkbox_false_val(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hcheckbox_text(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<String>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_checkbox_text(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hcheckbox_state(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_checkbox_state(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hborders_top(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_borders_top(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hborders_right(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_borders_right(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hborders_bottom(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_borders_bottom(prop)?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hborders_left(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<Border>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Self::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_borders_left(prop)?,
        });
        Ok(())
    }
}
