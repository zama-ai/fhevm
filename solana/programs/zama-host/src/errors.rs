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
    /// The instruction included undeclared trailing account metas.
    #[msg("instruction has unexpected remaining accounts")]
    UnexpectedRemainingAccounts,
    /// The attestation's handle list is empty or oversized, or its extra data exceeds the limit.
    #[msg("input attestation payload is malformed")]
    MalformedInputAttestation,
    /// The selected input handle index is outside the attestation handle list.
    #[msg("attestation handle index is invalid")]
    InvalidInputHandleIndex,
    /// The selected input handle does not match the requested handle.
    #[msg("attestation selected handle does not match")]
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
    /// The signer for an output does not match the encrypted value account authority the
    /// execution declared for it.
    #[msg("signer does not match the declared encrypted value account authority")]
    EncryptedValueAccountAuthorityMismatch,
    /// A deny-list witness is required but was not supplied.
    #[msg("deny-list witness account is required")]
    DenyRecordMissing,
    /// A deny-list witness is not canonical or has invalid contents.
    #[msg("deny-list account does not match the canonical PDA")]
    DenyRecordMismatch,
    /// The grant authority is denied by the configured deny-list.
    #[msg("grant authority subject is deny-listed")]
    SubjectDenied,
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
    /// An fhe_execute instruction exceeded the supported operation count.
    #[msg("fhe_execute operation count is invalid")]
    InvalidFheExecuteOperationCount,
    /// An fhe_execute instruction referenced a missing or malformed dynamic account.
    #[msg("fhe_execute account reference is invalid")]
    InvalidFheExecuteAccount,
    /// An fhe_execute instruction referenced a step output that no earlier step produced.
    #[msg("fhe_execute transient operand is missing")]
    FheExecuteEarlierStepMissing,
    /// An fhe_execute instruction produced the same handle twice, transient or persistent.
    #[msg("fhe_execute output handle is duplicated")]
    FheExecuteDuplicateHandle,
    /// An fhe_execute persistent output account already exists.
    #[msg("fhe_execute persistent output ACL record already exists")]
    FheExecuteOutputAlreadyInitialized,
    /// An execution containing a rand step must declare at least one persistent output,
    /// which anchors the compulsorily fresh rand seed (fhevm-internal#1853 W4).
    #[msg("fhe_execute rand step requires a persistent output in the execution")]
    FheExecuteRandRequiresPersistentOutput,
    /// A KMS context was defined with a duplicate signer address.
    #[msg("KMS context signer set contains a duplicate address")]
    DuplicateKmsSigner,
    /// The coprocessor-attested contract does not match the `fhe_execute` compute subject.
    #[msg("attested contract address does not match the output app account")]
    InputBindContractMismatch,
    /// An `fhe_execute` execution's summed HCU exceeds `max_hcu_per_tx` (or the running sum overflowed).
    #[msg("FHE op total HCU exceeds the per-transaction limit")]
    HcuTransactionLimitExceeded,
    /// An `fhe_execute` value's critical-path HCU exceeds `max_hcu_depth_per_tx` (or the depth sum overflowed).
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
    // ---- EncryptedValue ACL model ----
    /// An `EncryptedValue` account is not the canonical PDA for its encrypted value ID.
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
    /// Persistent `EncryptedValue` creation was requested with an empty subject list.
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
    /// A metering-band cap was set below `max_hcu_per_tx`, making a single legal execution impossible.
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

    /// Under a finite `hcu_block_cap_per_app`, an execution that binds no persistent input, no verified
    /// input, and no persistent output leaves `compute_subject` a free variable: the caller could
    /// churn fresh subjects to mint fresh per-slot meters and evade the cap (fhevm-internal#1744).
    /// Such an execution is also value-less — its transient outputs create no ACL leaf and are
    /// undecryptable — so it is rejected outright.
    #[msg("FHE execution anchors no persistent/verified binding under a finite HCU block cap")]
    FheExecuteUnanchoredUnderBlockCap,

    // ---- stateless public-decrypt verifier (verify_public_decrypt, fhevm-internal#1704) ----
    /// The supplied KMS context account is destroyed, is not the canonical PDA for the id the
    /// certificate committed to via signed `extra_data`, or has a mismatched stored id. Verification
    /// binds to the cert-named context (any live context), so a destroyed context — or an account
    /// that is not the one the cert names — fails closed here.
    #[msg("KMS context is destroyed or does not match the certificate's committed context")]
    InvalidKmsContext,
    /// The KMS `PublicDecryptVerification` certificate failed secp256k1 threshold verification
    /// against the cert-named context's signer set.
    #[msg("KMS public-decrypt certificate is invalid")]
    InvalidKmsCertificate,
    /// The MMR public-decrypt inclusion proof does not prove the exact handle public against the
    /// encrypted value account's current peaks.
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
    /// The KMS signer set contains the zero address, which can never be a valid recovered EVM signer.
    #[msg("KMS signer set contains the zero address")]
    ZeroKmsSigner,
    /// The supplied invalidation account is not at the canonical watermark address for
    /// the signing user, or its stored bump is not the canonical one. The watermark is
    /// keyed by the signer precisely so that one user cannot move another user's
    /// watermark; this is where that is enforced.
    #[msg("permit invalidation account is not the canonical account for the signer")]
    PermitInvalidationPdaMismatch,
    /// The account at the canonical watermark address is not a watermark record this
    /// program wrote: owned by another program, of the wrong size, carrying another
    /// record type's discriminator, or naming a different user than the signer. Rejected
    /// rather than reinterpreted or overwritten.
    #[msg("permit invalidation account is not a valid watermark record")]
    PermitInvalidationAccountInvalid,
    /// The runtime clock reports a time before the unix epoch. The watermark is an
    /// unsigned number of seconds, and coercing a negative time into it would jump the
    /// watermark to the far future and kill every permit the user will ever sign — so
    /// this fails closed instead.
    #[msg("clock is before the unix epoch")]
    ClockBeforeEpoch,

    /// A step referenced an interned dictionary index past the end of `FheExecuteArgs::dictionary`.
    #[msg("fhe_execute dictionary index out of bounds")]
    FheExecuteDictionaryIndexOutOfBounds,
    /// `FheExecuteArgs::account_count` does not match the actual remaining-accounts length.
    #[msg("fhe_execute declared account count mismatch")]
    FheExecuteAccountCountMismatch,
    /// A `StoredValue` operand referenced an account written by an earlier step.
    /// In-execution dependencies must use `EarlierStep`.
    #[msg("fhe_execute persistent operand was written earlier in the execution")]
    FheExecutePersistentOperandWrittenEarlier,
    /// An interned dictionary entry was never referenced by any step; an execution must not
    /// carry dead bytes.
    #[msg("fhe_execute dictionary entry is not referenced by any step")]
    FheExecuteDictionaryEntryUnreferenced,
    /// `0` is not a valid per-tx HCU limit: `u64::MAX` is the single "unlimited"
    /// sentinel across every HCU knob, and a `0` limit would reject every execution.
    #[msg("0 is not a valid HCU limit; use u64::MAX for unlimited")]
    HcuLimitZeroReserved,
}
