use crate::{Events, Options, PartialOptions, Selection, TableGrid};

#[derive(Debug)]
pub struct Table {
    pub(crate) grid: TableGrid,
    pub(crate) opts: Options,
    pub(crate) sel: Selection,
    pub(crate) events: Events,
}

impl Table {
    pub fn new(row_dim: u64, options: Option<PartialOptions>) -> Self {
        let mut opts: Options = if let Some(mut options) = options {
            Self::acr_validate_partial_opts(&mut options).expect("Validation failed");
            Options::merge(Options::new(), options)
        } else {
            Options::new()
        };
        opts.default_row_dim = row_dim;
        Table {
            grid: TableGrid::new(opts.max_undos.clone()),
            opts,
            sel: Selection::new(),
            events: Events::new(),
        }
    }
}
