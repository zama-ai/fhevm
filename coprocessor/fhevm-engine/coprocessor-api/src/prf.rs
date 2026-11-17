//! The PRF output route, `GET` and `HEAD` `/v1/prf/{prf}/labels/{label}/{padLength}`, as the
//! [PRF endpoint spec](https://github.com/zama-ai/tech-spec/blob/main/rfcs/038-coprocessor-prf-output-endpoint.md#api)
//! defines it.

use alloy_primitives::{hex, B256};
use ciphertext_attestation::PrfOutputRef;

/// The PRF output route. A `{name}` segment is one path parameter.
pub const PRF_OUTPUT_ROUTE: &str = "/v1/prf/{prf}/labels/{label}/{padLength}";

/// The longest pad, in bits, that the PRF output route serves. The database repeats this limit in
/// a CHECK on `pad_length`, and `prf-service/tests/pads.rs` fails if the two differ:
/// <https://github.com/zama-ai/fhevm/blob/main/coprocessor/fhevm-engine/db-migration/migrations/20261021120000_pads.sql>
pub const MAX_PAD_LENGTH: u16 = 2048;

/// The response header that carries the signed attestation of a PRF output, a JSON
/// [`ciphertext_attestation::PrfOutputAttestation`].
pub const PRF_ATTESTATION_HEADER: &str = "x-prf-attestation";

/// Builds the path of `output`, for example `/v1/prf/0/labels/<64 hex characters>/64`.
/// [`parse_prf_output_path`] reads it back.
pub fn prf_output_path(output: &PrfOutputRef) -> String {
    format!(
        "/v1/prf/{}/labels/{}/{}",
        output.prf_id,
        hex::encode(output.label),
        output.pad_length
    )
}

/// Reads the PRF output that a path names, or `None`. Only one encoding passes, so each output has
/// one URL: decimal numbers with no sign and no leading zero, a label of 64 lowercase hex
/// characters with no `0x`, and a pad length from 1 to [`MAX_PAD_LENGTH`].
pub fn parse_prf_output_path(path: &str) -> Option<PrfOutputRef> {
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
    Some(PrfOutputRef::new(
        decimal(prf_id)?,
        label_hex(label)?,
        decimal(pad_length).filter(|n| (1..=MAX_PAD_LENGTH).contains(n))?,
    ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Vectors {
        prf_output_paths: PathVectors,
        prf_attestation_header: String,
    }

    #[derive(Deserialize)]
    struct PathVectors {
        canonical: Vec<CanonicalPath>,
        rejected: Vec<String>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CanonicalPath {
        path: String,
        prf: u16,
        label: String,
        pad_length: u16,
    }

    fn vectors() -> Vectors {
        serde_json::from_str(include_str!("../tests/vectors.json")).unwrap()
    }

    #[test]
    fn parses_and_builds_the_canonical_paths_of_the_vectors() {
        for vector in vectors().prf_output_paths.canonical {
            let label = vector.label.parse().unwrap();
            let output = PrfOutputRef::new(vector.prf, label, vector.pad_length);
            assert_eq!(
                parse_prf_output_path(&vector.path),
                Some(output),
                "{}",
                vector.path
            );
            assert_eq!(prf_output_path(&output), vector.path);
        }
    }

    #[test]
    fn rejects_the_non_canonical_paths_of_the_vectors() {
        for path in vectors().prf_output_paths.rejected {
            assert_eq!(parse_prf_output_path(&path), None, "{path}");
        }
    }

    #[test]
    fn the_attestation_header_matches_the_vectors() {
        assert_eq!(PRF_ATTESTATION_HEADER, vectors().prf_attestation_header);
    }

    #[test]
    fn built_paths_follow_the_route_pattern() {
        let path = prf_output_path(&PrfOutputRef::new(3, B256::repeat_byte(0xab), 64));
        assert_eq!(path.split('/').count(), PRF_OUTPUT_ROUTE.split('/').count());
        for (segment, pattern) in path.split('/').zip(PRF_OUTPUT_ROUTE.split('/')) {
            assert!(pattern.starts_with('{') || segment == pattern, "{path}");
        }
    }
}
