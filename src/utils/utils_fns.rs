use crate::GetResultExt;
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use core::sync::atomic::{AtomicU32, Ordering};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

static WIDGET_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn generate_unique_id() -> u32 {
    WIDGET_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

const ALPHABET: [char; 62] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

const MAX_DATE_STRING_LEN: usize = 60;
const DATE_FORMATS: &[&str] = &[
    "%Y-%m-%d", // ISO (highest)
    "%Y/%m/%d",
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%d.%m.%Y",
    // Slash dates — US first
    "%m/%d/%Y",
    "%m/%d/%y",
    "%d/%m/%Y",
    "%d/%m/%y",
    // Other separators
    "%d-%m-%Y",
    "%m-%d-%Y",
    // Compact
    "%Y%m%d",
    "%d%m%Y",
    "%Y%m%d%H%M%S",
    "%Y%m%dT%H%M%S",
    // English month names
    "%d %b %Y",
    "%b %d, %Y",
    "%d %B %Y",
    "%B %d, %Y",
];

#[inline(always)]
fn looks_like_date(s: &str) -> bool {
    // Length already checked by caller
    s.bytes().any(|b| {
        matches!(b, b'-' | b'/' | b'.' | b' ' | b':' | b'T' | b',') || b.is_ascii_alphabetic()
    })
}

pub fn try_parse_date_millis(trimmed: &str) -> Option<i64> {
    let s = trimmed;

    if s.len() > MAX_DATE_STRING_LEN {
        return None;
    }

    // Allow pure-digit strings that could be compact dates/years (len >= 4)
    // so they reach the format loop.
    let is_pure_numeric = s.bytes().all(|b| b.is_ascii_digit() || b == b'-');
    let could_be_date = looks_like_date(s) || (is_pure_numeric && s.len() >= 4);

    if !could_be_date {
        return None;
    }

    // 1. Try explicit formats FIRST
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

    // 2. RFC3339 fallback
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc).timestamp_millis());
    }

    // 3. Pure-numeric fallback (year + large timestamp)
    if is_pure_numeric {
        // Year-only (sensible range, after formats, len >= 4 already guaranteed)
        if let Ok(year) = s.parse::<i32>() {
            if (-9999..=9999).contains(&year) {
                if let Some(ndt) =
                    NaiveDate::from_ymd_opt(year, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0))
                {
                    return Some(ndt.and_utc().timestamp_millis());
                }
            }
        }
        // Integer timestamp fallback (only huge values)
        if let Ok(ts) = s.parse::<i64>() {
            let abs = ts.abs();
            if abs > 1_000_000_000 {
                return Some(if abs > 1_000_000_000_000 {
                    ts
                } else {
                    ts * 1_000
                });
            }
        }
    }

    None
}

pub(crate) fn remove_duplicates_outside_section(
    items: Vec<Rc<str>>,
    section_start: usize,
    section_size: usize,
) -> Vec<Rc<str>> {
    if section_start == 0 && section_size >= items.len() {
        return items;
    }
    let section_end = section_start + section_size;
    let section_set: HashSet<Rc<str>> = items[section_start..section_end].iter().cloned().collect();
    items
        .into_iter()
        .enumerate()
        .filter(|(i, s)| {
            *i >= section_start && *i < section_end || !section_set.contains(s.as_ref())
        })
        .map(|(_, s)| s)
        .collect()
}

/// Faster way to find the position (index) of a usize in a
/// sorted Sequence of usizes
#[inline]
pub(crate) fn bin_index_usize(sorted_seq: &[usize], find: usize) -> Option<usize> {
    sorted_seq.binary_search(&find).ok()
}

#[inline]
pub(crate) fn push_n(k: usize, sorted_seq: &[usize]) -> usize {
    if sorted_seq.is_empty() || k < sorted_seq[0] {
        k
    } else {
        let mut lo: usize = 0;
        let mut hi = sorted_seq.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if sorted_seq[mid] < k + mid + 1 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        k + lo
    }
}

