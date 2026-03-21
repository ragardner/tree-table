use crate::{DateParseOptions, str_err};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};

// ─────────────────────────────────────────────────────────────────────────────
// Date parsing limits
// ─────────────────────────────────────────────────────────────────────────────
const MAX_DATE_STRING_LEN: usize = 60;

// Supported year range (including negative/BC years)
const MIN_YEAR: i32 = -9999;
const MAX_YEAR: i32 = 9999;

/// Year range considered plausible for legacy/business YYYYDDD ordinal dates.
///
/// Used as a heuristic in "auto" mode and enforced strictly in "legacy" mode.
/// In "scientific" mode this range is ignored (JD/MJD takes priority).
const LEGACY_ORDINAL_YEAR_RANGE: std::ops::RangeInclusive<i32> = 1850..=2300;

// ─────────────────────────────────────────────────────────────────────────────
// Astronomy date systems (pure-numeric special cases)
// ─────────────────────────────────────────────────────────────────────────────

/// Modified Julian Date range for 5-digit pure-numeric input.
/// Covers ~1968–2130.
const MJD_RANGE: std::ops::RangeInclusive<i64> = 40_000..=85_000;

/// Julian Day (JD) range for 7-digit pure-numeric input.
/// Covers ~5000 BC to ~10,700 AD
const JD_RANGE: std::ops::RangeInclusive<i64> = 1_400_000..=4_000_000;

/// When to treat a pure-numeric string as a Unix timestamp
/// (seconds if ≤11 digits, milliseconds otherwise).
const MIN_TIMESTAMP_DIGITS: usize = 9;
const MAX_NUMERIC_DATE_DIGITS: usize = 19;
const TIMESTAMP_SECONDS_MAX_DIGITS: usize = 11; // 9-11 digits = seconds

// ─────────────────────────────────────────────────────────────────────────────
// Astronomical conversion constants (used in MJD/JD paths)
// ─────────────────────────────────────────────────────────────────────────────
const MJD_EPOCH_OFFSET: f64 = 40_587.0; // MJD 0 = 1858-11-17
const JD_EPOCH_OFFSET: f64 = 2_440_587.5; // JD at 1970-01-01 00:00 UTC
const SECONDS_PER_DAY: f64 = 86_400.0;

// ─────────────────────────────────────────────────────────────────────────────
// Date formats for attempted conversion
// ─────────────────────────────────────────────────────────────────────────────
const DATE_FORMATS_UNAMBIGUOUS: &[&str] = &[
    // Datetime variants first (most specific)
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M",
    "%Y-%m-%d", // ISO (still high priority)
    "%Y/%m/%d %H:%M:%S",
    "%Y/%m/%d %H:%M",
    "%Y/%m/%d",
    "%Y%m%d%H%M%S",
    "%Y%m%dT%H%M%S",
    "%Y%m%d", // year-first compact
    "%d.%m.%Y %H:%M:%S",
    "%d.%m.%Y %H:%M",
    "%d.%m.%Y",
    "%d.%m.%y",
    "%d-%m-%Y %H:%M:%S",
    "%d-%m-%Y %H:%M",
    "%d-%m-%Y",
    "%d-%m-%y",
    "%y%m%d", // YYMMDD - still useful for non-pure-numeric or 6-digit cases
    // Note: "%Y%j" and "%y%j" are now handled exclusively in the
    // pure-numeric special path for proper mode control (auto/legacy/scientific).
    // Named months (completely unambiguous)
    "%d %b %Y %H:%M:%S",
    "%d %b %Y %H:%M",
    "%d %b %Y",
    "%b %d, %Y %H:%M:%S",
    "%b %d, %Y %H:%M",
    "%b %d, %Y",
    "%d %B %Y %H:%M:%S",
    "%d %B %Y %H:%M",
    "%d %B %Y",
    "%B %d, %Y %H:%M:%S",
    "%B %d, %Y %H:%M",
    "%B %d, %Y",
];
const DATE_FORMATS_DAY_FIRST: &[&str] = &[
    // Datetime first
    "%d/%m/%Y %H:%M:%S",
    "%d/%m/%Y %H:%M",
    "%d/%m/%y %H:%M:%S",
    "%d/%m/%y %H:%M",
    "%d/%m/%Y",
    "%d/%m/%y",
    "%d%m%Y", // compact day-first (no separator)
    "%d%m%y",
];
const DATE_FORMATS_MONTH_FIRST: &[&str] = &[
    // Datetime first
    "%m/%d/%Y %H:%M:%S",
    "%m/%d/%Y %H:%M",
    "%m/%d/%y %H:%M:%S",
    "%m/%d/%y %H:%M",
    "%m/%d/%Y",
    "%m/%d/%y",
    "%m%d%Y", // compact month-first (no separator)
    "%m%d%y",
    "%m-%d-%Y", // rare US hyphen style
    "%m-%d-%y",
];

