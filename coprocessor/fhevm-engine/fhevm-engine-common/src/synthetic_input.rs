//! Deterministic synthetic Gateway *input* injected by the **GCS (green) gw-listener only**,
//! so a quiet Gateway still produces a verified proof that can anchor the input-verification
//! consensus track.
//!
//! # Why
//!
//! The host-chain tracks have [`crate::synthetic_input`]'s counterpart in the host-listener:
//! synthetic FHE ops injected as raw logs. The Gateway track cannot use that trick, because
//! its work does not come from decoding logs into computations - it comes from a client-supplied
//! ZK proof blob in `verify_proofs.input`, which the zkproof-worker verifies, re-randomizes and
//! turns into handles. With no user submitting inputs during the dry-run window, that track
//! never anchors, and the upgrade times out.
//!
//! So green synthesizes the blob itself: one `FheUint64` input, proved under the *same* public
//! key and CRS the zkproof-worker will verify with.
//!
//! # Determinism
//!
//! This is the whole reason the approach works. `tfhe`'s
//! [`build_with_proof_packed_seeded`](tfhe::ProvenCompactCiphertextList) derives a single
//! `EncryptionRandomGenerator` from the caller's seed, and both the LWE encryption noise *and*
//! the ZK prover's own seed are drawn from it. The blob is therefore a pure function of
//! `(public key, CRS, aux data, plaintext, compute load, seed)` - every one of which is
//! identical at every operator. Same bytes everywhere means the same
//! `keccak(RAW_CT_HASH_DOMAIN_SEPARATOR || raw_ct)` blob hash in the zkproof-worker, hence the
//! same re-randomization and the same handles, hence a state hash that can reach unanimity.
//!
//! The seed itself is derived from on-chain values only: the proposal id, the version green is
//! upgrading *to*, the host chain the input is attributed to, and the Gateway block number.
//! Re-running the same window regenerates byte-identical bytes, so the insert dedupes on
//! conflict.
//!
//! # On the "keep the seed secret" warning
//!
//! `build_with_proof_packed_seeded` warns that leaking the seed breaks the encryption. That
//! does not apply here: the plaintext is the public constant
//! [`SYNTHETIC_INPUT_PLAINTEXT`], committed to in this source file. There is no secret to
//! protect. This function must never be used for real user input.
//!
//! # Cleanup
//!
//! [`synthetic_zk_proof_id`] is the marker. It sits far above any Gateway-issued id (see
//! [`SYNTHETIC_ZK_PROOF_ID_BASE`]), so cutover can find and delete the row and the handles
//! derived from it before merging `gcs.*` into `public` - otherwise the now-live green
//! transaction-sender would try to publish digests for an input no contract ever requested.

use std::time::Instant;

use alloy::primitives::keccak256;
use anyhow::Context as _;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::chain_id::ChainId;
use crate::utils::{safe_deserialize_key, safe_serialize};
use crate::zk_aux::{assemble_aux_data, ZK_AUX_DATA_SIZE};

/// Domain separator for the seed that drives encryption noise and the ZK prover.
const SYNTHETIC_INPUT_SEED_DOMAIN: &[u8] = b"FHEVM_BLUE_GREEN_SYNTHETIC_INPUT_SEED_V1";

/// Domain separator for the synthetic `zk_proof_id`.
const SYNTHETIC_INPUT_ID_DOMAIN: &[u8] = b"FHEVM_BLUE_GREEN_SYNTHETIC_INPUT_ID_V1";

/// Offset from `gw_start_block` at which the synthetic input is injected.
///
/// Not `gw_start_block` itself: that block is the alignment boundary the upgrade-controller
/// settles the Gateway side up to, and pre-start `verify_proofs` rows are pruned there. One
/// block of clearance keeps the synthetic row clear of that prune.
pub const SYNTHETIC_GW_BLOCK_OFFSET: i64 = 1;