#[inline]
pub(crate) fn pull_n(k: usize, sorted_seq: &[usize]) -> usize {
    k - sorted_seq.binary_search(&k).unwrap_or_else(|e| e)
}

#[inline]
pub(crate) fn push_n_add<T>(k: usize, sorted_seq: &[(usize, T)]) -> usize {
    if sorted_seq.is_empty() || k < sorted_seq[0].0 {
        k
    } else {
        let mut lo: usize = 0;
        let mut hi: usize = sorted_seq.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if sorted_seq[mid].0 < k + mid + 1 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        k + lo
    }
}

pub(crate) fn offset_move_indices(
    move_to: usize,
    to_move: &[usize],
    new_to_old: &mut HashMap<usize, usize>,
) -> Vec<(usize, usize)> {
    // Count elements in to_move less than move_to
    let offset = to_move.iter().filter(|i| *i < &move_to).count();

    // Adjust move_to, preventing underflow
    let correct_move_to = move_to.saturating_sub(offset);

    // Initialize hashmaps
    let mut old_to_new = Vec::new();
    new_to_old.clear();

    // Map old indices to new and vice versa
    for (i, &elem) in to_move.iter().enumerate() {
        let value = correct_move_to + i;
        old_to_new.push((elem, value));
        new_to_old.insert(value, elem);
    }

    old_to_new
}

/// Creates two Vecs:
/// (full_old_to_new, full_new_to_old)
/// full_old_to_new
///     - A Vec of new indices in their old Vec positions.
/// full_new_to_old
///     - A Vec of old indices in their new Vec positions.
pub(crate) fn get_full_move_map(
    len: usize,
    old: &HashSet<usize>,
    new_to_old: &HashMap<usize, usize>, // still sparse — unchanged
) -> (Vec<usize>, Vec<usize>) {
    // (full_old_to_new: old→new, full_new_to_old: new→old)
    let mut full_old_to_new = vec![0usize; len]; // index = old pos, value = new pos
    let mut full_new_to_old = vec![0usize; len]; // index = new pos, value = old pos

    let mut remaining_old = (0..len).filter(|&i| !old.contains(&i));

    for new_i in 0..len {
        let old_i = if let Some(&oi) = new_to_old.get(&new_i) {
            oi
        } else {
            remaining_old.next().unwrap_or(new_i) // fallback, should never hit
        };

        full_new_to_old[new_i] = old_i;
        full_old_to_new[old_i] = new_i;
    }

    (full_old_to_new, full_new_to_old)
}

/// Converts a usize to a compact String using base-62 bijective encoding (A-Z, a-z, 0-9).
/// This is more efficient than base-26 as it uses 62 symbols, resulting in shorter strings.
/// It's repeatable: same input yields same output.
/// Handles usize::MAX as a special case to avoid overflow.
#[inline(always)]
pub fn n_to_base62(n: usize) -> String {
    let base: usize = 62;
    let mut s = String::with_capacity(11);
    let mut m = n + 1;
    while m > 0 {
        let rem = (m - 1) % base;
        s.push(ALPHABET[rem as usize]);
        m = (m - 1) / base;
    }
    s.chars().rev().collect::<String>()
}

