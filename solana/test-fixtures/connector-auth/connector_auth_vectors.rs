//! Schema of the normative Connector-authorization vectors.
//!
//! Where the permit set fixes what a permit *is*, this set fixes what the host policy *decides*:
//! a record is a request, the host state it is authorized against, the moment it is authorized
//! at, and the outcome — with, for a rejection, the rule that rejected it and what a client
//! should do about it.
//!
//! Three conventions carry weight and are not stylistic.
//!
//! * **State is part of the record.** Each record carries the account set it was authorized
//!   against: pubkey, owning program, and raw data. A rule that reads an account cannot be
//!   pinned by a request alone, and paraphrasing the account ("a lineage whose handle moved on") would make
//!   the record depend on whoever wrote the paraphrase.
//! * **Every 64-bit number is a decimal string.** A JSON number reaches a TypeScript consumer as
//!   a double and silently loses precision above 2^53; chain ids and slots both go there.
//! * **A rejection names both its rule and its class.** The rule says which check refused the
//!   request; the class says whether repeating it can ever help. Neither field is derivable from
//!   the other: an inclusion proof that does not verify is terminal when the claimed leaf count is
//!   behind the observed one and retryable when it is not, under one rule name.
//!
//! Only serde is required, so this file compiles in any consumer's test target. The permit fields
//! use the same names and encodings as the permit set on purpose: a consumer that already reads
//! that set can reuse its own deserialization of them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Schema identifier written into every file this shape can parse.
pub const CONNECTOR_AUTH_VECTOR_SCHEMA: &str = "zama-solana-connector-auth-vectors/v1";

/// A vector file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorAuthVectorFile {
    /// Schema identifier; a consumer that does not recognize it must refuse the file rather than
    /// guess.
    pub schema: String,
    /// What this file covers, in prose.
    pub description: String,
    /// How to regenerate it.
    pub regenerate_with: String,
    /// The deployment every record is authorized against.
    pub deployment: Deployment,
    /// Transport keys, by name, kept out of the records: one key is 1600 hex characters.
    pub transport_keys: BTreeMap<String, String>,
    /// The records.
    pub vectors: Vec<ConnectorAuthVector>,
}

/// The deployment identity of the set: which program, which cluster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deployment {
    /// Host program id, hex.
    pub verifying_program_id: String,
    /// Cluster genesis hash, hex.
    pub genesis_hash: String,
    /// Chain id as a decimal string.
    pub chain_id_decimal: String,
    /// Chain id as `0x`-prefixed hex.
    pub chain_id_hex: String,
    /// Chain id as the eight big-endian bytes handles embed, hex.
    pub chain_id_be_bytes: String,
    /// How the chain id was derived from the genesis hash for these vectors, and whether that
    /// derivation is settled.
    pub chain_id_derivation: String,
}

/// Expected outcome of a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VectorResult {
    /// The request is authorized.
    Valid,
    /// The request must be rejected, by the rule named in `rule`.
    Invalid,
    /// The request is authorized, and whether it *should* be is a policy question recorded in
    /// `comment` rather than settled by this set.
    Acceptable,
}

/// What a client should do about a rejection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FailureClass {
    /// Nothing about this request will ever be authorized.
    Terminal,
    /// The same request may be authorized from a later observation.
    Transient,
    /// A disagreement between observers that is expected to converge.
    Retryable,
}

/// The KMS management state a record is authorized against.
///
/// Declared rather than derived: servability lives in KMS management state, not in host accounts,
/// so it cannot be read out of the account set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KmsPairStatus {
    /// The signed pair is servable.
    Servable,
    /// The context is unknown to this party.
    ContextUnknown,
    /// The epoch exists but is not active yet.
    EpochNotYetActive,
    /// The epoch belongs to a different context.
    EpochOfAnotherContext,
    /// Governance destroyed the context.
    ContextDestroyed,
    /// Governance destroyed the epoch.
    EpochDestroyed,
}

/// One record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorAuthVector {
    /// Stable identifier, referenced by `derived_from`.
    pub name: String,
    /// What this record is for, in prose.
    pub comment: String,
    /// Expected outcome.
    pub result: VectorResult,
    /// For a rejecting record: the rule that must reject it. See [`rule`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// For a rejecting record: what the client should do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<FailureClass>,
    /// For a rejecting record: the accepted record it was derived from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<String>,
    /// For a rejecting record: the single change applied to that base.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation: Option<String>,
    /// The request as it arrives over transport.
    pub request: WireRequest,
    /// The host state and the moment it was authorized at.
    pub observation: Observation,
}

