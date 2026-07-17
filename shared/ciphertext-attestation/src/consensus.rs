//! Consensus evaluation over a fetched attestation set.

use crate::{CiphertextAttestation, CiphertextFormat};
use alloy_primitives::{Address, B256, U256};
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroUsize,
};
use tracing::warn;

/// A winning consensus: the agreed material plus the distinct, in-registry signers that vouched
/// for it.
#[derive(Clone, Debug)]
pub struct Consensus {
    pub material: ConsensusMaterial,
    pub signers: HashSet<Address>,
}

/// The ciphertext material a consensus group agreed on.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConsensusMaterial {
    pub key_id: U256,
    pub ciphertext_digest: B256,
    pub sns_ciphertext_digest: B256,
    pub format: CiphertextFormat,
}

/// Why the consensus could not be reached.
#[derive(Debug, thiserror::Error)]
#[cfg_attr(debug_assertions, derive(PartialEq))]
#[error("consensus unreachable: {valid_signers} valid signer(s), threshold {threshold}")]
pub struct ConsensusError {
    valid_signers: usize,
    threshold: NonZeroUsize,
}

impl From<&CiphertextAttestation> for ConsensusMaterial {
    fn from(att: &CiphertextAttestation) -> Self {
        Self {
            key_id: att.key_id,
            ciphertext_digest: att.ciphertext_digest,
            sns_ciphertext_digest: att.sns_ciphertext_digest,
            format: att.format,
        }
    }
}

