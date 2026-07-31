//! Typed FHE value markers and the `Encrypted`/`Scalar` operand wrappers.

use std::marker::PhantomData;

use anchor_lang::prelude::Pubkey;

use crate::acl::EncryptedValueId;
use crate::operand::Operand;
use crate::validate::{handle_fhe_type, validate_encrypted_value_id, validate_supported_fhe_type};
use crate::{BatchBuildError, Result};

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
    pub const ADDRESS: Self = Self(7);
    pub const BYTES256: Self = Self(8);

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

pub type BoolHandle = Encrypted<Bool>;
pub type Uint64Handle = Encrypted<Uint<64>>;

/// Marker for encrypted address handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address;

/// Marker for opaque 256-byte encrypted values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bytes256;

mod sealed {
    use super::{Address, Bool, Bytes256, Uint};

    pub trait FheTypedSeal {}
    pub trait FheUintSeal {}
    pub trait FheRandomSeal {}
    pub trait FheNotSeal {}
    pub trait FheBitwiseSeal {}
    pub trait FheShiftSeal {}
    pub trait FheEqSeal {}
    pub trait FheNegSeal {}
    pub trait FheIsInSeal {}

    impl FheTypedSeal for Bool {}
    impl FheTypedSeal for Uint<8> {}
    impl FheTypedSeal for Uint<16> {}
    impl FheTypedSeal for Uint<32> {}
    impl FheTypedSeal for Uint<64> {}
    impl FheTypedSeal for Uint<128> {}
    impl FheTypedSeal for Address {}
    impl FheTypedSeal for Bytes256 {}

    impl FheUintSeal for Uint<8> {}
    impl FheUintSeal for Uint<16> {}
    impl FheUintSeal for Uint<32> {}
    impl FheUintSeal for Uint<64> {}
    impl FheUintSeal for Uint<128> {}

    impl FheRandomSeal for Bool {}
    impl FheRandomSeal for Uint<8> {}
    impl FheRandomSeal for Uint<16> {}
    impl FheRandomSeal for Uint<32> {}
    impl FheRandomSeal for Uint<64> {}
    impl FheRandomSeal for Uint<128> {}
    impl FheRandomSeal for Bytes256 {}

    // NOT / bitwise: Bool + Uint8..Uint128 + Uint256 (host `0 | 2..=6 | 8`).
    impl FheNotSeal for Bool {}
    impl FheNotSeal for Uint<8> {}
    impl FheNotSeal for Uint<16> {}
    impl FheNotSeal for Uint<32> {}
    impl FheNotSeal for Uint<64> {}
    impl FheNotSeal for Uint<128> {}
    impl FheNotSeal for Bytes256 {}

    impl FheBitwiseSeal for Bool {}
    impl FheBitwiseSeal for Uint<8> {}
    impl FheBitwiseSeal for Uint<16> {}
    impl FheBitwiseSeal for Uint<32> {}
    impl FheBitwiseSeal for Uint<64> {}
    impl FheBitwiseSeal for Uint<128> {}
    impl FheBitwiseSeal for Bytes256 {}

    // Shifts/rotations and Neg: Uint8..Uint128 + Uint256 (host `2..=6 | 8`).
    impl FheShiftSeal for Uint<8> {}
    impl FheShiftSeal for Uint<16> {}
    impl FheShiftSeal for Uint<32> {}
    impl FheShiftSeal for Uint<64> {}
    impl FheShiftSeal for Uint<128> {}
    impl FheShiftSeal for Bytes256 {}

    impl FheNegSeal for Uint<8> {}
    impl FheNegSeal for Uint<16> {}
    impl FheNegSeal for Uint<32> {}
    impl FheNegSeal for Uint<64> {}
    impl FheNegSeal for Uint<128> {}
    impl FheNegSeal for Bytes256 {}

    // Eq/Ne: Bool + Uint8..Uint128 + Uint160 + Uint256 (host `0 | 2..=8`).
    impl FheEqSeal for Bool {}
    impl FheEqSeal for Uint<8> {}
    impl FheEqSeal for Uint<16> {}
    impl FheEqSeal for Uint<32> {}
    impl FheEqSeal for Uint<64> {}
    impl FheEqSeal for Uint<128> {}
    impl FheEqSeal for Address {}
    impl FheEqSeal for Bytes256 {}

