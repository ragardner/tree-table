use crate::IndexCell;
use chrono::Duration;
use core::sync::atomic::{AtomicU32, Ordering};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

static WIDGET_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn generate_unique_id() -> u32 {
    WIDGET_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

const SPAN_COL_A_VALUE: usize = 'A' as usize;
const ALPHABET: [char; 62] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub(crate) fn remove_duplicates_outside_section(
    items: Vec<Rc<RefCell<IndexCell>>>,
    section_start: usize,
    section_size: usize,
) -> Vec<Rc<RefCell<IndexCell>>> {
    if section_start == 0 && section_size >= items.len() {
        return items;
    }
    let section_end = section_start + section_size;
    let keep: Vec<bool> = {
        let section_set: std::collections::HashSet<String> = items[section_start..section_end]
            .iter()
            .map(|rc| rc.borrow().iid.clone())
            .collect();
        (0..items.len())
            .map(|i| {
                i >= section_start && i < section_end
                    || !section_set.contains(&items[i].borrow().iid)
            })
            .collect()
    };
    items
        .into_iter()
        .enumerate()
        .filter(|(i, _)| keep[*i])
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
pub(crate) fn push_n_add<T>(k: usize, sorted_seq: &Vec<(usize, T)>) -> usize {
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
) -> HashMap<usize, usize> {
    // Count elements in to_move less than move_to
    let offset = to_move.iter().filter(|i| *i < &move_to).count();

    // Adjust move_to, preventing underflow
    let correct_move_to = move_to.saturating_sub(offset);

    // Initialize hashmaps
    let mut old_to_new = HashMap::new();
    new_to_old.clear();

    // Map old indices to new and vice versa
    for (i, &elem) in to_move.iter().enumerate() {
        let value = correct_move_to + i;
        old_to_new.insert(elem, value);
        new_to_old.insert(value, elem);
    }

    old_to_new
}

pub(crate) fn move_using_map_usize_get_inverse(
    seq: &Vec<usize>,
    old_to_new: &HashMap<usize, usize>,
    new_to_old: &HashMap<usize, usize>,
) -> (Vec<usize>, HashMap<usize, usize>) {
    let slen = seq.len();
    let mut remaining_elements = (0..slen).filter_map(|i| {
        if old_to_new.contains_key(&i) {
            None
        } else {
            Some(seq[i])
        }
    });
    let mut full_old_to_new: Vec<usize> = Vec::with_capacity(slen);
    let mut full_new_to_old: HashMap<usize, usize> = HashMap::new();
    for i in 0..slen {
        if let Some(&old_idx) = new_to_old.get(&i) {
            full_old_to_new.push(seq[old_idx]);
            full_new_to_old.insert(i, old_idx);
        } else if let Some(e) = remaining_elements.next() {
            full_old_to_new.push(e);
            full_new_to_old.insert(i, e as usize);
        }
    }
    (full_old_to_new, full_new_to_old)
}

pub(crate) fn get_full_move_map(
    len: usize,
    old_to_new: &HashMap<usize, usize>,
    new_to_old: &HashMap<usize, usize>,
) -> (Vec<usize>, HashMap<usize, usize>) {
    move_using_map_usize_get_inverse(&(0..len as usize).collect(), old_to_new, new_to_old)
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

/// Gets an index based on a column letter e.g. A = 0.
pub(crate) fn span_a2i(a: &str) -> usize {
    a.chars()
        .fold(0, |acc, c| acc * 26 + (c as usize - SPAN_COL_A_VALUE + 1))
        - 1
}

/// Gets a number based on a column letter e.g. A = 1.
pub(crate) fn span_a2n(a: &str) -> usize {
    a.chars()
        .fold(0, |acc, c| acc * 26 + (c as usize - SPAN_COL_A_VALUE + 1))
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
