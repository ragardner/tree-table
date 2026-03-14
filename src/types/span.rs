use crate::SelectionKind;
use crate::Table;
use crate::TableGrid;
use regex::Regex;
use std::sync::OnceLock;

// Define static OnceLock variables for each Regex
static PATTERN_ROW: OnceLock<Regex> = OnceLock::new();
static PATTERN_COL: OnceLock<Regex> = OnceLock::new();
static PATTERN_CELL: OnceLock<Regex> = OnceLock::new();
static PATTERN_RANGE: OnceLock<Regex> = OnceLock::new();
static PATTERN_ROW_RANGE: OnceLock<Regex> = OnceLock::new();
static PATTERN_ROW_START: OnceLock<Regex> = OnceLock::new();
static PATTERN_ROW_END: OnceLock<Regex> = OnceLock::new();
static PATTERN_COL_RANGE: OnceLock<Regex> = OnceLock::new();
static PATTERN_COL_START: OnceLock<Regex> = OnceLock::new();
static PATTERN_COL_END: OnceLock<Regex> = OnceLock::new();
static PATTERN_CELL_START: OnceLock<Regex> = OnceLock::new();
static PATTERN_CELL_END: OnceLock<Regex> = OnceLock::new();
static PATTERN_CELL_COL: OnceLock<Regex> = OnceLock::new();
static PATTERN_CELL_ROW: OnceLock<Regex> = OnceLock::new();
static PATTERN_ALL: OnceLock<Regex> = OnceLock::new();
const SPAN_COL_A_VALUE: usize = 'A' as usize;
pub const NO_END: usize = usize::MAX; // Represents "to the end of table"

/// Gets an index based on a column letter e.g. A = 0.
pub fn span_a2i(a: &str) -> usize {
    a.chars()
        .fold(0, |acc, c| acc * 26 + (c as usize - SPAN_COL_A_VALUE + 1))
        - 1
}

