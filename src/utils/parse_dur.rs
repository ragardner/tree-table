use chrono::Duration;

/// Parses either:
///   - ISO 8601 duration (P1DT2H30M45.5S, -P2W, PT1.5H, P3,5D, p1d, etc.)
///   - or legacy plain integer milliseconds
pub fn parse_duration(s: &str) -> Result<Duration, String> {
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
        if let Ok(dur) = parse_iso_duration(trimmed) {
            return Ok(dur);
        }
        // ISO failed → fall back to legacy milliseconds
    }

    // Legacy numeric path
    let ms: i64 = trimmed
        .parse()
        .map_err(|e| format!("Cannot parse '{}' as milliseconds: {}", trimmed, e))?;
    Ok(Duration::milliseconds(ms))
}

const NS_PER_WEEK: i128 = 604_800_000_000_000;
const NS_PER_DAY: i128 = 86_400_000_000_000;
const NS_PER_HOUR: i128 = 3_600_000_000_000;
const NS_PER_MINUTE: i128 = 60_000_000_000;
const NS_PER_SECOND: i128 = 1_000_000_000;

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
            (true, 'Y') | (true, 'M') => {
                return Err(
                    "Years (Y) and months (M) not supported in fixed-duration (use W/D only)"
                        .to_string(),
                );
            }
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
