//! The pad computation, behind a trait: tests and the e2e stack run without tfhe-rs.

use alloy::primitives::{keccak256, B256};

/// The two forms of one PRF output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PadOutputs {
    /// Before noise squashing. Only its digest is used.
    pub ct64: Vec<u8>,
    /// After noise squashing, compressed: the body that prf-service serves.
    pub ct128: Vec<u8>,
}

/// Computes the output of `(prf, label, pad_length)`. Every coprocessor must get the same bytes.
pub trait PadEvaluator: Send + Sync {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs>;
}

/// Fixed bytes from the inputs, one 32-byte chunk for each 4-bit block. No FHE.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockEvaluator;

impl PadEvaluator for MockEvaluator {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        let blocks = usize::from(pad_length.div_ceil(4));
        let bytes = |form: &[u8]| -> Vec<u8> {
            let chunk = keccak256(
                [
                    form,
                    &prf.to_le_bytes(),
                    label.as_slice(),
                    &pad_length.to_le_bytes(),
                ]
                .concat(),
            );
            chunk.repeat(blocks)
        };
        Ok(PadOutputs {
            ct64: bytes(b"ct64"),
            ct128: bytes(b"ct128"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_gives_the_same_bytes_for_the_same_input() {
        let label = B256::repeat_byte(0xab);
        let first = MockEvaluator.evaluate(1, label, 64).unwrap();
        assert_eq!(first, MockEvaluator.evaluate(1, label, 64).unwrap());
        assert_ne!(first.ct64, first.ct128);
        assert_eq!(first.ct128.len(), 16 * 32);
        assert_ne!(first, MockEvaluator.evaluate(2, label, 64).unwrap());
        assert_ne!(first, MockEvaluator.evaluate(1, label, 32).unwrap());
    }
}
