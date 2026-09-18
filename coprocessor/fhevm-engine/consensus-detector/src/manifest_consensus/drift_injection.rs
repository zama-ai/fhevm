//! Explicit manifest-only fault injection for controlled recovery exercises.
//!
//! Load once at startup. Descriptors are rebuilt from unchanged database values
//! on every pass, so applying the same fault survives retries and restarts.
use std::{io::ErrorKind, path::Path};

use alloy_primitives::B256;
use anyhow::{bail, Context};
use block_manifest::{BlockCiphertextDescriptor, CiphertextStatus};
use serde::Deserialize;
use tracing::{info, warn};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriftInjection {
    chain_id: i64,
    handle: B256,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    enabled: bool,
    chain_id: Option<i64>,
    handle: Option<B256>,
    fault: Option<Fault>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Fault {
    Ct64DigestBitFlip,
}

impl DriftInjection {
    /// A missing file is an inactive configuration, not a startup error.
    pub fn load(path: &Path) -> anyhow::Result<Option<Self>> {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                info!(path = %path.display(), "Drift injection file absent; injection disabled");
                return Ok(None);
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("read drift injection file {}", path.display()))
            }
        };
        let injection = Self::parse(&bytes)
            .with_context(|| format!("invalid drift injection file {}", path.display()))?;
        if let Some(injection) = &injection {
            warn!(path = %path.display(), chain_id = injection.chain_id, handle = %injection.handle,
                "DANGEROUS manifest ct64 digest injection enabled");
        }
        Ok(injection)
    }

    fn parse(bytes: &[u8]) -> anyhow::Result<Option<Self>> {
        let config: FileConfig = serde_json::from_slice(bytes)?;
        if !config.enabled {
            return Ok(None);
        }
        let chain_id = config
            .chain_id
            .context("enabled injection requires chain_id")?;
        if chain_id < 0 {
            bail!("chain_id must be non-negative");
        }
        let handle = config
            .handle
            .context("enabled injection requires a 32-byte handle")?;
        config.fault.context("enabled injection requires fault")?;
        Ok(Some(Self { chain_id, handle }))
    }

    pub(crate) fn apply(&self, chain_id: i64, descriptors: &mut [BlockCiphertextDescriptor]) {
        if chain_id != self.chain_id {
            return;
        }
        for descriptor in descriptors {
            if descriptor.handle != self.handle {
                continue;
            }
            if let CiphertextStatus::Computed { ct64_digest, .. } = &mut descriptor.status {
                let original_digest = *ct64_digest;
                ct64_digest.0[0] ^= 1;
                warn!(chain_id, handle = %self.handle, %original_digest, injected_digest = %ct64_digest,
                    "DANGEROUS manifest ct64 digest injection applied");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;
    use block_manifest::CiphertextFormat;

    fn config() -> Vec<u8> {
        format!(
            r#"{{"enabled":true,"chain_id":12345,"handle":"{}","fault":"ct64_digest_bit_flip"}}"#,
            B256::repeat_byte(0x42)
        )
        .into_bytes()
    }

    #[test]
    fn validates_configuration() {
        assert!(DriftInjection::parse(br#"{"enabled":false}"#)
            .unwrap()
            .is_none());
        assert!(DriftInjection::parse(&config()).unwrap().is_some());
        for invalid in [
            "{}",
            "{",
            r#"{"enabled":true}"#,
            r#"{"enabled":true,"chain_id":-1}"#,
            r#"{"enabled":false,"typo":1}"#,
            r#"{"enabled":true,"handle":"0x01"}"#,
            r#"{"enabled":true,"fault":"unknown"}"#,
        ] {
            assert!(
                DriftInjection::parse(invalid.as_bytes()).is_err(),
                "{invalid}"
            );
        }
    }

    #[test]
    fn configuration_is_a_startup_snapshot() {
        let dir = std::env::temp_dir().join(format!(
            "drift-injection-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("injection.json");
        assert!(DriftInjection::load(&path).unwrap().is_none());
        std::fs::write(&path, config()).unwrap();
        let loaded = DriftInjection::load(&path).unwrap().unwrap();
        std::fs::write(&path, b"invalid").unwrap();
        assert!(DriftInjection::load(&path).is_err());
        assert_eq!(loaded, DriftInjection::parse(&config()).unwrap().unwrap());
        assert!(DriftInjection::load(&dir).is_err());
        std::fs::remove_file(&path).unwrap();
        assert!(DriftInjection::load(&path).unwrap().is_none());
        std::fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn only_target_computed_ct64_changes_and_reloads_repeat_the_fault() {
        let injection = DriftInjection::parse(&config()).unwrap().unwrap();
        let original = vec![
            BlockCiphertextDescriptor::computed(
                injection.handle,
                U256::ONE,
                None,
                B256::repeat_byte(0x64),
                B256::repeat_byte(0x28),
                CiphertextFormat::UncompressedOnCpu,
            ),
            BlockCiphertextDescriptor::from_uncomputed(injection.handle),
            BlockCiphertextDescriptor::from_computation_error(injection.handle, None),
            BlockCiphertextDescriptor::computed(
                B256::ZERO,
                U256::ONE,
                None,
                B256::repeat_byte(0x64),
                B256::ZERO,
                CiphertextFormat::UncompressedOnCpu,
            ),
        ];
        let mut wrong_chain = original.clone();
        injection.apply(1, &mut wrong_chain);
        assert_eq!(wrong_chain, original);
        let mut expected = original.clone();
        if let CiphertextStatus::Computed { ct64_digest, .. } = &mut expected[0].status {
            ct64_digest.0[0] ^= 1;
        }
        for _ in 0..2 {
            let mut reloaded = original.clone();
            injection.apply(12345, &mut reloaded);
            assert_eq!(reloaded, expected);
        }
    }
}
