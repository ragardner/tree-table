use crate::EventData;
use crate::Table;
use crate::types::TreeMove;
use crate::utils::utils_fns::get_full_move_map;
use std::collections::HashMap;

impl Table {
    pub(crate) fn cr_move_rows(
        &mut self,
        event_data: &mut EventData,
        mut new_to_old: HashMap<usize, usize>,
        tree_undo_data: Option<&mut EventData>,
        tree_move: Option<TreeMove>,
        mod_tree: bool,
    ) -> Result<(), String> {
        // Modify tree
        if mod_tree && self.opts.tree_mode {
            self.cr_tree_move_rows_mod_cells(
                event_data,
                &mut new_to_old,
                tree_undo_data,
                tree_move,
            )?;
        }
        // Create full Vec of new positions
        let (full_old_to_new, full_new_to_old) = get_full_move_map(
            Self::acr_total_rows(&self.grid.index.cells),
            &event_data.moved_rows,
            &new_to_old,
        );
        self.grid.move_rows(&full_old_to_new, &full_new_to_old)?;
        Self::acr_remake_rows(
            &mut self.grid.canvas_rows,
            &mut self.grid.disp_rows,
            &self.grid.index.cells,
            self.opts.horizontal_grid_line_width.clone(),
        );
        Ok(())
    }
}
