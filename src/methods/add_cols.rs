use crate::CellProps;
use crate::Change;
use crate::EventData;
use crate::HeaderCell;
use crate::NO_END;
use crate::Span;
use crate::Table;
use crate::TableCell;
use crate::Val;
use crate::statics::DEFAULT_INHERIT;
use crate::str_err;
use std::rc::Rc;

impl Table {
    pub(crate) fn cr_add_cols(
        &mut self,
        cols: Vec<(usize, Vec<(usize, TableCell)>)>,
        header: Vec<(usize, HeaderCell)>,
        sorted_seq: Vec<usize>,
        event_data: &mut EventData,
        remake_cols: bool,
    ) {
        // insert into table data
        if !cols.is_empty() {
            self.grid.index.add_cols(Some(cols), &sorted_seq);
        }

        // push outstanding col numbers
        self.grid.push_cols(&sorted_seq);

        // insert into header data
        if !header.is_empty() {
            self.grid.header.add_cols(header);
        }

        // remake canvas relevant vars
        if remake_cols {
            Self::acr_remake_cols(
                &mut self.grid.canvas_cols,
                &mut self.grid.disp_cols,
                &self.grid.header.cells,
                self.opts.vertical_grid_line_width.clone(),
            );
        }

        // Update event_data, but not selections
        event_data.changes.push(Change::AddCols(sorted_seq));
    }

    pub(crate) fn cr_add_cols_get_args(
        &self,
        iids: Vec<String>,
        mut pos: usize,
        inherit: &str,
        vis: bool,
    ) -> Result<
        (
            Vec<(usize, Vec<(usize, TableCell)>)>,
            Vec<(usize, HeaderCell)>,
            Vec<usize>,
        ),
        String,
    > {
        let nrows = Self::acr_total_rows(&self.grid.index.cells);
        if pos > Self::acr_total_cols(&self.grid.header.cells) {
            pos = Self::acr_total_cols(&self.grid.header.cells);
        }
        let end = pos.checked_add(iids.len()).ok_or(str_err!("Max. cols"))?;
        let mut cols: Vec<(usize, Vec<(usize, TableCell)>)> = Vec::new();
        let mut header: Vec<(usize, HeaderCell)> = Vec::new();
        let mut sorted_seq: Vec<usize> = Vec::new();
        let empty_val: &Val = &self.opts.empty_val;
        let opt_inherit_col =
            self.cr_get_inherit_col(&self.grid.header.iid_to_col, pos, inherit)?;
        let do_inherit = opt_inherit_col.is_some();
        let inherit_col: usize = opt_inherit_col.unwrap_or(0);
        let header_props: Option<CellProps> = if do_inherit {
            self.grid.header.clone_props(&inherit_col)
        } else {
            None
        };
        let default_dim = self.opts.default_col_dim;

        // Create cols and header
        for (cn, iid) in (pos..end).zip(iids.into_iter()) {
            sorted_seq.push(cn);
            // Table – only columns that actually inherit any cell props (maintain sparsity)
            if do_inherit {
                let mut rows: Vec<(usize, TableCell)> = Vec::new();
                for rn in 0..nrows {
                    if let Some(props) = self.grid.index.clone_table_props(&rn, &inherit_col) {
                        rows.push((
                            rn,
                            TableCell {
                                val: empty_val.clone(),
                                props: Some(props),
                            },
                        ));
                    }
                }
                if !rows.is_empty() {
                    cols.push((cn, rows));
                }
            }
            // Header
            header.push((
                cn,
                HeaderCell {
                    val: Val::Str(iid.clone()),
                    iid: Rc::from(iid),
                    dim: default_dim,
                    vis,
                    props: header_props.clone(),
                },
            ));
        }
        Ok((cols, header, sorted_seq))
    }

    pub(crate) fn cr_ensure_widget_ncols(
        &mut self,
        max_col: usize,
        event_data: &mut EventData,
        remake_cols: bool,
    ) -> Result<usize, String> {
        let table_ncols = Self::acr_total_cols(&self.grid.header.cells);
        let cols_to_add: usize = if max_col + 1 > table_ncols {
            (max_col + 1) - table_ncols
        } else {
            0
        };
        if cols_to_add > 0 {
            let iids = (0..cols_to_add)
                .map(|_| self.grid.header.new_iid())
                .collect::<Result<Vec<String>, String>>()?;
            let cr_add_cols_pos = Self::acr_total_cols(&self.grid.header.cells);
            let (cols, header, sorted_seq) =
                self.cr_add_cols_get_args(iids, cr_add_cols_pos, DEFAULT_INHERIT, true)?;
            self.cr_add_cols(cols, header, sorted_seq, event_data, remake_cols);
        };
        Ok(cols_to_add)
    }

    pub(crate) fn cr_adjust_selections_after_col_insert(
        &mut self,
        disp_insert_pos: usize, // insertion point in DISPLAYED coordinates
        num_visible_added: usize,
    ) {
        if num_visible_added == 0 {
            return;
        }

        // We may append while iterating → only process the boxes that existed before this call
        let original_len = self.sel.selection_boxes.len();
        let mut i = 0;

        while i < original_len {
            let b = &mut self.sel.selection_boxes[i];

            if b.end_col == NO_END {
                i += 1;
                continue;
            }

            let start = b.col;
            let end = b.end_col; // inclusive

            if end < disp_insert_pos {
                // completely before insertion point → untouched
            } else if start >= disp_insert_pos {
                // completely at/after insertion point → shift whole box
                b.col += num_visible_added;
                b.end_col = b.end_col.saturating_add(num_visible_added);
            } else {
                // straddles the insertion point → split the box
                // (new columns are never selected)
                let left_end = disp_insert_pos.saturating_sub(1);

                if start <= left_end {
                    // shorten existing box to left part (inclusive)
                    b.end_col = left_end;

                    // append new box for right part (shifted)
                    let right_start = disp_insert_pos + num_visible_added;
                    let right_end = end.saturating_add(num_visible_added);

                    let new_coords = Span::new(
                        b.row,
                        b.end_row,
                        right_start,
                        right_end, // always inclusive — works for single or multi
                    );

                    self.sel.selection_boxes.push(new_coords);
                }
            }
            i += 1;
        }

        // Adjust the focused cell (the "cursor")
        // Shifts even on full-row selections
        if let Some(sel) = &mut self.sel.selected {
            if sel.col >= disp_insert_pos {
                sel.col += num_visible_added;
                sel.coords.col += num_visible_added;
                sel.coords.end_col = sel.coords.end_col.saturating_add(num_visible_added);
            }
        }
    }
}
