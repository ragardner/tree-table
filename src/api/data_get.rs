use crate::CellKey;
use crate::SpanKey;
use crate::Table;
use crate::Val;
use crate::args::GetDataIidsArgs;
use crate::args::GetDataSpanArgs;
use crate::args::GetDataYxArgs;
use crate::str_err;

impl Table {
    // TODO: Maybe dont clone
    /// Gets row data using an array of row ids or positions.
    pub fn get_data_iids(
        &self,
        kwargs: Option<GetDataIidsArgs>,
    ) -> Result<Vec<(String, Vec<(String, Val)>)>, String> {
        let grid = &*self.grid.borrow();
        let empty_val = &self.opts.borrow().empty_val;
        let mut result: Vec<(String, Vec<(String, Val)>)> = Vec::new();
        match kwargs {
            Some(kwargs) => {
                if kwargs.ignore_invalid {
                    for (row_iid, cols) in kwargs.rows.into_iter() {
                        let Some(rn) = grid.index.row(&row_iid) else {
                            continue;
                        };
                        let mut row_result: Vec<(String, Val)> = Vec::new();
                        for col_iid in cols.into_iter() {
                            let Some(cn) = grid.header.col(&col_iid) else {
                                continue;
                            };
                            row_result.push((
                                col_iid,
                                Self::acr_tcell_val(&grid.index.cells, rn, cn)
                                    .unwrap_or(empty_val)
                                    .clone(),
                            ));
                        }
                        result.push((row_iid, row_result));
                    }
                } else {
                    for (row_iid, cols) in kwargs.rows.into_iter() {
                        let rn = grid
                            .index
                            .row(&row_iid)
                            .ok_or(str_err!(format!("Row iid not found: {}", row_iid)))?;
                        let mut row_result: Vec<(String, Val)> = Vec::new();
                        for col_iid in cols.into_iter() {
                            let cn = grid
                                .header
                                .col(&col_iid)
                                .ok_or(str_err!(format!("Col iid not found: {}", col_iid)))?;
                            row_result.push((
                                col_iid,
                                Self::acr_tcell_val(&grid.index.cells, rn, cn)
                                    .unwrap_or(empty_val)
                                    .clone(),
                            ));
                        }
                        result.push((row_iid, row_result));
                    }
                }
            }
            None => {
                let num_cols = Self::acr_total_cols(&grid.header.cells);
                for (rn, icell) in grid.index.cells.iter().enumerate() {
                    let mut row_result: Vec<(String, Val)> = Vec::new();
                    for cn in 0..num_cols {
                        row_result.push((
                            Self::acr_borrow_ciid(&grid.header.cells, &cn)?.clone(),
                            Self::acr_tcell_val(&grid.index.cells, &rn, &cn)
                                .unwrap_or(empty_val)
                                .clone(),
                        ))
                    }
                    result.push((icell.iid.clone(), row_result));
                }
            }
        }
        Ok(result)
    }

    /// Gets table cell data using cell coordinates.
    pub fn get_data_yx(
        &self,
        kwargs: Option<GetDataYxArgs>,
    ) -> Result<Vec<(CellKey, Val)>, String> {
        let grid = &*self.grid.borrow();
        let empty_val = &self.opts.borrow().empty_val;
        let mut result: Vec<(CellKey, Val)> = Vec::new();
        if let Some(kwargs) = kwargs {
            for key in kwargs.coords.into_iter() {
                let rn = key.0 as usize;
                let cn = key.1 as usize;
                result.push((
                    key,
                    Self::acr_tcell_val(&grid.index.cells, &rn, &cn)
                        .unwrap_or(empty_val)
                        .clone(),
                ));
            }
        } else {
            let num_rows = Self::acr_total_rows(&grid.index.cells);
            let num_cols = Self::acr_total_cols(&grid.header.cells);
            for rn in 0..num_rows {
                for cn in 0..num_cols {
                    result.push((
                        CellKey(rn as u64, cn as u64),
                        Self::acr_tcell_val(&grid.index.cells, &rn, &cn)
                            .unwrap_or(empty_val)
                            .clone(),
                    ));
                }
            }
        }
        Ok(result)
    }

    /// Gets data as a 2d array using a key.
    pub fn get_data_span(&self, kwargs: Option<GetDataSpanArgs>) -> Result<Vec<Vec<Val>>, String> {
        let kwargs = kwargs.unwrap_or(GetDataSpanArgs {
            key: SpanKey::None,
            index: true,
            header: true,
            transpose: false,
            table: true,
        });
        let grid = &*self.grid.borrow();
        let empty_val = &self.opts.borrow().empty_val;
        let mut result: Vec<Vec<Val>> = Vec::new();

        let span = self.cr_key_to_span(kwargs.key)?;
        let rows = span.rows(self);
        let cols = span.cols(self);

        // Sub-arrays are columns
        if kwargs.transpose {
            if kwargs.index {
                let mut index_row = if kwargs.header {
                    vec![empty_val.clone()]
                } else {
                    vec![]
                };
                index_row.extend(rows.clone().map(|r| {
                    Self::acr_icell(&grid.index.cells, &r)
                        .map(|cell| &cell.val)
                        .unwrap_or(empty_val)
                        .clone()
                }));
                result.push(index_row);
            }
            if kwargs.header || kwargs.table {
                for col in cols.clone() {
                    let mut col_data: Vec<Val> = Vec::new();
                    if kwargs.header {
                        col_data.push(
                            Self::acr_hcell(&grid.header.cells, &col)
                                .map(|cell| &cell.val)
                                .unwrap_or(empty_val)
                                .clone(),
                        );
                    }
                    if kwargs.table {
                        col_data.extend(rows.clone().map(|r| {
                            Self::acr_tcell_val(&grid.index.cells, &r, &col)
                                .unwrap_or(empty_val)
                                .clone()
                        }));
                    }
                    result.push(col_data);
                }
            }
        // Sub-arrays are rows
        } else {
            if kwargs.header {
                let mut header_row: Vec<Val> = if kwargs.index {
                    vec![empty_val.clone()]
                } else {
                    vec![]
                };
                header_row.extend(cols.clone().map(|c| {
                    Self::acr_hcell(&grid.header.cells, &c)
                        .map(|cell| &cell.val)
                        .unwrap_or(empty_val)
                        .clone()
                }));
                result.push(header_row);
            }
            if kwargs.index || kwargs.table {
                for row in rows.clone() {
                    let mut row_data: Vec<Val> = Vec::new();
                    if kwargs.index {
                        row_data.push(
                            Self::acr_icell(&grid.index.cells, &row)
                                .map(|cell| &cell.val)
                                .unwrap_or(empty_val)
                                .clone(),
                        );
                    }
                    if kwargs.table {
                        row_data.extend(cols.clone().map(|c| {
                            Self::acr_tcell_val(&grid.index.cells, &row, &c)
                                .unwrap_or(empty_val)
                                .clone()
                        }));
                    }
                    result.push(row_data);
                }
            }
        }
        Ok(result)
    }
}
