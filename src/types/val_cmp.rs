use crate::Val;
use crate::utils::utils_fns::{natural_text_segments, natural_text_segments_cs};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bigdecimal::ToPrimitive;
use bigdecimal::num_bigint::{BigInt, Sign};
use deep_time::{Dt, Lang, ParseCfg};

// ─── OrderedF64 (clean f64 ordering for BTree/Ord) ───
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OrderedF64(pub f64);

impl Eq for OrderedF64 {}
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaturalKey {
    Blank,                                // 0
    Empty,                                // 1
    Bool(bool),                           // 2
    HugeNegative(bigdecimal::BigDecimal), // 3: all huge negatives (more negative first)
    Number(OrderedF64),                   // 4: everything that fits in f64 (including Val::Dec)
    HugePositive(bigdecimal::BigDecimal), // 5: all huge positives
    Date(Dt),                             // 6
    Text(Vec<Vec<NaturalSegment>>), // 7: natural / version / fast / date text (with number extraction)
    Lexical(String), // 8: Pure Lexical / Strict String Key (classic “dumb” computer sort)
    Unknown(String), // 9
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum XKey {
    Numeric(OrderedF64), // 0 (numbers, dates, durations, huge values → ±inf)
    Text(String),        // 1 (lowercased strings)
    Bool(bool),          // 2
    Nil,                 // 99 (null/undefined — always last)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NaturalSegment {
    Text(String),
    Number(OrderedF64),
}

impl Val {
    pub fn x_key(&self) -> XKey {
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
            | Val::Dur(_) => XKey::Numeric(self.x_numeric_value().unwrap_or(0.0).into()),
            // Group 1 – text (comes after all numerics)
            Val::Str(_) | Val::Chr(_) => XKey::Text(self.x_text_value()),
            // Group 2 – bool
            Val::Bl(b) => XKey::Bool(*b),
            // Group 99 – nil/undefined
            Val::Nil(_) | Val::Undf(_) => XKey::Nil,
        }
    }

    pub fn cmp_x(&self, other: &Val) -> core::cmp::Ordering {
        self.x_key().cmp(&other.x_key())
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
            Val::Dt(dt) => Some(dt.to_f64()),
            Val::Dur(d) => Some(d.to_f64()),
            _ => None,
        }
    }

    fn make_key<F>(&self, string_to_key: F) -> NaturalKey
    where
        F: Fn(&str) -> NaturalKey,
    {
        match self {
            Val::Nil(_) | Val::Undf(_) => NaturalKey::Blank,
            Val::Bl(b) => NaturalKey::Bool(*b),
            Val::Dt(dt) => NaturalKey::Date(*dt),
            Val::Dur(d) => NaturalKey::Number(d.to_f64().into()),
            Val::Bigint(v) => bigint_key(v),
            Val::Bigdec(v) => bigdec_key(v),
            other => {
                if let Some(n) = other.x_numeric_value() {
                    NaturalKey::Number(n.into())
                } else {
                    match other {
                        Val::Str(s) => {
                            let trimmed = s.trim();
                            if trimmed.is_empty() {
                                NaturalKey::Empty
                            } else {
                                string_to_key(trimmed)
                            }
                        }
                        Val::Chr(c) if c.is_whitespace() => NaturalKey::Empty,
                        Val::Chr(c) => string_to_key(&c.to_string()),
                        _ => NaturalKey::Unknown(format!("{:?}", other)),
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn natural_key(&self) -> NaturalKey {
        self.make_key(Self::natural_key_from_string)
    }

    #[inline(always)]
    pub fn date_key(&self) -> NaturalKey {
        self.make_key(Self::date_key_from_string)
    }

    #[inline(always)]
    pub fn version_key(&self) -> NaturalKey {
        self.make_key(Self::version_key_from_string)
    }

    #[inline(always)]
    pub fn fast_key(&self) -> NaturalKey {
        self.make_key(Self::fast_key_from_string)
    }

    #[inline(always)]
    pub fn natural_key_cs(&self) -> NaturalKey {
        self.make_key(Self::natural_key_cs_from_string)
    }

    #[inline(always)]
    pub fn lexical_key(&self) -> NaturalKey {
        NaturalKey::Lexical(self.to_string().to_lowercase())
    }

    #[inline(always)]
    pub fn lexical_cs_key(&self) -> NaturalKey {
        NaturalKey::Lexical(self.to_string())
    }

    //

    #[inline(always)]
    pub fn natural_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.natural_key().cmp(&other.natural_key())
    }

    #[inline(always)]
    pub fn date_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.date_key().cmp(&other.date_key())
    }

    #[inline(always)]
    pub fn version_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.version_key().cmp(&other.version_key())
    }

    #[inline(always)]
    pub fn fast_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.fast_key().cmp(&other.fast_key())
    }

    #[inline(always)]
    pub fn natural_cs_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.natural_key_cs().cmp(&other.natural_key_cs())
    }

    #[inline(always)]
    pub fn lexical_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.lexical_key().cmp(&other.lexical_key())
    }

    #[inline(always)]
    pub fn lexical_cs_cmp(&self, other: &Val) -> core::cmp::Ordering {
        self.lexical_cs_key().cmp(&other.lexical_cs_key())
    }

    fn natural_key_from_string(trimmed: &str) -> NaturalKey {
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return NaturalKey::Number(f.into());
            }
        }
        if let Ok(dur) = Dt::from_str_duration(trimmed, Lang::En) {
            return NaturalKey::Number(dur.to_f64().into());
        }
        if let Ok(ts) = Dt::from_str_parse(trimmed, &ParseCfg::DEFAULT) {
            return NaturalKey::Date(ts);
        }
        NaturalKey::Text(natural_components(trimmed))
    }

    fn natural_key_cs_from_string(trimmed: &str) -> NaturalKey {
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return NaturalKey::Number(f.into());
            }
        }
        if let Ok(ts) = Dt::from_str_parse(trimmed, &ParseCfg::DEFAULT) {
            return NaturalKey::Date(ts);
        }
        if let Ok(dur) = Dt::from_str_duration(trimmed, Lang::En) {
            return NaturalKey::Number(dur.to_f64().into());
        }
        NaturalKey::Text(cs_natural_components(trimmed))
    }

    fn date_key_from_string(trimmed: &str) -> NaturalKey {
        if let Ok(ts) = Dt::from_str_parse(trimmed, &ParseCfg::DEFAULT) {
            return NaturalKey::Date(ts);
        }
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return NaturalKey::Number(f.into());
            }
        }
        if let Ok(dur) = Dt::from_str_duration(trimmed, Lang::En) {
            return NaturalKey::Number(dur.to_f64().into());
        }
        NaturalKey::Text(natural_components(trimmed))
    }

    fn version_key_from_string(trimmed: &str) -> NaturalKey {
        NaturalKey::Text(version_components(trimmed))
    }

    fn fast_key_from_string(trimmed: &str) -> NaturalKey {
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return NaturalKey::Number(f.into());
            }
        }
        NaturalKey::Text(natural_components(trimmed))
    }
}

