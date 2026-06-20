use crate::Table;

impl Table {
    pub fn total_rows(&self) -> usize {
        Self::acr_total_rows(&self.grid.index.cells)
    }

    pub fn total_cols(&self) -> usize {
        Self::acr_total_cols(&self.grid.header.cells)
    }
}