/// The request: the permit, the signature over its envelope, and the handle entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireRequest {
    /// The eight signed permit fields.
    pub permit: WirePermit,
    /// Signature over the envelope, hex.
    pub signature: String,
    /// Handle entries, in request order.
    pub handles: Vec<WireHandleEntry>,
}

/// The eight signed permit fields, in transport form. Field names and encodings match the permit
/// set.
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

/// One handle entry. None of these fields are signed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireHandleEntry {
    /// Ciphertext handle, hex.
    pub handle: String,
    /// Ciphertext owner, hex: the signer for a direct entry, the delegator for a delegated one.
    pub owner: String,
    /// Lineage identity, hex.
    pub encrypted_value_id: String,
    /// The leaf count the access proof was built against, decimal string; `"0"` in current mode.
    pub proof_leaf_count: String,
    /// Access proof, hex; empty for current access.
    pub access_proof: String,
}

/// The state a record is authorized against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    /// The slot the account set was observed at, decimal string.
    pub observed_slot: String,
    /// The wall-clock second the validity window is evaluated at, decimal string.
    pub now_unix_seconds: String,
    /// KMS management state for the signed pair.
    pub kms_pair_status: KmsPairStatus,
    /// The accounts that exist at this observation. Any account not listed does not exist.
    pub accounts: Vec<RecordedAccount>,
}

/// One account of an observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedAccount {
    /// Account address, hex.
    pub pubkey: String,
    /// Owning program, hex.
    pub owner: String,
    /// Account data, hex — verbatim, including any bytes beyond the decoded body.
    pub data: String,
}

/// Rule names used by [`ConnectorAuthVector::rule`].
///
/// Part of the cross-implementation contract: each implementation maps its own error type onto
/// these names. They are deliberately coarser than any implementation's error enum — no indices,
/// no counts — because they have to be expressible wherever the rule is evaluated.
pub mod rule {
    /// The permit names another host program.
    pub const DEPLOYMENT_PROGRAM_MISMATCH: &str = "deployment-program-mismatch";
    /// The permit names another cluster.
    pub const DEPLOYMENT_CHAIN_ID_MISMATCH: &str = "deployment-chain-id-mismatch";
    /// Handles of one request embed different chain ids.
    pub const MIXED_EMBEDDED_CHAIN_IDS: &str = "mixed-embedded-chain-ids";
    /// A handle embeds a chain id other than the signed one.
    pub const EMBEDDED_CHAIN_ID_MISMATCH: &str = "embedded-chain-id-mismatch";
    /// The validity window has not opened.
    pub const WINDOW_NOT_YET_VALID: &str = "window-not-yet-valid";
    /// The validity window has closed.
    pub const WINDOW_EXPIRED: &str = "window-expired";
    /// The permit started before its signer's last revocation.
    pub const PERMIT_INVALIDATED: &str = "permit-invalidated";
    /// The invalidation record is not a readable watermark for this user.
    pub const WATERMARK_RECORD_INVALID: &str = "watermark-record-invalid";
    /// The signed KMS pair is not servable. The class distinguishes why.
    pub const KMS_PAIR_UNSERVABLE: &str = "kms-pair-unservable";
    /// The request names no handles.
    pub const EMPTY_HANDLES: &str = "empty-handles";
    /// The request names more handles than one atomic account snapshot can cover.
    pub const TOO_MANY_HANDLES: &str = "too-many-handles";
    /// An access proof does not decode.
    pub const ACCESS_PROOF_MALFORMED: &str = "access-proof-malformed";
    /// An access proof decoded with bytes left over.
    pub const ACCESS_PROOF_TRAILING_BYTES: &str = "access-proof-trailing-bytes";
    /// An access proof carries more siblings than the tree can produce.
    pub const ACCESS_PROOF_TOO_MANY_SIBLINGS: &str = "access-proof-too-many-siblings";
    /// The lineage account does not exist at this observation.
    pub const ENCRYPTED_VALUE_ACCOUNT_ABSENT: &str = "encrypted-value-account-absent";
    /// The lineage account belongs to another program.
    pub const ENCRYPTED_VALUE_ACCOUNT_FOREIGN_OWNER: &str = "encrypted-value-account-foreign-owner";
    /// The account is host-owned but is not a lineage.
    pub const ENCRYPTED_VALUE_ACCOUNT_WRONG_TYPE: &str = "encrypted-value-account-wrong-type";
    /// The account carries the lineage type but its body does not decode.
    pub const ENCRYPTED_VALUE_ACCOUNT_MALFORMED: &str = "encrypted-value-account-malformed";
    /// The lineage's own fields derive a different identity than the one claimed.
    pub const ENCRYPTED_VALUE_ID_MISMATCH: &str = "encrypted-value-id-mismatch";
    /// The named handle is not the lineage's current handle.
    pub const HANDLE_NOT_CURRENT: &str = "handle-not-current";
    /// The subject is not a current member of the lineage.
    pub const SUBJECT_NOT_A_MEMBER: &str = "subject-not-a-member";
    /// The proof did not establish inclusion against the observed peaks — because it did not
    /// verify, or because it names a leaf position the observation does not have. Both are the
    /// same observable fact from the outside, and both are classified by the leaf count the
    /// request claimed: below the observed one means rebuild, at or above it means retry.
    pub const INCLUSION_PROOF_DOES_NOT_VERIFY: &str = "inclusion-proof-does-not-verify";
    /// The lineage's ACL domain is outside the signed scope.
    pub const DOMAIN_NOT_ALLOWED: &str = "domain-not-allowed";
    /// No delegation record exists for the tuple at this observation.
    pub const DELEGATION_ABSENT: &str = "delegation-absent";
    /// The delegation was revoked.
    pub const DELEGATION_REVOKED: &str = "delegation-revoked";
    /// The delegation expired before this observation.
    pub const DELEGATION_EXPIRED: &str = "delegation-expired";
    /// The delegation record was written after this observation.
    pub const DELEGATION_NEWER_THAN_OBSERVATION: &str = "delegation-newer-than-observation";
    /// The delegation record names a different tuple than its address.
    pub const DELEGATION_TUPLE_MISMATCH: &str = "delegation-tuple-mismatch";
    /// The delegation record belongs to another program.
    pub const DELEGATION_FOREIGN_OWNER: &str = "delegation-foreign-owner";
    /// The account at the delegation address is host-owned but is not a delegation record.
    pub const DELEGATION_WRONG_ACCOUNT_TYPE: &str = "delegation-wrong-account-type";
    /// Both the app-specific row and the delegator's wildcard row exist, and neither is live.
    pub const DELEGATION_NO_LIVE_GRANT: &str = "delegation-no-live-grant";