    // IsIn: Uint8..Uint128 + Uint160 + Uint256 (host/EVM/coprocessor `2..=8`; no ebool).
    impl FheIsInSeal for Uint<8> {}
    impl FheIsInSeal for Uint<16> {}
    impl FheIsInSeal for Uint<32> {}
    impl FheIsInSeal for Uint<64> {}
    impl FheIsInSeal for Uint<128> {}
    impl FheIsInSeal for Address {}
    impl FheIsInSeal for Bytes256 {}
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

impl FheTyped for Address {
    const FHE_TYPE: FheType = FheType::ADDRESS;
}

impl FheTyped for Bytes256 {
    const FHE_TYPE: FheType = FheType::BYTES256;
}

/// Marker trait for integer FHE values accepted by arithmetic/comparison ops.
pub trait FheUint: FheTyped + sealed::FheUintSeal {}

impl FheUint for Uint<8> {}
impl FheUint for Uint<16> {}
impl FheUint for Uint<32> {}
impl FheUint for Uint<64> {}
impl FheUint for Uint<128> {}

/// Marker trait for FHE values accepted by host rand steps.
pub trait FheRandom: FheTyped + sealed::FheRandomSeal {}

impl FheRandom for Bool {}
impl FheRandom for Uint<8> {}
impl FheRandom for Uint<16> {}
impl FheRandom for Uint<32> {}
impl FheRandom for Uint<64> {}
impl FheRandom for Uint<128> {}
impl FheRandom for Bytes256 {}

/// Marker trait for FHE values accepted by bitwise NOT: Bool, Uint8..Uint128, Uint256.
pub trait FheNot: FheTyped + sealed::FheNotSeal {}

impl FheNot for Bool {}
impl FheNot for Uint<8> {}
impl FheNot for Uint<16> {}
impl FheNot for Uint<32> {}
impl FheNot for Uint<64> {}
impl FheNot for Uint<128> {}
impl FheNot for Bytes256 {}

/// Marker trait for values accepted by bitwise And/Or/Xor: Bool, Uint8..Uint128, Uint256.
pub trait FheBitwise: FheTyped + sealed::FheBitwiseSeal {}

impl FheBitwise for Bool {}
impl FheBitwise for Uint<8> {}
impl FheBitwise for Uint<16> {}
impl FheBitwise for Uint<32> {}
impl FheBitwise for Uint<64> {}
impl FheBitwise for Uint<128> {}
impl FheBitwise for Bytes256 {}

/// Marker trait for values accepted by shifts/rotations: Uint8..Uint128, Uint256.
pub trait FheShift: FheTyped + sealed::FheShiftSeal {}

impl FheShift for Uint<8> {}
impl FheShift for Uint<16> {}
impl FheShift for Uint<32> {}
impl FheShift for Uint<64> {}
impl FheShift for Uint<128> {}
impl FheShift for Bytes256 {}

/// Marker trait for values accepted by arithmetic negation: Uint8..Uint128, Uint256.
pub trait FheNeg: FheTyped + sealed::FheNegSeal {}

impl FheNeg for Uint<8> {}
impl FheNeg for Uint<16> {}
impl FheNeg for Uint<32> {}
impl FheNeg for Uint<64> {}
impl FheNeg for Uint<128> {}
impl FheNeg for Bytes256 {}

/// Marker trait for values accepted by Eq/Ne: Bool, Uint8..Uint128, Uint160, Uint256.
pub trait FheEq: FheTyped + sealed::FheEqSeal {}

impl FheEq for Bool {}
impl FheEq for Uint<8> {}
impl FheEq for Uint<16> {}
impl FheEq for Uint<32> {}
impl FheEq for Uint<64> {}
impl FheEq for Uint<128> {}
impl FheEq for Address {}
impl FheEq for Bytes256 {}

/// Marker trait for values accepted by IsIn: Uint8..Uint128, Uint160, Uint256.
pub trait FheIsIn: FheTyped + sealed::FheIsInSeal {}

impl FheIsIn for Uint<8> {}
impl FheIsIn for Uint<16> {}
impl FheIsIn for Uint<32> {}
impl FheIsIn for Uint<64> {}
impl FheIsIn for Uint<128> {}
impl FheIsIn for Address {}
impl FheIsIn for Bytes256 {}

/// Typed encrypted eval value.
///
/// Persistent values are constructed from app account state. Transient values are
/// returned by [`BatchBuilder`] methods and can only be fed to later steps in the
/// same builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encrypted<T> {
    operand: Operand,
    marker: PhantomData<T>,
}

impl<T: FheTyped> Encrypted<T> {
    /// Builds a persistent operand from a stable `EncryptedValue` encrypted value account. `handle`
    /// must be that encrypted value account's current handle; the host re-verifies this on-chain.
    pub fn persistent(handle: [u8; 32], key: EncryptedValueId) -> Result<Self> {
        validate_encrypted_value_id(&key)?;
        if handle_fhe_type(handle) != T::FHE_TYPE.byte() {
            return Err(BatchBuildError::UnsupportedFheType);
        }
        Ok(Self::from_operand(Operand::persistent(
            handle,
            key.address(),
        )))
    }
}

impl<T> Encrypted<T> {
    pub(crate) fn from_operand(operand: Operand) -> Self {
        Self {
            operand,
            marker: PhantomData,
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

impl Scalar<Address> {
    pub fn pubkey(value: Pubkey) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(value.as_ref());
        Self {
            bytes,
            marker: PhantomData,
        }
    }
}

impl Scalar<Bytes256> {
    pub fn from_bytes(value: [u8; 32]) -> Self {
        Self {
            bytes: value,
            marker: PhantomData,
        }
    }
}

/// Typed right-hand side accepted by binary eval ops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryRhs<T> {
    Encrypted(Encrypted<T>),
    Scalar(Scalar<T>),
}

impl<T> From<Encrypted<T>> for BinaryRhs<T> {
    fn from(value: Encrypted<T>) -> Self {
        Self::Encrypted(value)
    }
}

impl<T> From<Scalar<T>> for BinaryRhs<T> {
    fn from(value: Scalar<T>) -> Self {
        Self::Scalar(value)
    }
}

pub(crate) fn binary_rhs_operand<T>(rhs: impl Into<BinaryRhs<T>>) -> Operand {
    match rhs.into() {
        BinaryRhs::Encrypted(value) => value.operand(),
        BinaryRhs::Scalar(value) => Operand::scalar(value.bytes()),
    }
}
