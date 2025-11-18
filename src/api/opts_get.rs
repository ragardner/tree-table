use crate::Options;
use crate::Table;

impl Table {
    /// Get the tables default settings as Options.
    /// - This does not return the current settings.
    pub fn get_opts_default(&self) -> Result<Options, String> {
        let default_opts = Options::default();
        Ok(default_opts)
    }

    /// Returns a `Ref<Options>` guard that derefs to `&Options` and keeps the borrow alive.
    pub fn get_opts(&self) -> &Options {
        &self.opts
    }
}
