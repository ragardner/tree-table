pub mod api;
pub mod args;
mod methods;
pub mod table;
pub mod types;
pub mod utils;

pub use args::{
    AddArgs, AddNumArgs, AddProgressBarArgs, AddProgressBarPosArgs, AddSelectionArgs, BorderArgs,
    BordersArgs, CellAlignArgs, CheckboxArgs, DataFormatArgs, DelArgs, DelProgressBarArgs,
    DelProgressBarNameArgs, DropdownArgs, FillDataArgs, GetDataIidsArgs, GetDataSpanArgs,
    GetDataYxArgs, HighlightArgs, InsertArgs, ItemArgs, ItemColArgs, ItemResult, MoveArgs,
    MoveItemsArgs, NoteArgs, ReadonlyArgs, ResetArgs, SelectRowsColsArgs, SelectionClearArgs,
    SetBordersArgs, SetCheckboxArgs, SetDataArgs, SetDropdownArgs, SetNoteArgs, SetProgressBarArgs,
    SetProgressBarNameArgs, TreeExtendArgs, TreeOpenArgs, UndoRedoArgs,
};
pub use table::Table;
pub use types::{
    Border, BoundedDeque, CellAlign, CellBorders, CellKey, CellProps, Change, ChangeBorders,
    ChangeCellProps, ChangeCheckbox, ChangeDropdown, ChangeHeaderCell, ChangeIndexCell, ChangeNote,
    ChangeProgressBar, Checkbox, Color3, CyclingCounter, DataFormat, Dropdown, DropdownState,
    EventData, GetCellMutResultExt, GetCellResultExt, GetResultExt, HeaderCell, Highlight,
    IndexCell, MaxMap, NO_END, Note, Options, PartialOptions, ProgressBar, Selected, SelectionKind,
    SelectionState, Span, SpanKey, TableCell, TreeMove, Val, ValType, statics,
    string_to_data_format, try_hex_to_color3,
};

pub(crate) use types::Header;
pub(crate) use types::index::Index;
pub(crate) use types::selection::Selection;
pub(crate) use types::table_grid::TableGrid;
