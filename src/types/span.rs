use crate::Coords;
use crate::SelectionKind;
use crate::Table;
use crate::utils::utils_fns::{span_a2i, span_a2n};
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SpanKey {
    Span(Coords),
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub row: usize,
    pub end_row: Option<usize>,
    pub col: usize,
    pub end_col: Option<usize>,
}

impl Span {
    pub fn rows(&self, table: &Table) -> std::ops::Range<usize> {
        let start: usize = self.row.try_into().unwrap_or(usize::MAX);
        if let Some(end) = self.end_row {
            start..end.try_into().unwrap_or(usize::MAX)
        } else {
            start..table.cr_total_rows()
        }
    }

    pub fn cols(&self, table: &Table) -> std::ops::Range<usize> {
        let start: usize = self.col.try_into().unwrap_or(usize::MAX);
        if let Some(end) = self.end_col {
            start..end.try_into().unwrap_or(usize::MAX)
        } else {
            start..table.cr_total_cols()
        }
    }

    /// Returns an index.
    pub fn max_row(&self, table: &Table) -> usize {
        if let Some(end) = self.end_row {
            end.saturating_sub(1).try_into().unwrap_or(usize::MAX)
        } else {
            table.cr_total_rows().saturating_sub(1)
        }
    }

    /// Returns an index.
    pub fn max_col(&self, table: &Table) -> usize {
        if let Some(end) = self.end_col {
            end.saturating_sub(1).try_into().unwrap_or(usize::MAX)
        } else {
            table.cr_total_cols().saturating_sub(1)
        }
    }

    pub fn coords(&self) -> Coords {
        Coords::new(self.row, self.end_row, self.col, self.end_col)
    }

    pub fn kind(&self) -> SelectionKind {
        if self.end_row.is_none() {
            SelectionKind::Cols
        } else if self.end_col.is_none() {
            SelectionKind::Rows
        } else {
            SelectionKind::Cells
        }
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
        (fr, fc, ur, uc): (Option<usize>, Option<usize>, Option<usize>, Option<usize>),
    ) -> Self {
        SpanKey::Tuple4(fr, fc, ur, uc)
    }
}

