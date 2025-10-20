use crate::PartialOptions;
use crate::Table;

impl Table {
    pub fn set_opts(&self, options: Option<PartialOptions>) -> Result<(), String> {
        self.cr_merge_opts(options.unwrap_or_default())?;
        Ok(())
    }
}
