//! The pad computation, behind a trait: tests wrap it to count or fail evaluations.

use alloy::primitives::B256;
use anyhow::Context;
use fhevm_engine_common::utils::safe_serialize;
use tfhe::{prelude::SquashNoise, OneTimePadFheSecretMask, ServerKey};

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

/// Round the pad length up to whole bytes: four ciphertexts of 2 bits for each byte. Noise
/// squashing needs a multiple of 4 bits.
const PAD_BITS_MULTIPLE: u64 = 8;

/// The one-time pad of tfhe-rs: one OPRF call for each ciphertext, with seed `label ‖ le32(i)`.
pub struct TfheEvaluator {
    server_key: ServerKey,
}

impl TfheEvaluator {
    /// `server_key` must hold the transciphering key, the compression key and the noise-squashing
    /// keys.
    pub fn new(server_key: ServerKey) -> Self {
        Self { server_key }
    }
}

impl PadEvaluator for TfheEvaluator {
    fn evaluate(&self, _prf: u16, label: B256, pad_length: u16) -> anyhow::Result<PadOutputs> {
        // The key is thread-local, and each evaluation runs on a blocking thread of tokio.
        tfhe::set_server_key(self.server_key.clone());
        let bits = u64::from(pad_length).div_ceil(PAD_BITS_MULTIPLE) * PAD_BITS_MULTIPLE;
        let mask = OneTimePadFheSecretMask::new_random(label.as_slice(), bits).context("pad")?;
        let ct64 = mask.compress().context("ct64 compression")?;
        let ct128 = mask
            .squash_noise()
            .context("noise squashing")?
            .compress()
            .context("ct128 compression")?;
        Ok(PadOutputs {
            ct64: safe_serialize(&ct64),
            ct128: safe_serialize(&ct128),
        })
    }
}