/// Converts a 0-based column index back to Excel-style letter(s).
/// 0 → "A", 25 → "Z", 26 → "AA", etc.
pub fn col_index_to_letter(mut col: usize) -> String {
    let mut s = String::new();
    col += 1; // treat as 1-based for lettering
    while col > 0 {
        let rem = (col - 1) % 26;
        s.push((b'A' + rem as u8) as char);
        col = (col - 1) / 26;
    }
    s.chars().rev().collect()
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SpanKey {
    Span(Span),
    None,
    Un64(usize),
    Tuple2(Option<usize>, Option<usize>),
    Tuple4(Option<usize>, Option<usize>, Option<usize>, Option<usize>),
    TuplePair(
        (Option<usize>, Option<usize>),
        (Option<usize>, Option<usize>),
    ),
    Str(String),
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span {
    pub row: usize,
    pub end_row: usize, // NO_END = to end of table
    pub col: usize,
    pub end_col: usize, // NO_END = to end of table
}

impl Span {
    pub fn new(row: usize, end_row: usize, col: usize, end_col: usize) -> Self {
        Span {
            row,
            end_row,
            col,
            end_col,
        }
    }

    /// Returns the `SelectionKind`:
    /// - `end_row == NO_END` but `end_col != NO_END` → `Rows`
    /// - `end_col == NO_END` but `end_row != NO_END` → `Cols`
    /// - anything else → `Cells`
    pub fn kind(&self) -> SelectionKind {
        match (self.end_row == NO_END, self.end_col == NO_END) {
            (true, false) => SelectionKind::Rows,
            (false, true) => SelectionKind::Cols,
            _ => SelectionKind::Cells,
        }
    }

    pub fn rows(&self, grid: &TableGrid) -> std::ops::Range<usize> {
        let start = self.row;
        if self.end_row != NO_END {
            start..self.end_row + 1
        } else {
            start..Table::acr_total_rows(&grid.index.cells)
        }
    }

    pub fn rows_start_end(&self, grid: &TableGrid) -> (usize, usize) {
        let start = self.row;
        if self.end_row != NO_END {
            (start, self.end_row)
        } else {
            (
                start,
                Table::acr_total_rows(&grid.index.cells).saturating_sub(1),
            )
        }
    }

    pub fn cols(&self, grid: &TableGrid) -> std::ops::Range<usize> {
        let start = self.col;
        if self.end_col != NO_END {
            start..self.end_col + 1
        } else {
            start..Table::acr_total_cols(&grid.header.cells)
        }
    }

    pub fn cols_start_end(&self, grid: &TableGrid) -> (usize, usize) {
        let start = self.col;
        if self.end_col != NO_END {
            (start, self.end_col)
        } else {
            (
                start,
                Table::acr_total_cols(&grid.header.cells).saturating_sub(1),
            )
        }
    }

    pub fn max_row(&self, grid: &TableGrid) -> usize {
        if self.end_row != NO_END {
            self.end_row
        } else {
            Table::acr_total_rows(&grid.index.cells).saturating_sub(1)
        }
    }

    pub fn max_col(&self, grid: &TableGrid) -> usize {
        if self.end_col != NO_END {
            self.end_col
        } else {
            Table::acr_total_cols(&grid.header.cells).saturating_sub(1)
        }
    }

    /// Converts this `Span` back into a string in exactly the formats accepted by
    /// `parse_string_to_span` (and therefore by `cr_key_to_span` when given a `SpanKey::Str`).
    ///
    /// Examples of output (all round-trip correctly):
    /// - Full table: `":"`
    /// - Single row (full width): `"5"`
    /// - Row range (full width): `"5:10"`
    /// - Row from N to end: `"5:"`
    /// - Row from start to N: `":10"`
    /// - Single column (full height): `"B"`
    /// - Column range (full height): `"B:D"`
    /// - Column from N to end: `"B:"`
    /// - Column from start to N: `":D"`
    /// - Single cell: `"B7"`
    /// - Cell range: `"A1:C5"`
    /// - Fixed row, column range: `"B5:D"` or `"B5:"`
    /// - Fixed column, row range: `"B3:8"` or `"B3:"`
    pub fn to_cr_string(&self) -> String {
        if self.row == 0 && self.end_row == NO_END && self.col == 0 && self.end_col == NO_END {
            return ":".to_string();
        }

        let col_s = col_index_to_letter(self.col);
        let col_e = if self.end_col != NO_END {
            col_index_to_letter(self.end_col)
        } else {
            String::new()
        };
        let row_s = (self.row + 1).to_string();
        let row_e = if self.end_row != NO_END {
            (self.end_row + 1).to_string()
        } else {
            String::new()
        };

        // Only columns (full height)
        if self.row == 0 && self.end_row == NO_END {
            if self.col == self.end_col && self.end_col != NO_END {
                col_s
            } else if self.end_col == NO_END {
                format!("{}:", col_s)
            } else if self.col == 0 {
                format!(":{}", col_e)
            } else {
                format!("{}:{}", col_s, col_e)
            }
        }
        // Only rows (full width)
        else if self.col == 0 && self.end_col == NO_END {
            if self.row == self.end_row && self.end_row != NO_END {
                row_s
            } else if self.end_row == NO_END {
                format!("{}:", row_s)
            } else if self.row == 0 {
                format!(":{}", row_e)
            } else {
                format!("{}:{}", row_s, row_e)
            }
        }
        // Cell / mixed selection
        else {
            let start = format!("{}{}", col_s, row_s);
            if self.row == self.end_row && self.col == self.end_col && self.end_col != NO_END {
                return start; // single cell
            }

            let end = if self.end_row == NO_END && self.end_col == NO_END {
                String::new()
            } else if self.end_row == NO_END {
                col_e
            } else if self.end_col == NO_END {
                row_e
            } else {
                format!("{}{}", col_e, row_e)
            };

            if end.is_empty() {
                format!("{}:", start)
            } else {
                format!("{}:{}", start, end)
            }
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_cr_string())
    }
}

impl Default for SpanKey {
    fn default() -> Self {
        SpanKey::None
    }
}

impl From<usize> for SpanKey {
    fn from(row: usize) -> Self {
        SpanKey::Un64(row)
    }
}

impl From<(Option<usize>, Option<usize>)> for SpanKey {
    fn from((r, c): (Option<usize>, Option<usize>)) -> Self {
        SpanKey::Tuple2(r, c)
    }
}

impl From<(usize, usize)> for SpanKey {
    fn from((r, c): (usize, usize)) -> Self {
        SpanKey::Tuple2(Some(r), Some(c))
    }
}

impl From<(Option<usize>, Option<usize>, Option<usize>, Option<usize>)> for SpanKey {
    fn from(
        (fr, ur, fc, uc): (Option<usize>, Option<usize>, Option<usize>, Option<usize>),
    ) -> Self {
        SpanKey::Tuple4(fr, ur, fc, uc)
    }
}

impl
    From<(
        (Option<usize>, Option<usize>),
        (Option<usize>, Option<usize>),
    )> for SpanKey
{
    fn from(
        ((fr, ur), (fc, uc)): (
            (Option<usize>, Option<usize>),
            (Option<usize>, Option<usize>),
        ),
    ) -> Self {
        SpanKey::TuplePair((fr, ur), (fc, uc))
    }
}

impl From<&str> for SpanKey {
    fn from(s: &str) -> Self {
        SpanKey::Str(s.to_string())
    }
}

impl From<String> for SpanKey {
    fn from(s: String) -> Self {
        SpanKey::Str(s)
    }
}

// Helper functions to get each Regex, initializing if needed
fn get_pattern_row() -> &'static Regex {
    PATTERN_ROW.get_or_init(|| Regex::new(r"^(\d+)$").expect("Invalid: PATTERN_ROW"))
}
fn get_pattern_col() -> &'static Regex {
    PATTERN_COL.get_or_init(|| Regex::new(r"^([A-Z]+)$").expect("Invalid: PATTERN_COL"))
}
fn get_pattern_cell() -> &'static Regex {
    PATTERN_CELL.get_or_init(|| Regex::new(r"^([A-Z]+)(\d+)$").expect("Invalid: PATTERN_CELL"))
}
fn get_pattern_range() -> &'static Regex {
    PATTERN_RANGE.get_or_init(|| {
        Regex::new(r"^([A-Z]+)(\d+):([A-Z]+)(\d+)$").expect("Invalid: PATTERN_RANGE")
    })
}
fn get_pattern_row_range() -> &'static Regex {
    PATTERN_ROW_RANGE
        .get_or_init(|| Regex::new(r"^(\d+):(\d+)$").expect("Invalid: PATTERN_ROW_RANGE"))
}
fn get_pattern_row_start() -> &'static Regex {
    PATTERN_ROW_START.get_or_init(|| Regex::new(r"^(\d+):$").expect("Invalid: PATTERN_ROW_START"))
}
fn get_pattern_row_end() -> &'static Regex {
    PATTERN_ROW_END.get_or_init(|| Regex::new(r"^:(\d+)$").expect("Invalid: PATTERN_ROW_END"))
}
fn get_pattern_col_range() -> &'static Regex {
    PATTERN_COL_RANGE
        .get_or_init(|| Regex::new(r"^([A-Z]+):([A-Z]+)$").expect("Invalid: PATTERN_COL_RANGE"))
}
fn get_pattern_col_start() -> &'static Regex {
    PATTERN_COL_START
        .get_or_init(|| Regex::new(r"^([A-Z]+):$").expect("Invalid: PATTERN_COL_START"))
}
fn get_pattern_col_end() -> &'static Regex {
    PATTERN_COL_END.get_or_init(|| Regex::new(r"^:([A-Z]+)$").expect("Invalid: PATTERN_COL_END"))
}
fn get_pattern_cell_start() -> &'static Regex {
    PATTERN_CELL_START
        .get_or_init(|| Regex::new(r"^([A-Z]+)(\d+):$").expect("Invalid: PATTERN_CELL_START"))
}
fn get_pattern_cell_end() -> &'static Regex {
    PATTERN_CELL_END
        .get_or_init(|| Regex::new(r"^:([A-Z]+)(\d+)$").expect("Invalid: PATTERN_CELL_END"))
}
fn get_pattern_cell_col() -> &'static Regex {
    PATTERN_CELL_COL
        .get_or_init(|| Regex::new(r"^([A-Z]+)(\d+):([A-Z]+)$").expect("Invalid: PATTERN_CELL_COL"))
}
fn get_pattern_cell_row() -> &'static Regex {
    PATTERN_CELL_ROW
        .get_or_init(|| Regex::new(r"^([A-Z]+)(\d+):(\d+)$").expect("Invalid: PATTERN_CELL_ROW"))
}
fn get_pattern_all() -> &'static Regex {
    PATTERN_ALL.get_or_init(|| Regex::new(r"^:$").expect("Invalid: PATTERN_ALL"))
}