impl
    From<(
        (Option<usize>, Option<usize>),
        (Option<usize>, Option<usize>),
    )> for SpanKey
{
    fn from(
        ((fr, fc), (ur, uc)): (
            (Option<usize>, Option<usize>),
            (Option<usize>, Option<usize>),
        ),
    ) -> Self {
        SpanKey::TuplePair((fr, fc), (ur, uc))
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
            SpanKey::Span(coords) => Ok(Span {
                row: coords.row,
                end_row: coords.end_row,
                col: coords.col,
                end_col: coords.end_col,
            }),
            SpanKey::None => Ok(Span {
                row: 0,
                end_row: None,
                col: 0,
                end_col: None,
            }),
            SpanKey::Un64(row) => Ok(Span {
                row,
                end_row: Some(row + 1),
                col: 0,
                end_col: None,
            }),
            SpanKey::Tuple2(r, c) => Ok(Span {
                row: r.unwrap_or(0),
                end_row: r.map(|val| val + 1),
                col: c.unwrap_or(0),
                end_col: c.map(|val| val + 1),
            }),
            SpanKey::Tuple4(fr, ur, fc, uc) => Ok(Span {
                row: fr.unwrap_or(0),
                end_row: ur,
                col: fc.unwrap_or(0),
                end_col: uc,
            }),
            SpanKey::TuplePair((fr, ur), (fc, uc)) => Ok(Span {
                row: fr.unwrap_or(0),
                end_row: ur,
                col: fc.unwrap_or(0),
                end_col: uc,
            }),
            SpanKey::Str(s) => self.parse_string_to_span(&s),
        }
    }

    #[inline]
    fn parse_string_to_span(&self, s: &str) -> Result<Span, String> {
        if s.is_empty() {
            return Ok(Span {
                row: 0,
                end_row: None,
                col: 0,
                end_col: None,
            });
        }
        let s = s.to_uppercase();
        if let Some(m) = get_pattern_row().captures(&s) {
            let r = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse row number: {}", e))?
                - 1;
            Ok(Span {
                row: r,
                end_row: Some(r + 1),
                col: 0,
                end_col: None,
            })
        } else if let Some(m) = get_pattern_col().captures(&s) {
            let c = span_a2i(&m[1]);
            Ok(Span {
                row: 0,
                end_row: None,
                col: c,
                end_col: Some(span_a2n(&m[1])),
            })
        } else if let Some(m) = get_pattern_cell().captures(&s) {
            let c = span_a2i(&m[1]);
            let r = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse row number: {}", e))?
                - 1;
            Ok(Span {
                row: r,
                end_row: Some(r + 1),
                col: c,
                end_col: Some(c + 1),
            })
        } else if let Some(m) = get_pattern_range().captures(&s) {
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                - 1;
            let fc = span_a2i(&m[1]);
            let ur = m[4]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?;
            let uc = span_a2n(&m[3]);
            Ok(Span {
                row: fr,
                end_row: Some(ur),
                col: fc,
                end_col: Some(uc),
            })
        } else if let Some(m) = get_pattern_row_range().captures(&s) {
            let fr = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                - 1;
            let ur = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?;
            Ok(Span {
                row: fr,
                end_row: Some(ur),
                col: 0,
                end_col: None,
            })
        } else if let Some(m) = get_pattern_row_start().captures(&s) {
            let fr = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                - 1;
            Ok(Span {
                row: fr,
                end_row: None,
                col: 0,
                end_col: None,
            })
        } else if let Some(m) = get_pattern_row_end().captures(&s) {
            let ur = m[1]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?;
            Ok(Span {
                row: 0,
                end_row: Some(ur),
                col: 0,
                end_col: None,
            })
        } else if let Some(m) = get_pattern_col_range().captures(&s) {
            let fc = span_a2i(&m[1]);
            let uc = span_a2n(&m[2]);
            Ok(Span {
                row: 0,
                end_row: None,
                col: fc,
                end_col: Some(uc),
            })
        } else if let Some(m) = get_pattern_col_start().captures(&s) {
            let fc = span_a2i(&m[1]);
            Ok(Span {
                row: 0,
                end_row: None,
                col: fc,
                end_col: None,
            })
        } else if let Some(m) = get_pattern_col_end().captures(&s) {
            let uc = span_a2n(&m[1]);
            Ok(Span {
                row: 0,
                end_row: None,
                col: 0,
                end_col: Some(uc),
            })
        } else if let Some(m) = get_pattern_cell_start().captures(&s) {
            let fc = span_a2i(&m[1]);
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                - 1;
            Ok(Span {
                row: fr,
                end_row: None,
                col: fc,
                end_col: None,
            })
        } else if let Some(m) = get_pattern_cell_end().captures(&s) {
            let uc = span_a2n(&m[1]);
            let ur = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?;
            Ok(Span {
                row: 0,
                end_row: Some(ur),
                col: 0,
                end_col: Some(uc),
            })
        } else if let Some(m) = get_pattern_cell_col().captures(&s) {
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                - 1;
            let fc = span_a2i(&m[1]);
            let uc = span_a2n(&m[3]);
            Ok(Span {
                row: fr,
                end_row: None,
                col: fc,
                end_col: Some(uc),
            })
        } else if let Some(m) = get_pattern_cell_row().captures(&s) {
            let fr = m[2]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse from row: {}", e))?
                - 1;
            let fc = span_a2i(&m[1]);
            let ur = m[3]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse end row: {}", e))?;
            Ok(Span {
                row: fr,
                end_row: Some(ur),
                col: fc,
                end_col: None,
            })
        } else if get_pattern_all().is_match(&s) {
            Ok(Span {
                row: 0,
                end_row: None,
                col: 0,
                end_col: None,
            })
        } else {
            Err(format!("'{}' could not be converted to span.", s))
        }
    }
}
