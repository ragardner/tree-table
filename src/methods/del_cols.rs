use crate::EventData;
use crate::Table;

impl Table {
    pub(crate) fn cr_del_cols(
        &self,
        positions: Vec<usize>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        let grid = &mut *self.grid.borrow_mut();
        // TODO: close dropdowns and editors?

        // Retrieve data from header
        event_data.removed_header = grid.header.del_cols(&positions)?;

        // Retrieve data from table
        event_data.removed_table_cells = grid.index.del_cols(&positions);

        // Pull outstanding col numbers
        grid.pull_cols(&positions);

        // Remake cols
        Self::acr_remake_cols(
            &mut grid.canvas_cols,
            &mut grid.disp_cols,
            &grid.header.cells,
        );

        Ok(())
    }
}
