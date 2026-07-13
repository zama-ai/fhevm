//! Shared constants, PDA seeds, and protocol domain separators.

/// Version byte written to host protocol events.
pub const EVENT_VERSION: u8 = 1;
/// Version of the produced-public `fhe_eval` lifecycle batch.
pub const PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION: u8 = 1;
/// Maximum durable subjects on an `EncryptedValue` lineage (mirrors
/// `zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS`).
pub const MAX_ACL_SUBJECTS: usize = zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS;
/// PoC chain id used by tests and helpers that do not receive host config.
pub const SOLANA_POC_CHAIN_ID: u64 = 12345;
/// Seed for the singleton host config PDA.
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
/// Seed prefix for KMS context PDAs (one per `kmsContextId`, mirroring ProtocolConfig).
pub const KMS_CONTEXT_SEED: &[u8] = b"kms-context";
/// Seed prefix for grant deny-list records.
pub const DENY_SUBJECT_SEED: &[u8] = b"deny-subject";
/// Seed prefix for HCU trust-registry records (per-app block-cap bypass).
pub const HCU_TRUSTED_APP_SEED: &[u8] = b"hcu-trusted";
/// Seed prefix for per-app HCU block meter PDAs.
pub const HCU_BLOCK_METER_SEED: &[u8] = b"hcu-block-meter";
/// Seed prefix for user-decryption delegation records.
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";
/// Reserved app-context sentinel for wildcard user-decryption delegation rows.
pub const WILDCARD_APP_CONTEXT_BYTES: [u8; 32] = [0xff; 32];

/// Maximum number of FHE operations accepted by one composed eval.
pub const MAX_FHE_EVAL_OPS: usize = 16;
/// Maximum number of external encrypted-input handles in one signed proof.
pub const MAX_INPUT_PROOF_HANDLES: usize = 16;
/// Maximum opaque verifier payload bytes carried in one signed input proof.
pub const MAX_INPUT_PROOF_EXTRA_DATA: usize = 256;

pub(crate) const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
pub(crate) const COMPUTED_HANDLE_MARKER: u8 = 0xff;
/// Current handle encoding version byte.
pub const HANDLE_VERSION: u8 = 0;
