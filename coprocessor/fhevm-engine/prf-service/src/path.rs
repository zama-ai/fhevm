use alloy::primitives::B256;

pub const MAX_PAD_LENGTH: u16 = 2048;

#[derive(Debug, PartialEq, Eq)]
pub struct PrfPath {
    pub prf: u16,
    pub label: B256,
    pub pad_length: u16,
}

/// Parses the raw `/v1/prf/{prf}/labels/{label}/{padLength}` path. Only the canonical encoding passes.
pub fn parse(path: &str) -> Option<PrfPath> {
    let mut parts = path.strip_prefix("/v1/prf/")?.split('/');
    let (Some(prf), Some("labels"), Some(label), Some(pad_length), None) = (
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
    ) else {
        return None;
    };
    Some(PrfPath {
        prf: decimal(prf)?,
        label: label_hex(label)?,
        pad_length: decimal(pad_length).filter(|n| (1..=MAX_PAD_LENGTH).contains(n))?,
    })
}

// TODO: read the instances from the PRF module.
pub fn block_bits(prf: u16) -> Option<u32> {
    match prf {
        0 => Some(2),
        _ => None,
    }
}

fn decimal(s: &str) -> Option<u16> {
    let canonical =
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && (s == "0" || !s.starts_with('0'));
    canonical.then(|| s.parse().ok()).flatten()
}

fn label_hex(s: &str) -> Option<B256> {
    if s.len() != 64 || !s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return None;
    }
    let mut label = [0u8; 32];
    hex::decode_to_slice(s, &mut label).ok()?;
    Some(B256::from(label))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LABEL: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

    fn path(prf: &str, label: &str, pad_length: &str) -> String {
        format!("/v1/prf/{prf}/labels/{label}/{pad_length}")
    }

    #[test]
    fn accepts_canonical_paths() {
        for (prf, pad_length) in [("0", "1"), ("7", "64"), ("65535", "2048")] {
            let parsed = parse(&path(prf, LABEL, pad_length)).unwrap();
            assert_eq!(parsed.prf.to_string(), prf);
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
            assert_eq!(parse(&bad), None, "{bad}");
        }
    }

    #[test]
    fn serves_only_known_instances() {
        assert_eq!(block_bits(0), Some(2));
        assert_eq!(block_bits(1), None);
    }
}
