use crate::EventData;
use crate::Table;
use crate::TableGrid;
use crate::utils::utils_fns::get_full_move_map;
use std::collections::HashMap;

impl Table {
    /// event_data.moved_cols must contain old_to_new
    pub(crate) fn cr_move_cols(
        &self,
        grid: &mut TableGrid,
        event_data: &mut EventData,
        new_to_old: &HashMap<usize, usize>,
    ) -> Result<(), String> {
        // Create full Vec of new positions
        let (full_old_to_new, full_new_to_old) = get_full_move_map(
            Self::acr_total_cols(&grid.header.cells) as usize,
            &event_data.moved_cols,
            new_to_old,
        );
        grid.move_cols(&full_old_to_new, &full_new_to_old)?;
        Self::acr_remake_cols(
            &mut grid.canvas_cols,
            &mut grid.disp_cols,
            &grid.header.cells,
        );
        Ok(())
    }
}