/// The plaintext encrypted by the synthetic input. Public by construction - see the module
/// docs on why the seed needs no secrecy.
pub const SYNTHETIC_INPUT_PLAINTEXT: u64 = 0xA;

/// Contract address recorded on the synthetic input, and bound into its aux data. Not a real
/// contract: nothing calls it, and it exists only because the proof binds to one.
pub const SYNTHETIC_INPUT_CONTRACT_ADDRESS: &str = "0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a";

/// User address recorded on the synthetic input, and bound into its aux data. Not a real
/// account: nothing authenticates it.
pub const SYNTHETIC_INPUT_USER_ADDRESS: &str = "0x5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b";

/// Floor for synthetic `zk_proof_id`s.
///
/// Gateway ids come from a sequential counter in the `InputVerification` contract, so they
/// stay small for any realistic chain lifetime. Starting at 2^62 keeps synthetic ids clear of
/// them and makes the row recognizable on sight, while staying positive - `verify_proofs` has
/// `CHECK (zk_proof_id >= 0)`.
pub const SYNTHETIC_ZK_PROOF_ID_BASE: i64 = 1 << 62;

/// Everything the synthetic input is derived from. Every field is an on-chain value, so all
/// operators build byte-identical work from the same proposal.
#[derive(Debug, Clone, Copy)]
pub struct SyntheticInputContext<'a> {
    /// `CoprocessorUpgradeProposed.proposalId`.
    pub proposal_id: &'a [u8],
    /// `CoprocessorUpgradeProposed.softwareVersion`: the version green upgrades *to*, read
    /// from `upgrade_state.version` rather than the local compiled-in constant.
    pub target_version: &'a str,
    /// Host chain the input is attributed to. The zkproof-worker only picks up rows whose
    /// `chain_id` is a configured host chain, and the chain's ACL address goes into the aux
    /// data, so this is part of what the proof commits to.
    pub host_chain_id: ChainId,
    /// The designated Gateway block, `gw_start_block + SYNTHETIC_GW_BLOCK_OFFSET`.
    pub gw_block_number: i64,
}

impl SyntheticInputContext<'_> {
    /// Shared preimage for both derivations: domain separator, then every context field.
    ///
    /// Length-agnostic concatenation is safe here because only two fields are variable-length
    /// and they are followed by fixed-width fields, but the domain separators keep the two
    /// derivations disjoint regardless.
    fn preimage(&self, domain: &[u8]) -> Vec<u8> {
        let mut input = Vec::with_capacity(
            domain.len() + self.proposal_id.len() + self.target_version.len() + 16,
        );
        input.extend_from_slice(domain);
        input.extend_from_slice(self.proposal_id);
        input.extend_from_slice(self.target_version.as_bytes());
        input.extend_from_slice(&self.host_chain_id.as_u64().to_be_bytes());
        input.extend_from_slice(&self.gw_block_number.to_be_bytes());
        input
    }
}

/// Seed for the encryption noise and the ZK prover. See the module docs on determinism.
pub fn synthetic_input_seed(ctx: &SyntheticInputContext<'_>) -> [u8; 32] {
    keccak256(ctx.preimage(SYNTHETIC_INPUT_SEED_DOMAIN)).into()
}

/// Deterministic `verify_proofs.zk_proof_id` for the synthetic input, and the marker cutover
/// cleans up on.
pub fn synthetic_zk_proof_id(ctx: &SyntheticInputContext<'_>) -> i64 {
    let digest: [u8; 32] = keccak256(ctx.preimage(SYNTHETIC_INPUT_ID_DOMAIN)).into();
    // Low 32 bits of the digest, so the result cannot overflow past 2^62 + 2^32.
    let offset = u32::from_be_bytes([digest[28], digest[29], digest[30], digest[31]]);
    SYNTHETIC_ZK_PROOF_ID_BASE + i64::from(offset)
}

