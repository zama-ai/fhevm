//! App-facing helpers for preparing `zama-host` FHE evaluation requests.
//!
//! This crate targets the role-aware host ABI. App code describes encrypted
//! operands and durable outputs by pubkey; [`EvalBuilder`] validates the frame,
//! assigns host account indices, and records the signer/writable requirements for
//! every dynamic account. With the `cpi` feature, [`EvalPlan::resolve_accounts`]
//! preflights the dynamic account set and [`invoke_eval_signed_resolved`] turns
//! the plan plus resolved accounts into the exact `zama-host` CPI.
//!
//! The builder intentionally targets the current role-aware host eval ABI rather
//! than the older `execute_frame` prototype. Instruction-local intermediate
//! values are returned by builder methods as typed transient [`Encrypted`] values;
//! only [`Output::durable`] creates ACL state. Binary, ternary, trivial-encrypt,
//! rand, and verified input steps can be composed in one eval frame.

#![allow(unexpected_cfgs)]

mod accounts;
mod acl;
mod builder;
mod cpi;
mod lower;
mod operand;
mod plan;
#[cfg(test)]
mod tests;
mod types;
mod validate;

pub use accounts::{
    EvalAccountPurpose, EvalAccountRequirement, EvalAppAuthority, EvalOutputAuthorityRequirement,
};
#[cfg(feature = "cpi")]
pub use accounts::{EvalAccountResolutionError, ResolvedEvalAccounts};
pub use acl::{
    BoundedU64UpperBound, DurableLabel, DurableOutput, DurableOutputBirth, EncryptedValueKey,
    Output,
};
pub use builder::EvalBuilder;
#[cfg(feature = "cpi")]
pub use cpi::{
    invoke_eval_signed_resolved, invoke_eval_signed_with_builder, EvalCpiAccounts, EvalInvokeError,
};
pub use plan::EvalPlan;
pub use types::{
    Address, BinaryRhs, Bool, BoolHandle, Bytes256, Encrypted, FheBitwise, FheEq, FheIsIn, FheNeg,
    FheNot, FheRandom, FheShift, FheType, FheTyped, FheUint, Scalar, Uint, Uint64Handle,
};

/// Result type used by the builder helpers.
pub type Result<T> = std::result::Result<T, EvalBuildError>;

/// Builder failures that can be detected before invoking the host program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalBuildError {
    /// More accounts were referenced than fit in the host's `u8` wire indices.
    TooManyRemainingAccounts,
    /// The frame's interned constant pool outgrew the host's `u8` wire indices.
    TooManyPoolEntries,
    /// A transient operand referenced an operation that has not been produced.
    InvalidTransientReference,
    /// More ops were added than the host accepts (`MAX_FHE_EVAL_OPS`).
    TooManyOps,
    /// `finish` was called with no ops; the host rejects empty eval frames.
    EmptyOps,
    /// `finish` was called on a frame with a rand step but no durable output;
    /// the host anchors rand seeds to durable writes and rejects such frames.
    RandRequiresDurableOutput,
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
    /// The encrypted-input proof does not contain the selected handle.
    InvalidInputProof,
    /// A durable output subject list would be rejected by the host.
    InvalidSubjects,
    /// An encrypted-value key contains an app-domain pubkey the host would reject.
    InvalidEncryptedValueKey,
    /// The fixed app authority pubkey is not a valid signer identity.
    InvalidAppAuthority,
    /// A durable output's declared previous state is inconsistent (one of
    /// `previous_handle`/`previous_subjects` set without the other).
    InconsistentPreviousState,
    /// A lowered host account index does not match the eval plan account list.
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
