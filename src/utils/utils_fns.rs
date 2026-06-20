use crate::{GetResultExt, NaturalSegment, OrderedF64};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use hashbrown::{HashMap, HashSet};

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
            remaining_old.next().unwrap_or_else(|| new_i) // fallback, should never hit
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

#[inline(always)]
fn is_decimal_digit(c: char) -> bool {
    c.to_digit(10).is_some()
}

#[inline(always)]
pub fn natural_text_segments(s: &str) -> Vec<NaturalSegment> {
    let mut segments = Vec::with_capacity(8);
    let mut chars = s.chars().peekable();
    let mut text_buf = String::with_capacity(32);

    while let Some(c) = chars.next() {
        // 1. Signed number: "-42", "+7", "−3"
        if matches!(c, '-' | '+' | '\u{2212}')
            && chars.peek().map_or(false, |&n| is_decimal_digit(n))
        {
            if !text_buf.is_empty() {
                segments.push(NaturalSegment::Text(text_buf.to_lowercase()));
                text_buf.clear();
            }
            let negative = c == '-' || c == '\u{2212}';
            let mut value: i64 = 0;

            // Consume first digit (guaranteed by the peek)
            if let Some(d) = chars.next().and_then(|ch| ch.to_digit(10)) {
                value = d as i64;
            }
            // Remaining digits
            while let Some(&next) = chars.peek() {
                if let Some(nd) = next.to_digit(10) {
                    value = value.saturating_mul(10).saturating_add(nd as i64);
                    chars.next();
                } else {
                    break;
                }
            }
            let num = if negative { -value } else { value };
            segments.push(NaturalSegment::Number(OrderedF64(num as f64)));
            continue;
        }

        // 2. Unsigned digit run
        if let Some(d) = c.to_digit(10) {
            if !text_buf.is_empty() {
                segments.push(NaturalSegment::Text(text_buf.to_lowercase()));
                text_buf.clear();
            }
            let mut value = d as i64;
            while let Some(&next) = chars.peek() {
                if let Some(nd) = next.to_digit(10) {
                    value = value.saturating_mul(10).saturating_add(nd as i64);
                    chars.next();
                } else {
                    break;
                }
            }
            segments.push(NaturalSegment::Number(OrderedF64(value as f64)));
            continue;
        }

        // 3. Regular text
        text_buf.push(c);
    }

    if !text_buf.is_empty() {
        segments.push(NaturalSegment::Text(text_buf.to_lowercase()));
    }

    segments
}

#[inline(always)]
pub fn natural_text_segments_cs(s: &str) -> Vec<NaturalSegment> {
    let mut segments = Vec::with_capacity(8);
    let mut chars = s.chars().peekable();
    let mut text_buf = String::with_capacity(32);

    while let Some(c) = chars.next() {
        // 1. Signed number: "-42", "+7", "−3"
        if matches!(c, '-' | '+' | '\u{2212}')
            && chars.peek().map_or(false, |&n| is_decimal_digit(n))
        {
            if !text_buf.is_empty() {
                segments.push(NaturalSegment::Text(text_buf.clone()));
                text_buf.clear();
            }
            let negative = c == '-' || c == '\u{2212}';
            let mut value: i64 = 0;

            // Consume first digit (guaranteed by the peek)
            if let Some(d) = chars.next().and_then(|ch| ch.to_digit(10)) {
                value = d as i64;
            }
            // Remaining digits
            while let Some(&next) = chars.peek() {
                if let Some(nd) = next.to_digit(10) {
                    value = value.saturating_mul(10).saturating_add(nd as i64);
                    chars.next();
                } else {
                    break;
                }
            }
            let num = if negative { -value } else { value };
            segments.push(NaturalSegment::Number(OrderedF64(num as f64)));
            continue;
        }

        // 2. Unsigned digit run
        if let Some(d) = c.to_digit(10) {
            if !text_buf.is_empty() {
                segments.push(NaturalSegment::Text(text_buf.clone()));
                text_buf.clear();
            }
            let mut value = d as i64;
            while let Some(&next) = chars.peek() {
                if let Some(nd) = next.to_digit(10) {
                    value = value.saturating_mul(10).saturating_add(nd as i64);
                    chars.next();
                } else {
                    break;
                }
            }
            segments.push(NaturalSegment::Number(OrderedF64(value as f64)));
            continue;
        }

        // 3. Regular text
        text_buf.push(c);
    }

    if !text_buf.is_empty() {
        segments.push(NaturalSegment::Text(text_buf));
    }

    segments
}
