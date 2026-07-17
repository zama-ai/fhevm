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
    /// Retired error slot retained to preserve later Anchor error discriminants.
    /// Born-public outputs now use one bounded lifecycle batch per eval frame.
    #[msg("reserved FHE eval produced-public transport error")]
    FheEvalBornPublicFrameTooLarge,
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
    /// Raw `EncryptedValue` create/update would accept caller-chosen handles without provenance.
    #[msg("raw encrypted value lifecycle is disabled; use fhe_eval durable outputs")]
    RawEncryptedValueLifecycleDisabled,

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
    /// `make_handle_public` named a handle that is not the account's current handle.
    #[msg("encrypted value public handle does not match the account")]
    EncryptedValuePublicHandleMismatch,
    /// The caller subject is not allowed by the encrypted value.
    #[msg("encrypted value subject is not allowed")]
    SubjectNotAllowed,
    /// The caller subject is not a current member of the encrypted value.
    #[msg("encrypted value subject is not a current member")]
    SubjectNotFound,
    /// Durable `EncryptedValue` creation was requested with an empty subject list.
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
    /// The per-app in-slot HCU would exceed the block cap; also the `cap == 0` ban and a meter
    /// accumulation overflow (all fail closed). Analog of EVM `HCUBlockLimitExceeded`.
    #[msg("per-app in-slot HCU exceeds the block cap")]
    HcuBlockLimitExceeded,
    /// A metered (untrusted) app forwarded no block meter — fail closed rather than un-metered.
    #[msg("HCU block meter account is required for a metered app")]
    HcuBlockMeterMissing,
    /// The supplied block meter is not the canonical PDA / owner / recorded app.
    #[msg("HCU block meter account does not match the canonical PDA")]
    HcuBlockMeterMismatch,
    /// A present trust witness is not the canonical PDA / owner (only an absent witness is benign).
    #[msg("HCU trusted-app record does not match the canonical PDA")]
    HcuTrustedAppRecordMismatch,
    /// A metering-band cap was set below `max_hcu_per_tx`, making a single legal frame impossible.
    /// Analog of EVM `HCUPerBlockBelowMaxPerTx`.
    #[msg("HCU block cap is below max_hcu_per_tx")]
    HcuBlockCapBelowMaxPerTx,

    /// `fheMulDiv` divisor is a plaintext scalar that must never be zero (EVM parity).
    #[msg("fheMulDiv divisor must be non-zero")]
    MulDivDivisorZero,

    /// `fheDiv`/`fheRem` require a plaintext scalar divisor (EVM `IsNotScalar`).
    #[msg("fheDiv/fheRem divisor must be a plaintext scalar")]
    DivisorMustBeScalar,

    /// `fheDiv`/`fheRem` divisor is zero once truncated to the operand type (EVM `DivisionByZero`).
    #[msg("fheDiv/fheRem divisor must be non-zero")]
    DivisionByZero,

    /// The host `chain_id` does not carry the RFC-021 Solana chain-type high bit,
    /// or the EVM `gateway_chain_id` carries it. The ZamaHost is always a Solana
    /// host chain, so its chain id must set bit 63 while the gateway id (an EVM
    /// chain) must leave it clear.
    #[msg(
        "host chain id must set the Solana chain-type high bit and the gateway chain id must not"
    )]
    InvalidChainTypeBit,

    /// Under a finite `hcu_block_cap_per_app`, a frame that binds no durable input, no verified
    /// input, and no durable output leaves `compute_subject` a free variable: the caller could
    /// rotate fresh subjects to mint fresh per-slot meters and evade the cap (fhevm-internal#1744).
    /// Such a frame is also value-less — its transient outputs create no ACL leaf and are
    /// undecryptable — so it is rejected outright.
    #[msg("FHE eval frame anchors no durable/verified binding under a finite HCU block cap")]
    FheEvalUnanchoredUnderBlockCap,

    // ---- stateless public-decrypt verifier (verify_public_decrypt, fhevm-internal#1704) ----
    /// The KMS context is destroyed, is not the current canonical context, or is not the context
    /// the certificate committed to via signed `extra_data`. Verification binds to the CURRENT
    /// context, so a cert minted under a rotated-out context fails closed here.
    #[msg("KMS context is destroyed, not current, or does not match the certificate")]
    InvalidKmsContext,
    /// The KMS `PublicDecryptVerification` certificate failed secp256k1 threshold verification
    /// against the current context's signer set.
    #[msg("KMS public-decrypt certificate is invalid")]
    InvalidKmsCertificate,
    /// The MMR public-decrypt inclusion proof does not prove the exact handle public against the
    /// lineage's current peaks.
    #[msg("public-decrypt inclusion proof is invalid")]
    PublicDecryptProofInvalid,

    /// The coprocessor signer set is empty. Input verification requires at least one registered
    /// signer (analog of `EmptyKmsContext` for the coprocessor path).
    #[msg("coprocessor signer set must not be empty")]
    EmptyCoprocessorSignerSet,
    /// The coprocessor signer set exceeds `HostConfig::MAX_COPROCESSOR_SIGNERS` (analog of
    /// `TooManyKmsSigners`).
    #[msg("coprocessor signer set exceeds the maximum size")]
    TooManyCoprocessorSigners,
    /// The coprocessor threshold is zero or greater than the signer count; a valid n-of-m needs
    /// `1 <= threshold <= set.len()` (analog of `InvalidKmsThreshold`).
    #[msg("coprocessor threshold must be between 1 and the signer count")]
    InvalidCoprocessorThreshold,
    /// The coprocessor signer set contains a duplicate address. Threshold verification counts
    /// DISTINCT recovered signers, so a duplicate would silently raise the effective quorum
    /// (analog of `DuplicateKmsSigner`).
    #[msg("coprocessor signer set contains a duplicate signer")]
    DuplicateCoprocessorSigner,
    /// The coprocessor signer set contains the zero address, which can never be a valid recovered
    /// EVM signer.
    #[msg("coprocessor signer set contains the zero address")]
    ZeroCoprocessorSigner,
}
