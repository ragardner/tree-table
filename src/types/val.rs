use alloc::string::{String, ToString};
use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::BigInt;
use core::fmt;
use deep_time::Dt;
use rust_decimal::Decimal;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
// #[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Clone, Debug)]
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
    Bigint(BigInt),
    Fl64(f64),
    Bl(bool),
    Nil(()),
    Undf(()),
    Dt(Dt),
    Dur(Dt),
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
            Val::Dt(v) => write!(f, "{}", v),
            Val::Dur(v) => write!(f, "{}", v),
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
impl From<BigInt> for Val {
    fn from(val: BigInt) -> Self {
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

// deep-time
impl From<Dt> for Val {
    fn from(val: Dt) -> Self {
        Val::Dt(val)
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
