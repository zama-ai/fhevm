//! Internal operand representation shared by the builder, validators, and lowering.

use anchor_lang::prelude::Pubkey;
use std::marker::PhantomData;

use crate::acl::{AppScope, EncryptedValueId};

/// Makes a builder's `'id` lifetime invariant, so no two builders' identities are subtypes of one
/// another and a value cannot be coerced from one builder into the next. Zero-sized: the identity
/// exists only in the type checker, which is the point — the runtime tag it replaced was a
/// compile-time constant on SBF, so on-chain it never caught anything.
pub(crate) type BuilderIdentity<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

/// Persistent host operand: its handle and the `EncryptedValue` account holding it, with the
/// authority whose signature admits the read and the application the value belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PersistentOperand {
    pub(crate) handle: [u8; 32],
    pub(crate) encrypted_value: Pubkey,
    pub(crate) encrypted_value_account_authority: Pubkey,
    pub(crate) app: AppScope,
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
    },
    /// External input verified in-execution via a coprocessor attestation (EVM `fromExternal`). The
    /// `Vec`-bearing attestation is held by the [`FheExecutionBuilder`] and referenced by index; keeping
    /// only the index + `input_handle` here leaves the operand `Copy`. `input_handle` carries the
    /// FHE type for operand type-checks without touching the attestation.
    VerifiedInput {
        input_handle: [u8; 32],
        attestation_index: u8,
    },
    Scalar([u8; 32]),
}

impl Operand {
    pub(crate) fn persistent(handle: [u8; 32], key: &EncryptedValueId) -> Self {
        Self(OperandKind::Persistent(PersistentOperand {
            handle,
            encrypted_value: key.address(),
            encrypted_value_account_authority: key.encrypted_value_account_authority,
            app: key.app,
        }))
    }

    pub(crate) fn transient(producer_index: u8) -> Self {
        Self(OperandKind::Transient { producer_index })
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
