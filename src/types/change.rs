use alloc::vec::Vec;

use crate::{ChangeHeaderCell, ChangeIndexCell, HeaderCell, IndexCell, Selected, Span, TableCell};

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

    AddSelectionBoxes {
        count: usize,
        old_selected: Option<Selected>,
    },
    RemoveSelectionBoxes {
        removed: Vec<Span>,
        old_selected: Option<Selected>,
    },
    SetSelectionBoxes {
        old_selection_boxes: Vec<Span>,
        old_selected: Option<Selected>,
    },
}

impl Change {
    // ───── Change constructors ─────
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

    // ───── Selection box constructors ─────
    /// One or more boxes were pushed to the end.
    pub fn add_selection_boxes(count: usize, old_selected: Option<Selected>) -> Self {
        Change::AddSelectionBoxes {
            count,
            old_selected,
        }
    }

    /// One or more boxes were removed (any indices).
    pub fn remove_selection_boxes(removed: Vec<Span>, old_selected: Option<Selected>) -> Self {
        Change::RemoveSelectionBoxes {
            removed,
            old_selected,
        }
    }

    pub fn set_selection_boxes(
        old_selection_boxes: Vec<Span>,
        old_selected: Option<Selected>,
    ) -> Self {
        Change::SetSelectionBoxes {
            old_selection_boxes,
            old_selected,
        }
    }

    #[inline]
    pub fn is_selection_change(&self) -> bool {
        matches!(
            self,
            Change::AddSelectionBoxes { .. }
                | Change::RemoveSelectionBoxes { .. }
                | Change::SetSelectionBoxes { .. }
        )
    }
}
