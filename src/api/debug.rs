use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

use crate::Table;
use crate::log;

impl Table {
    // Existing macro-based log (uses println! — visible in native targets, silent in wasm)
    pub fn log_grid(&self) -> Result<(), String> {
        log!("Grid: {:?}", self.grid);
        Ok(())
    }

    pub fn log_index(&self) -> Result<(), String> {
        log!("Index: {:?}", self.grid.index);
        Ok(())
    }

    pub fn log_header(&self) -> Result<(), String> {
        log!("Header: {:?}", self.grid.header);
        Ok(())
    }

    pub fn log_selection(&self) -> Result<(), String> {
        log!("Selection: {:?}", self.sel);
        Ok(())
    }

    pub fn log_options(&self) -> Result<(), String> {
        log!("Options: {:?}", self.opts);
        Ok(())
    }

    pub fn log_disp_rows(&self) -> Result<(), String> {
        log!("Displayed Rows: {:?}", self.grid.disp_rows);
        Ok(())
    }

    pub fn log_canvas_rows(&self) -> Result<(), String> {
        log!("Canvas Rows: {:?}", self.grid.canvas_rows);
        Ok(())
    }

    pub fn log_disp_cols(&self) -> Result<(), String> {
        log!("Displayed Cols: {:?}", self.grid.disp_cols);
        Ok(())
    }

    pub fn log_canvas_cols(&self) -> Result<(), String> {
        log!("Canvas Cols: {:?}", self.grid.canvas_cols);
        Ok(())
    }

    pub fn log_undo_stack(&self) -> Result<(), String> {
        log!("Undo Stack Size: {}", self.grid.undo_stack.len());
        log!("Redo Stack Size: {}", self.grid.redo_stack.len());
        Ok(())
    }

    // String-returning debug functions (safe for wasm — wasm layer can console.log the result)
    // These avoid println! entirely and return formatted strings for flexible use
    pub fn debug_grid(&self) -> String {
        format!("Grid: {:?}", self.grid)
    }

    pub fn debug_index(&self) -> String {
        format!("Index: {:?}", self.grid.index)
    }

    pub fn debug_header(&self) -> String {
        format!("Header: {:?}", self.grid.header)
    }

    pub fn debug_selection(&self) -> String {
        format!("Selection: {:?}", self.sel)
    }

    pub fn debug_options(&self) -> String {
        format!("Options: {:?}", self.opts)
    }

    pub fn debug_disp_rows(&self) -> String {
        format!("Displayed Rows: {:?}", self.grid.disp_rows)
    }

    pub fn debug_canvas_rows(&self) -> String {
        format!("Canvas Rows: {:?}", self.grid.canvas_rows)
    }

    pub fn debug_disp_cols(&self) -> String {
        format!("Displayed Cols: {:?}", self.grid.disp_cols)
    }

    pub fn debug_canvas_cols(&self) -> String {
        format!("Canvas Cols: {:?}", self.grid.canvas_cols)
    }

    pub fn debug_sizes(&self) -> String {
        format!(
            "Total Rows: {} | Total Cols: {} | Undo Stack: {} | Redo Stack: {}",
            Self::acr_total_rows(&self.grid.index.cells),
            Self::acr_total_cols(&self.grid.header.cells),
            self.grid.undo_stack.len(),
            self.grid.redo_stack.len(),
        )
    }

    pub fn debug_tree_summary(&self) -> String {
        if !self.opts.tree_mode {
            return "Tree mode disabled".to_string();
        }
        let mut summary = String::new();
        summary.push_str(&format!(
            "Tree Mode Enabled | Total Rows: {}\n",
            Self::acr_total_rows(&self.grid.index.cells)
        ));
        summary.push_str("Root-level rows (visible tops):\n");
        for (rn, cell) in self.grid.index.cells.iter().enumerate() {
            if cell.par.is_empty() {
                summary.push_str(&format!(
                    "  Row {}: iid={} vis={} open={} children={}\n",
                    rn,
                    cell.iid,
                    cell.vis,
                    cell.open,
                    cell.chn.len()
                ));
            }
        }
        summary
    }

    pub fn debug_full(&self) -> String {
        let mut full = String::new();
        full.push_str(&self.debug_sizes());
        full.push_str("\n\n");
        full.push_str(&self.debug_options());
        full.push_str("\n\n");
        full.push_str(&self.debug_header());
        full.push_str("\n\n");
        full.push_str(&self.debug_index());
        full.push_str("\n\n");
        full.push_str(&self.debug_selection());
        full.push_str("\n\n");
        full.push_str(&self.debug_tree_summary());
        full
    }
}