impl Table {
    pub fn cr_key_to_span(&self, key: SpanKey) -> Result<Span, String> {
        match key {
            SpanKey::Span(coords) => Ok(coords),
            SpanKey::None => Ok(Span {
                row: 0,
                end_row: NO_END,
                col: 0,
                end_col: NO_END,
            }),
            SpanKey::Un64(row) => Ok(Span {
                row,
                end_row: row,
                col: 0,
                end_col: NO_END,
            }),
            SpanKey::Tuple2(r, c) => Ok(Span {
                row: r.unwrap_or(0),
                end_row: r.unwrap_or(NO_END),
                col: c.unwrap_or(0),
                end_col: c.unwrap_or(NO_END),
            }),
            SpanKey::Tuple4(fr, ur, fc, uc) => Ok(Span {
                row: fr.unwrap_or(0),
                end_row: ur.unwrap_or(NO_END),
                col: fc.unwrap_or(0),
                end_col: uc.unwrap_or(NO_END),
            }),
            SpanKey::TuplePair((fr, ur), (fc, uc)) => Ok(Span {
                row: fr.unwrap_or(0),
                end_row: ur.unwrap_or(NO_END),
                col: fc.unwrap_or(0),
                end_col: uc.unwrap_or(NO_END),
            }),
            SpanKey::Str(s) => self.parse_string_to_span(&s),
        }
    }

