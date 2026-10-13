#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod api;
pub mod args;
pub mod error;
mod methods;
pub mod table;
pub mod types;
pub mod utils;

pub use table::Table;

pub(crate) use args::*;
pub(crate) use types::Header;
pub(crate) use types::index::Index;
pub(crate) use types::selection::Selection;
pub(crate) use types::table_grid::TableGrid;
pub(crate) use types::*;
