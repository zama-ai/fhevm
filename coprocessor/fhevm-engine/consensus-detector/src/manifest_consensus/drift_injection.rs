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
    faults: Vec<HandleFault>,
    pause_healing: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    enabled: bool,
    chain_id: Option<i64>,
    handle: Option<B256>,
    fault: Option<Fault>,
    #[serde(default)]
    faults: Vec<HandleFault>,
    #[serde(default)]
    pause_healing: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct HandleFault {
    handle: B256,
    fault: Fault,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Fault {
    Ct64DigestBitFlip,
    Ct128DigestBitFlip,
    KeysetIdBitFlip,
    MissingHere,
    ErrorHere,
    UncomputedHere,
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
            warn!(path = %path.display(), chain_id = injection.chain_id, faults = injection.faults.len(), pause_healing = injection.pause_healing,
                "DANGEROUS manifest drift injection enabled");
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
        let mut faults = config.faults;
        match (config.handle, config.fault) {
            (Some(handle), Some(fault)) if faults.is_empty() => {
                faults.push(HandleFault { handle, fault })
            }
            (None, None) => {}
            _ => bail!("use either handle/fault or faults, not both or an incomplete pair"),
        }
        let mut handles = std::collections::BTreeSet::new();
        for fault in &faults {
            if !handles.insert(fault.handle) {
                bail!("duplicate injection handle {}", fault.handle);
            }
        }
        Ok(Some(Self {
            chain_id,
            faults,
            pause_healing: config.pause_healing,
        }))
    }

    /// Startup-only test gate. Publication and verification remain active.
    pub(crate) fn pauses_healing(&self) -> bool {
        self.pause_healing
    }

    pub(crate) fn apply(&self, chain_id: i64, descriptors: &mut Vec<BlockCiphertextDescriptor>) {
        if chain_id != self.chain_id {
            return;
        }
        descriptors.retain_mut(|descriptor| {
            let Some(injection) = self
                .faults
                .iter()
                .find(|fault| fault.handle == descriptor.handle)
            else {
                return true;
            };
            warn!(chain_id, handle = %descriptor.handle, fault = ?injection.fault,
                "DANGEROUS manifest drift injection applied");
            match injection.fault {
                Fault::MissingHere => return false,
                Fault::ErrorHere => {
                    descriptor.status = CiphertextStatus::Error {
                        error_message: Some("injected computation error".to_owned()),
                    }
                }
                Fault::UncomputedHere => descriptor.status = CiphertextStatus::Uncomputed,
                Fault::Ct64DigestBitFlip | Fault::Ct128DigestBitFlip | Fault::KeysetIdBitFlip => {
                    if let CiphertextStatus::Computed {
                        ct64_digest,
                        ct128_digest,
                        keyset_id,
                        ..
                    } = &mut descriptor.status
                    {
                        match injection.fault {
                            Fault::Ct64DigestBitFlip => ct64_digest.0[0] ^= 1,
                            Fault::Ct128DigestBitFlip => ct128_digest.0[0] ^= 1,
                            Fault::KeysetIdBitFlip => *keyset_id ^= alloy_primitives::U256::ONE,
                            _ => unreachable!(),
                        }
                    }
                }
            }
            true
        });
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
    fn multiple_faults_and_pause_are_validated_and_apply_independently() {
        let names = [
            "ct64_digest_bit_flip",
            "ct128_digest_bit_flip",
            "keyset_id_bit_flip",
            "missing_here",
            "error_here",
            "uncomputed_here",
        ];
        let faults: Vec<_> = names
            .iter()
            .enumerate()
            .map(|(i, fault)| {
                serde_json::json!({
                    "handle": B256::repeat_byte(i as u8 + 1), "fault": fault,
                })
            })
            .collect();
        let config = serde_json::json!({"enabled":true,"chain_id":12345,"pause_healing":true,"faults":faults});
        let injection = DriftInjection::parse(&serde_json::to_vec(&config).unwrap())
            .unwrap()
            .unwrap();
        assert!(injection.pauses_healing());
        let descriptors: Vec<_> = (1..=7)
            .map(|i| {
                BlockCiphertextDescriptor::computed(
                    B256::repeat_byte(i),
                    U256::from(2),
                    None,
                    B256::repeat_byte(0x64),
                    B256::repeat_byte(0x28),
                    CiphertextFormat::UncompressedOnCpu,
                )
            })
            .collect();
        let mut expected = descriptors.clone();
        if let CiphertextStatus::Computed { ct64_digest, .. } = &mut expected[0].status {
            ct64_digest.0[0] ^= 1;
        }
        if let CiphertextStatus::Computed { ct128_digest, .. } = &mut expected[1].status {
            ct128_digest.0[0] ^= 1;
        }
        if let CiphertextStatus::Computed { keyset_id, .. } = &mut expected[2].status {
            *keyset_id = U256::from(3);
        }
        expected[4] = BlockCiphertextDescriptor::from_computation_error(
            expected[4].handle,
            Some("injected computation error".to_owned()),
        );
        expected[5] = BlockCiphertextDescriptor::from_uncomputed(expected[5].handle);
        expected.remove(3);
        for _ in 0..2 {
            let mut actual = descriptors.clone();
            injection.apply(12345, &mut actual);
            assert_eq!(actual, expected);
        }
        let mut duplicate = config.clone();
        duplicate["faults"][1]["handle"] = duplicate["faults"][0]["handle"].clone();
        assert!(DriftInjection::parse(&serde_json::to_vec(&duplicate).unwrap()).is_err());
        let mut mixed = config;
        mixed["handle"] = serde_json::json!(B256::ZERO);
        mixed["fault"] = serde_json::json!("ct64_digest_bit_flip");
        assert!(DriftInjection::parse(&serde_json::to_vec(&mixed).unwrap()).is_err());
        let paused =
            DriftInjection::parse(br#"{"enabled":true,"chain_id":12345,"pause_healing":true}"#)
                .unwrap()
                .unwrap();
        assert!(paused.pauses_healing());
        assert!(paused.faults.is_empty());
        let idle = DriftInjection::parse(
            br#"{"enabled":true,"chain_id":12345,"pause_healing":false,"faults":[]}"#,
        )
        .unwrap()
        .unwrap();
        assert!(!idle.pauses_healing());
        assert!(idle.faults.is_empty());
        assert!(!DriftInjection::parse(&self::config())
            .unwrap()
            .unwrap()
            .pauses_healing());
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
                injection.faults[0].handle,
                U256::ONE,
                None,
                B256::repeat_byte(0x64),
                B256::repeat_byte(0x28),
                CiphertextFormat::UncompressedOnCpu,
            ),
            BlockCiphertextDescriptor::from_uncomputed(injection.faults[0].handle),
            BlockCiphertextDescriptor::from_computation_error(injection.faults[0].handle, None),
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
