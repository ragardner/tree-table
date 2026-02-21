use crate::Change;
use crate::EventData;
use crate::Table;
use crate::utils::utils_fns::get_full_move_map;
use std::collections::HashMap;
use std::collections::HashSet;

impl Table {
    pub(crate) fn cr_move_cols(
        &mut self,
        event_data: &mut EventData,
        old: HashSet<usize>,
        old_to_new: Vec<(usize, usize)>,
        new_to_old: &HashMap<usize, usize>,
    ) -> Result<(), String> {
        // Add to event_data
        event_data.changes.push(Change::MoveCols(old_to_new));

        // Create full Vec of new positions
        let (full_old_to_new, full_new_to_old) = get_full_move_map(
            Self::acr_total_cols(&self.grid.header.cells) as usize,
            &old,
            &new_to_old,
        );
        self.grid.move_cols(&full_old_to_new, &full_new_to_old)?;
        Self::acr_remake_cols(
            &mut self.grid.canvas_cols,
            &mut self.grid.disp_cols,
            &self.grid.header.cells,
            self.opts.vertical_grid_line_width.clone(),
        );
        Ok(())
    }
}
