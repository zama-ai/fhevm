//! The pad computation, behind a trait: tests and the e2e stack run without tfhe-rs.

use alloy::primitives::{keccak256, B256};
use std::{num::NonZeroUsize, time::Duration};

/// The two forms of one PRF output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PadOutputs {
    /// Before noise squashing, raw: the form that Unmask uses.
    pub ct64: Vec<u8>,
    /// After noise squashing, compressed: the body that prf-service serves.
    pub ct128: Vec<u8>,
}

/// Computes the output of `(prf, label, pad_length)`. Every coprocessor must get the same bytes.
pub trait PadEvaluator: Send + Sync {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs>;
}

/// Mock size of the raw ct64 of one 4-bit block: 256 KiB for a 64-bit pad, 8 MiB for 2,048 bits.
pub const MOCK_CT64_BLOCK_BYTES: usize = 16 * 1024;
/// Mock size of the compressed ct128 of one 4-bit block: about 100 KB for a 64-bit pad.
pub const MOCK_CT128_BLOCK_BYTES: usize = 6 * 1024;

/// Fixed bytes from the inputs, in the sizes of the real outputs (estimates). No FHE.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockEvaluator;

impl PadEvaluator for MockEvaluator {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        let blocks = usize::from(pad_length.div_ceil(4));
        let bytes = |form: &[u8], block_bytes: usize| -> Vec<u8> {
            let chunk = keccak256(
                [
                    form,
                    &prf.to_le_bytes(),
                    label.as_slice(),
                    &pad_length.to_le_bytes(),
                ]
                .concat(),
            );
            chunk.repeat(blocks * block_bytes / 32)
        };
        Ok(PadOutputs {
            ct64: bytes(b"ct64", MOCK_CT64_BLOCK_BYTES),
            ct128: bytes(b"ct128", MOCK_CT128_BLOCK_BYTES),
        })
    }
}

/// The mock with the OPRF latency: one PBS (about 13 ms on CPU) for each round of `parallel`
/// blocks. The sleep uses no CPU.
#[derive(Debug, Clone, Copy)]
pub struct TimedMockEvaluator {
    pub pbs_latency: Duration,
    pub parallel: NonZeroUsize,
    /// A fixed delay on top, for e2e tests that need a row to stay pending.
    pub extra: Duration,
}

impl TimedMockEvaluator {
    pub fn latency(&self, pad_length: u16) -> Duration {
        let rounds = usize::from(pad_length.div_ceil(4)).div_ceil(self.parallel.get());
        self.pbs_latency * u32::try_from(rounds).unwrap_or(u32::MAX) + self.extra
    }
}

impl PadEvaluator for TimedMockEvaluator {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        std::thread::sleep(self.latency(pad_length));
        MockEvaluator.evaluate(prf, label, pad_length)
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
        assert_eq!(first.ct64.len(), 16 * MOCK_CT64_BLOCK_BYTES);
        assert_eq!(first.ct128.len(), 16 * MOCK_CT128_BLOCK_BYTES);
        assert_ne!(first, MockEvaluator.evaluate(2, label, 64).unwrap());
        assert_ne!(first, MockEvaluator.evaluate(1, label, 32).unwrap());
    }

    #[test]
    fn timed_mock_takes_one_pbs_latency_for_each_round_of_blocks() {
        let mock = TimedMockEvaluator {
            pbs_latency: Duration::from_millis(13),
            parallel: NonZeroUsize::new(96).unwrap(),
            extra: Duration::ZERO,
        };
        assert_eq!(mock.latency(64), Duration::from_millis(13));
        assert_eq!(mock.latency(2048), Duration::from_millis(6 * 13));
        let two_cpus = TimedMockEvaluator {
            parallel: NonZeroUsize::new(2).unwrap(),
            ..mock
        };
        assert_eq!(two_cpus.latency(64), Duration::from_millis(8 * 13));
        let slow = TimedMockEvaluator {
            extra: Duration::from_secs(2),
            ..mock
        };
        assert_eq!(slow.latency(64), Duration::from_millis(2013));
    }
}
