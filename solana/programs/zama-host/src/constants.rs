//! Shared constants, PDA seeds, and protocol domain separators.

/// Version byte written to every host protocol event.
pub const EVENT_VERSION: u8 = 1;
/// High byte of the eight-byte chain-id field (handle bytes 22–29).
///
/// `0x00` is EVM: the host writes `uint64(chainId)`, which zero-extends, so a minted
/// EVM handle always has this byte clear. `0x01` is Solana. Any other value is refused.
pub const EVM_CHAIN_TYPE: u8 = 0x00;
pub const SOLANA_CHAIN_TYPE: u8 = 0x01;
const CHAIN_TYPE_SHIFT: u32 = 56;
pub const CLUSTER_TAG_MASK: u64 = 0x00ff_ffff_ffff_ffff;

/// High byte of `chain_id` (bits 56..63).
pub const fn chain_type_byte(chain_id: u64) -> u8 {
    (chain_id >> CHAIN_TYPE_SHIFT) as u8
}

pub const fn is_evm_host_chain_id(chain_id: u64) -> bool {
    chain_type_byte(chain_id) == EVM_CHAIN_TYPE
}

pub const fn is_solana_host_chain_id(chain_id: u64) -> bool {
    chain_type_byte(chain_id) == SOLANA_CHAIN_TYPE
}

/// A Solana host chain id: type byte `0x01` plus a 56-bit cluster tag.
pub const fn solana_host_chain_id(cluster_tag: u64) -> u64 {
    ((SOLANA_CHAIN_TYPE as u64) << CHAIN_TYPE_SHIFT) | (cluster_tag & CLUSTER_TAG_MASK)
}

/// Localnet sentinel used by tests and helpers that do not receive host config.
pub const SOLANA_POC_CHAIN_ID: u64 = solana_host_chain_id(12345);
/// Seed for the singleton host config PDA — the shared crate's constant, so the program and the
/// off-chain readers of the pause switch cannot drift on the seed.
pub use zama_solana_acl::HOST_CONFIG_SEED;
/// Seed prefix for KMS context PDAs (one per `kmsContextId`, mirroring ProtocolConfig).
pub const KMS_CONTEXT_SEED: &[u8] = b"kms-context";
/// Seed prefix for application `(program, scope)` deny-list records.
pub const DENY_SCOPE_SEED: &[u8] = b"deny-scope";
/// Seed prefix for HCU trust-registry records (per-application block-cap bypass).
pub const HCU_TRUSTED_APP_SEED: &[u8] = b"hcu-trusted";
/// Seed prefix for per-application HCU block meter PDAs.
pub const HCU_BLOCK_METER_SEED: &[u8] = b"hcu-block-meter";
/// Seed of the singleton random-seed nonce PDA.
pub const RAND_NONCE_SEED: &[u8] = b"rand-nonce";
/// Seed prefix for per-user permit-invalidation watermark records.
pub const PERMIT_INVALIDATION_SEED: &[u8] = b"permit-invalidation";
/// Seed prefix for user-decryption delegation records — the shared crate's constant, so the
/// program and the off-chain readers of the record cannot drift on the seed.
pub use zama_solana_acl::DELEGATION_SEED;
/// Reserved sentinel standing in for any encrypted store authority, carried by a
/// wildcard user-decryption delegation row — the shared crate's constant, under the host's
/// raw-bytes name.
pub use zama_solana_acl::WILDCARD_AUTHORITY as WILDCARD_AUTHORITY_BYTES;

/// Maximum number of FHE operations accepted by one composed execution.
///
/// The runtime boundary suite measures whole transactions, including transient store
/// open/close, under the fixed 32 KiB heap and 1,232-byte packet limit. Dependent
/// chains reach this cap; wide permissions and Store histories can hit a lower
/// limit. See `runtime-tests/cost-snapshots/fhe_execute_boundary.json` for each
/// measured shape and its binding resource. Raising this cap requires new
/// measurements, not extrapolation from the smallest execution.
pub const MAX_FHE_EXECUTION_STEPS: usize = 32;
/// At most 32 selected 32-byte handles fit the return-data channel, independently of step count.
pub const MAX_RETURNED_HANDLES: usize =
    anchor_lang::solana_program::program::MAX_RETURN_DATA / std::mem::size_of::<[u8; 32]>();
/// Maximum number of external encrypted-input handles attested in one coprocessor attestation.
pub const MAX_INPUT_ATTESTATION_HANDLES: usize = 16;
/// Maximum opaque verifier payload bytes carried in one coprocessor attestation.
pub const MAX_INPUT_ATTESTATION_EXTRA_DATA: usize = 256;

pub(crate) const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
pub(crate) const COMPUTED_HANDLE_MARKER: u8 = 0xff;
/// Current handle encoding version byte.
pub const HANDLE_VERSION: u8 = 0;
