//! Internal operand representation shared by the builder, validators, and lowering.

use anchor_lang::prelude::Pubkey;
#[cfg(not(target_os = "solana"))]
use std::sync::atomic::{AtomicU64, Ordering};

/// Persistent host operand identified by its `EncryptedValue` PDA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PersistentOperand {
    pub(crate) handle: [u8; 32],
    pub(crate) encrypted_value: Pubkey,
}

/// Raw operand used by the lowering implementation.
///
/// Public builders expose typed [`Encrypted`] values instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Operand(pub(crate) OperandKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperandKind {
    Persistent(PersistentOperand),
    Transient {
        producer_index: u8,
        builder_scope: BatchBuilderScope,
    },
    /// External input verified in-batch via a coprocessor attestation (EVM `fromExternal`). The
    /// `Vec`-bearing attestation is held by the [`BatchBuilder`] and referenced by index; keeping
    /// only the index + `input_handle` here leaves the operand `Copy`. `input_handle` carries the
    /// FHE type for operand type-checks without touching the attestation.
    VerifiedInput {
        input_handle: [u8; 32],
        attestation_index: u8,
    },
    Scalar([u8; 32]),
}

impl Operand {
    pub(crate) fn persistent(handle: [u8; 32], encrypted_value: Pubkey) -> Self {
        Self(OperandKind::Persistent(PersistentOperand {
            handle,
            encrypted_value,
        }))
    }

    pub(crate) fn transient(producer_index: u8, builder_scope: BatchBuilderScope) -> Self {
        Self(OperandKind::Transient {
            producer_index,
            builder_scope,
        })
    }

    pub(crate) fn scalar(value: [u8; 32]) -> Self {
        Self(OperandKind::Scalar(value))
    }

    pub(crate) fn verified_input(input_handle: [u8; 32], attestation_index: u8) -> Self {
        Self(OperandKind::VerifiedInput {
            input_handle,
            attestation_index,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BatchBuilderScope(pub(crate) u64);

#[cfg(not(target_os = "solana"))]
static NEXT_BATCH_BUILDER_SCOPE: AtomicU64 = AtomicU64::new(1);

#[cfg(not(target_os = "solana"))]
pub(crate) fn next_batch_builder_scope() -> BatchBuilderScope {
    BatchBuilderScope(NEXT_BATCH_BUILDER_SCOPE.fetch_add(1, Ordering::Relaxed))
}

#[cfg(target_os = "solana")]
pub(crate) fn next_batch_builder_scope() -> BatchBuilderScope {
    // SBF forbids writable static data (no `.data`/atomics), so on-chain every builder
    // shares scope 1: mixing operands across two builders created inside one instruction
    // is caught only by the producer-index bounds check there. Off-chain (where batches are
    // normally built and tested) the counter makes cross-builder mixing a hard error.
    BatchBuilderScope(1)
}
