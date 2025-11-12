use crate::EventData;
use crate::Options;
use crate::Table;
use crate::TableGrid;

impl Table {
    pub(crate) fn cr_del_cols(
        &self,
        grid: &mut TableGrid,
        opts: &Options,
        positions: Vec<usize>,
        event_data: &mut EventData,
    ) -> Result<(), String> {
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
            opts.vertical_grid_line_width.clone(),
        );

        Ok(())
    }
}
