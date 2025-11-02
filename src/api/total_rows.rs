use crate::Table;

impl Table {
    pub fn total_rows(&self) -> usize {
        let (grid, _, _) = self.cr_mut_borrows();
        Self::acr_total_rows(&grid.index.cells)
    }
}
