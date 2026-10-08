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
//! read the built execution first — for its value authorities, or for the application
//! whose deny record and meter it must pass — needs the execution in hand anyway.
//!
//! [`Store::get`] reads a typed handle from a slot. [`Store::set`] describes a slot write,
//! and [`Store::result`] describes permissions on a result without storing it in a slot.
//! [`StoreOutput::allow`] appends decrypt permission to the Store history;
//! [`StoreOutput::allow_transient`] shares computation rights through transaction transient store.
//! Intermediate [`Encrypted`] values can be consumed by later steps in the same execution.

#![allow(unexpected_cfgs)]

use anchor_lang::error_code;

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
mod store;
#[cfg(test)]
mod tests;
mod types;
pub use store::{Store, StoreId, StoreOutput};
mod validate;

pub use accounts::{
    ExecutionAccountPurpose, ExecutionAccountRequirement, ExecutionAuthorityRequirement,
};
#[cfg(feature = "cpi")]
pub use accounts::{ExecutionAccountResolutionError, ResolvedExecutionAccounts};
pub use acl::{AppScope, BoundedU64UpperBound};
pub use builder::FheExecutionBuilder;
pub use cost::{
    FheExecutionCost, APP_HEAP_RESERVE_BYTES, BUILD_HEAP_BUDGET_BYTES, CPIS_PER_SQUAT_CREATE,
    CPI_INSTRUCTION_DATA_LIMIT, INSTRUCTION_TRACE_FLOOR, PROGRAM_HEAP_BYTES,
    TRANSACTION_INSTRUCTION_TRACE_LIMIT,
};
#[cfg(feature = "cpi")]
pub use cpi::ExecutionCpiAccounts;
pub use execution::{FheExecution, ReturningFheExecution};
pub use types::{
    BinaryRhs, Bool, BoolHandle, Encrypted, FheHandle, FheType, FheTyped, FheUint, Scalar, Uint,
    Uint64Handle,
};

/// Result type used by the builder helpers.
pub type Result<T> = std::result::Result<T, FheExecutionError>;

