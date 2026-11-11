//! The reference outputs of a PRF instance. A lane checks its bytes with them at start, and
//! the consensus-detector compares the reference digests of the operators.

use alloy::primitives::{Keccak256, B256};

/// The reference `(label, pad_length)` list, in order. A test list until tfhe-rs gives the real
/// labels. A new list changes the digest of each instance, so each lane exits: change the list only
/// before the first instance is admitted.
const REFERENCE_LIST: [(u8, u16); 2] = [(1, 64), (2, 2048)];

/// The reference `(label, pad_length)` outputs of an instance, in list order.
pub fn reference_outputs() -> Vec<(B256, u16)> {
    REFERENCE_LIST
        .iter()
        .map(|&(label, pad_length)| (B256::with_last_byte(label), pad_length))
        .collect()
}

/// The S3 key of a reference output. No lifecycle rule deletes `pads-reference/`.
pub fn reference_key(prf_instance: u16, label: B256, pad_length: u16) -> String {
    format!("pads-reference/{prf_instance}/{label:x}/{pad_length}/ct128")
}

/// `keccak256` over `ciphertext_digest ‖ sns_ciphertext_digest` of each reference output, in list
/// order.
pub fn reference_digest(digests: impl IntoIterator<Item = (B256, B256)>) -> B256 {
    let mut hasher = Keccak256::new();
    for (ciphertext_digest, sns_ciphertext_digest) in digests {
        hasher.update(ciphertext_digest);
        hasher.update(sns_ciphertext_digest);
    }
    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::keccak256;

    #[test]
    fn the_digest_is_keccak_over_the_digest_pairs_in_order() {
        let [a, b, c, d] = [1, 2, 3, 4].map(B256::repeat_byte);
        let expected = keccak256([a, b, c, d].concat());
        assert_eq!(reference_digest([(a, b), (c, d)]), expected);
        assert_ne!(reference_digest([(c, d), (a, b)]), expected);
    }

    #[test]
    fn the_reference_key_has_no_0x() {
        assert_eq!(
            reference_key(3, B256::with_last_byte(1), 64),
            format!("pads-reference/3/{}01/64/ct128", "0".repeat(62))
        );
    }

    #[test]
    fn the_pad_lengths_are_in_range() {
        assert!(reference_outputs()
            .iter()
            .all(|&(_, pad_length)| (1..=2048).contains(&pad_length)));
    }
}
