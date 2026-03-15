use crate::Val;
use crate::ValType;
use crate::str_err;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use chrono::{DateTime, Duration, Utc};
use num_bigint::BigInt as NumBigInt;
use rust_decimal::Decimal;
use std::str::FromStr;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatBase {
    pub invalid_display: Option<String>,
    pub nullable: bool,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatStr {
    pub base: DataFormatBase,
    pub max_length: Option<u64>,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatFl64 {
    pub base: DataFormatBase,
    pub decimals: u64,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatDateTime {
    pub base: DataFormatBase,
    pub display_format: Option<String>,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatDuration {
    pub base: DataFormatBase,
    pub display_unit: Option<String>,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Str(DataFormatStr),
    Chr(DataFormatBase),
    Un8(DataFormatBase),
    Un16(DataFormatBase),
    Un32(DataFormatBase),
    Un64(DataFormatBase),
    Un128(DataFormatBase),
    Sn8(DataFormatBase),
    Sn16(DataFormatBase),
    Sn32(DataFormatBase),
    Sn64(DataFormatBase),
    Sn128(DataFormatBase),
    Fl64(DataFormatFl64),
    Dec(DataFormatFl64),
    Bigdec(DataFormatFl64),
    Bigint(DataFormatBase),
    Bl(DataFormatBase),
    Nil(DataFormatBase),
    Undf(DataFormatBase),
    Dt(DataFormatDateTime),
    Dur(DataFormatDuration),
}

impl DataFormat {
    fn default_base() -> DataFormatBase {
        DataFormatBase {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn string(max_length: Option<u64>) -> Self {
        let base = Self::default_base();
        Self::Str(DataFormatStr { base, max_length })
    }
    pub fn char() -> Self {
        let base = Self::default_base();
        Self::Chr(base)
    }
    pub fn u8() -> Self {
        let base = Self::default_base();
        Self::Un8(base)
    }
    pub fn u16() -> Self {
        let base = Self::default_base();
        Self::Un16(base)
    }
    pub fn u32() -> Self {
        let base = Self::default_base();
        Self::Un32(base)
    }
    pub fn u64() -> Self {
        let base = Self::default_base();
        Self::Un64(base)
    }
    pub fn u128() -> Self {
        let base = Self::default_base();
        Self::Un128(base)
    }
    pub fn i8() -> Self {
        let base = Self::default_base();
        Self::Sn8(base)
    }
    pub fn i16() -> Self {
        let base = Self::default_base();
        Self::Sn16(base)
    }
    pub fn i32() -> Self {
        let base = Self::default_base();
        Self::Sn32(base)
    }
    pub fn i64() -> Self {
        let base = Self::default_base();
        Self::Sn64(base)
    }
    pub fn i128() -> Self {
        let base = Self::default_base();
        Self::Sn128(base)
    }
    pub fn dec(decimals: u64) -> Self {
        let base = Self::default_base();
        Self::Dec(DataFormatFl64 { base, decimals })
    }
    pub fn bigdec(decimals: u64) -> Self {
        let base = Self::default_base();
        Self::Bigdec(DataFormatFl64 { base, decimals })
    }
    pub fn bigint() -> Self {
        let base = Self::default_base();
        Self::Bigint(base)
    }
    pub fn boolean() -> Self {
        let base = Self::default_base();
        Self::Bl(base)
    }
    pub fn null() -> Self {
        let base = Self::default_base();
        Self::Nil(base)
    }
    pub fn undefined() -> Self {
        let base = Self::default_base();
        Self::Undf(base)
    }
    pub fn f64(decimals: u64) -> Self {
        let base = Self::default_base();
        Self::Fl64(DataFormatFl64 { base, decimals })
    }
    pub fn datetime(display_format: Option<String>) -> Self {
        let base = Self::default_base();
        Self::Dt(DataFormatDateTime {
            base,
            display_format,
        })
    }
    pub fn duration(display_unit: Option<String>) -> Self {
        let base = Self::default_base();
        Self::Dur(DataFormatDuration { base, display_unit })
    }

    pub fn val_type(&self) -> ValType {
        match self {
            DataFormat::Str { .. } => ValType::Str,
            DataFormat::Chr { .. } => ValType::Chr,
            DataFormat::Un8 { .. } => ValType::Un8,
            DataFormat::Un16 { .. } => ValType::Un16,
            DataFormat::Un32 { .. } => ValType::Un32,
            DataFormat::Un64 { .. } => ValType::Un64,
            DataFormat::Un128 { .. } => ValType::Un128,
            DataFormat::Sn8 { .. } => ValType::Sn8,
            DataFormat::Sn16 { .. } => ValType::Sn16,
            DataFormat::Sn32 { .. } => ValType::Sn32,
            DataFormat::Sn64 { .. } => ValType::Sn64,
            DataFormat::Sn128 { .. } => ValType::Sn128,
            DataFormat::Fl64 { .. } => ValType::Fl64,
            DataFormat::Dec { .. } => ValType::Dec,
            DataFormat::Bigdec { .. } => ValType::Bigdec,
            DataFormat::Bigint { .. } => ValType::Bigint,
            DataFormat::Bl { .. } => ValType::Bl,
            DataFormat::Nil { .. } => ValType::Nil,
            DataFormat::Undf { .. } => ValType::Undf,
            DataFormat::Dt { .. } => ValType::Dt,
            DataFormat::Dur { .. } => ValType::Dur,
        }
    }

    /// Tries to format a borrowed `&Val` into this `DataFormat`'s exact type.
    /// When normalize is true it applies decimals, max_length, etc.
    pub fn try_format_val(&self, val: &Val, normalize: bool) -> Result<Val, String> {
        // Same type?
        if self.val_type() == val.val_type() {
            if normalize {
                return self.apply_formatting(val); // apply decimals, max_length etc.
            } else {
                return Ok(val.clone());
            }
        }

        let s = val.to_string();

        match self {
            DataFormat::Str(fmt) => {
                let mut res = s;
                if normalize && fmt.max_length.is_some() {
                    if let Some(max) = fmt.max_length {
                        res.truncate(max as usize);
                    }
                }
                Ok(Val::Str(res))
            }
            DataFormat::Chr(_) => s
                .chars()
                .next()
                .map(Val::Chr)
                .ok_or_else(|| "Cannot convert empty value to char".to_string()),

            DataFormat::Un8(_) => Self::parse_numeric::<u8>(&s).map(Val::Un8),
            DataFormat::Un16(_) => Self::parse_numeric::<u16>(&s).map(Val::Un16),
            DataFormat::Un32(_) => Self::parse_numeric::<u32>(&s).map(Val::Un32),
            DataFormat::Un64(_) => Self::parse_numeric::<u64>(&s).map(Val::Un64),
            DataFormat::Un128(_) => Self::parse_numeric::<u128>(&s).map(Val::Un128),

            DataFormat::Sn8(_) => Self::parse_numeric::<i8>(&s).map(Val::Sn8),
            DataFormat::Sn16(_) => Self::parse_numeric::<i16>(&s).map(Val::Sn16),
            DataFormat::Sn32(_) => Self::parse_numeric::<i32>(&s).map(Val::Sn32),
            DataFormat::Sn64(_) => Self::parse_numeric::<i64>(&s).map(Val::Sn64),
            DataFormat::Sn128(_) => Self::parse_numeric::<i128>(&s).map(Val::Sn128),

            DataFormat::Fl64(fmt) => Self::parse_fl64(&s, normalize.then_some(fmt.decimals)),
            DataFormat::Dec(fmt) => Self::parse_dec(&s, normalize.then_some(fmt.decimals)),

            DataFormat::Bigdec(_) => BigDecimal::from_str(s.trim())
                .map(Val::Bigdec)
                .map_err(|e| str_err!("Cannot parse '{}' as BigDecimal: {}", s, e)),
            DataFormat::Bigint(_) => NumBigInt::from_str(s.trim())
                .map(Val::Bigint)
                .map_err(|e| str_err!("Cannot parse '{}' as BigInt: {}", s, e)),

            DataFormat::Bl(_) => {
                let lower = s.trim().to_lowercase();
                Ok(Val::Bl(matches!(
                    lower.as_str(),
                    "true" | "1" | "yes" | "y"
                )))
            }

            DataFormat::Dt(_) => Self::parse_datetime(&s),
            DataFormat::Dur(_) => s
                .trim()
                .parse::<i64>()
                .map(Duration::milliseconds)
                .map(Val::Dur)
                .map_err(|e| str_err!("Cannot convert '{}' to Duration (ms expected): {}", s, e)),

            DataFormat::Nil(_) => Ok(Val::Nil(())),
            DataFormat::Undf(_) => Ok(Val::Undf(())),
        }
    }

    fn apply_formatting(&self, val: &Val) -> Result<Val, String> {
        match (self, val) {
            (DataFormat::Fl64(fmt), Val::Fl64(f)) => {
                let mut rounded = *f;
                if fmt.decimals > 0 {
                    let factor = 10f64.powi(fmt.decimals as i32);
                    rounded = (rounded * factor).round() / factor;
                } else {
                    rounded = rounded.round();
                }
                Ok(Val::Fl64(rounded))
            }
            (DataFormat::Dec(fmt), Val::Dec(d)) => Ok(Val::Dec(d.round_dp(fmt.decimals as u32))),
            (DataFormat::Str(fmt), Val::Str(s)) => {
                let mut res = s.clone();
                if let Some(max) = fmt.max_length {
                    res.truncate(max as usize);
                }
                Ok(Val::Str(res))
            }
            _ => Ok(val.clone()), // other same-type cases need no change
        }
    }

    fn parse_numeric<T: FromStr>(s: &str) -> Result<T, String>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        s.trim()
            .parse::<T>()
            .map_err(|e| str_err!("Cannot parse '{}' as this numeric type: {}", s, e))
    }

    fn parse_fl64(s: &str, decimals_opt: Option<u64>) -> Result<Val, String> {
        let mut f: f64 = s
            .trim()
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        if let Some(decimals) = decimals_opt {
            if decimals == 0 {
                f = f.round();
            } else {
                let factor = 10f64.powi(decimals as i32);
                f = (f * factor).round() / factor;
            }
        }
        Ok(Val::Fl64(f))
    }

    fn parse_dec(s: &str, decimals_opt: Option<u64>) -> Result<Val, String> {
        let d = if let Ok(d) = Decimal::from_str_exact(s.trim()) {
            d
        } else if let Ok(f) = s.trim().parse::<f64>() {
            Decimal::from_f64(f).ok_or(str_err!("Cannot convert f64 '{}' to exact Decimal", f))?
        } else {
            return Err(str_err!("Cannot parse '{}' as Decimal", s));
        };
        if let Some(decimals) = decimals_opt {
            Ok(Val::Dec(d.round_dp(decimals as u32)))
        } else {
            Ok(Val::Dec(d))
        }
    }

    fn parse_datetime(s: &str) -> Result<Val, String> {
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(Val::Dt(dt.with_timezone(&Utc)));
        }
        if let Ok(ts) = s.parse::<i64>() {
            if let Some(dt) = DateTime::from_timestamp_millis(ts) {
                return Ok(Val::Dt(dt));
            }
            if let Some(dt) = DateTime::from_timestamp(ts, 0) {
                return Ok(Val::Dt(dt));
            }
        }
        Err(str_err!(
            "Cannot convert '{}' to DateTime. Use RFC3339 or unix timestamp (ms).",
            s
        ))
    }

    /// Returns the final string to show in the treeview cell, clipboard, tooltip, or export.
    ///
    /// Priority order:
    /// 1. Per-cell `DataFormatBase::invalid_display` (if `Some(...)`)
    /// 2. `invalid_value`
    /// 3. Hard-coded fallbacks only for nullable Nil/Undf
    ///
    /// If the incoming `Val` type does **not** match this `DataFormat`,
    /// we treat it as invalid and follow the same priority.
    pub fn format_for_display(&self, val: &Val, invalid_value: &str) -> String {
        let base = self.base();

        // ────── 1. Type mismatch (coercion failed) → treat as invalid ──────
        if self.val_type() != val.val_type() {
            if let Some(txt) = &base.invalid_display {
                return txt.clone();
            }
            return invalid_value.to_string();
        }

        // ────── 2. Special / invalid values ──────
        let is_invalid_special = match val {
            Val::Nil(_) | Val::Undf(_) => true,
            Val::Str(s) if s.is_empty() && !base.nullable => true,
            _ => false,
        };

        if is_invalid_special {
            if let Some(txt) = &base.invalid_display {
                return txt.clone();
            }
            if !base.nullable || matches!(val, Val::Nil(_) | Val::Undf(_)) {
                return invalid_value.to_string();
            }
            return "".to_string();
        }

        // ────── 3. Normal formatting — types are guaranteed to match here ──────
        match self {
            DataFormat::Str(fmt) => {
                let Val::Str(s) = val else { unreachable!() };
                let mut res = s.clone();
                if let Some(max) = fmt.max_length {
                    res.truncate(max as usize);
                }
                res
            }

            DataFormat::Fl64(f) => {
                let Val::Fl64(v) = val else { unreachable!() };
                Self::format_float(*v, f.decimals)
            }

            DataFormat::Dec(f) => {
                let Val::Dec(v) = val else { unreachable!() };
                v.round_dp(f.decimals as u32).to_string()
            }

            DataFormat::Bigdec(_) | DataFormat::Bigint(_) => val.to_string(),

            DataFormat::Dt(fmt) => {
                let Val::Dt(dt) = val else { unreachable!() };
                if let Some(format_str) = &fmt.display_format {
                    dt.format(format_str).to_string()
                } else {
                    dt.to_rfc3339()
                }
            }

            DataFormat::Dur(fmt) => {
                let Val::Dur(d) = val else { unreachable!() };
                Self::format_duration(d, fmt.display_unit.as_deref())
            }

            // All other types have no special display rules
            _ => val.to_string(),
        }
    }

    #[inline(always)]
    fn base(&self) -> &DataFormatBase {
        match self {
            DataFormat::Str(v) => &v.base,
            DataFormat::Chr(v) => v,
            DataFormat::Un8(v) => v,
            DataFormat::Un16(v) => v,
            DataFormat::Un32(v) => v,
            DataFormat::Un64(v) => v,
            DataFormat::Un128(v) => v,
            DataFormat::Sn8(v) => v,
            DataFormat::Sn16(v) => v,
            DataFormat::Sn32(v) => v,
            DataFormat::Sn64(v) => v,
            DataFormat::Sn128(v) => v,
            DataFormat::Fl64(v) => &v.base,
            DataFormat::Dec(v) => &v.base,
            DataFormat::Bigdec(v) => &v.base,
            DataFormat::Bigint(v) => v,
            DataFormat::Bl(v) => v,
            DataFormat::Nil(v) => v,
            DataFormat::Undf(v) => v,
            DataFormat::Dt(v) => &v.base,
            DataFormat::Dur(v) => &v.base,
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
    fn format_duration(d: &Duration, unit: Option<&str>) -> String {
        let unit = unit.unwrap_or("ms");
        match unit.to_lowercase().as_str() {
            "s" => format!("{}s", d.num_seconds()),
            "m" => format!("{}m", d.num_minutes()),
            "h" => format!("{}h", d.num_hours()),
            "d" => format!("{}d", d.num_days()),
            _ => format!("{}ms", d.num_milliseconds()),
        }
    }
}

pub fn string_to_data_format(input: &str) -> Result<DataFormat, String> {
    let lower = input.to_lowercase();
    let default_base = DataFormatBase {
        invalid_display: None,
        nullable: false,
    };

    match lower.as_str() {
        "str" => Ok(DataFormat::Str(DataFormatStr {
            base: default_base,
            max_length: None,
        })),
        "chr" => Ok(DataFormat::Chr(default_base)),
        "un8" => Ok(DataFormat::Un8(default_base)),
        "un16" => Ok(DataFormat::Un16(default_base)),
        "un32" => Ok(DataFormat::Un32(default_base)),
        "un64" => Ok(DataFormat::Un64(default_base)),
        "un128" => Ok(DataFormat::Un128(default_base)),
        "sn8" => Ok(DataFormat::Sn8(default_base)),
        "sn16" => Ok(DataFormat::Sn16(default_base)),
        "sn32" => Ok(DataFormat::Sn32(default_base)),
        "sn64" => Ok(DataFormat::Sn64(default_base)),
        "sn128" => Ok(DataFormat::Sn128(default_base)),
        "fl64" => Ok(DataFormat::Fl64(DataFormatFl64 {
            base: default_base,
            decimals: 0, // Default to 0; adjust if a different default is preferred
        })),
        "dec" => Ok(DataFormat::Dec(DataFormatFl64 {
            base: default_base,
            decimals: 0,
        })),
        "bigdec" => Ok(DataFormat::Bigdec(DataFormatFl64 {
            base: default_base,
            decimals: 0,
        })),
        "bigint" => Ok(DataFormat::Bigint(default_base)),
        "bl" => Ok(DataFormat::Bl(default_base)),
        "nil" => Ok(DataFormat::Nil(default_base)),
        "undf" => Ok(DataFormat::Undf(default_base)),
        "dt" => Ok(DataFormat::Dt(DataFormatDateTime {
            base: default_base,
            display_format: None,
        })),
        "dur" => Ok(DataFormat::Dur(DataFormatDuration {
            base: default_base,
            display_unit: None,
        })),
        _ => Err(str_err!("Invalid data format: '{}'", input)),
    }
}