    /// Every rule name, for coverage checks.
    pub const ALL: &[&str] = &[
        DEPLOYMENT_PROGRAM_MISMATCH,
        DEPLOYMENT_CHAIN_ID_MISMATCH,
        MIXED_EMBEDDED_CHAIN_IDS,
        EMBEDDED_CHAIN_ID_MISMATCH,
        WINDOW_NOT_YET_VALID,
        WINDOW_EXPIRED,
        PERMIT_INVALIDATED,
        WATERMARK_RECORD_INVALID,
        KMS_PAIR_UNSERVABLE,
        EMPTY_HANDLES,
        TOO_MANY_HANDLES,
        ACCESS_PROOF_MALFORMED,
        ACCESS_PROOF_TRAILING_BYTES,
        ACCESS_PROOF_TOO_MANY_SIBLINGS,
        ENCRYPTED_VALUE_ACCOUNT_ABSENT,
        ENCRYPTED_VALUE_ACCOUNT_FOREIGN_OWNER,
        ENCRYPTED_VALUE_ACCOUNT_WRONG_TYPE,
        ENCRYPTED_VALUE_ACCOUNT_MALFORMED,
        ENCRYPTED_VALUE_ID_MISMATCH,
        HANDLE_NOT_CURRENT,
        SUBJECT_NOT_A_MEMBER,
        INCLUSION_PROOF_DOES_NOT_VERIFY,
        DOMAIN_NOT_ALLOWED,
        DELEGATION_ABSENT,
        DELEGATION_REVOKED,
        DELEGATION_EXPIRED,
        DELEGATION_NEWER_THAN_OBSERVATION,
        DELEGATION_TUPLE_MISMATCH,
        DELEGATION_FOREIGN_OWNER,
        DELEGATION_WRONG_ACCOUNT_TYPE,
        DELEGATION_NO_LIVE_GRANT,
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

impl ConnectorAuthVector {
    /// Looks up this record's transport key in the file's table.
    pub fn transport_key_bytes(&self, file: &ConnectorAuthVectorFile) -> Option<Vec<u8>> {
        file.transport_keys
            .get(&self.request.permit.transport_key)
            .and_then(|hex| from_hex(hex))
    }
}
