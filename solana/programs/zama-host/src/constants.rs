//! Shared constants, PDA seeds, role flags, and protocol domain separators.

/// Version byte written to host protocol events.
pub const EVENT_VERSION: u8 = 1;
/// Number of subjects embedded directly in an ACL record.
pub const MAX_ACL_SUBJECTS: usize = 8;
/// Maximum number of ACL grants accepted by one `allow_acl_subjects` call.
pub const MAX_ACL_SUBJECT_GRANTS_PER_CALL: usize = 32;
/// PoC chain id used by tests and helpers that do not receive host config.
pub const SOLANA_POC_CHAIN_ID: u64 = 12345;
/// True when local PoC/test-only instruction paths are compiled into the host.
#[cfg(feature = "poc")]
pub const POC_FEATURE_ENABLED: bool = true;
/// True when local PoC/test-only instruction paths are compiled into the host.
#[cfg(not(feature = "poc"))]
pub const POC_FEATURE_ENABLED: bool = false;

/// Seed for the singleton host config PDA.
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
/// Seed prefix for KMS context PDAs (one per `kmsContextId`, mirroring ProtocolConfig).
pub const KMS_CONTEXT_SEED: &[u8] = b"kms-context";
/// Seed prefix for canonical ACL record PDAs.
pub const ACL_RECORD_SEED: &[u8] = b"acl-record";
/// Seed prefix for overflow ACL permission PDAs.
pub const ACL_PERMISSION_SEED: &[u8] = b"acl-permission";
/// Seed prefix for grant deny-list records.
pub const DENY_SUBJECT_SEED: &[u8] = b"deny-subject";
/// Seed prefix for user-decryption delegation records.
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";
/// Reserved app-context sentinel for wildcard user-decryption delegation rows.
pub const WILDCARD_APP_CONTEXT_BYTES: [u8; 32] = [0xff; 32];
/// Seed prefix for host-owned material commitment records.
pub const HANDLE_MATERIAL_SEED: &[u8] = b"handle-material";

/// Subject may use a handle in host operations and decrypt checks.
pub const ACL_ROLE_USE: u8 = 0x01;
/// Subject may extend persistent ACL membership for a handle.
pub const ACL_ROLE_GRANT: u8 = 0x02;
/// Subject may mark a handle as publicly decryptable.
pub const ACL_ROLE_PUBLIC_DECRYPT: u8 = 0x04;
/// Subject is intended for FHE compute execution.
pub const ACL_ROLE_COMPUTE: u8 = 0x08;
/// All role bits currently understood by the host and KMS verifier.
pub const ACL_ROLE_KNOWN: u8 =
    ACL_ROLE_USE | ACL_ROLE_GRANT | ACL_ROLE_PUBLIC_DECRYPT | ACL_ROLE_COMPUTE;
/// Convenience role set for a full user/owner subject.
pub const ACL_ROLE_ALL: u8 = ACL_ROLE_USE | ACL_ROLE_GRANT | ACL_ROLE_PUBLIC_DECRYPT;
/// Convenience role set used for compute signer subjects.
pub const ACL_ROLE_COMPUTE_SUBJECT: u8 = ACL_ROLE_USE | ACL_ROLE_COMPUTE;
/// Convenience role set used for owner/user subjects in the token PoC.
pub const ACL_ROLE_USER: u8 = ACL_ROLE_ALL;
/// Maximum number of FHE operations accepted by one composed eval.
pub const MAX_FHE_EVAL_OPS: usize = 16;
/// Maximum number of external encrypted-input handles in one signed proof.
pub const MAX_INPUT_PROOF_HANDLES: usize = 16;
/// Maximum opaque verifier payload bytes carried in one signed input proof.
pub const MAX_INPUT_PROOF_EXTRA_DATA: usize = 256;
/// Material commitment state for committed/decryptable material.
pub const HANDLE_MATERIAL_STATE_COMMITTED: u8 = 1;

pub(crate) const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
pub(crate) const COMPUTED_HANDLE_MARKER: u8 = 0xff;
/// Current handle encoding version byte.
pub const HANDLE_VERSION: u8 = 0;
