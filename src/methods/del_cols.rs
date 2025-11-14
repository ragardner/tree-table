use crate::EventData;
use crate::Table;

impl Table {
    pub(crate) fn cr_del_cols(
        &mut self,
        positions: Vec<usize>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
        // TODO: close dropdowns and editors?

        // Retrieve data from header
        event_data.removed_header = self.grid.header.del_cols(&positions)?;

        // Retrieve data from table
        event_data.removed_table_cells = self.grid.index.del_cols(&positions);

        // Pull outstanding col numbers
        self.grid.pull_cols(&positions);

        // Remake cols
        Self::acr_remake_cols(
            &mut self.grid.canvas_cols,
            &mut self.grid.disp_cols,
            &self.grid.header.cells,
            self.opts.vertical_grid_line_width.clone(),
        );

        Ok(())
    }
}
