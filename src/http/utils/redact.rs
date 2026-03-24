//! Redaction helpers for Debug formatting of sensitive data.

use std::fmt;

/// Format function for derivative: completely redacts the value.
pub fn redact<T>(_val: &T, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[REDACTED]")
}

/// Format function for derivative: displays length instead of content.
pub fn redact_len(val: &str, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[len: {}]", val.len())
}

/// Format function for derivative: displays count instead of content for Vec.
pub fn redact_count<T>(val: &[T], f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[count: {}]", val.len())
}

/// Format function for derivative: displays count for Option<Vec>.
pub fn redact_count_opt<T>(val: &Option<Vec<T>>, f: &mut fmt::Formatter) -> fmt::Result {
    match val {
        Some(v) => write!(f, "[count: {}]", v.len()),
        None => write!(f, "None"),
    }
}

/// Format function for derivative: displays length for Bytes.
pub fn redact_bytes_len(val: &alloy::primitives::Bytes, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[len: {}]", val.len())
}
