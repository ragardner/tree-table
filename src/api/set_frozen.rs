use crate::Table;

impl Table {
    pub fn set_frozen_rows(&mut self, freeze: Vec<u64>) -> Result<(), String> {
        let max_row = Self::acr_total_rows(&self.grid.index.cells) as u64;
        let rows: Vec<usize> = freeze
            .into_iter()
            .filter_map(|r| if r < max_row { Some(r as usize) } else { None })
            .collect();
        self.grid.frozen_rows = rows;
        Ok(())
    }

    pub fn set_frozen_cols(&mut self, freeze: Vec<u64>) -> Result<(), String> {
        let max_col = Self::acr_total_cols(&self.grid.header.cells) as u64;
        let cols: Vec<usize> = freeze
            .into_iter()
            .filter_map(|c| if c < max_col { Some(c as usize) } else { None })
            .collect();
        self.grid.frozen_cols = cols;
        Ok(())
    }
}
