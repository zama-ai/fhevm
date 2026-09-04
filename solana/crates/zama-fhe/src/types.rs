//! Typed FHE value markers and the `Encrypted`/`Scalar` operand wrappers.

use std::marker::PhantomData;

use crate::acl::EncryptedValueId;
use crate::operand::{BuilderIdentity, Operand};
use crate::validate::{handle_fhe_type, validate_encrypted_value_id, validate_supported_fhe_type};
use crate::{FheExecutionBuildError, Result};

/// Typed FHE handle tag used by the host ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FheType(u8);

impl FheType {
    pub const BOOL: Self = Self(0);
    pub const UINT8: Self = Self(2);
    pub const UINT16: Self = Self(3);
    pub const UINT32: Self = Self(4);
    pub const UINT64: Self = Self(5);
    pub const UINT128: Self = Self(6);

    pub(crate) const fn byte(self) -> u8 {
        self.0
    }

    pub(crate) fn from_host_byte(byte: u8) -> Result<Self> {
        validate_supported_fhe_type(byte)?;
        Ok(Self(byte))
    }
}

/// Marker for encrypted bool handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool;

/// Marker for encrypted unsigned integer handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uint<const BITS: u16>;

pub type BoolHandle = StoredValue<Bool>;
pub type Uint64Handle = StoredValue<Uint<64>>;

mod sealed {
    use super::{Bool, Uint};

    pub trait FheTypedSeal {}
    pub trait FheUintSeal {}

    impl FheTypedSeal for Bool {}
    impl FheTypedSeal for Uint<8> {}
    impl FheTypedSeal for Uint<16> {}
    impl FheTypedSeal for Uint<32> {}
    impl FheTypedSeal for Uint<64> {}
    impl FheTypedSeal for Uint<128> {}

    impl FheUintSeal for Uint<8> {}
    impl FheUintSeal for Uint<16> {}
    impl FheUintSeal for Uint<32> {}
    impl FheUintSeal for Uint<64> {}
    impl FheUintSeal for Uint<128> {}
}

/// Compile-time FHE type tag for typed encrypted handles.
pub trait FheTyped: sealed::FheTypedSeal {
    const FHE_TYPE: FheType;
}

impl FheTyped for Bool {
    const FHE_TYPE: FheType = FheType::BOOL;
}

impl FheTyped for Uint<8> {
    const FHE_TYPE: FheType = FheType::UINT8;
}

impl FheTyped for Uint<16> {
    const FHE_TYPE: FheType = FheType::UINT16;
}

impl FheTyped for Uint<32> {
    const FHE_TYPE: FheType = FheType::UINT32;
}

impl FheTyped for Uint<64> {
    const FHE_TYPE: FheType = FheType::UINT64;
}

impl FheTyped for Uint<128> {
    const FHE_TYPE: FheType = FheType::UINT128;
}

/// Marker trait for integer FHE values accepted by arithmetic/comparison ops.
pub trait FheUint: FheTyped + sealed::FheUintSeal {}

impl FheUint for Uint<8> {}
impl FheUint for Uint<16> {}
impl FheUint for Uint<32> {}
impl FheUint for Uint<64> {}
impl FheUint for Uint<128> {}

/// Typed encrypted execution value.
///
/// Transient values are returned by
/// [`FheExecutionBuilder`](crate::FheExecutionBuilder) methods and can only be fed to later steps of the builder
/// that produced them: `'id` is that builder's identity, handed out by
/// [`FheExecution::build`](crate::FheExecution::build) as a fresh invariant lifetime, so mixing two builders'
/// values is a type error. A persistent value belongs to no builder and takes whatever identity its
/// use site needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encrypted<'id, T> {
    operand: Operand,
    marker: PhantomData<T>,
    identity: BuilderIdentity<'id>,
}

/// A persistent value as an operand: its handle plus the encrypted value account holding it.
///
/// Brand-free on purpose. A stored value belongs to no builder, so app code can read one out of
/// account state — with its own error handling — before it opens an execution, and then feed it to
/// whichever builder needs it. Only the values a builder hands back carry an identity ([`Encrypted`]),
/// because only those are meaningless outside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoredValue<T> {
    operand: Operand,
    marker: PhantomData<T>,
}

impl<T: FheTyped> StoredValue<T> {
    /// Builds a persistent operand from a stable `EncryptedValue` account. `handle` must be that
    /// account's current handle; the host re-verifies this on-chain.
    pub fn persistent(handle: [u8; 32], key: EncryptedValueId) -> Result<Self> {
        validate_encrypted_value_id(&key)?;
        let fhe_type = handle_fhe_type(handle);
        validate_supported_fhe_type(fhe_type)?;
        if fhe_type != T::FHE_TYPE.byte() {
            return Err(FheExecutionBuildError::UnsupportedFheType);
        }
        Ok(Self {
            operand: Operand::persistent(handle, key.address()),
            marker: PhantomData,
        })
    }
}

impl<T> From<StoredValue<T>> for Encrypted<'_, T> {
    fn from(value: StoredValue<T>) -> Self {
        Self::from_operand(value.operand)
    }
}

impl<T> Encrypted<'_, T> {
    pub(crate) fn from_operand(operand: Operand) -> Self {
        Self {
            operand,
            marker: PhantomData,
            identity: PhantomData,
        }
    }

    pub(crate) fn operand(self) -> Operand {
        self.operand
    }
}

/// Plaintext scalar bytes tagged by the encrypted type they can be paired with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar<T> {
    bytes: [u8; 32],
    marker: PhantomData<T>,
}

impl<T> Scalar<T> {
    pub(crate) fn bytes(self) -> [u8; 32] {
        self.bytes
    }

    fn from_low_bytes(value: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        bytes[32 - value.len()..].copy_from_slice(value);
        Self {
            bytes,
            marker: PhantomData,
        }
    }
}

impl Scalar<Uint<8>> {
    pub fn u8(value: u8) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<16>> {
    pub fn u16(value: u16) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<32>> {
    pub fn u32(value: u32) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<64>> {
    pub fn u64(value: u64) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<128>> {
    pub fn u128(value: u128) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Bool> {
    pub fn bool(value: bool) -> Self {
        let mut bytes = [0u8; 32];
        bytes[31] = u8::from(value);
        Self {
            bytes,
            marker: PhantomData,
        }
    }
}

/// Typed right-hand side accepted by binary execution ops. Carries the builder identity of the encrypted
/// arm; a scalar belongs to no builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryRhs<'id, T> {
    Encrypted(Encrypted<'id, T>),
    Scalar(Scalar<T>),
}

impl<'id, T> From<Encrypted<'id, T>> for BinaryRhs<'id, T> {
    fn from(value: Encrypted<'id, T>) -> Self {
        Self::Encrypted(value)
    }
}

impl<T> From<Scalar<T>> for BinaryRhs<'_, T> {
    fn from(value: Scalar<T>) -> Self {
        Self::Scalar(value)
    }
}

impl<T> From<StoredValue<T>> for BinaryRhs<'_, T> {
    fn from(value: StoredValue<T>) -> Self {
        Self::Encrypted(value.into())
    }
}

pub(crate) fn binary_rhs_operand<'id, T>(rhs: impl Into<BinaryRhs<'id, T>>) -> Operand {
    match rhs.into() {
        BinaryRhs::Encrypted(value) => value.operand(),
        BinaryRhs::Scalar(value) => Operand::scalar(value.bytes()),
    }
}
