use crate::path::PrfPath;
use ciphertext_attestation::keccak_b256;

pub struct Pad {
    pub ct64: Vec<u8>,
    pub ct128: Vec<u8>,
}

// TODO: replace with the pads that the pad-worker computes.
/// Deterministic mock pad: one 32-byte entry for each block, the same on all coprocessors.
pub fn mock_pad(path: &PrfPath, block_bits: u32) -> Pad {
    let blocks = u32::from(path.pad_length).div_ceil(block_bits);
    let entries = |tag: &[u8]| -> Vec<u8> {
        (0..blocks)
            .flat_map(|i| {
                let preimage = [
                    tag,
                    &path.prf.to_be_bytes(),
                    path.label.as_slice(),
                    &path.pad_length.to_be_bytes(),
                    &i.to_be_bytes(),
                ]
                .concat();
                keccak_b256(&preimage).0
            })
            .collect()
    };
    Pad {
        ct64: entries(b"MOCK_CT64"),
        ct128: entries(b"MOCK_PAD"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;

    fn path(pad_length: u16) -> PrfPath {
        PrfPath {
            prf: 0,
            label: B256::repeat_byte(0xaa),
            pad_length,
        }
    }

    #[test]
    fn one_entry_per_block() {
        for (pad_length, blocks) in [(1, 1), (2, 1), (3, 2), (2048, 1024)] {
            let pad = mock_pad(&path(pad_length), 2);
            assert_eq!(pad.ct64.len(), blocks * 32);
            assert_eq!(pad.ct128.len(), blocks * 32);
        }
    }

    #[test]
    fn deterministic_and_distinct() {
        let pad = mock_pad(&path(64), 2);
        assert_eq!(pad.ct128, mock_pad(&path(64), 2).ct128);
        assert_ne!(pad.ct128, pad.ct64);
        assert_ne!(pad.ct128[..32], mock_pad(&path(62), 2).ct128[..32]);
    }
}
