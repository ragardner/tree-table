use crate::EventData;
use crate::Table;
use std::collections::HashSet;

impl Table {
    pub(crate) fn cr_del_rows(
        &self,
        mut positions: Vec<usize>,
        event_data: &mut EventData,
        validate: bool,
    ) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        // TODO: close dropdowns and editors?

        // Deal with tree, copies affected index cells
        if self.opts.borrow().tree_mode {
            if validate {
                // Descendants of rows are also deleted
                // In order not to make mistakes any descendant rows are removed first
                // Then all the descendants are added back in
                positions = self.cr_tree_remove_and_add_descendants(
                    positions
                        .iter()
                        .filter_map(|rn| grid.index.iid(rn).cloned())
                        .collect::<HashSet<String>>(),
                )?;
            }
            Self::acr_tree_del_rows(
                &mut grid.index.cells,
                &grid.index.iid_to_row,
                &positions,
                event_data,
            )?;
        }

        // Retrieve data from index
        event_data.removed_index = grid.index.del_rows(&positions)?;

        // Pull outstanding row numbers
        grid.pull_rows(&positions);

        // Remake rows
        Self::acr_remake_rows(
            &mut grid.canvas_rows,
            &mut grid.disp_rows,
            &grid.index.cells,
        );

        Ok(())
    }
}
