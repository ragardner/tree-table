use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

use crate::str_err;

/// Parses either:
///   - ISO 8601 duration (P1Y, P6M, P1Y3M2DT4H, P1DT2H30M45.5S, -P2W, PT1.5H, etc.)
///     → Years/months use common averages: 1Y = 365.25 days, 1M ≈ 30.4375 days
///   - or legacy plain integer milliseconds
pub fn try_parse_duration(s: &str) -> Result<Duration, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err("empty duration".to_string());
    }

    // Prefer ISO 8601 if it starts with P (case-insensitive)
    let is_iso_candidate = trimmed
        .chars()
        .next()
        .map_or(false, |c| c.to_ascii_lowercase() == 'p')
        || (trimmed.starts_with('-')
            && trimmed
                .chars()
                .nth(1)
                .map_or(false, |c| c.to_ascii_lowercase() == 'p'));

    if is_iso_candidate {
        match parse_iso_duration(trimmed) {
            Ok(dur) => return Ok(dur),
            Err(e) => return Err(format!("Invalid ISO 8601 duration: {}", e)),
        }
    } else {
        // Legacy numeric path
        let ms: i64 = trimmed
            .parse()
            .map_err(|e| format!("Cannot parse '{}' as milliseconds: {}", trimmed, e))?;
        Ok(Duration::milliseconds(ms))
    }
}

const NS_PER_YEAR: i128 = 31_557_600_000_000_000; // 365.25 days (common Julian/Gregorian-average approximation)
const NS_PER_MONTH: i128 = 2_629_800_000_000_000; // 30.4375 days (exactly 365.25 / 12)
const NS_PER_WEEK: i128 = 604_800_000_000_000; // 7 days
const NS_PER_DAY: i128 = 86_400_000_000_000; // 24 hours
const NS_PER_HOUR: i128 = 3_600_000_000_000; // 60 minutes
const NS_PER_MINUTE: i128 = 60_000_000_000; // 60 seconds
const NS_PER_SECOND: i128 = 1_000_000_000; // 10^9 ns

fn parse_iso_duration(s: &str) -> Result<Duration, String> {
    let mut sign: i64 = 1;
    let mut input = s;
    if input.starts_with('-') {
        sign = -1;
        input = &input[1..];
    }

    // Strip P (case-insensitive) and reject whitespace
    let input = if let Some(rest) = input.strip_prefix(|c: char| c.to_ascii_lowercase() == 'p') {
        if rest.contains(|c: char| c.is_ascii_whitespace()) {
            return Err("ISO duration may not contain whitespace".to_string());
        }
        rest
    } else {
        return Err("ISO duration must start with P or -P".to_string());
    };

    // Split on T/t (case-insensitive)
    let t_pos = input.find(|c: char| c.to_ascii_lowercase() == 't');
    let (date_part, time_part) = match t_pos {
        Some(pos) => {
            if pos == input.len() - 1 {
                return Err("T separator present but no time components follow".to_string());
            }
            if input[pos + 1..]
                .find(|c: char| c.to_ascii_lowercase() == 't')
                .is_some()
            {
                return Err("Multiple 'T' separators not allowed".to_string());
            }
            (&input[..pos], &input[pos + 1..])
        }
        None => (input, ""),
    };

    let mut total_ns: i128 = 0;
    let mut has_fraction = false;

    parse_duration_part(date_part.as_bytes(), &mut total_ns, true, &mut has_fraction)?;
    parse_duration_part(
        time_part.as_bytes(),
        &mut total_ns,
        false,
        &mut has_fraction,
    )?;

    total_ns *= sign as i128;

    if total_ns > i64::MAX as i128 || total_ns < i64::MIN as i128 {
        return Err("Duration value out of range (max ~292 years)".to_string());
    }

    Ok(Duration::nanoseconds(total_ns as i64))
}

