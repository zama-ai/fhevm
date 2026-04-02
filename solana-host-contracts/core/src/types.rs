use crate::error::{HostContractError, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt;

pub const HANDLE_VERSION: u8 = 0;
pub type SignatureThreshold = u32;
pub const MAX_VERIFIER_SIGNERS: usize = 32;
pub const MAX_INPUT_HANDLES_PER_PROOF: usize = 64;
pub const MAX_INPUT_PROOF_BYTES: usize = 4096;
pub const MAX_DECRYPTION_HANDLES: usize = 64;
pub const MAX_DECRYPTION_PROOF_BYTES: usize = 4096;
pub const MAX_DECRYPTED_RESULT_BYTES: usize = 4096;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub const ZERO: Self = Self([0; 32]);

    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for Pubkey {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<solana_program::pubkey::Pubkey> for Pubkey {
    fn from(value: solana_program::pubkey::Pubkey) -> Self {
        Self(value.to_bytes())
    }
}

impl From<&solana_program::pubkey::Pubkey> for Pubkey {
    fn from(value: &solana_program::pubkey::Pubkey) -> Self {
        Self(value.to_bytes())
    }
}

impl From<Pubkey> for solana_program::pubkey::Pubkey {
    fn from(value: Pubkey) -> Self {
        Self::new_from_array(*value.as_bytes())
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct EvmAddress([u8; 20]);

impl EvmAddress {
    pub const ZERO: Self = Self([0; 20]);

    pub fn new(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

pub fn host_identity_from_evm_address(address: EvmAddress) -> Pubkey {
    let mut bytes = [0_u8; 32];
    bytes[12..].copy_from_slice(address.as_bytes());
    Pubkey::new(bytes)
}

impl From<[u8; 20]> for EvmAddress {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

#[derive(
    Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, BorshSerialize, BorshDeserialize,
)]
pub struct Handle([u8; 32]);

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({:02x?})", self.0)
    }
}

impl Handle {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn into_bytes(self) -> [u8; 32] {
        self.0
    }

    pub fn index(&self) -> u8 {
        self.0[21]
    }

    pub fn chain_id(&self) -> u64 {
        let mut bytes = [0_u8; 8];
        bytes.copy_from_slice(&self.0[22..30]);
        u64::from_be_bytes(bytes)
    }

    pub fn type_byte(&self) -> u8 {
        self.0[30]
    }

    pub fn version(&self) -> u8 {
        self.0[31]
    }

    pub fn fhe_type(&self) -> Result<FheType> {
        FheType::try_from(self.type_byte())
    }

    pub fn append_metadata(
        prehandle: [u8; 32],
        chain_id: u64,
        handle_type: FheType,
        index: u8,
    ) -> Self {
        let mut bytes = prehandle;
        bytes[21] = index;
        bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
        bytes[30] = handle_type as u8;
        bytes[31] = HANDLE_VERSION;
        Self(bytes)
    }
}

impl From<[u8; 32]> for Handle {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BorshSerialize, BorshDeserialize)]
pub struct ContextUserInputs {
    pub user_address: EvmAddress,
    pub contract_address: EvmAddress,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct KmsContextId([u8; 32]);

impl KmsContextId {
    pub const fn base() -> Self {
        let mut bytes = [0_u8; 32];
        bytes[0] = 0x07;
        Self(bytes)
    }

    pub fn next(mut self) -> Self {
        for idx in (0..self.0.len()).rev() {
            let (value, carry) = self.0[idx].overflowing_add(1);
            self.0[idx] = value;
            if !carry {
                break;
            }
        }
        self
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, BorshSerialize, BorshDeserialize,
)]
#[repr(u8)]
#[borsh(use_discriminant = true)]
pub enum Operator {
    FheAdd = 0,
    FheSub = 1,
    FheMul = 2,
    FheDiv = 3,
    FheRem = 4,
    FheBitAnd = 5,
    FheBitOr = 6,
    FheBitXor = 7,
    FheShl = 8,
    FheShr = 9,
    FheRotl = 10,
    FheRotr = 11,
    FheEq = 12,
    FheNe = 13,
    FheGe = 14,
    FheGt = 15,
    FheLe = 16,
    FheLt = 17,
    FheMin = 18,
    FheMax = 19,
    FheNeg = 20,
    FheNot = 21,
    VerifyInput = 22,
    Cast = 23,
    TrivialEncrypt = 24,
    FheIfThenElse = 25,
    FheRand = 26,
    FheRandBounded = 27,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, BorshSerialize, BorshDeserialize,
)]
#[repr(u8)]
#[borsh(use_discriminant = true)]
pub enum FheType {
    Bool = 0,
    Uint4 = 1,
    Uint8 = 2,
    Uint16 = 3,
    Uint32 = 4,
    Uint64 = 5,
    Uint128 = 6,
    Uint160 = 7,
    Uint256 = 8,
    Uint512 = 9,
    Uint1024 = 10,
    Uint2048 = 11,
    Uint2 = 12,
    Uint6 = 13,
    Uint10 = 14,
    Uint12 = 15,
    Uint14 = 16,
    Int2 = 17,
    Int4 = 18,
    Int6 = 19,
    Int8 = 20,
    Int10 = 21,
    Int12 = 22,
    Int14 = 23,
    Int16 = 24,
    Int32 = 25,
    Int64 = 26,
    Int128 = 27,
    Int160 = 28,
    Int256 = 29,
    AsciiString = 30,
    Int512 = 31,
    Int1024 = 32,
    Int2048 = 33,
    Uint24 = 34,
    Uint40 = 35,
    Uint48 = 36,
    Uint56 = 37,
    Uint72 = 38,
    Uint80 = 39,
    Uint88 = 40,
    Uint96 = 41,
    Uint104 = 42,
    Uint112 = 43,
    Uint120 = 44,
    Uint136 = 45,
    Uint144 = 46,
    Uint152 = 47,
    Uint168 = 48,
    Uint176 = 49,
    Uint184 = 50,
    Uint192 = 51,
    Uint200 = 52,
    Uint208 = 53,
    Uint216 = 54,
    Uint224 = 55,
    Uint232 = 56,
    Uint240 = 57,
    Uint248 = 58,
    Int24 = 59,
    Int40 = 60,
    Int48 = 61,
    Int56 = 62,
    Int72 = 63,
    Int80 = 64,
    Int88 = 65,
    Int96 = 66,
    Int104 = 67,
    Int112 = 68,
    Int120 = 69,
    Int136 = 70,
    Int144 = 71,
    Int152 = 72,
    Int168 = 73,
    Int176 = 74,
    Int184 = 75,
    Int192 = 76,
    Int200 = 77,
    Int208 = 78,
    Int216 = 79,
    Int224 = 80,
    Int232 = 81,
    Int240 = 82,
    Int248 = 83,
}

impl FheType {
    pub fn from_bit_width(bit_width: usize) -> Result<Self> {
        match bit_width {
            1 => Ok(Self::Bool),
            8 => Ok(Self::Uint8),
            16 => Ok(Self::Uint16),
            32 => Ok(Self::Uint32),
            64 => Ok(Self::Uint64),
            128 => Ok(Self::Uint128),
            160 => Ok(Self::Uint160),
            256 => Ok(Self::Uint256),
            _ => Err(HostContractError::UnsupportedType(Self::Bool)),
        }
    }

    pub fn bit_width(self) -> Option<u16> {
        match self {
            Self::Bool => Some(1),
            Self::Uint8 => Some(8),
            Self::Uint16 => Some(16),
            Self::Uint32 => Some(32),
            Self::Uint64 => Some(64),
            Self::Uint128 => Some(128),
            Self::Uint160 => Some(160),
            Self::Uint256 => Some(256),
            _ => None,
        }
    }
}

impl TryFrom<u8> for FheType {
    type Error = HostContractError;

    fn try_from(value: u8) -> Result<Self> {
        let fhe_type = match value {
            0 => Self::Bool,
            1 => Self::Uint4,
            2 => Self::Uint8,
            3 => Self::Uint16,
            4 => Self::Uint32,
            5 => Self::Uint64,
            6 => Self::Uint128,
            7 => Self::Uint160,
            8 => Self::Uint256,
            9 => Self::Uint512,
            10 => Self::Uint1024,
            11 => Self::Uint2048,
            12 => Self::Uint2,
            13 => Self::Uint6,
            14 => Self::Uint10,
            15 => Self::Uint12,
            16 => Self::Uint14,
            17 => Self::Int2,
            18 => Self::Int4,
            19 => Self::Int6,
            20 => Self::Int8,
            21 => Self::Int10,
            22 => Self::Int12,
            23 => Self::Int14,
            24 => Self::Int16,
            25 => Self::Int32,
            26 => Self::Int64,
            27 => Self::Int128,
            28 => Self::Int160,
            29 => Self::Int256,
            30 => Self::AsciiString,
            31 => Self::Int512,
            32 => Self::Int1024,
            33 => Self::Int2048,
            34 => Self::Uint24,
            35 => Self::Uint40,
            36 => Self::Uint48,
            37 => Self::Uint56,
            38 => Self::Uint72,
            39 => Self::Uint80,
            40 => Self::Uint88,
            41 => Self::Uint96,
            42 => Self::Uint104,
            43 => Self::Uint112,
            44 => Self::Uint120,
            45 => Self::Uint136,
            46 => Self::Uint144,
            47 => Self::Uint152,
            48 => Self::Uint168,
            49 => Self::Uint176,
            50 => Self::Uint184,
            51 => Self::Uint192,
            52 => Self::Uint200,
            53 => Self::Uint208,
            54 => Self::Uint216,
            55 => Self::Uint224,
            56 => Self::Uint232,
            57 => Self::Uint240,
            58 => Self::Uint248,
            59 => Self::Int24,
            60 => Self::Int40,
            61 => Self::Int48,
            62 => Self::Int56,
            63 => Self::Int72,
            64 => Self::Int80,
            65 => Self::Int88,
            66 => Self::Int96,
            67 => Self::Int104,
            68 => Self::Int112,
            69 => Self::Int120,
            70 => Self::Int136,
            71 => Self::Int144,
            72 => Self::Int152,
            73 => Self::Int168,
            74 => Self::Int176,
            75 => Self::Int184,
            76 => Self::Int192,
            77 => Self::Int200,
            78 => Self::Int208,
            79 => Self::Int216,
            80 => Self::Int224,
            81 => Self::Int232,
            82 => Self::Int240,
            83 => Self::Int248,
            _ => return Err(HostContractError::UnsupportedType(FheType::Bool)),
        };
        Ok(fhe_type)
    }
}
