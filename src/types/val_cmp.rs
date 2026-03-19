use crate::Val;
use bigdecimal::ToPrimitive;

// ─── Sort key types ───
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NaturalKey {
    Blank,                          // 0: Nil / Undf
    Empty,                          // 1: empty strings
    Bool(bool),                     // 2
    Number(f64),                    // 3: all numbers + strings that parse as float
    Date(i64),                      // 4: dates + strings that parse as date
    Text(Vec<Vec<NaturalSegment>>), // 5: path-aware text (depth + per-component)
    Unknown(String),                // 6: fallback
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaturalSegment {
    Text(String),
    Number(i64),
}

// Manual total-order implementation (required because of f64)
impl Eq for NaturalKey {}

impl Ord for NaturalKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (NaturalKey::Blank, NaturalKey::Blank) => Ordering::Equal,
            (NaturalKey::Blank, _) => Ordering::Less,
            (_, NaturalKey::Blank) => Ordering::Greater,

            (NaturalKey::Empty, NaturalKey::Empty) => Ordering::Equal,
            (NaturalKey::Empty, _) => Ordering::Less,
            (_, NaturalKey::Empty) => Ordering::Greater,

            (NaturalKey::Bool(a), NaturalKey::Bool(b)) => a.cmp(b),
            (NaturalKey::Bool(_), _) => Ordering::Less,
            (_, NaturalKey::Bool(_)) => Ordering::Greater,

            (NaturalKey::Number(a), NaturalKey::Number(b)) => a.total_cmp(b),
            (NaturalKey::Number(_), _) => Ordering::Less,
            (_, NaturalKey::Number(_)) => Ordering::Greater,

            (NaturalKey::Date(a), NaturalKey::Date(b)) => a.cmp(b),
            (NaturalKey::Date(_), _) => Ordering::Less,
            (_, NaturalKey::Date(_)) => Ordering::Greater,

            // Path-aware text: depth first, then component-by-component
            (NaturalKey::Text(a), NaturalKey::Text(b)) => {
                a.len().cmp(&b.len()).then_with(|| a.cmp(b))
            }
            (NaturalKey::Text(_), _) => Ordering::Less,
            (_, NaturalKey::Text(_)) => Ordering::Greater,

            (NaturalKey::Unknown(a), NaturalKey::Unknown(b)) => a.cmp(b),
        }
    }
}

impl Val {
    /// Custom comparison for sorting mixed `Val` values (used in tables/columns).
    ///
    /// How it works:
    ///   1. Groups values by type priority first (so different kinds never mix unexpectedly)
    ///   2. Within each group, compares the actual values
    ///
    /// Priority order:
    ///   0 → Numeric (all integers/floats/decimals/big numbers + dates + durations)
    ///   1 → Text (case-insensitive)
    ///   2 → Booleans (false before true)
    ///   99 → Blanks (Nil/Undf) last
    pub fn cmp_x(&self, other: &Val) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let p1 = self.x_sort_priority();
        let p2 = other.x_sort_priority();

        if p1 != p2 {
            return p1.cmp(&p2);
        }

