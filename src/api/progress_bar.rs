use crate::EventData;
use crate::ProgressBar;
use crate::Table;
use crate::args::AddProgressBarArgs;
use crate::try_hex_to_color3;

impl Table {
    pub fn add_progress_bar(&self, kwargs: AddProgressBarArgs) -> Result<(), String> {
        let progress_bar = Some(ProgressBar {
            bg: try_hex_to_color3(kwargs.bg)?,
            fg: try_hex_to_color3(kwargs.fg)?,
            pc: kwargs.percent,
            remove_on_complete: kwargs.remove_on_complete,
        });
        let (mut grid, opts, sel) = self.cr_mut_borrows();
        let row = kwargs.row as usize;
        let col = kwargs.col as usize;
        let name = kwargs.name;
        let mut event_data: EventData = self.cr_new_event(true, &opts, &sel);
        self.cr_ensure_widget_nrows(&mut grid, &opts, &sel, row, &mut event_data, true)?;
        self.cr_ensure_widget_ncols(&mut grid, &opts, &sel, col, &mut event_data, true)?;
        event_data
            .added_progress_bars
            .insert(name.clone(), (row, col));
        grid.named_cells.insert(name, (row, col));
        Self::acr_event_data_set_tprogressbar(
            &mut grid.index.cells,
            row,
            col,
            progress_bar,
            &opts.empty_val,
            &mut event_data,
        )?;
        if !event_data.table_cells.is_empty() {
            if opts.enabled_undo && kwargs.undo {
                grid.undo_stack.push_back(event_data);
            }
            // TODO: set refresh timer
        }
        Ok(())
    }
}