#[derive(Clone, Copy, PartialEq)]
enum DateParseMode {
    /// **Default mode** — Smart heuristic:
    /// - 5/7-digit pure-numeric inside `LEGACY_ORDINAL_YEAR_RANGE` (1850–2300) → treated as business ordinal (YYYYDDD / YYDDD)
    /// - Outside that range or invalid ordinal → treated as MJD or JD
    Auto,

    /// Business/legacy-only mode:
    /// Only accepts ordinal dates (YYYYDDD / YYDDD). No astronomy (JD/MJD) support.
    /// Strict and predictable for ERP/mainframe data.
    Legacy,

    /// Scientific / astronomy-first mode:
    /// Prioritizes MJD (5-digit) and JD (7-digit). Ordinals are only fallback.
    /// Use this when parsing data from astronomy tools or large numeric epochs.
    Scientific,
}

/// Assumed s is already trimmed by caller.
/// Returns the parsed `DateTime<Utc>` (midnight UTC if only a date was supplied)
/// or a descriptive error string via the `str_err!` macro.
pub fn try_parse_date(
    s: &str,
    opts_borrowed: &Option<DateParseOptions>,
) -> Result<DateTime<Utc>, String> {
    if s.is_empty() || s.len() > MAX_DATE_STRING_LEN {
        return Err(str_err!(
            "input string is empty or exceeds maximum length of {} characters",
            MAX_DATE_STRING_LEN
        ));
    }

    let mut mode_str = "auto";
    let mut day_first_opt: Option<bool> = None;

    if let Some(opts) = opts_borrowed {
        if let Some(m) = &opts.parse {
            mode_str = m.as_str();
        }
        day_first_opt = opts.day_first;
    }

    // ────── Explicit format path (highest priority — completely unchanged) ──────
    if mode_str.contains('%') {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, mode_str) {
            return Ok(ndt.and_utc());
        }
        if let Ok(date) = NaiveDate::parse_from_str(s, mode_str) {
            if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                return Ok(ndt.and_utc());
            }
        }
        return Err(str_err!(
            "failed to parse '{}' using explicit format '{}'",
            s,
            mode_str
        ));
    }

    // Resolve string → clean internal mode
    let mode = match mode_str {
        "legacy" => DateParseMode::Legacy,
        "scientific" => DateParseMode::Scientific,
        _ => DateParseMode::Auto, // "auto" or anything else = default
    };

    let is_pure_numeric = match classify_date_input(s) {
        Some(pure) => pure,
        None => {
            return Err(str_err!(
                "string does not look like a date (no digits or invalid characters): '{}'",
                s
            ));
        }
    };

    // ────── Pure numeric special handling (now mode-aware) ──────
    if is_pure_numeric {
        let sign_len = s.chars().take_while(|&c| c == '-' || c == '+').count();
        let digit_count = s.len() - sign_len;

        // Year-only (common to all modes)
        if (1..=4).contains(&digit_count) {
            if let Ok(year) = s.parse::<i32>() {
                if (MIN_YEAR..=MAX_YEAR).contains(&year) {
                    if let Some(ndt) =
                        NaiveDate::from_ymd_opt(year, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0))
                    {
                        return Ok(ndt.and_utc());
                    }
                }
            }
            return Err(str_err!(
                "year-only numeric value out of supported range: '{}'",
                s
            ));
        }

        // 5-digit handling
        if digit_count == 5 {
            let dt = match mode {
                DateParseMode::Legacy => parse_yyddd(s),
                DateParseMode::Scientific => parse_mjd(s).or_else(|| parse_yyddd(s)), // clean & matches 7-digit style
                DateParseMode::Auto => parse_yyddd(s).or_else(|| parse_mjd(s)),
            };
            if let Some(dt) = dt {
                return Ok(dt);
            }
        }

        // 7-digit handling
        if digit_count == 7 {
            let dt = match mode {
                DateParseMode::Legacy => parse_yyyyjjj(s),
                DateParseMode::Scientific => parse_jd(s).or_else(|| parse_yyyyjjj(s)),
                DateParseMode::Auto => parse_yyyyjjj(s).or_else(|| parse_jd(s)),
            };
            if let Some(dt) = dt {
                return Ok(dt);
            }
        }

        // Timestamp / large numeric fallback (common to all modes) ← this is perfect
        if (MIN_TIMESTAMP_DIGITS..=MAX_NUMERIC_DATE_DIGITS).contains(&digit_count) {
            if let Ok(ts) = s.parse::<i64>() {
                let millis = if digit_count <= TIMESTAMP_SECONDS_MAX_DIGITS {
                    ts.saturating_mul(1_000)
                } else {
                    ts
                };
                if let Some(dt) = DateTime::<Utc>::from_timestamp_millis(millis) {
                    return Ok(dt);
                }
                return Err(str_err!(
                    "numeric timestamp '{}' is outside the valid DateTime range",
                    ts
                ));
            }
        }

        if digit_count > MAX_NUMERIC_DATE_DIGITS {
            return Err(str_err!(
                "pure numeric input has too many digits ({} > {}): '{}'",
                digit_count,
                MAX_NUMERIC_DATE_DIGITS,
                s
            ));
        }
    }

    // ────── Format parsing — unambiguous first, then slash formats controlled by day_first ──────
    // helper
    let try_parse_fmt = |fmt: &str| -> Option<DateTime<Utc>> {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(ndt.and_utc());
        }
        if let Ok(date) = NaiveDate::parse_from_str(s, fmt) {
            return date.and_hms_opt(0, 0, 0).map(|ndt| ndt.and_utc());
        }
        None
    };
    // 1. Unambiguous formats (always tried first — no day_first dependency)
    for &fmt in DATE_FORMATS_UNAMBIGUOUS {
        if let Some(dt) = try_parse_fmt(fmt) {
            return Ok(dt);
        }
    }

    // 2. Region-dependent formats (the only ones that respect day_first_opt)
    let region_formats = match day_first_opt {
        Some(true) => DATE_FORMATS_DAY_FIRST,
        _ => DATE_FORMATS_MONTH_FIRST, // None or false → American (month-first) default
    };
    for &fmt in region_formats {
        if let Some(dt) = try_parse_fmt(fmt) {
            return Ok(dt);
        }
    }

    // RFC3339 last
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    Err(str_err!(
        "failed to parse date string '{}' with any supported format (including RFC3339)",
        s
    ))
}