/// Validates every fetched attestations and evaluates the majority threshold for a handle.
///
/// The address of the coprocessor's tx-sender is attached to each attestation for debugging.
/// An attestation counts only if its signature recovers to its embedded `signer` and that signer
/// is in the `allowed_signers` set.
/// Survivors are grouped by material tuple; the largest group wins if it gathers at least
/// `threshold` distinct signers.
///
/// `threshold` is a [`NonZeroUsize`]: a zero threshold would let any single attestation win,
/// so it is unrepresentable here and rejected where the threshold is loaded.
pub fn evaluate(
    handle: B256,
    coprocessor_context_id: U256,
    attestations: &[(Address, CiphertextAttestation)],
    allowed_signers: &HashSet<Address>,
    threshold: NonZeroUsize,
) -> Result<Consensus, ConsensusError> {
    let mut signer_groups: HashMap<ConsensusMaterial, HashSet<Address>> = HashMap::new();

    for (tx_sender, attestation) in attestations {
        if let Err(e) = attestation.verify(handle, coprocessor_context_id) {
            warn!(
                %tx_sender,
                "Discarding attestation with invalid signature: {e}"
            );
            continue;
        }
        if !allowed_signers.contains(&attestation.signer) {
            warn!(
                %tx_sender,
                signer = %attestation.signer,
                "Discarding attestation from signer outside the registry"
            );
            continue;
        }

        signer_groups
            .entry(ConsensusMaterial::from(attestation))
            .or_default()
            .insert(attestation.signer);
    }

    match signer_groups.into_iter().max_by(
        |(left_material, left_signers), (right_material, right_signers)| {
            left_signers
                .len()
                .cmp(&right_signers.len())
                // Equal-size groups cannot reach honest consensus, but the result must still
                // be deterministic for Byzantine or misconfigured thresholds.
                .then_with(|| right_material.cmp(left_material))
        },
    ) {
        Some((material, signers)) if signers.len() >= threshold.get() => {
            Ok(Consensus { material, signers })
        }
        Some((_, signers)) => Err(ConsensusError {
            valid_signers: signers.len(),
            threshold,
        }),
        None => Err(ConsensusError {
            valid_signers: 0,
            threshold,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CiphertextAttestationPayload, Version};
    use alloy_signer_local::PrivateKeySigner;

    const HANDLE: B256 = B256::repeat_byte(0xAA);
    const COPROCESSOR_CONTEXT_ID: U256 = U256::ONE;
    const KEY_ID: U256 = U256::from_limbs([0xdead_beef, 0, 0, 0]);
    const CT_DIGEST: B256 = B256::repeat_byte(0xBB);
    const SNS_DIGEST: B256 = B256::repeat_byte(0xCC);
    const FORMAT: CiphertextFormat = CiphertextFormat::UncompressedOnCpu;

    /// Signs an attestation for `HANDLE` with the given material fields.
    async fn signed(
        signer: &PrivateKeySigner,
        key_id: U256,
        ct_digest: B256,
        sns_digest: B256,
        format: CiphertextFormat,
    ) -> CiphertextAttestation {
        CiphertextAttestationPayload::new(
            Version::V1,
            HANDLE,
            key_id,
            COPROCESSOR_CONTEXT_ID,
            ct_digest,
            sns_digest,
            format,
        )
        .sign(signer)
        .await
        .unwrap()
    }

    /// Default-material attestation from `signer`.
    async fn default_att(signer: &PrivateKeySigner) -> CiphertextAttestation {
        signed(signer, KEY_ID, CT_DIGEST, SNS_DIGEST, FORMAT).await
    }

    /// The authorized signer set built from each signer's address.
    fn signer_set(signers: &[&PrivateKeySigner]) -> HashSet<Address> {
        signers.iter().map(|s| s.address()).collect()
    }

    /// Wraps attestations as fetcher output, one per (arbitrary) bucket address.
    fn fetched(atts: Vec<CiphertextAttestation>) -> Vec<(Address, CiphertextAttestation)> {
        atts.into_iter().map(|a| (a.signer, a)).collect()
    }

    /// Shorthand for the [`NonZeroUsize`] thresholds.
    fn nz(threshold: usize) -> NonZeroUsize {
        NonZeroUsize::new(threshold).unwrap()
    }

    #[tokio::test]
    async fn reaches_consensus_at_threshold() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1, &s2]);
        let atts = vec![default_att(&s1).await, default_att(&s2).await];

        let consensus = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(atts),
            &signers,
            nz(2),
        )
        .unwrap();
        assert_eq!(consensus.signers.len(), 2);
        assert_eq!(consensus.material.sns_ciphertext_digest, SNS_DIGEST);
        assert_eq!(consensus.material.key_id, KEY_ID);
    }

    #[tokio::test]
    async fn threshold_not_reached() {
        let s1 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1]);
        let atts = vec![default_att(&s1).await];

        let err = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(atts),
            &signers,
            nz(2),
        )
        .unwrap_err();
        assert_eq!(
            err,
            ConsensusError {
                valid_signers: 1,
                threshold: nz(2),
            }
        );
    }

    #[tokio::test]
    async fn signer_outside_registry_is_ignored() {
        let s1 = PrivateKeySigner::random();
        let intruder = PrivateKeySigner::random();
        // Only `s1` is authorized; `intruder`'s valid signature does not count.
        let signers = signer_set(&[&s1]);
        let atts = vec![default_att(&s1).await, default_att(&intruder).await];

        let err = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(atts),
            &signers,
            nz(2),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ConsensusError {
                valid_signers: 1,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn disagreement_splits_groups_below_threshold() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1, &s2]);
        // Same handle, divergent digests: two groups of one, neither hits 2.
        let other_sns = B256::repeat_byte(0xDD);
        let atts = vec![
            default_att(&s1).await,
            signed(&s2, KEY_ID, CT_DIGEST, other_sns, FORMAT).await,
        ];

        let err = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(atts),
            &signers,
            nz(2),
        )
        .unwrap_err();
        assert_eq!(
            err,
            ConsensusError {
                valid_signers: 1,
                threshold: nz(2),
            }
        );
    }

    #[tokio::test]
    async fn majority_group_wins_over_minority() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let s3 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1, &s2, &s3]);
        // s1+s2 agree on the default tuple; s3 dissents with a different format.
        let atts = vec![
            default_att(&s1).await,
            default_att(&s2).await,
            signed(
                &s3,
                KEY_ID,
                CT_DIGEST,
                SNS_DIGEST,
                CiphertextFormat::CompressedOnCpu,
            )
            .await,
        ];

        let consensus = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(atts),
            &signers,
            nz(2),
        )
        .unwrap();
        assert_eq!(consensus.signers.len(), 2);
        assert_eq!(consensus.material.format, FORMAT);
    }

    #[tokio::test]
    async fn equal_size_groups_use_deterministic_material_tie_break() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1, &s2]);
        let larger_digest = B256::repeat_byte(0xDD);
        let smaller_digest = B256::repeat_byte(0xBC);
        let atts = vec![
            signed(&s1, KEY_ID, larger_digest, SNS_DIGEST, FORMAT).await,
            signed(&s2, KEY_ID, smaller_digest, SNS_DIGEST, FORMAT).await,
        ];

        let consensus = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(atts),
            &signers,
            nz(1),
        )
        .unwrap();

        assert_eq!(consensus.signers.len(), 1);
        assert_eq!(consensus.material.ciphertext_digest, smaller_digest);
    }

    #[tokio::test]
    async fn invalid_signature_is_discarded() {
        let s1 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1]);
        let mut att = default_att(&s1).await;
        // Tamper with a signed field so recovery yields a different address.
        att.sns_ciphertext_digest = B256::repeat_byte(0xEE);

        let err = evaluate(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            &fetched(vec![att]),
            &signers,
            nz(1),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ConsensusError {
                valid_signers: 0,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn duplicate_signer_across_buckets_counts_once() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let signers = signer_set(&[&s1, &s2]);
        // s1 appears twice (same attestation served by two buckets); should not
        // alone satisfy a threshold of 2.
        let a1 = default_att(&s1).await;
        let atts = vec![(s1.address(), a1.clone()), (Address::repeat_byte(0x99), a1)];

        let err = evaluate(HANDLE, COPROCESSOR_CONTEXT_ID, &atts, &signers, nz(2)).unwrap_err();
        assert_eq!(
            err,
            ConsensusError {
                valid_signers: 1,
                threshold: nz(2),
            }
        );
    }
}
