use std::cell::RefCell;
use std::rc::Rc;

use crate::EventData;
use crate::ProgressBar;
use crate::Table;
use crate::args::AddProgressBarArgs;
use crate::try_hex_to_color3;

impl Table {
    pub fn add_progress_bar(
        &mut self,
        kwargs: AddProgressBarArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let progress_bar = Some(ProgressBar {
            name: kwargs.name.clone(),
            bg: try_hex_to_color3(kwargs.bg)?,
            fg: try_hex_to_color3(kwargs.fg)?,
            pc: kwargs.percent,
            remove_on_complete: kwargs.remove_on_complete,
        });
        let row = kwargs.row as usize;
        let col = kwargs.col as usize;
        let name = kwargs.name;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        self.cr_ensure_widget_nrows(row, &mut event_data, true)?;
        self.cr_ensure_widget_ncols(col, &mut event_data, true)?;
        self.grid.named_cells.insert(name, (row, col));
        Self::acr_event_data_set_tprogressbar(
            &mut self.grid.index.cells,
            row,
            col,
            progress_bar,
            &self.opts.empty_val,
            &mut event_data,
        )?;
        self.cr_event_modified(true)?;
        let shared = Rc::new(RefCell::new(event_data));
        if self.opts.enabled_undo && kwargs.undo && self.grid.undo_stack.get_maxlen() > 0 {
            self.grid.undo_stack.push_back(shared.clone());
        }
        Ok(kwargs.emit.then_some(shared))
    }
}
