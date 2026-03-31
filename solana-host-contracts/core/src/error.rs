use crate::types::{FheType, KmsContextId};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HostContractError {
    #[error("contract is paused")]
    Paused,
    #[error("sender is denied")]
    SenderDenied,
    #[error("sender is not allowed")]
    SenderNotAllowed,
    #[error("acl not allowed for caller")]
    ACLNotAllowed,
    #[error("handles list is empty")]
    HandlesListIsEmpty,
    #[error("delegate cannot be the contract address")]
    DelegateCannotBeContractAddress,
    #[error("sender cannot be the contract address")]
    SenderCannotBeContractAddress,
    #[error("sender cannot be the delegate")]
    SenderCannotBeDelegate,
    #[error("delegation not found")]
    NotDelegatedYet,
    #[error("delegation expiration is in the past")]
    ExpirationDateInThePast,
    #[error("delegation or revoke already performed in this slot")]
    AlreadyDelegatedOrRevokedInSameSlot,
    #[error("expiration date already set to same value")]
    ExpirationDateAlreadySetToSameValue,
    #[error("account already blocked")]
    AccountAlreadyBlocked,
    #[error("account is not blocked")]
    AccountNotBlocked,
    #[error("not a pauser")]
    NotPauser,
    #[error("signers set is empty")]
    SignersSetIsEmpty,
    #[error("too many signers, max {max}")]
    TooManySigners { max: usize },
    #[error("threshold cannot be null")]
    ThresholdIsNull,
    #[error("threshold is above signer count")]
    ThresholdIsAboveNumberOfSigners,
    #[error("signer is null")]
    SignerNull,
    #[error("signer already registered")]
    SignerAlreadyRegistered,
    #[error("kms signer already registered")]
    KmsAlreadySigner,
    #[error("invalid signature threshold: got {got}, need at least {needed}")]
    SignatureThresholdNotReached {
        got: usize,
        needed: crate::types::SignatureThreshold,
    },
    #[error("invalid kms signer")]
    InvalidKmsSigner,
    #[error("invalid signer")]
    InvalidSigner,
    #[error("empty input proof")]
    EmptyInputProof,
    #[error("input proof exceeds max size {max}")]
    InputProofTooLarge { max: usize },
    #[error("failed to deserialize input proof")]
    DeserializingInputProofFail,
    #[error("failed to deserialize decryption proof")]
    DeserializingDecryptionProofFail,
    #[error("failed to deserialize extra data")]
    DeserializingExtraDataFail,
    #[error("empty decryption proof")]
    EmptyDecryptionProof,
    #[error("decryption proof exceeds max size {max}")]
    DecryptionProofTooLarge { max: usize },
    #[error("too many handles in proof, max {max}")]
    TooManyHandlesInProof { max: usize },
    #[error("too many decryption handles, max {max}")]
    TooManyDecryptionHandles { max: usize },
    #[error("decrypted result exceeds max size {max}")]
    DecryptedResultTooLarge { max: usize },
    #[error("invalid chain id: expected {expected}, got {found}")]
    InvalidChainId { expected: u64, found: u64 },
    #[error("invalid index")]
    InvalidIndex,
    #[error("invalid input handle")]
    InvalidInputHandle,
    #[error("invalid handle version: expected {expected}, got {found}")]
    InvalidHandleVersion { expected: u8, found: u8 },
    #[error("unsupported extra data version {0}")]
    UnsupportedExtraDataVersion(u8),
    #[error("invalid kms context {0:?}")]
    InvalidKmsContext(KmsContextId),
    #[error("current kms context cannot be destroyed")]
    CurrentKmsContextCannotBeDestroyed,
    #[error("incompatible types")]
    IncompatibleTypes,
    #[error("unsupported type {0:?}")]
    UnsupportedType(FheType),
    #[error("division by zero")]
    DivisionByZero,
    #[error("invalid type")]
    InvalidType,
    #[error("operation requires a scalar second operand")]
    IsNotScalar,
    #[error("scalar flag is not boolean")]
    ScalarByteIsNotBoolean,
    #[error("not a power of two")]
    NotPowerOfTwo,
    #[error("upper bound above maximum type value")]
    UpperBoundAboveMaxTypeValue,
    #[error("unsupported operator {0:?}")]
    UnsupportedOperator(crate::types::Operator),
    #[error("hcu block limit exceeded")]
    HCUBlockLimitExceeded,
    #[error("hcu transaction limit exceeded")]
    HCUTransactionLimitExceeded,
    #[error("hcu transaction depth limit exceeded")]
    HCUTransactionDepthLimitExceeded,
    #[error("hcu per block below max per transaction")]
    HCUPerBlockBelowMaxPerTx,
    #[error("max hcu per transaction below max hcu depth")]
    MaxHCUPerTxBelowDepth,
    #[error("unsupported operation pricing")]
    UnsupportedOperationPricing,
    #[error("account already block-whitelisted")]
    AlreadyBlockHCUWhitelisted,
    #[error("account is not block-whitelisted")]
    NotBlockHCUWhitelisted,
    #[error("caller is not the upgrade authority")]
    NotUpgradeAuthority,
    #[error("invalid state version transition: current {current}, requested {requested}")]
    InvalidStateVersionTransition { current: u32, requested: u32 },
}

pub type Result<T> = std::result::Result<T, HostContractError>;
