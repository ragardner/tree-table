use alloc::format;
use alloc::string::{String, ToString};

const BOOL_TRUE_VALUES: &[&str] = &["true", "yes", "y", "on", "t", "enabled", "active"];
const BOOL_FALSE_VALUES: &[&str] = &["false", "no", "n", "off", "f", "disabled", "inactive"];

/// Flexible bool coercion used when the user/developer explicitly requests
/// a conversion to boolean (e.g. in table column formatting).
///
/// - Numbers are language-independent and extremely useful.
/// - Common English/config words are accepted case-insensitively.
/// - Everything else fails with a clear error (no silent `false`).
pub fn try_parse_bool(s: &str) -> Result<bool, String> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Err("Cannot convert empty value to boolean".to_string());
    }

    // 1. Numeric coercion first (works in any language)
    if let Ok(num) = trimmed.parse::<f64>() {
        return Ok(num != 0.0);
    }

    // 2. String aliases (case-insensitive)
    let lower = trimmed.to_lowercase();

    if BOOL_TRUE_VALUES.contains(&lower.as_str()) {
        Ok(true)
    } else if BOOL_FALSE_VALUES.contains(&lower.as_str()) {
        Ok(false)
    } else {
        Err(format!(
            "Cannot coerce '{}' to boolean.\n\n\
                 Accepted values (case-insensitive):\n\
                 • true / yes / y / on / t / enabled / active\n\
                 • false / no / n / off / f / disabled / inactive\n\
                 • Any non-zero number → true\n\
                 • 0 → false",
            s
        ))
    }
}
