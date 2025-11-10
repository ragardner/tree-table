pub mod api;
pub mod args;
mod methods;
pub mod new;
pub mod types;
pub mod utils;

pub use new::Table;
pub use types::{
    Border, BoundedDeque, CellBorders, CellKey, CellProps, ChangeCellProps, ChangeHeaderCell,
    ChangeIndexCell, Checkbox, Color3, Coords, CyclingCounter, DataFormat, Dropdown, EventData,
    GetCellMutResultExt, GetCellResultExt, GetResultExt, HeaderCell, Highlight, IndexCell, MaxMap,
    Note, Options, PartialOptions, ProgressBar, Selected, SelectionBox, SelectionKind,
    SelectionState, Span, SpanKey, TableCell, TreeMove, Val, ValType, ValidationMessage, statics,
    string_to_data_format, try_hex_to_color3,
};

pub(crate) use types::Header;
pub(crate) use types::event_callbacks::Events;
pub(crate) use types::index::Index;
pub(crate) use types::selection::Selection;
pub(crate) use types::table_grid::TableGrid;
