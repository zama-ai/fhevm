use crate::{ClientCoreError, Result};

/// Defines the bit width for different encryption types.
///
/// Discriminant values match the on-chain protocol:
/// `ebool=0, euint8=2, euint16=3, ...` — discriminant 1 is reserved/unused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionType {
    Bit1,   // Boolean
    Bit8,   // uint8
    Bit16,  // uint16
    Bit32,  // uint32
    Bit64,  // uint64
    Bit128, // uint128
    Bit160, // address
    Bit256, // uint256
}

impl EncryptionType {
    /// Get the number of bits for this encryption type.
    pub fn bit_width(&self) -> usize {
        match self {
            Self::Bit1 => 1,
            Self::Bit8 => 8,
            Self::Bit16 => 16,
            Self::Bit32 => 32,
            Self::Bit64 => 64,
            Self::Bit128 => 128,
            Self::Bit160 => 160,
            Self::Bit256 => 256,
        }
    }

    /// Get the discriminant value used in handle computation.
    pub fn discriminant(&self) -> u8 {
        match self {
            Self::Bit1 => 0,   // ebool
            Self::Bit8 => 2,   // euint8
            Self::Bit16 => 3,  // euint16
            Self::Bit32 => 4,  // euint32
            Self::Bit64 => 5,  // euint64
            Self::Bit128 => 6, // euint128
            Self::Bit160 => 7, // eaddress
            Self::Bit256 => 8, // euint256
        }
    }

    /// Get the encryption type from a discriminant value.
    pub fn from_discriminant(disc: u8) -> Result<Self> {
        match disc {
            0 => Ok(Self::Bit1),
            2 => Ok(Self::Bit8),
            3 => Ok(Self::Bit16),
            4 => Ok(Self::Bit32),
            5 => Ok(Self::Bit64),
            6 => Ok(Self::Bit128),
            7 => Ok(Self::Bit160),
            8 => Ok(Self::Bit256),
            _ => Err(ClientCoreError::InvalidParams(format!(
                "Unknown type discriminant: {disc}"
            ))),
        }
    }

    /// Get the canonical FHE type name (e.g. "ebool", "euint64", "eaddress").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bit1 => "ebool",
            Self::Bit8 => "euint8",
            Self::Bit16 => "euint16",
            Self::Bit32 => "euint32",
            Self::Bit64 => "euint64",
            Self::Bit128 => "euint128",
            Self::Bit160 => "eaddress",
            Self::Bit256 => "euint256",
        }
    }

    /// Parse from a canonical FHE type name.
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "ebool" => Ok(Self::Bit1),
            "euint8" => Ok(Self::Bit8),
            "euint16" => Ok(Self::Bit16),
            "euint32" => Ok(Self::Bit32),
            "euint64" => Ok(Self::Bit64),
            "euint128" => Ok(Self::Bit128),
            "eaddress" => Ok(Self::Bit160),
            "euint256" => Ok(Self::Bit256),
            _ => Err(ClientCoreError::InvalidParams(format!(
                "Unknown FHE type: '{s}'. Valid types: ebool, euint8, euint16, \
                 euint32, euint64, euint128, eaddress, euint256"
            ))),
        }
    }

    /// Get the encryption type from a bit width.
    pub fn from_bit_width(bit_width: usize) -> Result<Self> {
        match bit_width {
            1 => Ok(Self::Bit1),
            8 => Ok(Self::Bit8),
            16 => Ok(Self::Bit16),
            32 => Ok(Self::Bit32),
            64 => Ok(Self::Bit64),
            128 => Ok(Self::Bit128),
            160 => Ok(Self::Bit160),
            256 => Ok(Self::Bit256),
            _ => Err(ClientCoreError::InvalidParams(format!(
                "Unsupported bit width: {bit_width}"
            ))),
        }
    }
}