    #[inline]
    fn parse_string_to_span(&self, s: &str) -> Result<Span, String> {
        if s.is_empty() {
            return Ok(Span {
                row: 0,
                end_row: NO_END,
                col: 0,
                end_col: NO_END,
            });
        }
        let s = s.to_uppercase();
        if let Some(m) = get_pattern_row().captures(&s) {
            let r = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse row number: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: r,
                end_row: r,
                col: 0,
                end_col: NO_END,
            })
        } else if let Some(m) = get_pattern_col().captures(&s) {
            let c = span_a2i(&m[1]);
            Ok(Span {
                row: 0,
                end_row: NO_END,
                col: c,
                end_col: c,
            })
        } else if let Some(m) = get_pattern_cell().captures(&s) {
            let c = span_a2i(&m[1]);
            let r = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse row number: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: r,
                end_row: r,
                col: c,
                end_col: c,
            })
        } else if let Some(m) = get_pattern_range().captures(&s) {
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                .saturating_sub(1);
            let fc = span_a2i(&m[1]);
            let ur = m[4]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?
                .saturating_sub(1);
            let uc = span_a2i(&m[3]);
            Ok(Span {
                row: fr,
                end_row: ur,
                col: fc,
                end_col: uc,
            })
        } else if let Some(m) = get_pattern_row_range().captures(&s) {
            let fr = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                .saturating_sub(1);
            let ur = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: fr,
                end_row: ur,
                col: 0,
                end_col: NO_END,
            })
        } else if let Some(m) = get_pattern_row_start().captures(&s) {
            let fr = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: fr,
                end_row: NO_END,
                col: 0,
                end_col: NO_END,
            })
        } else if let Some(m) = get_pattern_row_end().captures(&s) {
            let ur = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: 0,
                end_row: ur,
                col: 0,
                end_col: NO_END,
            })
        } else if let Some(m) = get_pattern_col_range().captures(&s) {
            let fc = span_a2i(&m[1]);
            let uc = span_a2i(&m[2]);
            Ok(Span {
                row: 0,
                end_row: NO_END,
                col: fc,
                end_col: uc,
            })
        } else if let Some(m) = get_pattern_col_start().captures(&s) {
            let fc = span_a2i(&m[1]);
            Ok(Span {
                row: 0,
                end_row: NO_END,
                col: fc,
                end_col: NO_END,
            })
        } else if let Some(m) = get_pattern_col_end().captures(&s) {
            let uc = span_a2i(&m[1]);
            Ok(Span {
                row: 0,
                end_row: NO_END,
                col: 0,
                end_col: uc,
            })
        } else if let Some(m) = get_pattern_cell_start().captures(&s) {
            let fc = span_a2i(&m[1]);
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: fr,
                end_row: NO_END,
                col: fc,
                end_col: NO_END,
            })
        } else if let Some(m) = get_pattern_cell_end().captures(&s) {
            let uc = span_a2i(&m[1]);
            let ur = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: 0,
                end_row: ur,
                col: 0,
                end_col: uc,
            })
        } else if let Some(m) = get_pattern_cell_col().captures(&s) {
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                .saturating_sub(1);
            let fc = span_a2i(&m[1]);
            let uc = span_a2i(&m[3]);
            Ok(Span {
                row: fr,
                end_row: NO_END,
                col: fc,
                end_col: uc,
            })
        } else if let Some(m) = get_pattern_cell_row().captures(&s) {
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                .saturating_sub(1);
            let fc = span_a2i(&m[1]);
            let ur = m[3]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?
                .saturating_sub(1);
            Ok(Span {
                row: fr,
                end_row: ur,
                col: fc,
                end_col: NO_END,
            })
        } else if get_pattern_all().is_match(&s) {
            Ok(Span {
                row: 0,
                end_row: NO_END,
                col: 0,
                end_col: NO_END,
            })
        } else {
            Err(format!("'{}' could not be converted to span.", s))
        }
    }

    /// Convenience wrapper: `table.cr_span_to_string(&my_span)` → same as `my_span.to_cr_string()`.
    pub fn cr_span_to_string(&self, span: &Span) -> String {
        span.to_cr_string()
    }
}
