// use crate::log;
use crate::Table;
use crate::log;

impl Table {
    pub fn log_grid(&self) -> Result<(), String> {
        log!("Grid: {:?}", self.grid);
        Ok(())
    }
}
