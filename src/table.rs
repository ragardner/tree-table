use crate::{Events, Options, PartialOptions, Selection, TableGrid};

#[derive(Clone, Debug)]
pub struct Table {
    pub(crate) grid: TableGrid,
    pub(crate) opts: Options,
    pub(crate) sel: Selection,
    pub(crate) events: Events,
}

impl Table {
    pub fn new(default_row_dim: u64, options: Option<PartialOptions>) -> Self {
        let mut opts = Options::new();
        if let Some(mut partial) = options {
            Self::acr_validate_partial_opts(&mut partial)
                .expect("Invalid PartialOptions passed to Table::new");
            opts = Options::merge(opts, partial);
        }
        opts.default_row_dim = default_row_dim;
        Self {
            grid: TableGrid::new(opts.max_undos.clone()),
            opts,
            sel: Selection::new(),
            events: Events::new(),
        }
    }
}
