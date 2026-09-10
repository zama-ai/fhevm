//! Shared constants, PDA seeds, and protocol domain separators.

/// Version byte written to every host protocol event.
pub const EVENT_VERSION: u8 = 1;
/// RFC-021 reserves the high bit (bit 63) of the u64 chain id as the host
/// `chain_type` marker: when set, the host chain is Solana rather than an EVM
/// chain. EVM chain ids keep this bit clear. The remaining 63 bits carry the
/// logical chain id.
pub const SOLANA_CHAIN_TYPE_BIT: u64 = 1 << 63;
/// PoC Solana host chain id used by tests and helpers that do not receive host
/// config. Carries the RFC-021 chain-type high bit so it satisfies the
/// repository-wide invariant that every Solana host chain id sets bit 63.
pub const SOLANA_POC_CHAIN_ID: u64 = SOLANA_CHAIN_TYPE_BIT | 12345;
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
