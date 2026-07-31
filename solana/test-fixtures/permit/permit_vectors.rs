//! Schema of the normative permit vectors, shared by every implementation.
//!
//! The vectors are consumed byte-identically by the SDK, the relayer, the Connector,
//! KMS Core and the KMS client. This file is the Rust half of that contract: include
//! it with `#[path]` from a test target, deserialize `permit_v1.json`, and run each
//! record through your own verifier. Non-Rust consumers mirror the same field names.
//!
//! Two conventions carry weight and are not stylistic:
//!
//! * **Every 64-bit number is a decimal string.** A JSON number would arrive in a
//!   TypeScript consumer as an IEEE-754 double and silently lose precision above
//!   2^53 — and the chain id routinely exceeds that, because it is derived from a
//!   genesis hash and carries the host-kind high bit. Strings make the loss
//!   impossible rather than unlikely.
//! * **Every rejection names its rule, and every rejecting record names the single
//!   mutation that produced it.** A negative vector that fails "somehow" tests
//!   nothing: an implementation could reject it for an unrelated reason and look
//!   correct. `derived_from` plus `mutation` state that the base record is accepted
//!   and exactly one thing was changed.
//!
//! Only serde is required, so this file compiles in any consumer's test target.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Schema identifier written into every file this shape can parse.
pub const PERMIT_VECTOR_SCHEMA: &str = "zama-solana-permit-vectors/v1";

/// A vector file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermitVectorFile {
    /// Schema identifier; a consumer that does not recognize it must refuse the file
    /// rather than guess.
    pub schema: String,
    /// What this file covers, in prose.
    pub description: String,
    /// How to regenerate it.
    pub regenerate_with: String,
    /// The deployment identity every record is signed against.
    pub deployment: Deployment,
    /// Transport keys, by name. Kept out of the records because a single key is 1600
    /// hex characters and would make every diff unreadable.
    pub transport_keys: BTreeMap<String, String>,
    /// The records.
    pub vectors: Vec<PermitVector>,
}

/// The deployment domain shared by the records: which cluster and which program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deployment {
    /// Cluster genesis hash, hex.
    pub genesis_hash: String,
    /// Chain id as a decimal string.
    pub chain_id_decimal: String,
    /// Chain id as `0x`-prefixed hex.
    pub chain_id_hex: String,
    /// Chain id as the eight big-endian bytes handles embed, hex.
    pub chain_id_be_bytes: String,
    /// How the chain id was derived from the genesis hash for these vectors, and
    /// whether that derivation is settled.
    pub chain_id_derivation: String,
}

/// Expected outcome of a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VectorResult {
    /// The permit is well formed and its signature verifies.
    Valid,
    /// The permit must be rejected, by the rule named in `rule`.
    Invalid,
    /// The permit passes this layer, and whether it may be *used* is decided by rules
    /// outside it. `comment` says what is undecided.
    Acceptable,
}

/// One record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermitVector {
    /// Stable identifier, referenced by `derived_from`.
    pub name: String,
    /// What this record is for, in prose.
    pub comment: String,
    /// Expected outcome.
    pub result: VectorResult,
    /// For a rejecting record: the rule that must reject it. See [`rule`] names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// For a rejecting record: the accepted record it was derived from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<String>,
    /// For a rejecting record: the single change applied to that base.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation: Option<String>,
    /// The permit as it arrives over transport.
    pub permit: WirePermit,
    /// The KMS routing material parsed out of the signed routing field, when it parses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_routing: Option<KmsRoutingRecord>,
    /// Signature over the envelope, hex.
    pub signature: String,
    /// The canonical text a verifier reconstructs, as a string — present when the
    /// permit's typed form is well formed. Readability copy of `permit_text_bytes`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permit_text: Option<String>,
    /// The canonical text bytes, hex. This is the normative field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permit_text_bytes: Option<String>,
    /// The envelope bytes the signature is checked over, hex.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envelope_bytes: Option<String>,
    /// What the signer actually signed, when that is *not* the canonical text — the
    /// records that model a wallet fed a non-canonical rendering. Present so other
    /// implementations can reproduce the signature instead of trusting it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_text: Option<String>,
}

/// The eight signed fields, in transport form. Widths and lengths are whatever the
/// record declares — several records declare wrong ones on purpose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WirePermit {
    /// User pubkey, hex.
    pub user_pubkey: String,
    /// Name of the transport key in the file's `transport_keys` table.
    pub transport_key: String,
    /// ACL domain keys, hex, in the order signed.
    pub allowed_acl_domain_keys: Vec<String>,
    /// Validity-window start, decimal string.
    pub start_timestamp: String,
    /// Validity-window length, decimal string.
    pub duration_seconds: String,
    /// Verifying program id, hex.
    pub verifying_program_id: String,
    /// Chain id, decimal string.
    pub chain_id: String,
    /// Signed KMS routing field, hex.
    pub extra_data: String,
}

