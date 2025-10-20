use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Utc};
use num_bigint::BigInt as NumBigInt;
use rust_decimal::Decimal;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Val {
    Str(String),
    Chr(char),
    Un8(u8),
    Un16(u16),
    Un32(u32),
    Un64(u64),
    Un128(u128),
    Sn8(i8),
    Sn16(i16),
    Sn32(i32),
    Sn64(i64),
    Sn128(i128),
    Dec(Decimal),
    Bigdec(BigDecimal),
    Bigint(NumBigInt),
    Fl64(f64),
    Bl(bool),
    Nil(()),
    Undf(()),
    Dt(DateTime<Utc>),
    Dur(Duration),
}

// Mirror non-data holding enum for Val
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValType {
    Str,
    Chr,
    Un8,
    Un16,
    Un32,
    Un64,
    Un128,
    Sn8,
    Sn16,
    Sn32,
    Sn64,
    Sn128,
    Dec,
    Bigdec,
    Bigint,
    Fl64,
    Bl,
    Nil,
    Undf,
    Dt,
    Dur,
}

impl ValType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValType::Str => "str",
            ValType::Chr => "chr",
            ValType::Un8 => "un8",
            ValType::Un16 => "un16",
            ValType::Un32 => "un32",
            ValType::Un64 => "un64",
            ValType::Un128 => "un128",
            ValType::Sn8 => "sn8",
            ValType::Sn16 => "sn16",
            ValType::Sn32 => "sn32",
            ValType::Sn64 => "sn64",
            ValType::Sn128 => "sn128",
            ValType::Dec => "dec",
            ValType::Bigdec => "bigdec",
            ValType::Bigint => "bigint",
            ValType::Fl64 => "fl64",
            ValType::Bl => "bl",
            ValType::Nil => "nil",
            ValType::Undf => "undf",
            ValType::Dt => "dt",
            ValType::Dur => "dur",
        }
    }

    pub fn type_id(&self) -> u8 {
        match self {
            ValType::Str => 0,
            ValType::Chr => 1,
            ValType::Un8 => 2,
            ValType::Un16 => 3,
            ValType::Un32 => 4,
            ValType::Un64 => 5,
            ValType::Un128 => 6,
            ValType::Sn8 => 7,
            ValType::Sn16 => 8,
            ValType::Sn32 => 9,
            ValType::Sn64 => 10,
            ValType::Sn128 => 11,
            ValType::Dec => 12,
            ValType::Bigdec => 13,
            ValType::Bigint => 14,
            ValType::Fl64 => 15,
            ValType::Bl => 16,
            ValType::Nil => 17,
            ValType::Undf => 18,
            ValType::Dt => 19,
            ValType::Dur => 20,
        }
    }
}

impl Val {
    pub fn val_type(&self) -> ValType {
        match self {
            Val::Str(_) => ValType::Str,
            Val::Chr(_) => ValType::Chr,
            Val::Un8(_) => ValType::Un8,
            Val::Un16(_) => ValType::Un16,
            Val::Un32(_) => ValType::Un32,
            Val::Un64(_) => ValType::Un64,
            Val::Un128(_) => ValType::Un128,
            Val::Sn8(_) => ValType::Sn8,
            Val::Sn16(_) => ValType::Sn16,
            Val::Sn32(_) => ValType::Sn32,
            Val::Sn64(_) => ValType::Sn64,
            Val::Sn128(_) => ValType::Sn128,
            Val::Dec(_) => ValType::Dec,
            Val::Bigdec(_) => ValType::Bigdec,
            Val::Bigint(_) => ValType::Bigint,
            Val::Fl64(_) => ValType::Fl64,
            Val::Bl(_) => ValType::Bl,
            Val::Nil(_) => ValType::Nil,
            Val::Undf(_) => ValType::Undf,
            Val::Dt(_) => ValType::Dt,
            Val::Dur(_) => ValType::Dur,
        }
    }

    pub fn type_id(&self) -> u8 {
        self.val_type().type_id()
    }
}