fn bigint_key(v: &BigInt) -> NaturalKey {
    if let Some(f) = v.to_f64() {
        if f.is_finite() {
            return NaturalKey::Number(f.into());
        }
    }
    if v.sign() == Sign::Minus {
        NaturalKey::HugeNegative(bigdecimal::BigDecimal::from(v.clone()))
    } else {
        NaturalKey::HugePositive(bigdecimal::BigDecimal::from(v.clone()))
    }
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

#[inline(always)]
fn split_components(s: &str, separators: &[char]) -> Vec<Vec<NaturalSegment>> {
    s.split(|c| separators.contains(&c))
        .map(|comp| comp.trim())
        .filter(|comp| !comp.is_empty())
        .map(natural_text_segments)
        .collect()
}

#[inline(always)]
fn cs_split_components(s: &str, separators: &[char]) -> Vec<Vec<NaturalSegment>> {
    s.split(|c| separators.contains(&c))
        .map(|comp| comp.trim())
        .filter(|comp| !comp.is_empty())
        .map(natural_text_segments_cs)
        .collect()
}

// Used by natural_key + date_key (conservative path splitting)
#[inline(always)]
fn natural_components(s: &str) -> Vec<Vec<NaturalSegment>> {
    split_components(s, &['/', '\\', '-'])
}

#[inline(always)]
fn cs_natural_components(s: &str) -> Vec<Vec<NaturalSegment>> {
    cs_split_components(s, &['/', '\\', '-'])
}

// Used by version_key (aggressive version splitting)
#[inline(always)]
fn version_components(s: &str) -> Vec<Vec<NaturalSegment>> {
    split_components(s, &['/', '\\', '.', '_', '-'])
}
