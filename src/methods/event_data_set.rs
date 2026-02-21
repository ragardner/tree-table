use crate::CellBorders;
use crate::Change;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::EventData;
use crate::HeaderCell;
use crate::Highlight;
use crate::IndexCell;
use crate::Note;
use crate::ProgressBar;
use crate::Table;
use crate::Val;

impl Table {
    // TODO: AND MAYBE DATA FORMATTING PROP SETTER FUNCTIONS
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

    #[inline]
    pub(crate) fn acr_event_data_fmt_set_ival(
        cells: &mut Vec<IndexCell>,
        row: usize,
        val: Val,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        let is_valid = Self::acr_val_valid_i(cells, &row, &val, false);
        let fmt_opt = Self::acr_icell_data_format(cells, row)?;
        if fmt_opt.is_some() || is_valid {
            let formatted_val = if let Some(fmt_ref) = fmt_opt {
                Self::acr_format_data(&fmt_ref, val)
            } else {
                val
            };
            let cell = Table::acr_icell_mut(cells, row)?;
            event_data.changes.push(Change::IndexCell {
                row,
                change: cell.record_set_val(formatted_val),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub(crate) fn acr_event_data_fmt_set_hval(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        val: Val,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        let fmt = Self::acr_hcell_data_format(cells, col)?;
        if fmt.is_some() || Self::acr_val_valid_h(cells, &col, &val, false) {
            let val = if let Some(fmt) = fmt {
                Self::acr_format_hdata(fmt, val)
            } else {
                val
            };
            let cell = Table::acr_hcell_mut(cells, col)?;
            event_data.changes.push(Change::HeaderCell {
                col,
                change: cell.record_set_val(val),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub(crate) fn acr_event_data_set_tval(
        cells: &mut Vec<IndexCell>,
        row: usize,
        col: usize,
        val: Val,
        empty_val: &Val,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        if Self::acr_val_valid_t(cells, &row, &col, &val, false) {
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

    #[inline]
    pub(crate) fn acr_event_data_set_ival(
        cells: &mut Vec<IndexCell>,
        row: usize,
        val: Val,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        if Self::acr_val_valid_i(cells, &row, &val, false) {
            let cell = Table::acr_icell_mut(cells, row)?;
            event_data.changes.push(Change::IndexCell {
                row,
                change: cell.record_set_val(val),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hval(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        val: Val,
        event_data: &mut EventData,
    ) -> Result<bool, String> {
        if Self::acr_val_valid_h(cells, &col, &val, false) {
            let cell = Table::acr_hcell_mut(cells, col)?;
            event_data.changes.push(Change::HeaderCell {
                col,
                change: cell.record_set_val(val),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub(crate) fn acr_event_data_set_ivis(
        cells: &mut Vec<IndexCell>,
        row: usize,
        vis: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_vis(vis),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hvis(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        vis: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_vis(vis),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_open(
        cells: &mut Vec<IndexCell>,
        row: usize,
        open: bool,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_open(open),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_talign(
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
    pub(crate) fn acr_event_data_set_ialign(
        cells: &mut Vec<IndexCell>,
        row: usize,
        prop: Option<String>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
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
        let cell = Table::acr_icell_mut(cells, row)?;
        event_data.changes.push(Change::IndexCell {
            row,
            change: cell.record_set_props_readonly(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_halign(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<String>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_checkbox(prop),
        });
        Ok(())
    }

    #[inline]
    pub(crate) fn acr_event_data_set_hdataformat(
        cells: &mut Vec<HeaderCell>,
        col: usize,
        prop: Option<DataFormat>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
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
        let cell = Table::acr_hcell_mut(cells, col)?;
        event_data.changes.push(Change::HeaderCell {
            col,
            change: cell.record_set_props_readonly(prop),
        });
        Ok(())
    }
}
