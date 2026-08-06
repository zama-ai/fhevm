//! App-facing helpers for preparing one `zama-host` `fhe_execute` invocation.
//!
//! An [`FheExecution`] is that invocation: an ordered walk of dependent steps, its interned
//! constant dictionary, and the dynamic account list its `u8` indices address, sealed together and
//! validated once. It is deliberately not a batch — the steps are not independent items processed
//! together, each one reads what the step before it produced.
//!
//! This crate targets the role-aware host ABI. App code describes encrypted
//! operands and persistent outputs by pubkey; [`FheExecutionBuilder`] validates the execution,
//! assigns host account indices, and records the signer/writable requirements for
//! every dynamic account. With the `cpi` feature, [`FheExecution::resolve_accounts`]
//! preflights the dynamic account set and [`FheExecution::invoke`] turns the execution plus
//! those resolved accounts into the exact `zama-host` CPI. That build, resolve,
//! invoke sequence is the only way the SDK reaches the host: an app program that
//! knows its signer set up front can write it in three calls, and one that has to
//! read the built execution first — for its output authorities, or for the subjects it
//! newly grants — needs the execution in hand anyway.
//!
//! The builder intentionally targets the current role-aware host fhe_execute ABI rather
//! than the older `execute_frame` prototype (RFC-024's name for that sketch). Instruction-local intermediate
//! values are returned by builder methods as typed transient [`Encrypted`] values;
//! only [`Output::persistent`] creates ACL state. Binary, ternary, trivial-encrypt,
//! rand, and verified input steps can be composed in one execution.

#![allow(unexpected_cfgs)]

mod accounts;
mod acl;
mod builder;
mod cpi;
mod execution;
#[cfg(test)]
mod heap_budget;
mod lower;
mod operand;
#[cfg(test)]
mod tests;
mod types;
mod validate;

pub use accounts::{
    ExecutionAccountPurpose, ExecutionAccountRequirement, ExecutionEncryptedValueAccountAuthority,
    ExecutionOutputAuthorityRequirement,
};
#[cfg(feature = "cpi")]
pub use accounts::{ExecutionAccountResolutionError, ResolvedExecutionAccounts};
pub use acl::{
    BoundedU64UpperBound, Domain, EncryptedValueId, EncryptedValueLabel, Output, PersistentOutput,
    PersistentOutputBinding,
};
pub use builder::{FheExecutionBuilder, MAX_ON_CHAIN_EXECUTION_STEPS};
#[cfg(feature = "cpi")]
pub use cpi::ExecutionCpiAccounts;
pub use execution::FheExecution;
pub use types::{
    Address, BinaryRhs, Bool, BoolHandle, Bytes256, Encrypted, FheBitwise, FheEq, FheIsIn, FheNeg,
    FheNot, FheRandom, FheShift, FheType, FheTyped, FheUint, Scalar, StoredValue, Uint,
    Uint64Handle,
};

/// Result type used by the builder helpers.
pub type Result<T> = std::result::Result<T, FheExecutionBuildError>;

/// Builder failures that can be detected before invoking the host program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FheExecutionBuildError {
    /// More accounts were referenced than fit in the host's `u8` wire indices.
    TooManyRemainingAccounts,
    /// The execution's interned constant dictionary outgrew the host's `u8` wire indices.
    TooManyDictionaryEntries,
    /// An interned dictionary entry is not referenced by any step (host parity:
    /// `FheExecuteDictionaryEntryUnreferenced`).
    UnreferencedDictionaryEntry,
    /// A step referenced a dictionary index past the end of the interned dictionary (host
    /// parity: `FheExecuteDictionaryIndexOutOfBounds`).
    DictionaryIndexOutOfBounds,
    /// A transient operand referenced an operation that has not been produced.
    InvalidTransientReference,
    /// A persistent operand referenced an account written by an earlier step.
    /// Use the producer returned by that step for the new value, or consume the
    /// old persistent value before writing the account.
    PersistentOperandWrittenEarlier,
    /// More steps were added than the host accepts (`MAX_FHE_EXECUTION_STEPS`).
    TooManySteps,
    /// More steps were added than a program can build and invoke on Anchor's default 32 KB heap
    /// (`MAX_ON_CHAIN_EXECUTION_STEPS`). Only ever returned on-chain: the build and the packet come out of
    /// one bump region that is never freed, so past this count the allocator aborts the instruction
    /// with no error of its own. Build such an execution off-chain, or install a larger heap and enable
    /// the crate's `raised-heap` feature.
    TooManyStepsForDefaultHeap,
    /// `finish` was called with no steps; the host rejects empty executions.
    EmptySteps,
    /// `finish` was called on an execution with a rand step but no persistent output;
    /// the host anchors rand seeds to persistent writes and rejects such executions.
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
    /// The domain or the authority component of an encrypted value ID is the default pubkey, which
    /// the host rejects.
    InvalidEncryptedValueId,
    /// The fixed encrypted value account authority is the default pubkey, so it can never sign.
    InvalidEncryptedValueAccountAuthority,
    /// A lowered host account index does not match the execution account list.
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
