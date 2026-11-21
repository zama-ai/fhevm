//! The pad computation, behind a trait: tests and the e2e stack run without tfhe-rs.

use alloy::primitives::{keccak256, B256};
use fhevm_engine_common::pad::mock::MockOneTimePadMask;
use std::{num::NonZeroUsize, time::Duration};

/// The two forms of one PRF output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PadOutputs {
    /// The form before noise squashing, compressed. Unmask uses it.
    pub ct64: Vec<u8>,
    /// After noise squashing, compressed: the body that prf-service serves.
    pub ct128: Vec<u8>,
}

/// Computes the output of `(prf, label, pad_length)`. Every coprocessor must get the same bytes.
pub trait PadEvaluator: Send + Sync {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs>;
}

/// Mock size of the compressed ct128 of one 4-bit block: about 100 KB for a 64-bit pad.
pub const MOCK_CT128_BLOCK_BYTES: usize = 6 * 1024;

/// Gives the mock ct64 of `fhevm_engine_common::pad::mock`, which Unmask can read, and fixed
/// ct128 bytes. Both have the estimated sizes of the real outputs. It does no FHE.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockEvaluator;

impl PadEvaluator for MockEvaluator {
    fn evaluate(&self, prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        let ct64 = MockOneTimePadMask::new_random(prf, label, usize::from(pad_length))?
            .compress()
            .into_bytes();
        let chunk = keccak256(
            [
                b"ct128".as_slice(),
                &prf.to_le_bytes(),
                label.as_slice(),
                &pad_length.to_le_bytes(),
            ]
            .concat(),
        );
        let blocks = usize::from(pad_length.div_ceil(4));
        Ok(PadOutputs {
            ct64,
            ct128: chunk.repeat(blocks * MOCK_CT128_BLOCK_BYTES / 32),
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
        // The ct64 of a 64-bit pad is about 2 kB.
        assert_eq!(first.ct64.len(), 2048);
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
