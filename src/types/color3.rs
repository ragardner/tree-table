#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Color3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn hex_to_color3(hex: &str) -> Result<Color3, &'static str> {
    let hex_clean = if hex.starts_with('#') { &hex[1..] } else { hex };

    if hex_clean.len() != 6 {
        return Err(
            "Hex string must be exactly 6 hexadecimal digits (optionally prefixed with '#')",
        );
    }

    let r = u8::from_str_radix(&hex_clean[0..2], 16).map_err(|_| "Invalid hexadecimal digits")?;
    let g = u8::from_str_radix(&hex_clean[2..4], 16).map_err(|_| "Invalid hexadecimal digits")?;
    let b = u8::from_str_radix(&hex_clean[4..6], 16).map_err(|_| "Invalid hexadecimal digits")?;

    Ok(Color3 { r, g, b })
}

pub fn try_hex_to_color3(opt_hex: Option<String>) -> Result<Option<Color3>, String> {
    match opt_hex {
        None => Ok(None),
        Some(hex) => Color3::try_from(hex.as_str())
            .map(Some)
            .map_err(|e| e.to_string()),
    }
}

pub fn color3_to_hex(color: &Color3) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}

impl ToString for Color3 {
    fn to_string(&self) -> String {
        color3_to_hex(self)
    }
}

impl std::convert::TryFrom<&str> for Color3 {
    type Error = &'static str;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        hex_to_color3(hex)
    }
}
