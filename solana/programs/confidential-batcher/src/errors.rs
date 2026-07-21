//! Program-specific errors returned by confidential-batcher instructions.
//!
//! Append new variants at the tail only; error codes are part of the app ABI.

use anchor_lang::prelude::*;

/// Errors returned by the confidential batcher.
#[error_code]
pub enum BatcherError {
    /// The deposit confidential mint does not wrap the vault's underlying mint.
    #[msg("deposit confidential mint does not wrap the vault underlying mint")]
    DepositMintVaultMismatch,
    /// The shares confidential mint does not wrap the vault's share mint.
    #[msg("shares confidential mint does not wrap the vault share mint")]
    SharesMintVaultMismatch,
    /// The supplied confidential mint does not match the batcher config.
    #[msg("confidential mint does not match the batcher config")]
    ConfidentialMintMismatch,
    /// The supplied vault does not match the batcher config.
    #[msg("vault does not match the batcher config")]
    VaultMismatch,
    /// The supplied batch does not belong to this batcher.
    #[msg("batch does not belong to this batcher")]
    BatchBatcherMismatch,
    /// The previous batch account is required (or wrong) for this open.
    #[msg("previous batch account missing or not the immediately preceding batch")]
    PreviousBatchMismatch,
    /// A new batch cannot open while the previous batch is still pending.
    #[msg("previous batch is still pending")]
    PreviousBatchStillPending,
    /// The instruction requires a pending batch.
    #[msg("batch is not pending")]
    BatchNotPending,
    /// The instruction requires a dispatched batch.
    #[msg("batch is not dispatched")]
    BatchNotDispatched,
    /// The instruction requires a settled batch.
    #[msg("batch is not settled")]
    BatchNotSettled,
    /// Dispatch was attempted before the batch reached its minimum age.
    #[msg("batch is younger than the minimum batch age")]
    BatchTooYoung,
    /// A derived token or lineage account did not match its canonical address.
    #[msg("account does not match its canonical derived address")]
    DerivedAccountMismatch,
    /// An encrypted-value account was malformed or not owned by the host.
    #[msg("encrypted value account is invalid")]
    EncryptedValueInvalid,
    /// The batcher produced an invalid FHE eval plan (internal invariant).
    #[msg("invalid FHE eval plan")]
    InvalidFheEvalPlan,
    /// The deposit record's shares were already claimed.
    #[msg("shares already claimed for this deposit record")]
    AlreadyClaimed,
    /// The frozen share rate overflowed u64 (vault share price below the
    /// supported domain).
    #[msg("share rate does not fit u64")]
    ShareRateOverflow,
    /// A batch index computation overflowed.
    #[msg("batch index overflowed")]
    BatchIndexOverflow,
    /// The batch share account's balance decreased across the vault deposit
    /// (unreachable; guards the delta computation).
    #[msg("batch share balance decreased across the vault deposit")]
    ShareBalanceUnderflow,
}
