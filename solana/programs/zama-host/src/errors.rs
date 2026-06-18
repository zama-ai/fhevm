//! Program errors for the ZamaHost Anchor program.
//!
//! Error names are part of the client-facing Anchor IDL. Keep them specific to
//! the failed invariant so tests and off-chain tooling can assert on the reason
//! without parsing logs.

use anchor_lang::prelude::*;

/// Errors returned by ZamaHost instruction handlers.
#[error_code]
pub enum ZamaHostError {
    /// The signer is not the configured host admin.
    #[msg("host config admin does not match signer")]
    HostConfigAdminMismatch,
    /// A production-shaped instruction was attempted while the host is paused.
    #[msg("host config account is paused")]
    HostConfigPaused,
    /// The host config account is not the canonical singleton or has invalid shape.
    #[msg("host config account is invalid")]
    HostConfigMismatch,
    /// The host config initializer supplied zero or unsupported fields.
    #[msg("host config initialization fields are invalid")]
    InvalidHostConfig,
    /// The instruction included undeclared trailing account metas.
    #[msg("instruction has unexpected remaining accounts")]
    UnexpectedRemainingAccounts,
    /// The mock encrypted-input bind path is disabled in [`HostConfig`](crate::HostConfig).
    #[msg("mock input binding is disabled")]
    MockInputDisabled,
    /// The mock input signer is not the configured input verifier authority.
    #[msg("mock input verifier authority does not match config")]
    MockInputVerifierMismatch,
    /// The input verifier authority account does not match host config.
    #[msg("input verifier authority does not match config")]
    InputVerifierMismatch,
    /// The material commitment authority account does not match host config.
    #[msg("material authority does not match config")]
    MaterialAuthorityMismatch,
    /// A material commitment account is not the canonical PDA for its ACL record.
    #[msg("material commitment account does not match the canonical PDA")]
    MaterialCommitmentPdaMismatch,
    /// A material commitment payload or account is invalid.
    #[msg("material commitment is invalid")]
    InvalidMaterialCommitment,
    /// The ACL record is already sealed to a material commitment.
    #[msg("ACL record already has sealed material")]
    MaterialAlreadySealed,
    /// A signed input proof has an invalid handle list, payload, or binding.
    #[msg("input proof is invalid")]
    InvalidInputProof,
    /// The selected input handle index is outside the proof handle list.
    #[msg("input proof handle index is invalid")]
    InvalidInputHandleIndex,
    /// The selected input handle does not match the requested handle.
    #[msg("input proof selected handle does not match")]
    InvalidInputHandle,
    /// The coprocessor EIP-712 input attestation failed secp256k1 threshold verification.
    #[msg("coprocessor input attestation is invalid")]
    InvalidInputAttestation,
    /// The gateway verifier config (coprocessor signer / verifying contract) is unset.
    #[msg("gateway verifier config is not set")]
    GatewayVerifierConfigUnset,
    /// A KMS context must define at least one signer.
    #[msg("KMS context has no signers")]
    EmptyKmsContext,
    /// A KMS context exceeds the maximum supported signer count.
    #[msg("KMS context exceeds the maximum signer count")]
    TooManyKmsSigners,
    /// A KMS threshold is zero or exceeds the signer count.
    #[msg("KMS context threshold is invalid")]
    InvalidKmsThreshold,
    /// A new KMS context id must be the current id plus one (monotonic).
    #[msg("KMS context id is not the next sequential id")]
    InvalidKmsContextId,
    /// The current active KMS context cannot be destroyed.
    #[msg("current KMS context cannot be destroyed")]
    CurrentKmsContextCannotBeDestroyed,
    /// The input handle version byte is unsupported.
    #[msg("input handle version is unsupported")]
    InvalidInputHandleVersion,
    /// The input handle chain id does not match host config.
    #[msg("input handle chain id does not match host config")]
    InvalidInputHandleChain,
    /// The input handle FHE type id is unsupported.
    #[msg("input handle FHE type is unsupported")]
    InvalidInputHandleType,
    /// The requested FHE type id is unsupported.
    #[msg("FHE type is unsupported")]
    UnsupportedFheType,
    /// A binary operation operand type does not match the operation type.
    #[msg("binary FHE operand type is incompatible")]
    BinaryOperandTypeMismatch,
    /// A bounded random request has an invalid upper bound.
    #[msg("bounded random upper bound is invalid")]
    InvalidRandomUpperBound,
    /// No matching Ed25519 verifier pre-instruction was found for the input proof.
    #[msg("input proof Ed25519 signature is missing or malformed")]
    InputProofSignatureMissing,
    /// A test event shim was called while test shims are disabled.
    #[msg("test event shims are disabled")]
    TestShimsDisabled,
    /// The test event shim signer is not the configured test authority.
    #[msg("test shim authority does not match config")]
    TestShimAuthorityMismatch,
    /// The app account did not sign the ACL birth instruction.
    #[msg("ACL app account authority does not match app account")]
    AppAccountAuthorityMismatch,
    /// The supplied nonce key does not match the app metadata fields.
    #[msg("ACL record nonce key does not match")]
    AclNonceKeyMismatch,
    /// The ACL record account is not the canonical PDA for its stored nonce.
    #[msg("ACL record address is not the canonical PDA for its nonce key")]
    AclRecordPdaMismatch,
    /// The ACL record nonce sequence does not match the expected sequence.
    #[msg("ACL record nonce sequence does not match")]
    AclNonceSequenceMismatch,
    /// The ACL record domain key does not match the expected domain.
    #[msg("ACL record domain key does not match")]
    AclDomainKeyMismatch,
    /// The ACL record app account does not match the expected app account.
    #[msg("ACL record app account does not match")]
    AclAppAccountMismatch,
    /// The ACL record encrypted value label does not match the expected label.
    #[msg("ACL record encrypted value label does not match")]
    AclEncryptedValueLabelMismatch,
    /// The ACL record stores a different handle than the instruction requested.
    #[msg("ACL record handle does not match")]
    AclHandleMismatch,
    /// The subject is not allowed by the ACL record.
    #[msg("ACL record subject is not allowed")]
    AclSubjectMismatch,
    /// The subject exists but does not carry the role required by the instruction.
    #[msg("ACL record subject lacks the required role")]
    AclSubjectRoleMismatch,
    /// Public decrypt release must happen through allow_for_decryption after handle birth.
    #[msg("public decrypt cannot be set during ACL record birth")]
    PublicDecryptAtBirthUnsupported,
    /// Inline subject capacity was exceeded without overflow witnesses, or input was empty.
    #[msg("ACL record has too many inline subjects")]
    AclSubjectCapacityExceeded,
    /// An overflow permission PDA is required but was not supplied.
    #[msg("ACL permission account is required for an overflow subject")]
    AclPermissionMissing,
    /// An overflow permission account is not the canonical PDA.
    #[msg("ACL permission account does not match the canonical PDA")]
    AclPermissionPdaMismatch,
    /// An overflow permission account does not match the requested record or subject.
    #[msg("ACL permission account data does not match the requested subject")]
    AclPermissionMismatch,
    /// A deny-list witness is required but was not supplied.
    #[msg("ACL deny-list account is required")]
    AclDenyRecordMissing,
    /// A deny-list witness is not canonical or has invalid contents.
    #[msg("ACL deny-list account does not match the canonical PDA")]
    AclDenyRecordMismatch,
    /// The grant authority is denied by the configured deny-list.
    #[msg("ACL authority subject is deny-listed")]
    AclSubjectDenied,
    /// A delegation account is not the canonical PDA for its tuple.
    #[msg("delegation record does not match the canonical PDA")]
    DelegationPdaMismatch,
    /// A delegation tuple is self-referential, expired, or otherwise invalid.
    #[msg("delegation tuple is invalid")]
    InvalidDelegation,
    /// The delegation has already been revoked.
    #[msg("delegation has already been revoked")]
    DelegationRevoked,
    /// Delegation state was already updated in the current slot.
    #[msg("delegation was already updated in the current slot")]
    DelegationUpdatedInCurrentSlot,
    /// The slot-hash sysvar did not contain the expected previous hash.
    #[msg("previous bank hash is not available")]
    PreviousBankHashUnavailable,
    /// A caller-supplied output handle does not match the host formula.
    #[msg("computed handle does not match host formula")]
    ComputedHandleMismatch,
    /// A PDA account was not fresh or canonical after creation.
    #[msg("PDA creation target is invalid")]
    PdaCreationMismatch,
    /// An FHE eval instruction exceeded the supported operation count.
    #[msg("FHE eval operation count is invalid")]
    InvalidFheEvalOperationCount,
    /// An FHE eval instruction would exceed the bounded event log budget.
    #[msg("FHE eval event log budget is exceeded")]
    FheEvalEventLogBudgetExceeded,
    /// An FHE eval instruction referenced a missing or malformed dynamic account.
    #[msg("FHE eval account reference is invalid")]
    InvalidFheEvalAccount,
    /// An FHE eval instruction referenced a transient output that was not produced earlier.
    #[msg("FHE eval transient operand is missing")]
    FheEvalTransientMissing,
    /// An FHE eval instruction produced the same transient handle twice.
    #[msg("FHE eval output handle is duplicated")]
    FheEvalDuplicateHandle,
    /// An FHE eval durable output account already exists.
    #[msg("FHE eval durable output ACL record already exists")]
    FheEvalOutputAlreadyInitialized,
    /// An FHE eval context id must be non-zero.
    #[msg("FHE eval context id is invalid")]
    InvalidFheEvalContext,
    /// A transient session nonce must be non-zero.
    #[msg("transient session nonce is invalid")]
    InvalidTransientSessionNonce,
    /// A transient session account is not canonical for its authority and nonce.
    #[msg("transient session account does not match the canonical PDA")]
    TransientSessionPdaMismatch,
    /// A transient session has an invalid state for the requested operation.
    #[msg("transient session state is invalid")]
    TransientSessionStateInvalid,
    /// A transient session is expired for capability consumption.
    #[msg("transient session is expired")]
    TransientSessionExpired,
    /// A transient session authority does not match the required signer.
    #[msg("transient session authority does not match signer")]
    TransientSessionAuthorityMismatch,
    /// A transient session refund recipient does not match the stored recipient.
    #[msg("transient session refund recipient does not match")]
    TransientSessionRefundMismatch,
    /// A transient session capacity is invalid.
    #[msg("transient session capacity is invalid")]
    TransientSessionCapacityInvalid,
    /// A transient session capability is missing or does not match the requested handle.
    #[msg("transient capability is missing or mismatched")]
    TransientCapabilityMismatch,
    /// A transient session capability has already been consumed.
    #[msg("transient capability is consumed")]
    TransientCapabilityConsumed,
    /// A transient session consume did not prove same-transaction creation.
    #[msg("transient session creation was not found in this transaction")]
    TransientSessionCreationMissing,
    /// A transient session capability does not authorize the requested role or subject.
    #[msg("transient capability is not authorized")]
    TransientCapabilityUnauthorized,
    /// A transient capability cannot authorize the requested durable output.
    #[msg("transient capability does not allow this durable output")]
    TransientCapabilityOutputDenied,
    /// Transient capabilities are not allowed to authorize public decrypt.
    #[msg("transient capability cannot authorize public decrypt")]
    TransientCapabilityPublicDecryptDenied,
    /// A transient capability requires the instructions sysvar for receiver validation.
    #[msg("transient capability receiver cannot be verified")]
    TransientCapabilityReceiverMissing,
    /// A KMS context was defined with a duplicate signer address.
    #[msg("KMS context signer set contains a duplicate address")]
    DuplicateKmsSigner,
    /// The coprocessor-attested contract does not match the output ACL app account.
    #[msg("attested contract address does not match the output app account")]
    InputBindContractMismatch,
    /// The coprocessor-attested user is not among the output ACL subjects.
    #[msg("attested user address is not an output ACL subject")]
    InputBindUserNotSubject,
    /// A value derived from a verified external input may not be parked in a transient session:
    /// the session capability does not carry the attested binding, so consuming it later would
    /// drop the replay guard. Such values must flow to a durable output that binds the attestation.
    #[msg("verified-input-derived value cannot be written to a transient session")]
    InputBindTransientSessionUnsupported,
}
