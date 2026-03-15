use crate::Border;
use crate::CellAlign;
use crate::CellBorders;
use crate::Change;
use crate::Checkbox;
use crate::Color3;
use crate::DataFormat;
use crate::Dropdown;
use crate::DropdownState;
use crate::EventData;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;

impl Table {
    #[inline]
    pub(crate) fn acr_event_data_fmt_set_tval(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        val: Val,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        let fmt_opt = Self::acr_tcell_data_format(cells, row, col);
        let valid = Self::acr_val_valid_t(&cells, &row, &col, &val, false);
        if fmt_opt.is_some() || valid {
            let val = if let Some(fmt_ref) = fmt_opt {
                Self::acr_format_data(&fmt_ref, val)
            } else {
                val
            };
            let cell = Table::acr_icell_mut(cells, row)?;
            event_data.changes.push(Change::IndexCell {
                row,
                change: cell.record_set_table_cell_val(col, val, empty_val.clone()),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // #[inline]
    // pub(crate) fn acr_event_data_set_tval(
    //     cells: &mut Vec<IndexCell>,
    //     row: usize,
    //     col: usize,
    //     val: Val,
    //     empty_val: &Val,
    //     event_data: &mut EventData,
    // ) -> Result<bool, String> {
    //     if Self::acr_val_valid_t(cells, &row, &col, &val, false) {
    //         let cell = Table::acr_icell_mut(cells, row)?;
    //         event_data.changes.push(Change::IndexCell {
    //             row,
    //             change: cell.record_set_table_cell_val(col, val, empty_val.clone()),
    //         });
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }

    #[inline]
    pub(crate) fn acr_event_data_set_talign(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<CellAlign>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_align(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tborders(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<CellBorders>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_borders(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tcheckbox(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Checkbox>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_checkbox(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdata_format(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<DataFormat>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        // TODO: adjust cell Val?
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_data_format(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdropdown(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Dropdown>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_dropdown(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_thighlight(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Highlight>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_highlight(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tnote(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Note>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_note(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tprogressbar(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<ProgressBar>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_progress_bar(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_treadonly(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: bool,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_readonly(col, prop, empty_val.clone()),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tprogressbar_bg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Color3>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_progress_bar_bg(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tprogressbar_fg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Color3>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_progress_bar_fg(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tprogressbar_pc(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: f64,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_progress_bar_pc(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tprogressbar_remove_on_complete(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: bool,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_progress_bar_remove_on_complete(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tnote_text(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: String,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_note_text(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tnote_bg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Color3>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_note_bg(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tnote_fg(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Color3>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_note_fg(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tnote_readonly(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: bool,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_note_readonly(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdropdown_vals(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Vec<Val>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_dropdown_vals(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdropdown_default_val(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Val>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_dropdown_default_val(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdropdown_validation(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: bool,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_dropdown_validation(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdropdown_text(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<String>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_dropdown_text(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tdropdown_state(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: DropdownState,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_dropdown_state(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tcheckbox_true_val(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Val,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_checkbox_true_val(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tcheckbox_false_val(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Val,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_checkbox_false_val(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tcheckbox_text(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<String>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_checkbox_text(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tcheckbox_state(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: bool,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_checkbox_state(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tborders_top(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Border>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_borders_top(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tborders_right(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Border>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_borders_right(col, prop, empty_val.clone())?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tborders_bottom(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Border>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_borders_bottom(
                col,
                prop,
                empty_val.clone(),
            )?,
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tborders_left(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        prop: Option<Border>,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_table_cell_props_borders_left(col, prop, empty_val.clone())?,
        });
        Ok(())
    }
}
