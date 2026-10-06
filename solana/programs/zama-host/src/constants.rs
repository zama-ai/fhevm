//! Shared constants, PDA seeds, and protocol domain separators.

/// Version byte written to every host protocol event.
pub const EVENT_VERSION: u8 = 1;
/// Localnet sentinel used by tests and helpers that do not receive host config.
pub const SOLANA_POC_CHAIN_ID: u64 = zama_solana_acl::host_chain::solana_host_chain_id(12345);
/// Seed of the singleton host config PDA: `[seed]`.
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
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
/// Seed prefix for pauser records.
pub const PAUSER_SEED: &[u8] = b"pauser";
/// Seed prefix for per-user permit-invalidation watermark records.
pub use zama_solana_acl::PERMIT_INVALIDATION_SEED;
/// The application a wildcard user-decryption delegation row carries in both its program and its
/// scope position — the shared crate's constant.
pub use zama_solana_acl::WILDCARD_APP;

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
