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
mod cost;
mod cpi;
mod execution;
#[cfg(test)]
mod heap_budget;
mod heap_tally;
mod lower;
mod operand;
mod ops;
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
pub use builder::FheExecutionBuilder;
pub use cost::{
    instruction_trace_floor, FheExecutionCost, APP_HEAP_RESERVE_BYTES, BUILD_HEAP_BUDGET_BYTES,
    CPIS_PER_PERSISTENT_CREATE, CPI_INSTRUCTION_DATA_LIMIT, MAX_PERSISTENT_CREATES,
    PROGRAM_HEAP_BYTES, TRANSACTION_INSTRUCTION_TRACE_LIMIT,
};
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
    /// More steps were added than the host accepts (`MAX_FHE_EXECUTION_STEPS`) — the one step
    /// ceiling, on-chain and off. The heap no longer bounds the step count by itself: the
    /// builder's own budget ([`ExceedsBuildHeapBudget`](Self::ExceedsBuildHeapBudget)) holds
    /// every admitted shape inside the fixed 32 KB region, which cannot be raised (DD-046).
    TooManySteps,
    /// The step's persistent creates and events would push the transaction past Solana's
    /// 64-instruction trace even in the minimal wrapper — one app instruction invoking
    /// `fhe_execute` once — so the execution could never land. Three system CPIs per created
    /// output is the binding term: at most 20 creates fit one execution. Split the creates
    /// across executions, or update existing accounts instead (updates cost no CPI). See
    /// [`instruction_trace_floor`].
    ExceedsInstructionTraceLimit,
    /// The serialized `fhe_execute` packet exceeds the 10 KiB the runtime allows a CPI to
    /// carry ([`CPI_INSTRUCTION_DATA_LIMIT`]), and the packet always travels by CPI — so the
    /// runtime would reject the invoke. Verified-input attestations are the heavy term
    /// (roughly 1 KiB each at maximum size); split them across executions.
    ExceedsCpiInstructionDataLimit,
    /// Building, serializing, and invoking this execution would request more of the program's
    /// fixed, never-freeing 32 KB heap than the builder's budget
    /// ([`BUILD_HEAP_BUDGET_BYTES`]) — on-chain it would abort the instruction with no error
    /// at all once the region ran out. The builder tallies every byte it asks the allocator
    /// for and charges the invoke-side account tables up front (both validated byte-for-byte
    /// against a counting allocator), so this fires exactly when the instruction cannot
    /// survive. Fewer persistent outputs, narrower subject lists, or fewer embedded
    /// attestations shrink the shape; splitting the work across executions always works.
    ExceedsBuildHeapBudget,
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
