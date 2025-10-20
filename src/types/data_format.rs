use crate::ValType;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatBase {
    pub invalid_display: Option<String>,
    pub nullable: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatStr {
    pub base: DataFormatBase,
    pub max_length: Option<u64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatFl64 {
    pub base: DataFormatBase,
    pub decimals: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatDateTime {
    pub base: DataFormatBase,
    pub display_format: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataFormatDuration {
    pub base: DataFormatBase,
    pub display_unit: Option<String>,
}

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
            DataFormat::Bl { .. } => ValType::Bl,
            DataFormat::Nil { .. } => ValType::Nil,
            DataFormat::Undf { .. } => ValType::Undf,
            DataFormat::Dt { .. } => ValType::Dt,
            DataFormat::Dur { .. } => ValType::Dur,
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
        _ => Err(format!("Invalid data format: '{}'", input)),
    }
}
