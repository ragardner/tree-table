use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::{BigDecimal, FromPrimitive};
use core::str::FromStr;
use deep_time::{Dt, Lang, ParseCfg};
use rust_decimal::Decimal;

use crate::Val;
use crate::str_err;
use crate::utils::parse_bool::try_parse_bool;

#[cfg(feature = "serde")]
use crate::statics::get_true;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Str {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
        max_length: Option<u64>,
    },
    Chr {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Un8 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Un16 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Un32 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Un64 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Un128 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Sn8 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Sn16 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Sn32 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Sn64 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Sn128 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Fl64 {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
        decimals: Option<u64>,
    },
    Dec {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
        decimals: Option<u64>,
    },
    Bigdec {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
        decimals: Option<u64>,
    },
    Bigint {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Bl {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Nil {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Undf {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
    },
    Dt {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
        display_format: Option<String>,
        parse_options: Option<ParseCfg>,
    },
    Dur {
        invalid_display: Option<String>,
        #[cfg_attr(feature = "serde", serde(default = "get_true"))]
        nullable: bool,
        display_unit: Option<String>,
    },
}

impl DataFormat {
    /// Tries to format a borrowed `&Val` into this `DataFormat`'s exact type.
    ///
    /// Returns `Cow::Borrowed(original)` when types already match and `normalize = false`
    /// (zero-cost fast path — no clone).
    /// Returns `Cow::Owned(new_val)` in every other success case.
    pub fn try_format_val<'a>(
        &self,
        val: &'a Val,
        normalize: bool,
    ) -> Result<Cow<'a, Val>, String> {
        if self.compatible_with(val) {
            if normalize {
                return self.apply_formatting(val).map(Cow::Owned);
            } else {
                return Ok(Cow::Borrowed(val));
            }
        }

        // Type conversion path (different type)
        let s = val.to_string();

        let new_val = match self {
            DataFormat::Str { max_length, .. } => {
                let mut res = s;
                if normalize && max_length.is_some() {
                    if let Some(max) = max_length {
                        res.truncate(*max as usize);
                    }
                }
                Val::Str(res)
            }
            DataFormat::Chr { .. } => s
                .chars()
                .next()
                .map(Val::Chr)
                .ok_or_else(|| "Cannot convert empty value to char".to_string())?,

            DataFormat::Un8 { .. } => Self::parse_numeric::<u8>(&s).map(Val::Un8)?,
            DataFormat::Un16 { .. } => Self::parse_numeric::<u16>(&s).map(Val::Un16)?,
            DataFormat::Un32 { .. } => Self::parse_numeric::<u32>(&s).map(Val::Un32)?,
            DataFormat::Un64 { .. } => Self::parse_numeric::<u64>(&s).map(Val::Un64)?,
            DataFormat::Un128 { .. } => Self::parse_numeric::<u128>(&s).map(Val::Un128)?,
            DataFormat::Sn8 { .. } => Self::parse_numeric::<i8>(&s).map(Val::Sn8)?,
            DataFormat::Sn16 { .. } => Self::parse_numeric::<i16>(&s).map(Val::Sn16)?,
            DataFormat::Sn32 { .. } => Self::parse_numeric::<i32>(&s).map(Val::Sn32)?,
            DataFormat::Sn64 { .. } => Self::parse_numeric::<i64>(&s).map(Val::Sn64)?,
            DataFormat::Sn128 { .. } => Self::parse_numeric::<i128>(&s).map(Val::Sn128)?,

            DataFormat::Fl64 { decimals, .. } => {
                let rounding = if normalize { *decimals } else { None };
                Self::parse_fl64(&s, rounding)?
            }
            DataFormat::Dec { decimals, .. } => {
                let rounding = if normalize { *decimals } else { None };
                Self::parse_dec(&s, rounding)?
            }
            DataFormat::Bigdec { decimals, .. } => {
                let rounding = if normalize { *decimals } else { None };
                Self::parse_bigdec(&s, rounding)?
            }
            DataFormat::Bigint { .. } => BigInt::from_str(s.trim())
                .map(Val::Bigint)
                .map_err(|e| str_err!("Cannot parse '{}' as BigInt: {}", s, e))?,

            DataFormat::Bl { .. } => Val::Bl(try_parse_bool(&s)?),
            DataFormat::Dt { parse_options, .. } => {
                let opts = match parse_options {
                    Some(po) => po,
                    None => &ParseCfg::DEFAULT,
                };
                Dt::from_str_parse(&s.trim(), opts)
                    .map(Val::Dt)
                    .map_err(|e| str_err!("Cannot convert '{}' to Date: {}", s, e))?
            }
            DataFormat::Dur { .. } => Dt::from_str_duration(&s.trim(), Lang::En)
                .map(Val::Dur)
                .map_err(|e| str_err!("Cannot convert '{}' to Duration: {}", s, e))?,

            DataFormat::Nil { .. } => Val::Nil(()),
            DataFormat::Undf { .. } => Val::Undf(()),
        };

        Ok(Cow::Owned(new_val))
    }

    fn apply_formatting(&self, val: &Val) -> Result<Val, String> {
        match (self, val) {
            // ────── Updated for Option<u64> ──────
            (DataFormat::Fl64 { decimals, .. }, Val::Fl64(f)) => {
                if let Some(d) = *decimals {
                    let mut rounded = *f;
                    if d == 0 {
                        rounded = rounded.round();
                    } else {
                        let factor = 10f64.powi(d as i32);
                        rounded = (rounded * factor).round() / factor;
                    }
                    Ok(Val::Fl64(rounded))
                } else {
                    Ok(Val::Fl64(*f)) // full precision
                }
            }
            (DataFormat::Dec { decimals, .. }, Val::Dec(d)) => {
                if let Some(dp) = *decimals {
                    Ok(Val::Dec(d.round_dp(dp as u32)))
                } else {
                    Ok(Val::Dec(d.clone())) // full precision
                }
            }
            (DataFormat::Bigdec { decimals, .. }, Val::Bigdec(d)) => {
                if let Some(dp) = *decimals {
                    let rounded = d.with_scale(dp as i64);
                    Ok(Val::Bigdec(rounded))
                } else {
                    Ok(Val::Bigdec(d.clone())) // full precision
                }
            }
            (DataFormat::Str { max_length, .. }, Val::Str(s)) => {
                let mut res = s.clone();
                if let Some(max) = max_length {
                    res.truncate(*max as usize);
                }
                Ok(Val::Str(res))
            }
            _ => Ok(val.clone()),
        }
    }

    fn parse_numeric<T: FromStr>(s: &str) -> Result<T, String>
    where
        <T as FromStr>::Err: core::fmt::Display,
    {
        s.trim()
            .parse::<T>()
            .map_err(|e| str_err!("Cannot parse '{}' as this numeric type: {}", s, e))
    }

    fn parse_fl64(s: &str, rounding: Option<u64>) -> Result<Val, String> {
        let mut f: f64 = s
            .trim()
            .parse()
            .map_err(|e: core::num::ParseFloatError| e.to_string())?;
        if let Some(decimals) = rounding {
            if decimals == 0 {
                f = f.round();
            } else {
                let factor = 10f64.powi(decimals as i32);
                f = (f * factor).round() / factor;
            }
        }
        // None = full precision (no rounding)
        Ok(Val::Fl64(f))
    }

    fn parse_dec(s: &str, rounding: Option<u64>) -> Result<Val, String> {
        let d = if let Ok(d) = Decimal::from_str_exact(s.trim()) {
            d
        } else if let Ok(f) = s.trim().parse::<f64>() {
            Decimal::from_f64(f)
                .ok_or_else(|| str_err!("Cannot convert f64 '{}' to exact Decimal", f))?
        } else {
            return Err(str_err!("Cannot parse '{}' as Decimal", s));
        };
        if let Some(decimals) = rounding {
            Ok(Val::Dec(d.round_dp(decimals as u32)))
        } else {
            Ok(Val::Dec(d)) // full precision
        }
    }

    fn parse_bigdec(s: &str, rounding: Option<u64>) -> Result<Val, String> {
        let mut d = BigDecimal::from_str(s.trim())
            .map_err(|e| str_err!("Cannot parse '{}' as BigDecimal: {}", s, e))?;
        if let Some(decimals) = rounding {
            d = d.with_scale(decimals as i64);
        }
        // None = full precision (no change)
        Ok(Val::Bigdec(d))
    }

    /// Returns the final string to show in the cell, clipboard, tooltip, or export.
    pub fn format_for_display(&self, val: &Val, invalid_value: &str) -> String {
        let (invalid_display, nullable) = self.common_fields();

        // ────── 1. Type mismatch (coercion failed) → treat as invalid ──────
        if !self.compatible_with(val) {
            if let Some(txt) = invalid_display {
                return txt.clone();
            }
            return invalid_value.to_string();
        }

        // ────── 2. Special / invalid values ──────
        let is_invalid_special = match val {
            Val::Nil(_) | Val::Undf(_) => true,
            Val::Str(s) if s.is_empty() && !nullable => true,
            _ => false,
        };

        if is_invalid_special {
            if let Some(txt) = invalid_display {
                return txt.clone();
            }
            if !nullable || matches!(val, Val::Nil(_) | Val::Undf(_)) {
                return invalid_value.to_string();
            }
            return "".to_string();
        }

        // ────── 3. Normal formatting — types are guaranteed to match here ──────
        match self {
            DataFormat::Str { max_length, .. } => {
                let Val::Str(s) = val else { unreachable!() };
                let mut res = s.clone();
                if let Some(max) = max_length {
                    res.truncate(*max as usize);
                }
                res
            }

            DataFormat::Fl64 { decimals, .. } => {
                let Val::Fl64(v) = val else { unreachable!() };
                if let Some(d) = *decimals {
                    Self::format_float(*v, d) // existing helper (u64)
                } else {
                    v.to_string() // full precision
                }
            }

            DataFormat::Dec { decimals, .. } => {
                let Val::Dec(v) = val else { unreachable!() };
                if let Some(d) = *decimals {
                    v.round_dp(d as u32).to_string()
                } else {
                    v.to_string() // full precision
                }
            }

            DataFormat::Bigdec { decimals, .. } => {
                let Val::Bigdec(v) = val else { unreachable!() };
                if let Some(d) = *decimals {
                    v.with_scale(d as i64).to_string()
                } else {
                    v.to_string() // full precision
                }
            }

            DataFormat::Bigint { .. } => val.to_string(),

            DataFormat::Dt { display_format, .. } => {
                let Val::Dt(dt) = val else { unreachable!() };
                if let Some(format_str) = display_format {
                    dt.to_str(format_str, Lang::En)
                        .unwrap_or_else(|_| invalid_value.to_string())
                } else {
                    dt.to_str_iso8601()
                }
            }

            DataFormat::Dur { display_unit, .. } => {
                let Val::Dur(d) = val else { unreachable!() };
                Self::format_duration(d, display_unit.as_deref())
            }

            // All other types have no special display rules
            _ => val.to_string(),
        }
    }

    #[inline(always)]
    fn format_float(mut f: f64, decimals: u64) -> String {
        if decimals == 0 {
            f = f.round();
        } else {
            let factor = 10f64.powi(decimals as i32);
            f = (f * factor).round() / factor;
        }
        format!("{:.1$}", f, decimals as usize)
    }

    #[inline(always)]
    fn format_duration(d: &Dt, unit: Option<&str>) -> String {
        let unit = unit.unwrap_or("ms");
        match unit.to_lowercase().as_str() {
            "s" => format!("{}s", d.to_sec_f()),
            "m" => format!("{}m", d.to_mins().0),
            "h" => format!("{}h", d.to_hours().0),
            "d" => format!("{}d", d.to_days().0),
            _ => format!("{}ms", d.to_ms().0),
        }
    }
}

impl DataFormat {
    pub fn string(max_length: Option<u64>, nullable: bool) -> Self {
        Self::Str {
            invalid_display: None,
            nullable,
            max_length,
        }
    }

    pub fn char(nullable: bool) -> Self {
        Self::Chr {
            invalid_display: None,
            nullable,
        }
    }

    pub fn u8(nullable: bool) -> Self {
        Self::Un8 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn u16(nullable: bool) -> Self {
        Self::Un16 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn u32(nullable: bool) -> Self {
        Self::Un32 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn u64(nullable: bool) -> Self {
        Self::Un64 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn u128(nullable: bool) -> Self {
        Self::Un128 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn i8(nullable: bool) -> Self {
        Self::Sn8 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn i16(nullable: bool) -> Self {
        Self::Sn16 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn i32(nullable: bool) -> Self {
        Self::Sn32 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn i64(nullable: bool) -> Self {
        Self::Sn64 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn i128(nullable: bool) -> Self {
        Self::Sn128 {
            invalid_display: None,
            nullable,
        }
    }

    pub fn f64(decimals: Option<u64>, nullable: bool) -> Self {
        Self::Fl64 {
            invalid_display: None,
            nullable,
            decimals,
        }
    }

    pub fn dec(decimals: Option<u64>, nullable: bool) -> Self {
        Self::Dec {
            invalid_display: None,
            nullable,
            decimals,
        }
    }

    pub fn bigdec(decimals: Option<u64>, nullable: bool) -> Self {
        Self::Bigdec {
            invalid_display: None,
            nullable,
            decimals,
        }
    }

    pub fn bigint(nullable: bool) -> Self {
        Self::Bigint {
            invalid_display: None,
            nullable,
        }
    }

    pub fn boolean(nullable: bool) -> Self {
        Self::Bl {
            invalid_display: None,
            nullable,
        }
    }

    pub fn null(nullable: bool) -> Self {
        Self::Nil {
            invalid_display: None,
            nullable,
        }
    }

    pub fn undefined(nullable: bool) -> Self {
        Self::Undf {
            invalid_display: None,
            nullable,
        }
    }

    pub fn datetime(
        display_format: Option<String>,
        parse_options: Option<ParseCfg>,
        nullable: bool,
    ) -> Self {
        Self::Dt {
            invalid_display: None,
            nullable,
            display_format,
            parse_options,
        }
    }

    pub fn duration(display_unit: Option<String>, nullable: bool) -> Self {
        Self::Dur {
            invalid_display: None,
            nullable,
            display_unit,
        }
    }

    #[inline(always)]
    pub fn compatible_with(&self, val: &Val) -> bool {
        matches!(
            (self, val),
            (DataFormat::Str { .. }, Val::Str(_))
                | (DataFormat::Chr { .. }, Val::Chr(_))
                | (DataFormat::Un8 { .. }, Val::Un8(_))
                | (DataFormat::Un16 { .. }, Val::Un16(_))
                | (DataFormat::Un32 { .. }, Val::Un32(_))
                | (DataFormat::Un64 { .. }, Val::Un64(_))
                | (DataFormat::Un128 { .. }, Val::Un128(_))
                | (DataFormat::Sn8 { .. }, Val::Sn8(_))
                | (DataFormat::Sn16 { .. }, Val::Sn16(_))
                | (DataFormat::Sn32 { .. }, Val::Sn32(_))
                | (DataFormat::Sn64 { .. }, Val::Sn64(_))
                | (DataFormat::Sn128 { .. }, Val::Sn128(_))
                | (DataFormat::Fl64 { .. }, Val::Fl64(_))
                | (DataFormat::Dec { .. }, Val::Dec(_))
                | (DataFormat::Bigdec { .. }, Val::Bigdec(_))
                | (DataFormat::Bigint { .. }, Val::Bigint(_))
                | (DataFormat::Bl { .. }, Val::Bl(_))
                | (DataFormat::Nil { .. }, Val::Nil(_))
                | (DataFormat::Undf { .. }, Val::Undf(_))
                | (DataFormat::Dt { .. }, Val::Dt(_))
                | (DataFormat::Dur { .. }, Val::Dur(_))
        )
    }

    #[inline(always)]
    pub fn common_fields(&self) -> (&Option<String>, bool) {
        match self {
            DataFormat::Str {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Chr {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Un8 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Un16 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Un32 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Un64 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Un128 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Sn8 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Sn16 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Sn32 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Sn64 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Sn128 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Fl64 {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Dec {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Bigdec {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Bigint {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Bl {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Nil {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Undf {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Dt {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
            DataFormat::Dur {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
        }
    }
}

pub fn string_to_data_format(input: &str) -> Result<DataFormat, String> {
    let lower = input.to_lowercase();

    match lower.as_str() {
        "str" => Ok(DataFormat::string(None, false)),
        "chr" => Ok(DataFormat::char(false)),
        "un8" => Ok(DataFormat::u8(false)),
        "un16" => Ok(DataFormat::u16(false)),
        "un32" => Ok(DataFormat::u32(false)),
        "un64" => Ok(DataFormat::u64(false)),
        "un128" => Ok(DataFormat::u128(false)),
        "sn8" => Ok(DataFormat::i8(false)),
        "sn16" => Ok(DataFormat::i16(false)),
        "sn32" => Ok(DataFormat::i32(false)),
        "sn64" => Ok(DataFormat::i64(false)),
        "sn128" => Ok(DataFormat::i128(false)),
        "fl64" => Ok(DataFormat::f64(None, false)),
        "dec" => Ok(DataFormat::dec(None, false)),
        "bigdec" => Ok(DataFormat::bigdec(None, false)),
        "bigint" => Ok(DataFormat::bigint(false)),
        "bl" => Ok(DataFormat::boolean(false)),
        "nil" => Ok(DataFormat::null(false)),
        "undf" => Ok(DataFormat::undefined(false)),
        "dt" => Ok(DataFormat::datetime(None, None, false)),
        "dur" => Ok(DataFormat::duration(None, false)),
        _ => Err(str_err!("Invalid data format: '{}'", input)),
    }
}