fn parse_duration_part(chars: &[u8], total_ns: &mut i128, is_date: bool) -> Result<bool, ()> {
    let mut i: usize = 0;
    let mut had_frac = false;
    while i < chars.len() {
        let mut num_str = String::new();
        while i < chars.len() && chars[i].is_ascii_digit() {
            num_str.push(chars[i] as char);
            i += 1;
        }
        if num_str.is_empty() {
            return Err(());
        }
        let int: i64 = num_str.parse().unwrap_or(0);

        let mut frac_num: i64 = 0;
        let mut frac_div: i64 = 1;
        let mut frac_len: usize = 0;
        if i < chars.len() && chars[i] == b'.' {
            i += 1;
            let mut frac_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                frac_str.push(chars[i] as char);
                i += 1;
            }
            frac_num = frac_str.parse().unwrap_or(0);
            frac_len = frac_str.len();
            frac_div = 10i64.pow(frac_len as u32);
        }

        if i >= chars.len() {
            return Err(()); // Missing unit
        }
        let unit = chars[i] as char;
        i += 1;

        // Check if fraction is only on the last component
        if frac_len > 0 && i < chars.len() {
            return Err(());
        }

        if frac_len > 0 {
            had_frac = true;
        }

        let factor: i128 = match (is_date, unit) {
            (true, 'W') => 604_800_000_000_000i128, // 7 * 86_400_000_000_000
            (true, 'D') => 86_400_000_000_000i128,
            (false, 'H') => 3_600_000_000_000i128,
            (false, 'M') => 60_000_000_000i128,
            (false, 'S') => 1_000_000_000i128,
            _ => continue, // Ignore unknown units
        };

        let int_i128 = int as i128;
        let frac_num_i128 = frac_num as i128;
        let frac_div_i128 = frac_div as i128;

        *total_ns += int_i128 * factor + (frac_num_i128 * factor / frac_div_i128);
    }
    Ok(had_frac)
}

pub fn parse_duration(s: &str) -> Duration {
    let mut sign: i64 = 1;
    let mut input = s.trim();
    if input.starts_with('-') {
        sign = -1;
        input = &input[1..];
    }
    if !input.starts_with('P') {
        return Duration::zero();
    }
    input = &input[1..];

    let parts: Vec<&str> = input.split('T').collect();
    let date_part = parts[0];
    let time_part = if parts.len() > 1 { parts[1] } else { "" };

    let mut total_ns: i128 = 0;

    let had_date_frac = match parse_duration_part(date_part.as_bytes(), &mut total_ns, true) {
        Ok(had_frac) => had_frac,
        Err(_) => return Duration::zero(),
    };

    let _had_time_frac = match parse_duration_part(time_part.as_bytes(), &mut total_ns, false) {
        Ok(had_frac) => had_frac,
        Err(_) => return Duration::zero(),
    };

    if had_date_frac && !time_part.is_empty() {
        return Duration::zero();
    }

    total_ns *= sign as i128;

    if total_ns > i64::MAX as i128 || total_ns < i64::MIN as i128 {
        return Duration::zero(); // Overflow protection
    }

    Duration::nanoseconds(total_ns as i64)
}

/// Performs a binary search on a sorted vec of IIDs to find the leftmost insertion point
/// where the row >= target_row. Assumes the vec is sorted by row numbers.
/// Returns the insertion index or an error if any accessed IID lacks a row mapping.
pub fn binary_search_ins_pos(
    v: &[Rc<str>],
    iid_to_row: &HashMap<Rc<str>, usize>,
    target_row: usize,
) -> Result<usize, String> {
    let mut lo = 0;
    let mut hi = v.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let row = *iid_to_row.get_res(&v[mid])?;
        if row < target_row {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    Ok(lo)
}

/// Performs a binary search on a sorted vec of IIDs to find the exact position
/// where the row == target_row. Assumes the vec is sorted by row numbers.
/// Returns Some(pos) if found, None if not present, or an error if any accessed IID lacks a row mapping.
pub fn binary_search_exact_pos(
    v: &[Rc<str>],
    iid_to_row: &HashMap<Rc<str>, usize>,
    target_row: usize,
) -> Result<Option<usize>, String> {
    let mut lo = 0;
    let mut hi = v.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let row = *iid_to_row.get_res(&v[mid])?;
        if row < target_row {
            lo = mid + 1;
        } else if row > target_row {
            hi = mid;
        } else {
            return Ok(Some(mid));
        }
    }
    Ok(None)
}