/// Same parsing logic as `try_parse_date`, but returns the Unix timestamp
/// in **milliseconds** since the epoch (1970-01-01 00:00:00 UTC).
///
/// - Returns `Some(millis)` on successful parse (can be negative for pre-1970 dates).
/// - Returns `None` for any parse failure (empty string, invalid format, out-of-range, etc.).
///   No error string is returned — ideal for sorting, indexing, comparisons, etc.
///
/// This function reuses the exact same logic, modes, and options as `try_parse_date`.
pub fn try_parse_date_millis(s: &str, opts_borrowed: &Option<DateParseOptions>) -> Option<i64> {
    try_parse_date(s, opts_borrowed)
        .ok()
        .map(|dt| dt.timestamp_millis())
}

/// Returns:
///     Some(is_pure_numeric)  → string looks like a valid date, proceed
///     None                   → quick reject, not a date
#[inline(always)]
fn classify_date_input(s: &str) -> Option<bool> {
    let mut has_digit = false;
    let mut has_indicator = false;
    let mut is_pure_numeric = true;
    for &b in s.as_bytes() {
        if b.is_ascii_digit() {
            has_digit = true;
            if has_indicator {
                break;
            }
        } else if matches!(b, b'/' | b'.' | b' ' | b':' | b'T' | b',') {
            has_indicator = true;
            if has_digit {
                break;
            }
            is_pure_numeric = false;
        } else if b.is_ascii_alphabetic() {
            has_indicator = true;
            if has_digit {
                break;
            }
            is_pure_numeric = false;
        } else if b == b'-' || b == b'+' {
            has_indicator = true;
            if has_digit {
                break;
            }
        } else {
            is_pure_numeric = false;
        }
    }
    let could_be_date = has_digit && (has_indicator || (is_pure_numeric && s.len() >= 4));
    if could_be_date {
        Some(is_pure_numeric)
    } else {
        None
    }
}

/// 5-digit legacy ordinal: YYDDD (used by legacy + auto modes)
fn parse_yyddd(s: &str) -> Option<DateTime<Utc>> {
    NaiveDate::parse_from_str(s, "%y%j")
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0).map(|ndt| ndt.and_utc()))
}

/// 7-digit legacy ordinal: YYYYDDD (only accepted inside LEGACY_ORDINAL_YEAR_RANGE)
fn parse_yyyyjjj(s: &str) -> Option<DateTime<Utc>> {
    let date = NaiveDate::parse_from_str(s, "%Y%j").ok()?;
    if !LEGACY_ORDINAL_YEAR_RANGE.contains(&date.year()) {
        return None;
    }
    date.and_hms_opt(0, 0, 0).map(|ndt| ndt.and_utc())
}

/// 5-digit Modified Julian Date (MJD)
fn parse_mjd(s: &str) -> Option<DateTime<Utc>> {
    let n: i64 = s.parse().ok()?;
    if !MJD_RANGE.contains(&n) {
        return None;
    }
    let mjd = n as f64;
    let unix_seconds = (mjd - MJD_EPOCH_OFFSET) * SECONDS_PER_DAY;
    let millis = (unix_seconds * 1_000.0).round() as i64;
    DateTime::<Utc>::from_timestamp_millis(millis)
}

/// 7-digit Julian Day (JD)
fn parse_jd(s: &str) -> Option<DateTime<Utc>> {
    let n: i64 = s.parse().ok()?;
    if !JD_RANGE.contains(&n) {
        return None;
    }
    let jd = n as f64;
    let unix_seconds = (jd - JD_EPOCH_OFFSET) * SECONDS_PER_DAY;
    let millis = (unix_seconds * 1_000.0).round() as i64;
    DateTime::<Utc>::from_timestamp_millis(millis)
}
