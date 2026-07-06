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
    /// Public decrypt release must happen through an explicit make-public instruction, never at birth.
    #[msg("public decrypt cannot be set at encrypted value birth")]
    PublicDecryptAtBirthUnsupported,
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
    FheEvalAllowedLocalMissing,
    /// An FHE eval instruction produced the same transient handle twice.
    #[msg("FHE eval output handle is duplicated")]
    FheEvalDuplicateHandle,
    /// An FHE eval durable output account already exists.
    #[msg("FHE eval durable output ACL record already exists")]
    FheEvalOutputAlreadyInitialized,
    /// An FHE eval context id must be non-zero.
    #[msg("FHE eval context id is invalid")]
    InvalidFheEvalContext,
    /// A derived durable output may not be made public-decryptable by a non-authorized subject.
    #[msg("transient capability cannot authorize public decrypt")]
    DerivedOutputPublicDecryptDenied,
    /// A KMS context was defined with a duplicate signer address.
    #[msg("KMS context signer set contains a duplicate address")]
    DuplicateKmsSigner,
    /// The coprocessor-attested contract does not match the `fhe_eval` compute subject.
    #[msg("attested contract address does not match the output app account")]
    InputBindContractMismatch,
    /// The coprocessor-attested user is not among the output ACL subjects.
    #[msg("attested user address is not an output ACL subject")]
    InputBindUserNotSubject,
    /// An `fhe_eval` frame's summed HCU exceeds `max_hcu_per_tx` (or the running sum overflowed).
    #[msg("FHE op total HCU exceeds the per-transaction limit")]
    HcuTransactionLimitExceeded,
    /// An `fhe_eval` value's critical-path HCU exceeds `max_hcu_depth_per_tx` (or the depth sum overflowed).
    #[msg("FHE op depth HCU exceeds the per-transaction depth limit")]
    HcuTransactionDepthLimitExceeded,
    /// The HCU cost table has no row for this op / FHE type / scalar combination (fail-closed).
    #[msg("no HCU cost is defined for this op / type / scalar combination")]
    HcuUnknownCost,
    /// A limit setter would violate the ordering invariant `max_hcu_per_tx >= max_hcu_depth_per_tx`.
    #[msg("HCU limits violate max_hcu_per_tx >= max_hcu_depth_per_tx")]
    HcuLimitOrderingInvalid,
    /// The attested `contract_chain_id` does not match the host chain id (EVM `contractChainId == block.chainid`).
    #[msg("attested contract chain id does not match the host chain id")]
    AttestationChainIdMismatch,

    // ---- RFC-024 EncryptedValue ACL model ----
    /// An `EncryptedValue` account is not the canonical PDA for its value key.
    #[msg("encrypted value account does not match the canonical PDA")]
    EncryptedValuePdaMismatch,
    /// An `EncryptedValue` account has an unexpected owner or discriminator.
    #[msg("encrypted value account is not a valid EncryptedValue account")]
    EncryptedValueAccountInvalid,
    /// A subject list would exceed `MAX_ENCRYPTED_VALUE_SUBJECTS`.
    #[msg("encrypted value subject capacity exceeded")]
    EncryptedValueSubjectCapacityExceeded,
    /// `previous_handle`/`previous_subjects` did not match the account's current state.
    #[msg("encrypted value previous state does not match the account")]
    PreviousStateMismatch,
    /// The caller subject is not allowed by the encrypted value.
    #[msg("encrypted value subject is not allowed")]
    SubjectNotAllowed,
    /// The caller subject is not a current member of the encrypted value.
    #[msg("encrypted value subject is not a current member")]
    SubjectNotFound,
    /// `create_encrypted_value` was called with an empty subject list.
    #[msg("encrypted value must be created with at least one subject")]
    EncryptedValueEmptySubjects,
    /// `remove_subject` would leave the encrypted value with no current subjects.
    #[msg("encrypted value must retain at least one subject")]
    EncryptedValueLastSubject,
    /// The MMR peaks/leaf-count invariant was violated.
    #[msg("encrypted value MMR state is inconsistent")]
    EncryptedValueMmrInconsistent,
    /// The MMR peak count reached the representational cap.
    #[msg("encrypted value MMR peak capacity exceeded")]
    EncryptedValueMmrPeakCapacityExceeded,
}