        // Same group → compare values inside the group
        match p1 {
            0 => {
                // Numeric group
                let na = self.x_numeric_value().unwrap();
                let nb = other.x_numeric_value().unwrap();
                na.partial_cmp(&nb).unwrap_or(Ordering::Equal)
            }
            1 => {
                // Text group
                self.x_text_value().cmp(&other.x_text_value())
            }
            2 => {
                // Boolean group
                if let (Val::Bl(a), Val::Bl(b)) = (self, other) {
                    a.cmp(b)
                } else {
                    Ordering::Equal
                }
            }
            _ => Ordering::Equal, // blanks or future types
        }
    }

    /// Internal type priority used by `cmp_x`.
    fn x_sort_priority(&self) -> u8 {
        match self {
            // 0 = everything that can be treated as a number
            Val::Un8(_)
            | Val::Un16(_)
            | Val::Un32(_)
            | Val::Un64(_)
            | Val::Un128(_)
            | Val::Sn8(_)
            | Val::Sn16(_)
            | Val::Sn32(_)
            | Val::Sn64(_)
            | Val::Sn128(_)
            | Val::Fl64(_)
            | Val::Dec(_)
            | Val::Bigdec(_)
            | Val::Bigint(_)
            | Val::Dt(_)
            | Val::Dur(_) => 0,

            // 1 = text
            Val::Str(_) | Val::Chr(_) => 1,

            // 2 = booleans
            Val::Bl(_) => 2,

            // 99 = blanks last
            Val::Nil(_) | Val::Undf(_) => 99,
        }
    }

    /// Text representation for case-insensitive comparison (only used in text group).
    fn x_text_value(&self) -> String {
        match self {
            Val::Str(s) => s.to_lowercase(),
            Val::Chr(c) => c.to_lowercase().collect(),
            _ => String::new(),
        }
    }

    /// Numeric representation for the numeric group (safe to unwrap when priority == 0).
    pub fn x_numeric_value(&self) -> Option<f64> {
        match self {
            Val::Un8(v) => Some(*v as f64),
            Val::Un16(v) => Some(*v as f64),
            Val::Un32(v) => Some(*v as f64),
            Val::Un64(v) => Some(*v as f64),
            Val::Un128(v) => Some(*v as f64),
            Val::Sn8(v) => Some(*v as f64),
            Val::Sn16(v) => Some(*v as f64),
            Val::Sn32(v) => Some(*v as f64),
            Val::Sn64(v) => Some(*v as f64),
            Val::Sn128(v) => Some(*v as f64),
            Val::Fl64(v) => Some(*v),
            Val::Dec(v) => v.to_f64(),
            Val::Bigdec(v) => v.to_f64(),
            Val::Bigint(v) => v.to_f64(),
            Val::Dt(dt) => Some(dt.timestamp_millis() as f64),
            Val::Dur(d) => Some(d.num_milliseconds() as f64),
            _ => None,
        }
    }

    /// Returns a full sort key for **natural** (human-friendly) ordering.
    ///
    /// Priority:
    ///   0. Blanks
    ///   1. Empty strings
    ///   2. Booleans
    ///   3. Numbers (incl. strings that parse as float)
    ///   4. Dates (incl. strings that parse as date)
    ///   5. Text / file paths (now with depth + per-component natural ordering)
    ///   6. Everything else
    pub fn natural_key(&self) -> NaturalKey {
        match self {
            Val::Nil(_) | Val::Undf(_) => NaturalKey::Blank,

            Val::Str(s) if s.trim().is_empty() => NaturalKey::Empty,

            Val::Bl(b) => NaturalKey::Bool(*b),

            Val::Dt(dt) => NaturalKey::Date(dt.timestamp_millis()),

            Val::Dur(d) => NaturalKey::Number(d.num_milliseconds() as f64),

            _ if self.x_numeric_value().is_some() => {
                NaturalKey::Number(self.x_numeric_value().unwrap())
            }

            Val::Str(s) => Self::natural_key_from_string(s),
            Val::Chr(c) => NaturalKey::Text(vec![natural_text_segments(&c.to_string())]),

            other => NaturalKey::Unknown(other.to_string()),
        }
    }

    /// Direct comparison (minimal allocations).
    pub fn natural_cmp(&self, other: &Val) -> std::cmp::Ordering {
        self.natural_key().cmp(&other.natural_key())
    }

    // ─── Private helpers ───
    fn natural_key_from_string(s: &str) -> NaturalKey {
        let trimmed = s.trim();

        if let Ok(f) = trimmed.parse::<f64>() {
            return NaturalKey::Number(f);
        }

        if let Some(ts) = Self::try_parse_date_millis(trimmed) {
            return NaturalKey::Date(ts);
        }

        NaturalKey::Text(natural_path_key(s))
    }

    fn try_parse_date_millis(s: &str) -> Option<i64> {
        let trimmed = s.trim();

        // Fast path first: pure integer timestamp (most common non-date case is text anyway)
        if let Ok(ts) = trimmed.parse::<i64>() {
            return Some(if ts.abs() > 1_000_000_000 {
                ts
            } else {
                ts * 1000
            });
        }

        // Only try RFC3339 if it wasn't a pure integer
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(trimmed) {
            return Some(dt.with_timezone(&chrono::Utc).timestamp_millis());
        }

        None
    }

    /// Date-first sorting key (exactly like your Python `date_sort_key`).
    ///
    /// For strings only:
    ///   - Try to parse as date FIRST
    ///   - Only if that fails → try as float/number
    ///   - Then fall back to path-aware natural text
    ///
    /// All other types (native numbers, dates, booleans, blanks) keep the same priority as before.
    pub fn date_key(&self) -> NaturalKey {
        match self {
            Val::Nil(_) | Val::Undf(_) => NaturalKey::Blank,

            Val::Str(s) if s.trim().is_empty() => NaturalKey::Empty,

            Val::Bl(b) => NaturalKey::Bool(*b),

            Val::Dt(dt) => NaturalKey::Date(dt.timestamp_millis()),

            Val::Dur(d) => NaturalKey::Number(d.num_milliseconds() as f64),

            // Native numbers stay in priority 3
            _ if self.x_numeric_value().is_some() => {
                NaturalKey::Number(self.x_numeric_value().unwrap())
            }

            Val::Str(s) => Self::date_key_from_string(s),
            Val::Chr(c) => NaturalKey::Text(vec![natural_text_segments(&c.to_string())]),

            other => NaturalKey::Unknown(other.to_string()),
        }
    }

    /// Direct comparison (use exactly like natural_cmp).
    pub fn date_cmp(&self, other: &Val) -> std::cmp::Ordering {
        self.date_key().cmp(&other.date_key())
    }

    // ─── Private helper for the date-first string path ───
    fn date_key_from_string(s: &str) -> NaturalKey {
        let trimmed = s.trim();

        // DATE FIRST (this is the only difference from natural_key)
        if let Some(ts) = Self::try_parse_date_millis(trimmed) {
            return NaturalKey::Date(ts);
        }

        if let Ok(f) = trimmed.parse::<f64>() {
            return NaturalKey::Number(f);
        }

        NaturalKey::Text(natural_path_key(s))
    }

    /// Version-aware sorting key — optimized for semantic versions.
    ///
    /// Priority (exactly like your Python `version_sort_key`, but improved):
    ///   0. Blanks
    ///   1. Empty strings
    ///   2. Booleans
    ///   3. Numbers (native only)
    ///   4. Dates (native only)
    ///   5. Text / versions / paths
    ///   6. Everything else
    ///
    /// For strings:
    ///   - No float parsing
    ///   - No date parsing (keeps pure focus on versions)
    ///   - Splits on BOTH path separators (/, \) AND version separators (., -, _)
    pub fn version_key(&self) -> NaturalKey {
        match self {
            Val::Nil(_) | Val::Undf(_) => NaturalKey::Blank,

            Val::Str(s) if s.trim().is_empty() => NaturalKey::Empty,

            Val::Bl(b) => NaturalKey::Bool(*b),

            Val::Dt(dt) => NaturalKey::Date(dt.timestamp_millis()),

            Val::Dur(d) => NaturalKey::Number(d.num_milliseconds() as f64),

            _ if self.x_numeric_value().is_some() => {
                NaturalKey::Number(self.x_numeric_value().unwrap())
            }

            Val::Str(s) => Self::version_key_from_string(s),
            Val::Chr(c) => NaturalKey::Text(vec![natural_text_segments(&c.to_string())]),

            other => NaturalKey::Unknown(other.to_string()),
        }
    }

    pub fn version_cmp(&self, other: &Val) -> std::cmp::Ordering {
        self.version_key().cmp(&other.version_key())
    }

    fn version_key_from_string(s: &str) -> NaturalKey {
        NaturalKey::Text(version_components_key(s))
    }
}

