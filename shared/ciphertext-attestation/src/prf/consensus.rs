//! PRF-specific consensus: what a round is about and what the Coprocessors agree on.

use alloy_primitives::B256;

/// The longest pad, in bits, that the PRF endpoint serves.
pub const MAX_PAD_LENGTH: u16 = 2048;

/// A PRF evaluation: the subject of a PRF output consensus round.
///
/// All fields are bound by the signature but absent from the wire form. The verifier rebuilds
/// them from the request path.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrfOutputRef {
    pub prf_id: u16,
    pub label: B256,
    pub pad_length: u16,
}

impl PrfOutputRef {
    pub fn new(prf_id: u16, label: B256, pad_length: u16) -> Self {
        Self {
            prf_id,
            label,
            pad_length,
        }
    }

    /// Parses the endpoint path `/v1/prf/{prf}/labels/{label}/{padLength}`. Only one encoding passes.
    pub fn from_path(path: &str) -> Option<Self> {
        let mut parts = path.strip_prefix("/v1/prf/")?.split('/');
        let (Some(prf_id), Some("labels"), Some(label), Some(pad_length), None) = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) else {
            return None;
        };
        Some(Self {
            prf_id: decimal(prf_id)?,
            label: label_hex(label)?,
            pad_length: decimal(pad_length).filter(|n| (1..=MAX_PAD_LENGTH).contains(n))?,
        })
    }
}

/// A decimal `u16` with no sign and no leading zero.
fn decimal(s: &str) -> Option<u16> {
    let canonical =
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && (s == "0" || !s.starts_with('0'));
    canonical.then(|| s.parse().ok()).flatten()
}

/// 64 lowercase hex characters with no `0x`.
fn label_hex(s: &str) -> Option<B256> {
    if !s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return None;
    }
    let mut label = [0u8; 32];
    hex::decode_to_slice(s, &mut label).ok()?;
    Some(B256::from(label))
}

impl std::fmt::Display for PrfOutputRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "label {}, pad length {} (prf {})",
            self.label, self.pad_length, self.prf_id
        )
    }
}

/// The PRF output material a consensus group agreed on: the digest of the SnS PRF output.
pub type PrfMaterial = B256;

#[cfg(test)]
mod tests {
    use super::*;

    const LABEL: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

    fn path(prf: &str, label: &str, pad_length: &str) -> String {
        format!("/v1/prf/{prf}/labels/{label}/{pad_length}")
    }

    #[test]
    fn parses_canonical_paths() {
        for (prf, pad_length) in [("0", "1"), ("7", "64"), ("65535", "2048")] {
            let parsed = PrfOutputRef::from_path(&path(prf, LABEL, pad_length)).unwrap();
            assert_eq!(parsed.prf_id.to_string(), prf);
            assert_eq!(hex::encode(parsed.label), LABEL);
            assert_eq!(parsed.pad_length.to_string(), pad_length);
        }
    }

    #[test]
    fn rejects_non_canonical_paths() {
        let upper = LABEL.to_uppercase();
        let prefixed = format!("0x{}", &LABEL[2..]);
        let short = &LABEL[1..];
        let encoded = format!("%30{}", &LABEL[1..]);
        for bad in [
            path("00", LABEL, "64"),
            path("01", LABEL, "64"),
            path("+1", LABEL, "64"),
            path("", LABEL, "64"),
            path("65536", LABEL, "64"),
            path("%30", LABEL, "64"),
            path("0", &upper, "64"),
            path("0", &prefixed, "64"),
            path("0", short, "64"),
            path("0", &encoded, "64"),
            path("0", LABEL, "0"),
            path("0", LABEL, "064"),
            path("0", LABEL, "2049"),
            path("0", LABEL, "64/"),
            format!("/v1/prf/0/label/{LABEL}/64"),
            format!("/v2/prf/0/labels/{LABEL}/64"),
        ] {
            assert_eq!(PrfOutputRef::from_path(&bad), None, "{bad}");
        }
    }
}