/// The routing material a conforming implementation parses out of `extra_data`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KmsRoutingRecord {
    /// Version byte, decimal string for symmetry with the other numbers.
    pub version: String,
    /// KMS context id, hex.
    pub kms_context_id: String,
    /// KMS epoch id, hex.
    pub kms_epoch_id: String,
}

/// Rule names used by `PermitVector::rule`.
///
/// The names are part of the cross-implementation contract: each implementation maps
/// its own error type onto these, which is how "every verifier rejects for the same
/// reason" becomes checkable rather than aspirational. They are deliberately coarser
/// than any implementation's error enum — no indices, no lengths.
pub mod rule {
    /// An identity field was not 32 bytes.
    pub const IDENTITY_WIDTH: &str = "identity-width";
    /// Too many ACL domain keys.
    pub const TOO_MANY_ACL_DOMAIN_KEYS: &str = "too-many-acl-domain-keys";
    /// ACL domain keys not strictly ascending in byte order.
    pub const ACL_DOMAIN_KEYS_NOT_ASCENDING: &str = "acl-domain-keys-not-ascending";
    /// A repeated ACL domain key.
    pub const DUPLICATE_ACL_DOMAIN_KEY: &str = "duplicate-acl-domain-key";
    /// Validity window of zero length or longer than a year.
    pub const DURATION_OUT_OF_RANGE: &str = "duration-out-of-range";
    /// Start beyond the latest representable timestamp.
    pub const START_TIMESTAMP_OUT_OF_RANGE: &str = "start-timestamp-out-of-range";
    /// Transport key of a length other than the single accepted one.
    pub const TRANSPORT_KEY_LENGTH: &str = "transport-key-length";
    /// KMS routing field with an unknown version byte, or empty.
    pub const UNKNOWN_KMS_ROUTING_VERSION: &str = "unknown-kms-routing-version";
    /// KMS routing field of a length that does not match its version.
    pub const KMS_ROUTING_LENGTH: &str = "kms-routing-length";
    /// The signature does not verify over the reconstructed envelope.
    pub const SIGNATURE_MISMATCH: &str = "signature-mismatch";
    /// The user pubkey cannot be used as an Ed25519 verifying key.
    pub const UNUSABLE_USER_PUBKEY: &str = "unusable-user-pubkey";

    /// Every rule name, for coverage checks.
    pub const ALL: &[&str] = &[
        IDENTITY_WIDTH,
        TOO_MANY_ACL_DOMAIN_KEYS,
        ACL_DOMAIN_KEYS_NOT_ASCENDING,
        DUPLICATE_ACL_DOMAIN_KEY,
        DURATION_OUT_OF_RANGE,
        START_TIMESTAMP_OUT_OF_RANGE,
        TRANSPORT_KEY_LENGTH,
        UNKNOWN_KMS_ROUTING_VERSION,
        KMS_ROUTING_LENGTH,
        SIGNATURE_MISMATCH,
        UNUSABLE_USER_PUBKEY,
    ];
}

// ---------------------------------------------------------------------------
// Hex, without a dependency
// ---------------------------------------------------------------------------

/// Encodes bytes as lowercase hex.
pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from_digit(u32::from(byte >> 4), 16).expect("nibble"));
        out.push(char::from_digit(u32::from(byte & 0x0f), 16).expect("nibble"));
    }
    out
}

/// Decodes lowercase or uppercase hex. Returns `None` on odd length or a bad digit.
pub fn from_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let bytes = hex.as_bytes();
    let mut out = Vec::with_capacity(hex.len() / 2);
    for pair in bytes.chunks(2) {
        let high = (pair[0] as char).to_digit(16)?;
        let low = (pair[1] as char).to_digit(16)?;
        out.push((high * 16 + low) as u8);
    }
    Some(out)
}

impl PermitVector {
    /// Looks up this record's transport key in the file's table.
    pub fn transport_key_bytes(&self, file: &PermitVectorFile) -> Option<Vec<u8>> {
        file.transport_keys
            .get(&self.permit.transport_key)
            .and_then(|hex| from_hex(hex))
    }

    /// The 64 signature bytes.
    pub fn signature_bytes(&self) -> Option<[u8; 64]> {
        let bytes = from_hex(&self.signature)?;
        bytes.try_into().ok()
    }
}