fn parse_duration_part(
    chars: &[u8],
    total_ns: &mut i128,
    is_date: bool,
    has_fraction: &mut bool,
) -> Result<(), String> {
    let mut i = 0;
    while i < chars.len() {
        if *has_fraction {
            return Err("No components allowed after a fractional component".to_string());
        }

        // Parse integer
        let start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        if start == i {
            return Err("Expected a number".to_string());
        }
        let int: i64 = std::str::from_utf8(&chars[start..i])
            .unwrap()
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;

        // Optional fraction (`.` or `,`)
        let mut frac_num: i64 = 0;
        let mut frac_digits: usize = 0;
        if i < chars.len() && matches!(chars[i], b'.' | b',') {
            i += 1;
            let frac_start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            frac_digits = i - frac_start;
            if frac_digits == 0 {
                return Err("Empty fraction after decimal/comma".to_string());
            }
            if frac_digits > 9 {
                return Err("Too many decimal places (max 9 for nanosecond precision)".to_string());
            }
            frac_num = std::str::from_utf8(&chars[frac_start..i])
                .unwrap()
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
        }

        if i >= chars.len() {
            return Err("Missing unit after number".to_string());
        }

        let unit = (chars[i] as char).to_ascii_uppercase();
        i += 1;

        let factor = match (is_date, unit) {
            (true, 'Y') => NS_PER_YEAR,
            (true, 'M') => NS_PER_MONTH,
            (true, 'W') => NS_PER_WEEK,
            (true, 'D') => NS_PER_DAY,
            (false, 'H') => NS_PER_HOUR,
            (false, 'M') => NS_PER_MINUTE,
            (false, 'S') => NS_PER_SECOND,
            _ => return Err(format!("Unknown duration unit: {}", unit)),
        };

        let int_i128 = int as i128;
        let frac_contrib = if frac_digits > 0 {
            let frac_div = 10i64.pow(frac_digits as u32) as i128;
            (frac_num as i128 * factor) / frac_div
        } else {
            0
        };

        *total_ns += int_i128 * factor + frac_contrib;

        if frac_digits > 0 {
            *has_fraction = true;
        }
    }
    Ok(())
}

const MAX_DATE_STRING_LEN: usize = 60;
const DATE_FORMATS: &[&str] = &[
    "%Y-%m-%d", // ISO (highest)
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y/%m/%d",
    "%Y%m%d",
    "%Y%m%d%H%M%S",
    "%Y%m%dT%H%M%S",
    // Slash & dot dates
    "%d.%m.%Y",
    "%m/%d/%Y",
    "%m/%d/%y",
    "%d/%m/%Y",
    "%d/%m/%y",
    "%d-%m-%Y",
    "%m-%d-%Y",
    // Compact (8-digit → 6-digit → 5-digit → 7-digit)
    "%d%m%Y",
    "%y%m%d", // YYMMDD
    "%m%d%y", // MMDDYY
    "%d%m%y", // DDMMYY
    "%y%j",   // YYDDD (5-digit legacy ordinal date)
    "%Y%j",   // YYYYDDD ordinal date
    // English month names
    "%d %b %Y",
    "%b %d, %Y",
    "%d %B %Y",
    "%B %d, %Y",
];

/// Assumed s is already trimmed by caller.
pub fn try_parse_date_i64(s: &str) -> Option<i64> {
    if s.is_empty() || s.len() > MAX_DATE_STRING_LEN {
        return None;
    }

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
    if !could_be_date {
        return None;
    }

    if is_pure_numeric {
        let sign_len = s.chars().take_while(|&c| c == '-' || c == '+').count();
        let digit_count = s.len() - sign_len;

        // Year-only: precisely 1–4 digits (including negative/BC years)
        if (1..=4).contains(&digit_count) {
            if let Ok(year) = s.parse::<i32>() {
                if (-9999..=9999).contains(&year) {
                    if let Some(ndt) =
                        NaiveDate::from_ymd_opt(year, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0))
                    {
                        return Some(ndt.and_utc().timestamp_millis());
                    }
                }
            }
        }

        // 5-digit special handling — legacy YYDDD + astronomy MJD
        if digit_count == 5 {
            // 1. Legacy YYDDD (most common in enterprise/legacy systems)
            if let Ok(date) = NaiveDate::parse_from_str(s, "%y%j") {
                if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                    return Some(ndt.and_utc().timestamp_millis());
                }
            }

            // 2. Modified Julian Date (MJD) — astronomy/scientific tables
            if let Ok(n) = s.parse::<i64>() {
                if n > 0 && (40_000..=85_000).contains(&n) {
                    // ~1968 to ~2130
                    let mjd = n as f64;
                    let unix_seconds = (mjd - 40_587.0) * 86_400.0;
                    let millis = (unix_seconds * 1_000.0).round() as i64;
                    return Some(millis);
                }
            }
        }

        // 7-digit special handling — Julian Day + YYYYDDD
        if digit_count == 7 {
            if let Ok(n) = s.parse::<i64>() {
                // 1. Julian Day (JD) — astronomy
                if n > 0 && (2_000_000..=4_000_000).contains(&n) {
                    // ~500 BC to ~10,000 AD
                    let jd = n as f64;
                    let unix_seconds = (jd - 2_440_587.5) * 86_400.0;
                    let millis = (unix_seconds * 1_000.0).round() as i64;
                    return Some(millis);
                }
                // 2. YYYYDDD ordinal fallback
                if let Ok(date) = NaiveDate::parse_from_str(s, "%Y%j") {
                    if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                        return Some(ndt.and_utc().timestamp_millis());
                    }
                }
            }
        }

        // Timestamp / large astronomical handling
        if (9..=19).contains(&digit_count) {
            if let Ok(ts) = s.parse::<i64>() {
                if digit_count <= 11 {
                    return Some(ts.saturating_mul(1_000));
                } else {
                    return Some(ts);
                }
            }
        }

        if digit_count > 19 {
            return None;
        }
    }

    // Try all formats
    for &fmt in DATE_FORMATS {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(ndt.and_utc().timestamp_millis());
        }
        if let Ok(date) = NaiveDate::parse_from_str(s, fmt) {
            if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                return Some(ndt.and_utc().timestamp_millis());
            }
        }
    }

    // RFC3339 fallback
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc).timestamp_millis());
    }

    None
}

