use crate::ChangeHeaderCell;
use crate::ChangeIndexCell;
use crate::HeaderCell;
use crate::IndexCell;
use crate::Selected;
use crate::SelectionBox;
use crate::TableCell;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    IndexCell {
        row: usize,
        change: ChangeIndexCell,
    },
    HeaderCell {
        col: usize,
        change: ChangeHeaderCell,
    },

    AddRows(Vec<usize>),
    AddCols(Vec<usize>),

    RemoveRows(Vec<(usize, IndexCell)>), // (position, row)

    RemoveCols {
        removed_header: Vec<(usize, HeaderCell)>,
        removed_table: Vec<(usize, Vec<(usize, TableCell)>)>,
    },

    MoveRows(Vec<(usize, usize)>), // from → to
    MoveCols(Vec<(usize, usize)>),

    UpdateSelection {
        old_selection_boxes: Vec<SelectionBox>,
        old_selected: Option<Selected>,
        new_selection_boxes: Vec<SelectionBox>,
        new_selected: Option<Selected>,
    },
    SaveSelection {
        old_selection_boxes: Vec<SelectionBox>,
        old_selected: Option<Selected>,
    },
}

impl Change {
    // ───── Cell edit constructors ─────
    pub fn index_cell(row: usize, change: ChangeIndexCell) -> Self {
        Change::IndexCell { row, change }
    }

    pub fn header_cell(col: usize, change: ChangeHeaderCell) -> Self {
        Change::HeaderCell { col, change }
    }

    // ───── Structural constructors ─────
    pub fn add_rows(positions: Vec<usize>) -> Self {
        Change::AddRows(positions)
    }

    pub fn add_cols(positions: Vec<usize>) -> Self {
        Change::AddCols(positions)
    }

    pub fn remove_rows(rows: Vec<(usize, IndexCell)>) -> Self {
        Change::RemoveRows(rows)
    }

    pub fn remove_cols(
        removed_header: Vec<(usize, HeaderCell)>,
        removed_table: Vec<(usize, Vec<(usize, TableCell)>)>,
    ) -> Self {
        Change::RemoveCols {
            removed_header,
            removed_table,
        }
    }

    pub fn move_rows(map: Vec<(usize, usize)>) -> Self {
        Change::MoveRows(map)
    }

    pub fn move_cols(map: Vec<(usize, usize)>) -> Self {
        Change::MoveCols(map)
    }

    pub fn update_selection(
        old_selection_boxes: Vec<SelectionBox>,
        old_selected: Option<Selected>,
        new_selection_boxes: Vec<SelectionBox>,
        new_selected: Option<Selected>,
    ) -> Self {
        Change::UpdateSelection {
            old_selection_boxes,
            old_selected,
            new_selection_boxes,
            new_selected,
        }
    }

    pub fn save_selection(
        old_selection_boxes: Vec<SelectionBox>,
        old_selected: Option<Selected>,
    ) -> Self {
        Change::SaveSelection {
            old_selection_boxes,
            old_selected,
        }
    }
}
