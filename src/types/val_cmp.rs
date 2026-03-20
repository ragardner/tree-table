use crate::Val;
use crate::utils::utils_fns::try_parse_date_millis;
use bigdecimal::ToPrimitive;
use num_bigint::Sign;

// ─── OrderedF64 (clean f64 ordering for BTree/Ord) ───
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OrderedF64(pub f64);

impl Eq for OrderedF64 {}
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl From<f64> for OrderedF64 {
    fn from(f: f64) -> Self {
        OrderedF64(f)
    }
}
impl From<i64> for OrderedF64 {
    fn from(i: i64) -> Self {
        OrderedF64(i as f64)
    }
}

// ─── Sort key types ───
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaturalKey {
    Blank,                                // 0
    Empty,                                // 1
    Bool(bool),                           // 2
    HugeNegative(bigdecimal::BigDecimal), // 3: all huge negatives (more negative first)
    Number(OrderedF64),                   // 4: everything that fits in f64 (including Val::Dec)
    HugePositive(bigdecimal::BigDecimal), // 5: all huge positives
    Date(i64),                            // 6
    Text(Vec<Vec<NaturalSegment>>),       // 7
    Unknown(String),                      // 8
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaturalSegment {
    Text(String),
    Number(OrderedF64),
}

impl Val {
    /// Custom comparison for sorting mixed `Val` values (used in tables/columns).
    pub fn cmp_x(&self, other: &Val) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let p1 = self.x_sort_priority();
        let p2 = other.x_sort_priority();
        if p1 != p2 {
            return p1.cmp(&p2);
        }
        match p1 {
            0 => match (self.x_numeric_value(), other.x_numeric_value()) {
                (Some(a), Some(b)) => a.total_cmp(&b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            1 => self.x_text_value().cmp(&other.x_text_value()),
            2 => match (self, other) {
                (Val::Bl(a), Val::Bl(b)) => a.cmp(b),
                _ => Ordering::Equal,
            },
            _ => Ordering::Equal,
        }
    }

    fn x_sort_priority(&self) -> u8 {
        match self {
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

            Val::Str(_) | Val::Chr(_) => 1,
            Val::Bl(_) => 2,
            Val::Nil(_) | Val::Undf(_) => 99,
        }
    }

    fn x_text_value(&self) -> String {
        match self {
            Val::Str(s) => s.to_lowercase(),
            Val::Chr(c) => c.to_lowercase().collect(),
            _ => String::new(),
        }
    }

    /// Numeric representation for the numeric group.
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
            // Huge values stay in the numeric group for cmp_x
            Val::Bigdec(v) => v.to_f64().or_else(|| {
                Some(if v < &bigdecimal::BigDecimal::from(0) {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                })
            }),
            Val::Bigint(v) => v.to_f64().or_else(|| {
                Some(if v.sign() == Sign::Minus {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                })
            }),
            Val::Dt(dt) => Some(dt.timestamp_millis() as f64),
            Val::Dur(d) => Some(d.num_milliseconds() as f64),
            _ => None,
        }
    }

    fn make_key<F>(&self, string_to_key: F) -> NaturalKey
    where
        F: Fn(&str) -> NaturalKey,
    {
        match self {
            Val::Nil(_) | Val::Undf(_) => NaturalKey::Blank,
            Val::Str(s) if s.trim().is_empty() => NaturalKey::Empty,
            Val::Chr(c) if c.is_whitespace() => NaturalKey::Empty,
            Val::Bl(b) => NaturalKey::Bool(*b),
            Val::Dt(dt) => NaturalKey::Date(dt.timestamp_millis()),
            Val::Dur(d) => NaturalKey::Number(d.num_milliseconds().into()),
            Val::Bigint(v) => bigint_key(v),
            Val::Bigdec(v) => bigdec_key(v),
            other => {
                if let Some(n) = other.x_numeric_value() {
                    NaturalKey::Number(n.into())
                } else if let Val::Str(s) = other {
                    string_to_key(s.trim())
                } else if let Val::Chr(c) = other {
                    string_to_key(&c.to_string())
                } else {
                    NaturalKey::Unknown(format!("{:?}", other))
                }
            }
        }
    }

    pub fn natural_key(&self) -> NaturalKey {
        self.make_key(Self::natural_key_from_string)
    }

    pub fn date_key(&self) -> NaturalKey {
        self.make_key(Self::date_key_from_string)
    }

    pub fn version_key(&self) -> NaturalKey {
        self.make_key(Self::version_key_from_string)
    }

    pub fn natural_cmp(&self, other: &Val) -> std::cmp::Ordering {
        self.natural_key().cmp(&other.natural_key())
    }

    fn natural_key_from_string(trimmed: &str) -> NaturalKey {
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return NaturalKey::Number(f.into());
            }
        }
        if let Some(ts) = try_parse_date_millis(trimmed) {
            return NaturalKey::Date(ts);
        }
        NaturalKey::Text(natural_components(trimmed))
    }

    pub fn date_cmp(&self, other: &Val) -> std::cmp::Ordering {
        self.date_key().cmp(&other.date_key())
    }

    fn date_key_from_string(trimmed: &str) -> NaturalKey {
        if let Some(ts) = try_parse_date_millis(trimmed) {
            return NaturalKey::Date(ts);
        }
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return NaturalKey::Number(f.into());
            }
        }
        NaturalKey::Text(natural_components(trimmed))
    }

    pub fn version_cmp(&self, other: &Val) -> std::cmp::Ordering {
        self.version_key().cmp(&other.version_key())
    }

    fn version_key_from_string(trimmed: &str) -> NaturalKey {
        NaturalKey::Text(version_components(trimmed))
    }
}

fn bigint_key(v: &num_bigint::BigInt) -> NaturalKey {
    if let Some(f) = v.to_f64() {
        if f.is_finite() {
            return NaturalKey::Number(f.into());
        }
    }
    let bd = bigdecimal::BigDecimal::from(v.clone());
    bigdec_key(&bd)
}

fn bigdec_key(bd: &bigdecimal::BigDecimal) -> NaturalKey {
    if let Some(f) = bd.to_f64() {
        if f.is_finite() {
            return NaturalKey::Number(f.into());
        }
    }
    if bd < &bigdecimal::BigDecimal::from(0) {
        NaturalKey::HugeNegative(bd.clone())
    } else {
        NaturalKey::HugePositive(bd.clone())
    }
}

fn split_components(s: &str, separators: &[char]) -> Vec<Vec<NaturalSegment>> {
    s.split(|c| separators.contains(&c))
        .map(|comp| comp.trim())
        .filter(|comp| !comp.is_empty())
        .map(natural_text_segments)
        .collect()
}

// Used by natural_key + date_key (conservative path splitting)
fn natural_components(s: &str) -> Vec<Vec<NaturalSegment>> {
    split_components(s, &['/', '\\', '-'])
}

// Used by version_key (aggressive version splitting)
fn version_components(s: &str) -> Vec<Vec<NaturalSegment>> {
    split_components(s, &['/', '\\', '.', '_', '-'])
}

#[inline(always)]
fn is_decimal_digit(c: char) -> bool {
    c.to_digit(10).is_some()
}

fn natural_text_segments(s: &str) -> Vec<NaturalSegment> {
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

            // Consume first digit (we already know it exists from the peek)
            if let Some(digit_c) = chars.next() {
                if let Some(d) = digit_c.to_digit(10) {
                    value = d as i64;
                }
            }

            while let Some(&next) = chars.peek() {
                if let Some(d) = next.to_digit(10) {
                    value = value.saturating_mul(10).saturating_add(d as i64);
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