/// True for any id [`synthetic_zk_proof_id`] could have produced. Used by cutover cleanup and
/// by callers that want to exclude synthetic rows from ordinary accounting.
pub fn is_synthetic_zk_proof_id(zk_proof_id: i64) -> bool {
    zk_proof_id >= SYNTHETIC_ZK_PROOF_ID_BASE
}

/// The aux data the synthetic input's proof is bound to.
///
/// Must match what the zkproof-worker assembles when it verifies: it takes
/// `contract_address` and `user_address` from the `verify_proofs` row (so, the constants
/// above), the chain id from that row, and the ACL address from its host-chains cache.
pub fn synthetic_aux_data(
    host_chain_id: ChainId,
    acl_contract_address: &str,
) -> anyhow::Result<[u8; ZK_AUX_DATA_SIZE]> {
    assemble_aux_data(
        SYNTHETIC_INPUT_CONTRACT_ADDRESS,
        SYNTHETIC_INPUT_USER_ADDRESS,
        acl_contract_address,
        host_chain_id,
    )
}

/// The public key and CRS the synthetic proof is built under.
pub struct InputProvingMaterial {
    pub pks: tfhe::CompactPublicKey,
    pub crs: tfhe::zk::CompactPkeCrs,
}

/// Load the proving material, selected exactly as the zkproof-worker selects its verifying
/// material: the latest `keys` row and the latest `crs` row by `sequence_number`.
///
/// Matching that selection is the point - a proof built under a different key than the
/// verifier uses is simply invalid. Deliberately reads only `pks_key`, not the ~400 MB server
/// key `DbKeyCache` would pull, since proving needs nothing else.
///
/// `keys` and `crs` are *not* duplicated into the GCS schema (see the upgrade-controller's
/// `COPROCESSOR_TABLES`), so both stacks read the same `public` rows and the search_path plays
/// no part here. Prover and verifier therefore agree by construction — but note the corollary:
/// this probe exercises green's serialization, scheduling and re-randomization, not a key
/// rotation, because there is no per-stack key material to diverge.
pub async fn load_input_proving_material(pool: &PgPool) -> anyhow::Result<InputProvingMaterial> {
    let started = Instant::now();

    let key_row = sqlx::query("SELECT pks_key FROM keys ORDER BY sequence_number DESC LIMIT 1")
        .fetch_optional(pool)
        .await?
        .context("no rows in `keys`: cannot build a synthetic input")?;
    let pks_key: Vec<u8> = key_row.try_get("pks_key")?;
    let pks: tfhe::CompactPublicKey =
        safe_deserialize_key(&pks_key).context("deserializing pks_key")?;

    let crs_row = sqlx::query("SELECT crs FROM crs ORDER BY sequence_number DESC LIMIT 1")
        .fetch_optional(pool)
        .await?
        .context("no rows in `crs`: cannot build a synthetic input")?;
    let crs_bytes: Vec<u8> = crs_row.try_get("crs")?;
    let crs: tfhe::zk::CompactPkeCrs =
        safe_deserialize_key(&crs_bytes).context("deserializing crs")?;

    info!(
        elapsed_ms = started.elapsed().as_millis(),
        "GCS: loaded public key and CRS for synthetic input"
    );
    Ok(InputProvingMaterial { pks, crs })
}