/// Every failure this crate reports, from building an execution through invoking the host. Codes
/// start at 10_000, clear of Anchor's own (below 6000) and of every program's error
/// range (6000 up), so an app program can return them with `?` and its clients still tell them
/// apart from its own.
#[error_code(offset = 10_000)]
#[derive(PartialEq, Eq)]
pub enum FheExecutionError {
    #[msg("A store slot read by the execution is not present")]
    MissingStoreSlot,
    #[msg("A requested result was not produced by any step")]
    ResultNotProduced,
    #[msg("The store history does not match the execution's slot writes")]
    StoreHistoryMismatch,
    #[msg("Too many result grants")]
    TooManyResultGrants,
    /// More accounts were referenced than fit in the host's `u8` wire indices, counted at build
    /// and again at invoke once the deny records are appended.
    #[msg("More accounts than fit in the host's u8 account indices")]
    TooManyRemainingAccounts,
    /// The execution's interned constant dictionary outgrew the host's `u8` wire indices.
    #[msg("More dictionary entries than fit in the host's u8 indices")]
    TooManyDictionaryEntries,
    /// An interned dictionary entry is not referenced by any step (host parity:
    /// `FheExecuteDictionaryEntryUnreferenced`).
    #[msg("A dictionary entry is not referenced by any step")]
    UnreferencedDictionaryEntry,
    /// A step referenced a dictionary index past the end of the interned dictionary (host
    /// parity: `FheExecuteDictionaryIndexOutOfBounds`).
    #[msg("A step references a dictionary index out of bounds")]
    DictionaryIndexOutOfBounds,
    /// A transient operand referenced an operation that has not been produced.
    #[msg("A transient operand references an operation not yet produced")]
    InvalidTransientReference,
    /// Two effects write the same Store slot in one execution.
    #[msg("Two effects write the same store slot")]
    DuplicateSlotWrite,
    /// More steps were added than the host accepts (`MAX_FHE_EXECUTION_STEPS`) — the one step
    /// ceiling, on-chain and off. The heap no longer bounds the step count by itself: the
    /// builder's own budget ([`ExceedsBuildHeapBudget`](Self::ExceedsBuildHeapBudget)) holds
    /// every admitted shape inside the fixed 32 KB region, which cannot be raised (DD-046).
    #[msg("More steps than the host accepts")]
    TooManySteps,
    #[msg("More effects than the host accepts")]
    TooManyEffects,
    /// The serialized `fhe_execute` packet exceeds the 10 KiB the runtime allows a CPI to
    /// carry ([`CPI_INSTRUCTION_DATA_LIMIT`]), and the packet always travels by CPI — so the
    /// runtime would reject the invoke. Verified-input attestations are the heaviest term
    /// (roughly 1 KiB each at maximum size), but the build-heap budget refuses them first:
    /// six maximum-size attestations exceed it while five fill under half the packet.
    #[msg("The fhe_execute packet exceeds the CPI instruction data limit")]
    ExceedsCpiInstructionDataLimit,
    /// Building, serializing, and invoking this execution would request more of the program's
    /// fixed, never-freeing 32 KB heap than the builder's budget
    /// ([`BUILD_HEAP_BUDGET_BYTES`]) — on-chain it would abort the instruction with no error
    /// at all once the region ran out. The builder tallies every byte it asks the allocator
    /// for and charges the invoke-side account tables up front (both validated byte-for-byte
    /// against a counting allocator), so this fires exactly when the instruction cannot
    /// survive. Fewer persistent outputs, shorter allow lists, or fewer embedded
    /// attestations shrink the shape; splitting the work across executions always works.
    #[msg("The execution exceeds the builder's heap budget")]
    ExceedsBuildHeapBudget,
    /// `finish` was called with no steps; the host rejects empty executions.
    #[msg("The execution has no steps")]
    EmptySteps,
    /// Persistent values of two applications `(program, scope)` under the execution's default
    /// authority; the host meters, deny-checks and seeds one application per execution
    /// (`FheExecuteMixedScopes`). Values under an additional signing authority are that
    /// program's own.
    #[msg("Persistent values of two applications under one authority")]
    MixedScopes,
    /// A scalar was supplied as the left-hand operand. The host invariant is
    /// scalar-RHS-only: the left operand must be an encrypted handle.
    #[msg("A scalar was supplied as the left-hand operand")]
    ScalarLhsOperand,
    /// A scalar was supplied where the host requires an encrypted operand.
    #[msg("A scalar was supplied where an encrypted operand is required")]
    ScalarEncryptedOperand,
    /// The declared FHE type is not accepted by the host ABI.
    #[msg("The FHE type is not supported by the host")]
    UnsupportedFheType,
    /// A bounded random upper bound is zero, not a power of two, or too wide for euint64.
    #[msg("The random upper bound is invalid")]
    InvalidRandomUpperBound,
    /// The declared binary output type is not valid for the selected operator.
    #[msg("The binary output type is not valid for the operator")]
    UnsupportedBinaryOutputType,
    /// Binary operand handle types do not match the selected operator.
    #[msg("The binary operand types do not match the operator")]
    BinaryOperandTypeMismatch,
    /// Ternary operand handle types do not match the selected operator.
    #[msg("The ternary operand types do not match the operator")]
    TernaryOperandTypeMismatch,
    /// An allowed key is the zero key or repeats another (host parity: `InvalidAllowKey`).
    #[msg("An allowed key is zero or repeated")]
    InvalidAllowKey,
    /// The fixed encrypted store authority is the default pubkey, so it can never sign.
    #[msg("The execution authority is the default pubkey")]
    InvalidExecutionAuthority,
    /// A lowered host account index does not match the execution account list.
    #[msg("A host account index does not match the execution account list")]
    InvalidRemainingAccountReference,
    /// A verified-input operand referenced an attestation not registered with the builder.
    #[msg("A verified input references an unregistered attestation")]
    MissingVerifiedInput,
    /// `sum`/`is_in` exceeded the coprocessor's max operand count for the type.
    #[msg("Too many operands for sum or is_in")]
    TooManyReductionOperands,
    /// `mul_div` was given a zero divisor; the host rejects it (EVM DivisionByZero parity).
    #[msg("mul_div divisor is zero")]
    MulDivDivisorZero,
    /// `div`/`rem` require a plaintext scalar divisor (EVM `IsNotScalar`).
    #[msg("div and rem require a scalar divisor")]
    DivisorMustBeScalar,
    /// `div`/`rem` divisor is zero (EVM `DivisionByZero`).
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("A dynamic account was supplied more than once")]
    DuplicateDynamicAccount,
    #[msg("A supplied dynamic account is not required by the execution")]
    UnexpectedDynamicAccount,
    #[msg("A dynamic account required by the execution was not supplied")]
    MissingDynamicAccount,
    #[msg("A dynamic account the execution writes was supplied read-only")]
    DynamicAccountNotWritable,
    #[msg("A value authority was supplied more than once")]
    DuplicateStoreAuthority,
    #[msg("A supplied value authority is not required by the execution")]
    UnexpectedStoreAuthority,
    #[msg("A value authority required by the execution was not supplied")]
    MissingStoreAuthority,
    /// The program account passed for the CPI is not the zama-host program. The log's compared
    /// values are the received key (Left) and `zama_host::ID` (Right).
    #[msg("The host program account is not zama-host")]
    HostProgramMismatch,
    /// The authority account passed for the CPI is not the authority the execution was built for.
    /// The log's compared values are the received key (Left) and the execution's authority (Right).
    #[msg("The authority account is not the execution's authority")]
    ExecutionAuthorityMismatch,
}
