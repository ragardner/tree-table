use crate::Val;
use crate::str_err;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use chrono::{DateTime, Duration, Utc};
use num_bigint::BigInt as NumBigInt;
use rust_decimal::Decimal;
use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Str {
        invalid_display: Option<String>,
        nullable: bool,
        max_length: Option<u64>,
    },
    Chr {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Un8 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Un16 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Un32 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Un64 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Un128 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Sn8 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Sn16 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Sn32 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Sn64 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Sn128 {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Fl64 {
        invalid_display: Option<String>,
        nullable: bool,
        decimals: u64,
    },
    Dec {
        invalid_display: Option<String>,
        nullable: bool,
        decimals: u64,
    },
    Bigdec {
        invalid_display: Option<String>,
        nullable: bool,
        decimals: u64,
    },
    Bigint {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Bl {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Nil {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Undf {
        invalid_display: Option<String>,
        nullable: bool,
    },
    Dt {
        invalid_display: Option<String>,
        nullable: bool,
        display_format: Option<String>,
    },
    Dur {
        invalid_display: Option<String>,
        nullable: bool,
        display_unit: Option<String>,
    },
    Path {
        invalid_display: Option<String>,
        nullable: bool,
    },
}

impl DataFormat {
    pub fn string(max_length: Option<u64>) -> Self {
        Self::Str {
            invalid_display: None,
            nullable: true,
            max_length,
        }
    }
    pub fn char() -> Self {
        Self::Chr {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn u8() -> Self {
        Self::Un8 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn u16() -> Self {
        Self::Un16 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn u32() -> Self {
        Self::Un32 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn u64() -> Self {
        Self::Un64 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn u128() -> Self {
        Self::Un128 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn i8() -> Self {
        Self::Sn8 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn i16() -> Self {
        Self::Sn16 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn i32() -> Self {
        Self::Sn32 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn i64() -> Self {
        Self::Sn64 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn i128() -> Self {
        Self::Sn128 {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn dec(decimals: u64) -> Self {
        Self::Dec {
            invalid_display: None,
            nullable: true,
            decimals,
        }
    }
    pub fn bigdec(decimals: u64) -> Self {
        Self::Bigdec {
            invalid_display: None,
            nullable: true,
            decimals,
        }
    }
    pub fn bigint() -> Self {
        Self::Bigint {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn boolean() -> Self {
        Self::Bl {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn null() -> Self {
        Self::Nil {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn undefined() -> Self {
        Self::Undf {
            invalid_display: None,
            nullable: true,
        }
    }
    pub fn f64(decimals: u64) -> Self {
        Self::Fl64 {
            invalid_display: None,
            nullable: true,
            decimals,
        }
    }
    pub fn datetime(display_format: Option<String>) -> Self {
        Self::Dt {
            invalid_display: None,
            nullable: true,
            display_format,
        }
    }
    pub fn duration(display_unit: Option<String>) -> Self {
        Self::Dur {
            invalid_display: None,
            nullable: true,
            display_unit,
        }
    }
    pub fn path() -> Self {
        Self::Path {
            invalid_display: None,
            nullable: true,
        }
    }

    #[inline(always)]
    fn compatible_with(&self, val: &Val) -> bool {
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
                | (DataFormat::Path { .. }, Val::Path(_))
        )
    }

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
                Self::parse_fl64(&s, normalize.then_some(*decimals))?
            }
            DataFormat::Dec { decimals, .. } => {
                Self::parse_dec(&s, normalize.then_some(*decimals))?
            }

            DataFormat::Bigdec { .. } => BigDecimal::from_str(s.trim())
                .map(Val::Bigdec)
                .map_err(|e| str_err!("Cannot parse '{}' as BigDecimal: {}", s, e))?,

            DataFormat::Bigint { .. } => NumBigInt::from_str(s.trim())
                .map(Val::Bigint)
                .map_err(|e| str_err!("Cannot parse '{}' as BigInt: {}", s, e))?,

            DataFormat::Bl { .. } => {
                let lower = s.trim().to_lowercase();
                Val::Bl(matches!(lower.as_str(), "true" | "1" | "yes" | "y"))
            }

            DataFormat::Dt { .. } => Self::parse_datetime(&s)?,
            DataFormat::Dur { .. } => s
                .trim()
                .parse::<i64>()
                .map(Duration::milliseconds)
                .map(Val::Dur)
                .map_err(|e| str_err!("Cannot convert '{}' to Duration (ms expected): {}", s, e))?,
            DataFormat::Path { .. } => Val::Path(PathBuf::from(s)),
            DataFormat::Nil { .. } => Val::Nil(()),
            DataFormat::Undf { .. } => Val::Undf(()),
        };

        Ok(Cow::Owned(new_val))
    }

    fn apply_formatting(&self, val: &Val) -> Result<Val, String> {
        // 100% identical cloning behavior to your original — I did not add or remove any .clone()
        match (self, val) {
            (DataFormat::Fl64 { decimals, .. }, Val::Fl64(f)) => {
                let mut rounded = *f;
                if *decimals > 0 {
                    let factor = 10f64.powi(*decimals as i32);
                    rounded = (rounded * factor).round() / factor;
                } else {
                    rounded = rounded.round();
                }
                Ok(Val::Fl64(rounded))
            }
            (DataFormat::Dec { decimals, .. }, Val::Dec(d)) => {
                Ok(Val::Dec(d.round_dp(*decimals as u32)))
            }
            (DataFormat::Str { max_length, .. }, Val::Str(s)) => {
                let mut res = s.clone(); // ← exactly the same .clone() you already had
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
            Decimal::from_f64(f)
                .ok_or_else(|| str_err!("Cannot convert f64 '{}' to exact Decimal", f))?
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

    #[inline(always)]
    fn common_fields(&self) -> (&Option<String>, bool) {
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
            DataFormat::Path {
                invalid_display,
                nullable,
                ..
            } => (invalid_display, *nullable),
        }
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
                Self::format_float(*v, *decimals)
            }

            DataFormat::Dec { decimals, .. } => {
                let Val::Dec(v) = val else { unreachable!() };
                v.round_dp(*decimals as u32).to_string()
            }

            DataFormat::Bigdec { .. } | DataFormat::Bigint { .. } => val.to_string(),

            DataFormat::Dt { display_format, .. } => {
                let Val::Dt(dt) = val else { unreachable!() };
                if let Some(format_str) = display_format {
                    dt.format(format_str).to_string()
                } else {
                    dt.to_rfc3339()
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

    match lower.as_str() {
        "str" => Ok(DataFormat::Str {
            invalid_display: None,
            nullable: false,
            max_length: None,
        }),
        "chr" => Ok(DataFormat::Chr {
            invalid_display: None,
            nullable: false,
        }),
        "un8" => Ok(DataFormat::Un8 {
            invalid_display: None,
            nullable: false,
        }),
        "un16" => Ok(DataFormat::Un16 {
            invalid_display: None,
            nullable: false,
        }),
        "un32" => Ok(DataFormat::Un32 {
            invalid_display: None,
            nullable: false,
        }),
        "un64" => Ok(DataFormat::Un64 {
            invalid_display: None,
            nullable: false,
        }),
        "un128" => Ok(DataFormat::Un128 {
            invalid_display: None,
            nullable: false,
        }),
        "sn8" => Ok(DataFormat::Sn8 {
            invalid_display: None,
            nullable: false,
        }),
        "sn16" => Ok(DataFormat::Sn16 {
            invalid_display: None,
            nullable: false,
        }),
        "sn32" => Ok(DataFormat::Sn32 {
            invalid_display: None,
            nullable: false,
        }),
        "sn64" => Ok(DataFormat::Sn64 {
            invalid_display: None,
            nullable: false,
        }),
        "sn128" => Ok(DataFormat::Sn128 {
            invalid_display: None,
            nullable: false,
        }),
        "fl64" => Ok(DataFormat::Fl64 {
            invalid_display: None,
            nullable: false,
            decimals: 0,
        }),
        "dec" => Ok(DataFormat::Dec {
            invalid_display: None,
            nullable: false,
            decimals: 0,
        }),
        "bigdec" => Ok(DataFormat::Bigdec {
            invalid_display: None,
            nullable: false,
            decimals: 0,
        }),
        "bigint" => Ok(DataFormat::Bigint {
            invalid_display: None,
            nullable: false,
        }),
        "bl" => Ok(DataFormat::Bl {
            invalid_display: None,
            nullable: false,
        }),
        "nil" => Ok(DataFormat::Nil {
            invalid_display: None,
            nullable: false,
        }),
        "undf" => Ok(DataFormat::Undf {
            invalid_display: None,
            nullable: false,
        }),
        "dt" => Ok(DataFormat::Dt {
            invalid_display: None,
            nullable: false,
            display_format: None,
        }),
        "dur" => Ok(DataFormat::Dur {
            invalid_display: None,
            nullable: false,
            display_unit: None,
        }),
        "path" => Ok(DataFormat::Path {
            invalid_display: None,
            nullable: false,
        }),
        _ => Err(str_err!("Invalid data format: '{}'", input)),
    }
}
