//! App-facing helpers for preparing `zama-host` fhe_execute batch requests.
//!
//! This crate targets the role-aware host ABI. App code describes encrypted
//! operands and persistent outputs by pubkey; [`BatchBuilder`] validates the batch,
//! assigns host account indices, and records the signer/writable requirements for
//! every dynamic account. With the `cpi` feature, [`Batch::resolve_accounts`]
//! preflights the dynamic account set and [`invoke_batch_signed_resolved`] turns
//! the batch plus resolved accounts into the exact `zama-host` CPI.
//!
//! The builder intentionally targets the current role-aware host eval ABI rather
//! than the older `execute_frame` prototype. Instruction-local intermediate
//! values are returned by builder methods as typed transient [`Encrypted`] values;
//! only [`Output::persistent`] creates ACL state. Binary, ternary, trivial-encrypt,
//! rand, and verified input steps can be composed in one batch.

#![allow(unexpected_cfgs)]

mod accounts;
mod acl;
mod batch;
mod builder;
mod cpi;
mod lower;
mod operand;
#[cfg(test)]
mod tests;
mod types;
mod validate;

pub use accounts::{
    BatchAccountPurpose, BatchAppAuthority, EvalAccountRequirement, EvalOutputAuthorityRequirement,
};
#[cfg(feature = "cpi")]
pub use accounts::{EvalAccountResolutionError, ResolvedEvalAccounts};
pub use acl::{
    BoundedU64UpperBound, EncryptedValueId, Output, PersistentLabel, PersistentOutput,
    PersistentOutputBinding,
};
pub use batch::Batch;
pub use builder::BatchBuilder;
#[cfg(feature = "cpi")]
pub use cpi::{
    invoke_batch_signed_resolved, invoke_batch_signed_with_builder, BatchCpiAccounts,
    BatchInvokeError,
};
pub use types::{
    Address, BinaryRhs, Bool, BoolHandle, Bytes256, Encrypted, FheBitwise, FheEq, FheIsIn, FheNeg,
    FheNot, FheRandom, FheShift, FheType, FheTyped, FheUint, Scalar, Uint, Uint64Handle,
};

/// Result type used by the builder helpers.
pub type Result<T> = std::result::Result<T, BatchBuildError>;

/// Builder failures that can be detected before invoking the host program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchBuildError {
    /// More accounts were referenced than fit in the host's `u8` wire indices.
    TooManyRemainingAccounts,
    /// The batch's interned constant dictionary outgrew the host's `u8` wire indices.
    TooManyDictionaryEntries,
    /// An interned dictionary entry is not referenced by any step (host parity:
    /// `FheExecutePoolEntryUnreferenced`).
    UnreferencedDictionaryEntry,
    /// A step referenced a dictionary index past the end of the interned dictionary (host
    /// parity: `FheExecutePoolIndexOutOfBounds`).
    DictionaryIndexOutOfBounds,
    /// A transient operand referenced an operation that has not been produced.
    InvalidTransientReference,
    /// A persistent operand referenced an account written by an earlier step.
    /// Use the producer returned by that step for the new value, or consume the
    /// old persistent value before writing the account.
    PersistentOperandWrittenEarlier,
    /// More ops were added than the host accepts (`MAX_FHE_BATCH_OPS`).
    TooManyOps,
    /// `finish` was called with no ops; the host rejects empty batches.
    EmptyOps,
    /// `finish` was called on a batch with a rand step but no persistent output;
    /// the host anchors rand seeds to persistent writes and rejects such batches.
    RandRequiresPersistentOutput,
    /// A scalar was supplied as the left-hand operand. The host invariant is
    /// scalar-RHS-only: the left operand must be an encrypted handle.
    ScalarLhsOperand,
    /// A scalar was supplied where the host requires an encrypted operand.
    ScalarEncryptedOperand,
    /// The declared FHE type is not accepted by the host ABI.
    UnsupportedFheType,
    /// A bounded random upper bound is zero, not a power of two, or too wide for euint64.
    InvalidRandomUpperBound,
    /// The declared binary output type is not valid for the selected operator.
    UnsupportedBinaryOutputType,
    /// Binary operand handle types do not match the selected operator.
    BinaryOperandTypeMismatch,
    /// Ternary operand handle types do not match the selected operator.
    TernaryOperandTypeMismatch,
    /// A persistent output subject list would be rejected by the host.
    InvalidSubjects,
    /// An encrypted-value key contains an app-domain pubkey the host would reject.
    InvalidEncryptedValueId,
    /// The fixed app authority pubkey is not a valid signer identity.
    InvalidAppAuthority,
    /// A persistent output's declared previous state is inconsistent (one of
    /// `previous_handle`/`previous_subjects` set without the other).
    InconsistentPreviousState,
    /// A lowered host account index does not match the batch account list.
    InvalidRemainingAccountReference,
    /// A verified-input operand referenced an attestation not registered with the builder.
    MissingVerifiedInput,
    /// `sum`/`is_in` exceeded the coprocessor's max operand count for the type.
    TooManyReductionOperands,
    /// `mul_div` was given a zero divisor; the host rejects it (EVM DivisionByZero parity).
    MulDivDivisorZero,
    /// `div`/`rem` require a plaintext scalar divisor (EVM `IsNotScalar`).
    DivisorMustBeScalar,
    /// `div`/`rem` divisor is zero once truncated to the operand type (EVM `DivisionByZero`).
    DivisionByZero,
}
