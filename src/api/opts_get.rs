use crate::Options;
use crate::Table;
use std::cell::Ref;

impl Table {
    /// Get the tables default settings as Options.
    /// - This does not return the current settings.
    pub fn get_opts_default(&self) -> Result<Options, String> {
        let default_opts = Options::default();
        Ok(default_opts)
    }

    /// Returns a `Ref<Options>` guard that derefs to `&Options` and keeps the borrow alive.
    pub fn get_opts(&self) -> Result<Ref<'_, Options>, String> {
        self.opts.try_borrow().map_err(|e| e.to_string())
    }
}