impl Default for Val {
    fn default() -> Self {
        Val::Nil(())
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Str(s) => write!(f, "{}", s),
            Val::Chr(c) => write!(f, "{}", c),
            Val::Un8(v) => write!(f, "{}", v),
            Val::Un16(v) => write!(f, "{}", v),
            Val::Un32(v) => write!(f, "{}", v),
            Val::Un64(v) => write!(f, "{}", v),
            Val::Un128(v) => write!(f, "{}", v),
            Val::Sn8(v) => write!(f, "{}", v),
            Val::Sn16(v) => write!(f, "{}", v),
            Val::Sn32(v) => write!(f, "{}", v),
            Val::Sn64(v) => write!(f, "{}", v),
            Val::Sn128(v) => write!(f, "{}", v),
            Val::Dec(v) => write!(f, "{}", v),
            Val::Bigdec(v) => write!(f, "{}", v),
            Val::Bigint(v) => write!(f, "{}", v),
            Val::Fl64(v) => write!(f, "{}", v),
            Val::Bl(b) => write!(f, "{}", b),
            Val::Nil(_) => write!(f, "null"),
            Val::Undf(_) => write!(f, "undefined"),
            Val::Dt(dt) => write!(f, "{}", dt.to_rfc3339()),
            Val::Dur(d) => write!(f, "{}ms", d.num_milliseconds()),
        }
    }
}

// Strings and characters
impl From<String> for Val {
    fn from(val: String) -> Self {
        Val::Str(val)
    }
}
impl From<&str> for Val {
    fn from(val: &str) -> Self {
        Val::Str(val.to_string())
    }
}
impl From<char> for Val {
    fn from(val: char) -> Self {
        Val::Chr(val)
    }
}

// Unsigned integers
impl From<u8> for Val {
    fn from(val: u8) -> Self {
        Val::Un8(val)
    }
}
impl From<u16> for Val {
    fn from(val: u16) -> Self {
        Val::Un16(val)
    }
}
impl From<u32> for Val {
    fn from(val: u32) -> Self {
        Val::Un32(val)
    }
}
impl From<u64> for Val {
    fn from(val: u64) -> Self {
        Val::Un64(val)
    }
}
impl From<u128> for Val {
    fn from(val: u128) -> Self {
        Val::Un128(val)
    }
}

// Signed integers
impl From<i8> for Val {
    fn from(val: i8) -> Self {
        Val::Sn8(val)
    }
}
impl From<i16> for Val {
    fn from(val: i16) -> Self {
        Val::Sn16(val)
    }
}
impl From<i32> for Val {
    fn from(val: i32) -> Self {
        Val::Sn32(val)
    }
}
impl From<i64> for Val {
    fn from(val: i64) -> Self {
        Val::Sn64(val)
    }
}
impl From<i128> for Val {
    fn from(val: i128) -> Self {
        Val::Sn128(val)
    }
}

// Other Numbers
impl From<Decimal> for Val {
    fn from(val: Decimal) -> Self {
        Val::Dec(val)
    }
}
impl From<BigDecimal> for Val {
    fn from(val: BigDecimal) -> Self {
        Val::Bigdec(val)
    }
}
impl From<NumBigInt> for Val {
    fn from(val: NumBigInt) -> Self {
        Val::Bigint(val)
    }
}
impl From<f64> for Val {
    fn from(val: f64) -> Self {
        Val::Fl64(val)
    }
}

// Boolean
impl From<bool> for Val {
    fn from(val: bool) -> Self {
        Val::Bl(val)
    }
}

// chrono
impl From<DateTime<Utc>> for Val {
    fn from(val: DateTime<Utc>) -> Self {
        Val::Dt(val)
    }
}
impl From<Duration> for Val {
    fn from(val: Duration) -> Self {
        Val::Dur(val)
    }
}

// Option
impl<T> From<Option<T>> for Val
where
    T: Into<Val>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => val.into(),
            None => Val::Nil(()),
        }
    }
}
