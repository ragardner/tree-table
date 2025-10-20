use crate::{Events, Options, PartialOptions, Selection, TableGrid};
use std::cell::RefCell;

#[derive(Debug)]
pub struct Table {
    pub(crate) grid: RefCell<TableGrid>,
    pub(crate) opts: RefCell<Options>,
    pub(crate) sel: RefCell<Selection>,
    pub(crate) events: RefCell<Events>,
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
            grid: RefCell::new(TableGrid::new(opts.max_undos.clone())),
            opts: RefCell::new(opts),
            sel: RefCell::new(Selection::new()),
            events: RefCell::new(Events::new()),
        }
    }
}