// ─── Path-aware splitter (depth + per-component natural segments) ───
fn natural_path_key(s: &str) -> Vec<Vec<NaturalSegment>> {
    let separators = ['/', '\\'];
    s.split(|c| separators.contains(&c))
        // ignore empty parts from trailing slashes
        .filter(|comp| !comp.is_empty())
        .map(natural_text_segments)
        .collect()
}

// ─── Dedicated version splitter ───
fn version_components_key(s: &str) -> Vec<Vec<NaturalSegment>> {
    let separators = ['/', '\\', '.', '-', '_'];
    s.split(|c| separators.contains(&c))
        .filter(|comp| !comp.is_empty())
        .map(natural_text_segments)
        .collect()
}

// ─── Natural segments for ONE component ───
fn natural_text_segments(s: &str) -> Vec<NaturalSegment> {
    let mut segments = Vec::new();
    let mut chars = s.chars().peekable();
    let mut current_text = String::new();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            if !current_text.is_empty() {
                segments.push(NaturalSegment::Text(current_text.to_lowercase()));
                current_text = String::new();
            }

            let mut num_str = String::new();
            num_str.push(c);
            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    num_str.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(num) = num_str.parse::<i64>() {
                segments.push(NaturalSegment::Number(num));
            }
        } else {
            current_text.push(c);
        }
    }

    if !current_text.is_empty() {
        segments.push(NaturalSegment::Text(current_text.to_lowercase()));
    }

    segments
}

/*
MISSING KEYS:

1. Pure Lexical / Strict String Key (lexical_key / lexical_cmp)
Highest priority — this is the classic “dumb” computer sort that everyone expects when the smart keys feel too clever.

Pure string comparison (no number splitting, no dates, no versions, no paths)
Two variants: case-insensitive and case-sensitive
Extremely common request: “just sort the text normally”

2. Fast / Lightweight Key (fast_key)
Directly from your original Python code (the missing fast_sort_key).

Ultra-fast: only basic digit splitting, no float parsing, no date parsing
Perfect for huge tables where you still want some natural ordering but maximum speed

3. Case-Sensitive Natural Key
Same logic as natural_key but respects case ("Zebra" before "apple").

4. Smart / Intelligent Key (smart_key)
One “do what I mean” key that tries strategies in order: version → date → number → natural text.
*/