/// Build the `verify_proofs.input` blob: one `FheUint64` input with a ZK proof, byte-identical
/// at every operator that calls this with the same context and key material.
///
/// CPU-bound for seconds - callers should run it on a blocking thread.
pub fn build_synthetic_input(
    material: &InputProvingMaterial,
    aux_data: &[u8],
    seed: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let started = Instant::now();

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&material.pks);
    builder.push(SYNTHETIC_INPUT_PLAINTEXT);
    let list = builder
        // `Verify` matches what real clients send, so the zkproof-worker's conformance
        // params accept the list unchanged.
        .build_with_proof_packed_seeded(
            &material.crs,
            aux_data,
            tfhe::zk::ZkComputeLoad::Verify,
            seed,
        )
        .map_err(|err| anyhow::anyhow!("building synthetic proven list: {err}"))?;

    let blob = safe_serialize(&list);
    info!(
        elapsed_ms = started.elapsed().as_millis(),
        blob_len = blob.len(),
        "GCS: built synthetic input proof"
    );
    Ok(blob)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> SyntheticInputContext<'static> {
        SyntheticInputContext {
            proposal_id: &[7u8; 32],
            target_version: "0.15.0",
            host_chain_id: ChainId::try_from(12345_u64).unwrap(),
            gw_block_number: 900,
        }
    }

    #[test]
    fn derivation_is_deterministic() {
        assert_eq!(synthetic_input_seed(&ctx()), synthetic_input_seed(&ctx()));
        assert_eq!(synthetic_zk_proof_id(&ctx()), synthetic_zk_proof_id(&ctx()));
    }

    /// Every context field must feed both derivations, so two proposals, two target versions,
    /// two chains or two blocks can never collide on the same seed or id.
    #[test]
    fn derivation_is_scoped_to_the_whole_context() {
        let base = ctx();
        let variants: Vec<SyntheticInputContext<'_>> = vec![
            SyntheticInputContext {
                proposal_id: &[8u8; 32],
                ..base
            },
            SyntheticInputContext {
                target_version: "0.16.0",
                ..base
            },
            SyntheticInputContext {
                host_chain_id: ChainId::try_from(54321_u64).unwrap(),
                ..base
            },
            SyntheticInputContext {
                gw_block_number: 901,
                ..base
            },
        ];

        for variant in &variants {
            assert_ne!(
                synthetic_input_seed(&base),
                synthetic_input_seed(variant),
                "seed ignores a context field: {variant:?}"
            );
            assert_ne!(
                synthetic_zk_proof_id(&base),
                synthetic_zk_proof_id(variant),
                "zk_proof_id ignores a context field: {variant:?}"
            );
        }
    }

    /// The two derivations share a preimage shape, so only the domain separators keep them
    /// apart. Guard against someone dropping one.
    #[test]
    fn seed_and_id_use_distinct_domains() {
        let seed = synthetic_input_seed(&ctx());
        let id_digest: [u8; 32] = keccak256(ctx().preimage(SYNTHETIC_INPUT_ID_DOMAIN)).into();
        assert_ne!(seed, id_digest);
    }

    #[test]
    fn synthetic_ids_are_positive_and_recognizable() {
        let id = synthetic_zk_proof_id(&ctx());
        assert!(
            id >= SYNTHETIC_ZK_PROOF_ID_BASE,
            "below the synthetic floor"
        );
        assert!(id > 0, "verify_proofs has CHECK (zk_proof_id >= 0)");
        assert!(is_synthetic_zk_proof_id(id));
        // A realistic Gateway-issued id must not be mistaken for a synthetic one.
        assert!(!is_synthetic_zk_proof_id(0));
        assert!(!is_synthetic_zk_proof_id(1_000_000));
    }

    #[test]
    fn aux_data_binds_the_acl_address_and_chain() {
        let chain = ChainId::try_from(12345_u64).unwrap();
        let acl = "0x3333333333333333333333333333333333333333";
        let aux = synthetic_aux_data(chain, acl).expect("aux data");

        assert_eq!(aux.len(), ZK_AUX_DATA_SIZE);
        assert_eq!(
            &aux[40..60],
            alloy::primitives::Address::parse_checksummed(
                "0x3333333333333333333333333333333333333333",
                None
            )
            .unwrap()
            .as_slice(),
        );
        // chain id, 32-byte big endian
        assert_eq!(&aux[84..], &12345_u64.to_be_bytes());

        let other = synthetic_aux_data(ChainId::try_from(999_u64).unwrap(), acl).unwrap();
        assert_ne!(aux, other);
    }
}