/// Assumed s is already trimmed by caller.
/// Returns the parsed `DateTime<Utc>` (midnight UTC if only a date was supplied)
/// or a descriptive error string via the `str_err!` macro.
pub fn try_parse_date(s: &str) -> Result<DateTime<Utc>, String> {
    if s.is_empty() || s.len() > MAX_DATE_STRING_LEN {
        return Err(str_err!(
            "input string is empty or exceeds maximum length of {} characters",
            MAX_DATE_STRING_LEN
        ));
    }

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
    if !could_be_date {
        return Err(str_err!(
            "string does not look like a date (no digits or invalid characters): '{}'",
            s
        ));
    }

    if is_pure_numeric {
        let sign_len = s.chars().take_while(|&c| c == '-' || c == '+').count();
        let digit_count = s.len() - sign_len;

        // Year-only: precisely 1–4 digits (including negative/BC years)
        if (1..=4).contains(&digit_count) {
            if let Ok(year) = s.parse::<i32>() {
                if (-9999..=9999).contains(&year) {
                    if let Some(ndt) =
                        NaiveDate::from_ymd_opt(year, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0))
                    {
                        return Ok(ndt.and_utc());
                    }
                }
            }
            // If we got here the year was out of range
            return Err(str_err!(
                "year-only numeric value out of supported range: '{}'",
                s
            ));
        }

        // 5-digit special handling — legacy YYDDD + astronomy MJD
        if digit_count == 5 {
            // 1. Legacy YYDDD (most common in enterprise/legacy systems)
            if let Ok(date) = NaiveDate::parse_from_str(s, "%y%j") {
                if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                    return Ok(ndt.and_utc());
                }
            }

            // 2. Modified Julian Date (MJD) — astronomy/scientific tables
            if let Ok(n) = s.parse::<i64>() {
                if n > 0 && (40_000..=85_000).contains(&n) {
                    let mjd = n as f64;
                    let unix_seconds = (mjd - 40_587.0) * 86_400.0;
                    let millis = (unix_seconds * 1_000.0).round() as i64;
                    if let Some(dt) = DateTime::<Utc>::from_timestamp_millis(millis) {
                        return Ok(dt);
                    }
                    return Err(str_err!(
                        "MJD value '{}' produced a timestamp outside the supported range",
                        s
                    ));
                }
            }
        }

        // 7-digit special handling — Julian Day + YYYYDDD
        if digit_count == 7 {
            if let Ok(n) = s.parse::<i64>() {
                // 1. Julian Day (JD) — astronomy
                if n > 0 && (2_000_000..=4_000_000).contains(&n) {
                    let jd = n as f64;
                    let unix_seconds = (jd - 2_440_587.5) * 86_400.0;
                    let millis = (unix_seconds * 1_000.0).round() as i64;
                    if let Some(dt) = DateTime::<Utc>::from_timestamp_millis(millis) {
                        return Ok(dt);
                    }
                    return Err(str_err!(
                        "Julian Day value '{}' produced a timestamp outside the supported range",
                        s
                    ));
                }
                // 2. YYYYDDD ordinal fallback
                if let Ok(date) = NaiveDate::parse_from_str(s, "%Y%j") {
                    if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                        return Ok(ndt.and_utc());
                    }
                }
            }
        }

        // Timestamp / large astronomical handling
        if (9..=19).contains(&digit_count) {
            if let Ok(ts) = s.parse::<i64>() {
                let millis = if digit_count <= 11 {
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

        if digit_count > 19 {
            return Err(str_err!(
                "pure numeric input has too many digits ({} > 19): '{}'",
                digit_count,
                s
            ));
        }
    }

    // Try all formats
    for &fmt in DATE_FORMATS {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Ok(ndt.and_utc());
        }
        if let Ok(date) = NaiveDate::parse_from_str(s, fmt) {
            if let Some(ndt) = date.and_hms_opt(0, 0, 0) {
                return Ok(ndt.and_utc());
            }
        }
    }

    // RFC3339 fallback
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // If we reach here, nothing matched
    Err(str_err!(
        "failed to parse date string '{}' with any supported format (including RFC3339)",
        s
    ))
}
