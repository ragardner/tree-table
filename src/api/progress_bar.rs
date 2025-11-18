use crate::EventData;
use crate::ProgressBar;
use crate::Table;
use crate::args::AddProgressBarArgs;
use crate::try_hex_to_color3;

impl Table {
    pub fn add_progress_bar(&mut self, kwargs: AddProgressBarArgs) -> Result<(), String> {
        let progress_bar = Some(ProgressBar {
            bg: try_hex_to_color3(kwargs.bg)?,
            fg: try_hex_to_color3(kwargs.fg)?,
            pc: kwargs.percent,
            remove_on_complete: kwargs.remove_on_complete,
        });
        let row = kwargs.row as usize;
        let col = kwargs.col as usize;
        let name = kwargs.name;
        let mut event_data: EventData = self.cr_new_event(true);
        self.cr_ensure_widget_nrows(row, &mut event_data, true)?;
        self.cr_ensure_widget_ncols(col, &mut event_data, true)?;
        event_data
            .added_progress_bars
            .insert(name.clone(), (row, col));
        self.grid.named_cells.insert(name, (row, col));
        Self::acr_event_data_set_tprogressbar(
            &mut self.grid.index.cells,
            row,
            col,
            progress_bar,
            &self.opts.empty_val,
            &mut event_data,
        )?;
        if !event_data.table_cells.is_empty() {
            if self.opts.enabled_undo && kwargs.undo {
                self.grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
