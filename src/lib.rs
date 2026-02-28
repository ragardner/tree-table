pub mod api;
pub mod args;
mod methods;
pub mod table;
pub mod types;
pub mod utils;

pub use args::{
    AddArgs, AddNumArgs, AddProgressBarArgs, AddProgressBarPosArgs, AddSelectionArgs, CheckboxArgs,
    DataFormatArgs, DelArgs, DelProgressBarArgs, DropdownArgs, FillDataArgs, GetDataIidsArgs,
    GetDataSpanArgs, GetDataYxArgs, HighlightArgs, InsertArgs, ItemArgs, ItemColArgs, ItemResult,
    MoveArgs, MoveItemsArgs, NoteArgs, ResetArgs, SelectRowsColsArgs, SelectionClearArgs,
    SetCheckboxArgs, SetDataArgs, SetDropdownArgs, SetNoteArgs, SetProgressBarArgs, TreeExtendArgs,
    TreeOpenArgs, UndoRedoArgs,
};
pub use table::Table;
pub use types::{
    Border, BoundedDeque, CellBorders, CellKey, CellProps, Change, ChangeCellProps, ChangeCheckbox,
    ChangeDropdown, ChangeHeaderCell, ChangeIndexCell, ChangeNote, ChangeProgressBar, Checkbox,
    Color3, Coords, CyclingCounter, DataFormat, Dropdown, DropdownState, EventData,
    GetCellMutResultExt, GetCellResultExt, GetResultExt, HeaderCell, Highlight, IndexCell, MaxMap,
    Note, Options, PartialOptions, ProgressBar, Selected, SelectionBox, SelectionKind,
    SelectionState, Span, SpanKey, TableCell, TreeMove, Val, ValType, statics,
    string_to_data_format, try_hex_to_color3,
};

pub(crate) use types::Header;
pub(crate) use types::index::Index;
pub(crate) use types::selection::Selection;
pub(crate) use types::table_grid::TableGrid;
