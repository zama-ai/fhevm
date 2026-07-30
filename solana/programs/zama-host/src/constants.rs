//! Shared constants, PDA seeds, and protocol domain separators.

/// Version byte written to host protocol events.
pub const EVENT_VERSION: u8 = 1;
/// Version of the produced-public `fhe_eval` lifecycle batch.
pub const PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION: u8 = 1;
/// Maximum durable subjects on an `EncryptedValue` encrypted value account (mirrors
/// `zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS`).
pub const MAX_ACL_SUBJECTS: usize = zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS;
/// RFC-021 reserves the high bit (bit 63) of the u64 chain id as the host
/// `chain_type` marker: when set, the host chain is Solana rather than an EVM
/// chain. EVM chain ids keep this bit clear. The remaining 63 bits carry the
/// logical chain id.
pub const SOLANA_CHAIN_TYPE_BIT: u64 = 1 << 63;
/// PoC Solana host chain id used by tests and helpers that do not receive host
/// config. Carries the RFC-021 chain-type high bit so it satisfies the
/// repository-wide invariant that every Solana host chain id sets bit 63.
pub const SOLANA_POC_CHAIN_ID: u64 = SOLANA_CHAIN_TYPE_BIT | 12345;
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
///
/// Derived from measured budgets on the interned wire format (fhevm-internal#1853 W8), not chosen
/// a priori. Measured on the max-op cost-snapshot frame: a marginal chained step costs ~9 bytes of
/// instruction data and ~3,700 CU. At 32 ops the maximum frame measures ~450 bytes of instruction
/// data (the whole signed transaction stays under the 1,232-byte packet limit with >=150 bytes of
/// envelope headroom — asserted by `mollusk_fhe_eval_max_op_transaction_fits_packet`) and ~150k CU
/// (under the 200k default budget, so no compute-budget instruction is required). 48 ops would
/// exceed the default CU budget and leave <5% packet headroom for realistic account envelopes.
/// The heap-heaviest legal frame shape (all steps born-public durable creates) fits 20 creates
/// on the 32KB bump heap — a hard boundary, since the Anchor default allocator serves a fixed
/// 32KB region even when a larger heap frame is requested; such frames revert cleanly beyond it
/// (measured; pinned by `mollusk_fhe_eval_born_public_heap_boundary`). Wire
/// indices (`producer_index`, pool and account indices) are `u8`, bounding any future raise
/// at 256.
pub const MAX_FHE_EVAL_OPS: usize = 32;
/// Maximum number of external encrypted-input handles in one signed proof.
pub const MAX_INPUT_PROOF_HANDLES: usize = 16;
/// Maximum opaque verifier payload bytes carried in one signed input proof.
pub const MAX_INPUT_PROOF_EXTRA_DATA: usize = 256;

pub(crate) const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
pub(crate) const COMPUTED_HANDLE_MARKER: u8 = 0xff;
/// Current handle encoding version byte.
pub const HANDLE_VERSION: u8 = 0;
