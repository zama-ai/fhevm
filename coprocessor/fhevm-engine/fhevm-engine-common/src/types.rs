use alloy::providers::RootProvider;
use alloy_provider::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use anyhow::Result;
use bigdecimal::num_bigint::BigInt;
use tfhe::integer::bigint::StaticUnsignedBigInt;
use tfhe::integer::ciphertext::BaseRadixCiphertext;
use tfhe::integer::U256;
use tfhe::prelude::{CiphertextList, FheDecrypt};
use tfhe::shortint::Ciphertext;
use tfhe::{CompressedCiphertextList, CompressedCiphertextListBuilder};

use crate::utils::{safe_deserialize, safe_serialize};

#[derive(Debug)]
pub enum FhevmError {
    UnknownFheOperation(i32),
    UnknownFheType(i32),
    DeserializationError(Box<dyn std::error::Error + Sync + Send>),
    CiphertextExpansionError(tfhe::Error),
    CiphertextExpansionUnsupportedCiphertextKind(tfhe::FheTypes),
    FheOperationOnlyOneOperandCanBeScalar {
        fhe_operation: i32,
        fhe_operation_name: String,
        scalar_operand_count: usize,
        max_scalar_operands: usize,
    },
    FheOperationDoesntSupportScalar {
        fhe_operation: i32,
        fhe_operation_name: String,
        scalar_requested: bool,
        scalar_supported: bool,
    },
    FheOperationOnlySecondOperandCanBeScalar {
        scalar_input_index: usize,
        only_allowed_scalar_input_index: usize,
    },
    FheOperationDoesntHaveUniformTypesAsInput {
        fhe_operation: i32,
        fhe_operation_name: String,
        operand_types: Vec<i16>,
    },
    FheOperationScalarDivisionByZero {
        lhs_handle: String,
        rhs_value: String,
        fhe_operation: i32,
        fhe_operation_name: String,
    },
    FheOperationDoesntSupportEbytesAsInput {
        lhs_handle: String,
        rhs_handle: String,
        fhe_operation: i32,
        fhe_operation_name: String,
    },
    UnexpectedOperandCountForFheOperation {
        fhe_operation: i32,
        fhe_operation_name: String,
        expected_operands: usize,
        got_operands: usize,
    },
    OperationDoesntSupportBooleanInputs {
        fhe_operation: i32,
        fhe_operation_name: String,
        operand_type: i16,
    },
    FheIfThenElseUnexpectedOperandTypes {
        fhe_operation: i32,
        fhe_operation_name: String,
        first_operand_type: i16,
        first_expected_operand_type: i16,
        first_expected_operand_type_name: String,
    },
    FheIfThenElseMismatchingSecondAndThirdOperatorTypes {
        fhe_operation: i32,
        fhe_operation_name: String,
        second_operand_type: i16,
        third_operand_type: i16,
    },
    UnexpectedCastOperandTypes {
        fhe_operation: i32,
        fhe_operation_name: String,
        expected_operator_combination: Vec<String>,
        got_operand_combination: Vec<String>,
    },
    UnexpectedCastOperandSizeForScalarOperand {
        fhe_operation: i32,
        fhe_operation_name: String,
        expected_scalar_operand_bytes: usize,
        got_bytes: usize,
    },
    AllInputsForTrivialEncryptionMustBeScalar {
        fhe_operation: i32,
        fhe_operation_name: String,
    },
    UnexpectedTrivialEncryptionOperandSizeForScalarOperand {
        fhe_operation: i32,
        fhe_operation_name: String,
        expected_scalar_operand_bytes: usize,
        got_bytes: usize,
    },
    UnexpectedRandOperandSizeForOutputType {
        fhe_operation: i32,
        fhe_operation_name: String,
        expected_operand_bytes: usize,
        got_bytes: usize,
    },
    RandOperationUpperBoundCannotBeZero {
        fhe_operation: i32,
        fhe_operation_name: String,
        upper_bound_value: String,
    },
    RandOperationInputsMustAllBeScalar {
        fhe_operation: i32,
        fhe_operation_name: String,
        scalar_operand_count: usize,
        expected_scalar_operand_count: usize,
    },
    BadInputs,
    MissingTfheRsData,
    InvalidHandle,
    UnsupportedFheTypes {
        fhe_operation: String,
        input_types: Vec<&'static str>,
    },
    UnknownCastType {
        fhe_operation: String,
        type_to_cast_to: i16,
    },
}

impl std::error::Error for FhevmError {}

impl std::fmt::Display for FhevmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownFheOperation(op) => {
                write!(f, "Unknown fhe operation: {}", op)
            }
            Self::UnknownFheType(op) => {
                write!(f, "Unknown fhe type: {}", op)
            }
            Self::DeserializationError(e) => {
                write!(f, "error deserializing ciphertext: {:?}", e)
            }
            Self::CiphertextExpansionError(e) => {
                write!(f, "error expanding compact ciphertext list: {:?}", e)
            }
            Self::CiphertextExpansionUnsupportedCiphertextKind(e) => {
                write!(
                    f,
                    "unsupported tfhe type found while expanding ciphertexts: {:?}",
                    e
                )
            }
            Self::FheOperationDoesntSupportScalar {
                fhe_operation,
                fhe_operation_name,
                ..
            } => {
                write!(f, "fhe operation number {fhe_operation} ({fhe_operation_name}) doesn't support scalar computation")
            }
            Self::FheOperationDoesntHaveUniformTypesAsInput {
                fhe_operation,
                fhe_operation_name,
                operand_types,
            } => {
                write!(f, "fhe operation number {fhe_operation} ({fhe_operation_name}) expects uniform types as input, received: {:?}", operand_types)
            }
            Self::FheOperationScalarDivisionByZero {
                lhs_handle,
                rhs_value,
                fhe_operation,
                fhe_operation_name,
            } => {
                write!(f, "zero on the right side of scalar division, lhs handle: {lhs_handle}, rhs value: {rhs_value}, fhe operation: {fhe_operation} fhe operation name:{fhe_operation_name}")
            }
            Self::FheOperationDoesntSupportEbytesAsInput {
                lhs_handle,
                rhs_handle: rhs_value,
                fhe_operation,
                fhe_operation_name,
            } => {
                write!(f, "zero on the right side of scalar division, lhs handle: {lhs_handle}, rhs value: {rhs_value}, fhe operation: {fhe_operation} fhe operation name:{fhe_operation_name}")
            }
            Self::UnexpectedOperandCountForFheOperation {
                fhe_operation,
                fhe_operation_name,
                expected_operands,
                got_operands,
            } => {
                write!(f, "fhe operation number {fhe_operation} ({fhe_operation_name}) received unexpected operand count, expected: {expected_operands}, received: {got_operands}")
            }
            Self::OperationDoesntSupportBooleanInputs {
                fhe_operation,
                fhe_operation_name,
                operand_type,
            } => {
                write!(f, "fhe operation number {fhe_operation} ({fhe_operation_name}) does not support booleans as inputs, input type: {operand_type}")
            }
            Self::FheOperationOnlySecondOperandCanBeScalar {
                scalar_input_index,
                only_allowed_scalar_input_index,
            } => {
                write!(f, "computation has scalar operand which is not the second operand, scalar input index: {scalar_input_index}, only allowed scalar input index: {only_allowed_scalar_input_index}")
            }
            Self::UnexpectedCastOperandTypes {
                fhe_operation,
                fhe_operation_name,
                expected_operator_combination,
                got_operand_combination,
            } => {
                write!(f, "unexpected operand types for cast, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}, expected operand combination: {:?}, got operand combination: {:?}", expected_operator_combination, got_operand_combination)
            }
            Self::UnexpectedCastOperandSizeForScalarOperand {
                fhe_operation,
                fhe_operation_name,
                expected_scalar_operand_bytes,
                got_bytes,
            } => {
                write!(f, "unexpected operand size for cast, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}, expected bytes: {}, got bytes: {}", expected_scalar_operand_bytes, got_bytes)
            }
            Self::AllInputsForTrivialEncryptionMustBeScalar {
                fhe_operation,
                fhe_operation_name,
            } => {
                write!(f, "all inputs for trivial encryption must be scalar, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}")
            }
            Self::UnexpectedTrivialEncryptionOperandSizeForScalarOperand {
                fhe_operation,
                fhe_operation_name,
                expected_scalar_operand_bytes,
                got_bytes,
            } => {
                write!(f, "unexpected operand size for trivial encryption, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}, expected bytes: {}, got bytes: {}", expected_scalar_operand_bytes, got_bytes)
            }
            Self::FheIfThenElseUnexpectedOperandTypes {
                fhe_operation,
                fhe_operation_name,
                first_operand_type,
                first_expected_operand_type,
                ..
            } => {
                write!(f, "fhe if then else first operand should always be FheBool, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}, first operand type: {first_operand_type}, first operand expected type: {first_expected_operand_type}")
            }
            Self::FheIfThenElseMismatchingSecondAndThirdOperatorTypes {
                fhe_operation,
                fhe_operation_name,
                second_operand_type,
                third_operand_type,
            } => {
                write!(f, "fhe if then else second and third operand types don't match, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}, second operand type: {second_operand_type}, third operand type: {third_operand_type}")
            }
            Self::FheOperationOnlyOneOperandCanBeScalar {
                fhe_operation,
                fhe_operation_name,
                scalar_operand_count,
                max_scalar_operands,
            } => {
                write!(f, "only one operand can be scalar, fhe operation: {fhe_operation}, fhe operation name: {fhe_operation_name}, second operand count: {scalar_operand_count}, max scalar operands: {max_scalar_operands}")
            }
            Self::UnexpectedRandOperandSizeForOutputType {
                fhe_operation,
                fhe_operation_name,
                expected_operand_bytes,
                got_bytes,
            } => {
                write!(f, "operation must have only one byte for output operand type {fhe_operation} ({fhe_operation_name}) expects bytes {}, received: {}", expected_operand_bytes, got_bytes)
            }
            Self::RandOperationUpperBoundCannotBeZero {
                fhe_operation,
                fhe_operation_name,
                upper_bound_value,
            } => {
                write!(f, "rand bounded operation cannot receive zero as upper bound {fhe_operation} ({fhe_operation_name}) received: {}", upper_bound_value)
            }
            Self::RandOperationInputsMustAllBeScalar {
                fhe_operation,
                fhe_operation_name,
                scalar_operand_count,
                expected_scalar_operand_count,
            } => {
                write!(f, "operation must have all operands as scalar {fhe_operation} ({fhe_operation_name}) expected scalar operands {}, received: {}", expected_scalar_operand_count, scalar_operand_count)
            }
            Self::BadInputs => {
                write!(f, "Bad inputs")
            }
            Self::MissingTfheRsData => {
                write!(f, "Missing TFHE-rs data")
            }
            Self::InvalidHandle => {
                write!(f, "Invalid ciphertext handle")
            }
            Self::UnsupportedFheTypes {
                fhe_operation,
                input_types,
            } => {
                write!(
                    f,
                    "Unsupported type combination for fhe operation {fhe_operation}: {:?}",
                    input_types
                )
            }
            Self::UnknownCastType {
                fhe_operation,
                type_to_cast_to,
            } => {
                write!(
                    f,
                    "Unknown type to cast to for fhe operation {fhe_operation}: {}",
                    type_to_cast_to
                )
            }
        }
    }
}

#[derive(Clone)]
pub enum SupportedFheCiphertexts {
    FheBool(tfhe::FheBool),
    FheUint4(tfhe::FheUint4),
    FheUint8(tfhe::FheUint8),
    FheUint16(tfhe::FheUint16),
    FheUint32(tfhe::FheUint32),
    FheUint64(tfhe::FheUint64),
    FheUint128(tfhe::FheUint128),
    FheUint160(tfhe::FheUint160),
    FheUint256(tfhe::FheUint256),
    FheBytes64(tfhe::FheUint512),
    FheBytes128(tfhe::FheUint1024),
    FheBytes256(tfhe::FheUint2048),
    // big endian unsigned integer bytes
    Scalar(Vec<u8>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIter)]
#[repr(i8)]
pub enum SupportedFheOperations {
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
    FheCast = 23,
    FheTrivialEncrypt = 24,
    FheIfThenElse = 25,
    FheRand = 26,
    FheRandBounded = 27,
    FheGetInputCiphertext = 32,
}

#[derive(PartialEq, Eq)]
pub enum FheOperationType {
    Binary,
    Unary,
    Other,
}

impl SupportedFheCiphertexts {
    pub fn serialize(&self) -> (i16, Vec<u8>) {
        let type_num = self.type_num();
        match self {
            SupportedFheCiphertexts::FheBool(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint4(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint8(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint16(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint32(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint64(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint128(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint160(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheUint256(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheBytes64(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheBytes128(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::FheBytes256(v) => (type_num, safe_serialize(v)),
            SupportedFheCiphertexts::Scalar(_) => {
                panic!("we should never need to serialize scalar")
            }
        }
    }

    pub fn to_ciphertext64(self) -> BaseRadixCiphertext<Ciphertext> {
        match self {
            SupportedFheCiphertexts::FheBool(v) => {
                BaseRadixCiphertext::from(vec![v.into_raw_parts()])
            }
            SupportedFheCiphertexts::FheUint4(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint8(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint16(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint32(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint64(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint128(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint160(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheUint256(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheBytes64(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheBytes128(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::FheBytes256(v) => v.into_raw_parts().0,
            SupportedFheCiphertexts::Scalar(_) => {
                panic!("scalar cannot be converted to regular ciphertext")
            }
        }
    }

    pub fn type_num(&self) -> i16 {
        match self {
            // values taken to match with solidity library
            SupportedFheCiphertexts::FheBool(_) => 0,
            SupportedFheCiphertexts::FheUint4(_) => 1,
            SupportedFheCiphertexts::FheUint8(_) => 2,
            SupportedFheCiphertexts::FheUint16(_) => 3,
            SupportedFheCiphertexts::FheUint32(_) => 4,
            SupportedFheCiphertexts::FheUint64(_) => 5,
            SupportedFheCiphertexts::FheUint128(_) => 6,
            SupportedFheCiphertexts::FheUint160(_) => 7,
            SupportedFheCiphertexts::FheUint256(_) => 8,
            SupportedFheCiphertexts::FheBytes64(_) => 9,
            SupportedFheCiphertexts::FheBytes128(_) => 10,
            SupportedFheCiphertexts::FheBytes256(_) => 11,
            SupportedFheCiphertexts::Scalar(_) => {
                // need this for tracing as we join types of computation for a trace
                200
            }
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            SupportedFheCiphertexts::FheBool(..) => "FheBool",
            SupportedFheCiphertexts::FheUint4(..) => "FheUint4",
            SupportedFheCiphertexts::FheUint8(..) => "FheUint8",
            SupportedFheCiphertexts::FheUint16(..) => "FheUint16",
            SupportedFheCiphertexts::FheUint32(..) => "FheUint32",
            SupportedFheCiphertexts::FheUint64(..) => "FheUint64",
            SupportedFheCiphertexts::FheUint128(..) => "FheUint128",
            SupportedFheCiphertexts::FheUint160(..) => "FheUint160",
            SupportedFheCiphertexts::FheUint256(..) => "FheUint256",
            SupportedFheCiphertexts::FheBytes64(..) => "FheBytes64",
            SupportedFheCiphertexts::FheBytes128(..) => "FheBytes128",
            SupportedFheCiphertexts::FheBytes256(..) => "FheBytes256",
            SupportedFheCiphertexts::Scalar(..) => "Scalar",
        }
    }

    pub fn decrypt(&self, client_key: &tfhe::ClientKey) -> String {
        match self {
            SupportedFheCiphertexts::FheBool(v) => v.decrypt(client_key).to_string(),
            SupportedFheCiphertexts::FheUint4(v) => {
                FheDecrypt::<u8>::decrypt(v, client_key).to_string()
            }
            SupportedFheCiphertexts::FheUint8(v) => {
                FheDecrypt::<u8>::decrypt(v, client_key).to_string()
            }
            SupportedFheCiphertexts::FheUint16(v) => {
                FheDecrypt::<u16>::decrypt(v, client_key).to_string()
            }
            SupportedFheCiphertexts::FheUint32(v) => {
                FheDecrypt::<u32>::decrypt(v, client_key).to_string()
            }
            SupportedFheCiphertexts::FheUint64(v) => {
                FheDecrypt::<u64>::decrypt(v, client_key).to_string()
            }
            SupportedFheCiphertexts::FheUint128(v) => {
                FheDecrypt::<u128>::decrypt(v, client_key).to_string()
            }
            SupportedFheCiphertexts::FheUint160(v) => {
                let dec = FheDecrypt::<U256>::decrypt(v, client_key);
                let mut slice: [u8; 32] = [0; 32];
                dec.copy_to_be_byte_slice(&mut slice);
                let final_slice = &slice[slice.len() - 20..];
                BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, final_slice).to_string()
            }
            SupportedFheCiphertexts::FheUint256(v) => {
                let dec = FheDecrypt::<U256>::decrypt(v, client_key);
                let mut slice: [u8; 32] = [0; 32];
                dec.copy_to_be_byte_slice(&mut slice);
                BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &slice).to_string()
            }
            SupportedFheCiphertexts::FheBytes64(v) => {
                let dec = FheDecrypt::<StaticUnsignedBigInt<8>>::decrypt(v, client_key);
                let mut slice: [u8; 64] = [0; 64];
                dec.copy_to_be_byte_slice(&mut slice);
                BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &slice).to_string()
            }
            SupportedFheCiphertexts::FheBytes128(v) => {
                let dec = FheDecrypt::<StaticUnsignedBigInt<16>>::decrypt(v, client_key);
                let mut slice: [u8; 128] = [0; 128];
                dec.copy_to_be_byte_slice(&mut slice);
                BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &slice).to_string()
            }
            SupportedFheCiphertexts::FheBytes256(v) => {
                let dec = FheDecrypt::<StaticUnsignedBigInt<32>>::decrypt(v, client_key);
                let mut slice: [u8; 256] = [0; 256];
                dec.copy_to_be_byte_slice(&mut slice);
                BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, &slice).to_string()
            }
            SupportedFheCiphertexts::Scalar(v) => {
                BigInt::from_bytes_be(bigdecimal::num_bigint::Sign::Plus, v).to_string()
            }
        }
    }

    pub fn compress(&self) -> (i16, Vec<u8>) {
        let type_num = self.type_num();
        match self {
            SupportedFheCiphertexts::Scalar(v) => {
                // Scalars are not ciphertexts, so we just serialize them directly
                (type_num, safe_serialize(v))
            }
            _ => {
                let mut builder = CompressedCiphertextListBuilder::new();
                match self {
                    SupportedFheCiphertexts::FheBool(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint4(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint8(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint16(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint32(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint64(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint128(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint160(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheUint256(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheBytes64(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheBytes128(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::FheBytes256(c) => builder.push(c.clone()),
                    SupportedFheCiphertexts::Scalar(_) => unreachable!(), // Handled above
                };
                let list = builder.build().expect("ciphertext compression");
                (type_num, safe_serialize(&list))
            }
        }
    }

    #[cfg(feature = "gpu")]
    pub fn decompress(ct_type: i16, list: &[u8], gpu_idx: usize) -> Result<Self> {
        // Handle scalar decompression separately
        if ct_type == 200 {
            let scalar_data: Vec<u8> = safe_deserialize(list)?;
            return Ok(SupportedFheCiphertexts::Scalar(scalar_data));
        }
        
        use crate::gpu_memory::{release_memory_on_gpu, reserve_memory_on_gpu};
        let ctlist: CompressedCiphertextList = safe_deserialize(list)?;
        let mut reserved_mem = 0;
        if let Ok(Some(decomp_size)) = ctlist.get_decompression_size_on_gpu(gpu_idx) {
            reserved_mem = decomp_size;
        };
        reserve_memory_on_gpu(reserved_mem, gpu_idx);
        let res = Self::decompress_impl(ct_type, &ctlist);
        release_memory_on_gpu(reserved_mem, gpu_idx);
        res
    }

    #[cfg(not(feature = "gpu"))]
    pub fn decompress(ct_type: i16, list: &[u8], _: usize) -> Result<Self> {
        // Handle scalar decompression separately
        if ct_type == 200 {
            let scalar_data: Vec<u8> = safe_deserialize(list)?;
            return Ok(SupportedFheCiphertexts::Scalar(scalar_data));
        }
        
        let ctlist: CompressedCiphertextList = safe_deserialize(list)?;
        Self::decompress_impl(ct_type, &ctlist)
    }

    // Decompress without checking if enough GPU memory is available -
    // used when GPU featre is active, but decompressing on CPU
    pub fn decompress_no_memcheck(ct_type: i16, list: &[u8]) -> Result<Self> {
        // Handle scalar decompression separately
        if ct_type == 200 {
            let scalar_data: Vec<u8> = safe_deserialize(list)?;
            return Ok(SupportedFheCiphertexts::Scalar(scalar_data));
        }
        
        let ctlist: CompressedCiphertextList = safe_deserialize(list)?;
        Self::decompress_impl(ct_type, &ctlist)
    }

    pub fn decompress_impl(ct_type: i16, list: &CompressedCiphertextList) -> Result<Self> {
        match ct_type {
            0 => Ok(SupportedFheCiphertexts::FheBool(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            1 => Ok(SupportedFheCiphertexts::FheUint4(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            2 => Ok(SupportedFheCiphertexts::FheUint8(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            3 => Ok(SupportedFheCiphertexts::FheUint16(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            4 => Ok(SupportedFheCiphertexts::FheUint32(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            5 => Ok(SupportedFheCiphertexts::FheUint64(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            6 => Ok(SupportedFheCiphertexts::FheUint128(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            7 => Ok(SupportedFheCiphertexts::FheUint160(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            8 => Ok(SupportedFheCiphertexts::FheUint256(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            9 => Ok(SupportedFheCiphertexts::FheBytes64(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            10 => Ok(SupportedFheCiphertexts::FheBytes128(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            11 => Ok(SupportedFheCiphertexts::FheBytes256(
                list.get(0)?.ok_or(FhevmError::MissingTfheRsData)?,
            )),
            _ => Err(FhevmError::UnknownFheType(ct_type as i32).into()),
        }
    }

    pub fn is_ebytes(&self) -> bool {
        match self {
            SupportedFheCiphertexts::FheBytes64(_)
            | SupportedFheCiphertexts::FheBytes128(_)
            | SupportedFheCiphertexts::FheBytes256(_) => true,
            SupportedFheCiphertexts::FheBool(_)
            | SupportedFheCiphertexts::FheUint4(_)
            | SupportedFheCiphertexts::FheUint8(_)
            | SupportedFheCiphertexts::FheUint16(_)
            | SupportedFheCiphertexts::FheUint32(_)
            | SupportedFheCiphertexts::FheUint64(_)
            | SupportedFheCiphertexts::FheUint128(_)
            | SupportedFheCiphertexts::FheUint160(_)
            | SupportedFheCiphertexts::FheUint256(_)
            | SupportedFheCiphertexts::Scalar(_) => false,
        }
    }
}

impl SupportedFheOperations {
    pub fn op_type(&self) -> FheOperationType {
        match self {
            SupportedFheOperations::FheAdd
            | SupportedFheOperations::FheSub
            | SupportedFheOperations::FheMul
            | SupportedFheOperations::FheDiv
            | SupportedFheOperations::FheRem
            | SupportedFheOperations::FheBitAnd
            | SupportedFheOperations::FheBitOr
            | SupportedFheOperations::FheBitXor
            | SupportedFheOperations::FheShl
            | SupportedFheOperations::FheShr
            | SupportedFheOperations::FheRotl
            | SupportedFheOperations::FheRotr
            | SupportedFheOperations::FheEq
            | SupportedFheOperations::FheNe
            | SupportedFheOperations::FheGe
            | SupportedFheOperations::FheGt
            | SupportedFheOperations::FheLe
            | SupportedFheOperations::FheLt
            | SupportedFheOperations::FheMin
            | SupportedFheOperations::FheMax => FheOperationType::Binary,
            SupportedFheOperations::FheNot | SupportedFheOperations::FheNeg => {
                FheOperationType::Unary
            }
            SupportedFheOperations::FheIfThenElse
            | SupportedFheOperations::FheCast
            | SupportedFheOperations::FheTrivialEncrypt
            | SupportedFheOperations::FheRand
            | SupportedFheOperations::FheRandBounded => FheOperationType::Other,
            SupportedFheOperations::FheGetInputCiphertext => FheOperationType::Other,
        }
    }

    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            SupportedFheOperations::FheEq
                | SupportedFheOperations::FheNe
                | SupportedFheOperations::FheGe
                | SupportedFheOperations::FheGt
                | SupportedFheOperations::FheLe
                | SupportedFheOperations::FheLt
        )
    }

    pub fn does_have_more_than_one_scalar(&self) -> bool {
        matches!(
            self,
            SupportedFheOperations::FheRand
                | SupportedFheOperations::FheRandBounded
                | SupportedFheOperations::FheTrivialEncrypt
        )
    }

    pub fn supports_bool_inputs(&self) -> bool {
        matches!(
            self,
            SupportedFheOperations::FheEq
                | SupportedFheOperations::FheNe
                | SupportedFheOperations::FheNot
                | SupportedFheOperations::FheBitAnd
                | SupportedFheOperations::FheBitOr
                | SupportedFheOperations::FheBitXor
        )
    }

    pub fn supports_ebytes_inputs(&self) -> bool {
        match self {
            SupportedFheOperations::FheBitAnd
            | SupportedFheOperations::FheBitOr
            | SupportedFheOperations::FheBitXor
            | SupportedFheOperations::FheShl
            | SupportedFheOperations::FheShr
            | SupportedFheOperations::FheRotl
            | SupportedFheOperations::FheRotr
            | SupportedFheOperations::FheEq
            | SupportedFheOperations::FheNe
            | SupportedFheOperations::FheNot
            | SupportedFheOperations::FheRand
            | SupportedFheOperations::FheRandBounded
            | SupportedFheOperations::FheIfThenElse
            | SupportedFheOperations::FheTrivialEncrypt
            | SupportedFheOperations::FheCast => true,
            SupportedFheOperations::FheGe
            | SupportedFheOperations::FheGt
            | SupportedFheOperations::FheLe
            | SupportedFheOperations::FheLt
            | SupportedFheOperations::FheMin
            | SupportedFheOperations::FheMax
            | SupportedFheOperations::FheNeg
            | SupportedFheOperations::FheAdd
            | SupportedFheOperations::FheSub
            | SupportedFheOperations::FheMul
            | SupportedFheOperations::FheDiv
            | SupportedFheOperations::FheRem
            | SupportedFheOperations::FheGetInputCiphertext => false,
        }
    }
}

impl TryFrom<i16> for SupportedFheOperations {
    type Error = FhevmError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Ok(SupportedFheOperations::FheAdd),
            1 => Ok(SupportedFheOperations::FheSub),
            2 => Ok(SupportedFheOperations::FheMul),
            3 => Ok(SupportedFheOperations::FheDiv),
            4 => Ok(SupportedFheOperations::FheRem),
            5 => Ok(SupportedFheOperations::FheBitAnd),
            6 => Ok(SupportedFheOperations::FheBitOr),
            7 => Ok(SupportedFheOperations::FheBitXor),
            8 => Ok(SupportedFheOperations::FheShl),
            9 => Ok(SupportedFheOperations::FheShr),
            10 => Ok(SupportedFheOperations::FheRotl),
            11 => Ok(SupportedFheOperations::FheRotr),
            12 => Ok(SupportedFheOperations::FheEq),
            13 => Ok(SupportedFheOperations::FheNe),
            14 => Ok(SupportedFheOperations::FheGe),
            15 => Ok(SupportedFheOperations::FheGt),
            16 => Ok(SupportedFheOperations::FheLe),
            17 => Ok(SupportedFheOperations::FheLt),
            18 => Ok(SupportedFheOperations::FheMin),
            19 => Ok(SupportedFheOperations::FheMax),
            20 => Ok(SupportedFheOperations::FheNeg),
            21 => Ok(SupportedFheOperations::FheNot),
            23 => Ok(SupportedFheOperations::FheCast),
            24 => Ok(SupportedFheOperations::FheTrivialEncrypt),
            25 => Ok(SupportedFheOperations::FheIfThenElse),
            26 => Ok(SupportedFheOperations::FheRand),
            27 => Ok(SupportedFheOperations::FheRandBounded),
            32 => Ok(SupportedFheOperations::FheGetInputCiphertext),
            _ => Err(FhevmError::UnknownFheOperation(value as i32)),
        };

        // ensure we're always having the same value serialized back and forth
        if let Ok(v) = &res {
            assert_eq!(*v as i16, value);
        }

        res
    }
}

// we get i32 from protobuf (smaller types unsupported)
// but in database we store i16
impl TryFrom<i32> for SupportedFheOperations {
    type Error = FhevmError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let initial_value: i16 = value
            .try_into()
            .map_err(|_| FhevmError::UnknownFheOperation(value))?;

        let final_value: Result<SupportedFheOperations, Self::Error> = initial_value.try_into();
        final_value
    }
}

impl From<SupportedFheOperations> for i16 {
    fn from(value: SupportedFheOperations) -> Self {
        value as i16
    }
}

pub type Handle = Vec<u8>;
pub const HANDLE_LEN: usize = 32;

pub fn get_ct_type(handle: &[u8]) -> Result<i16, FhevmError> {
    match handle.len() {
        HANDLE_LEN => Ok(handle[30] as i16),
        _ => Err(FhevmError::InvalidHandle),
    }
}

pub fn is_ebytes_type(inp: i16) -> bool {
    (9..=11).contains(&inp)
}

#[derive(Copy, Clone, Debug)]
pub enum AllowEvents {
    AllowedAccount = 0,
    AllowedForDecryption = 1,
}

pub enum AllowEventsError {
    InvalidValue(i16),
}

impl TryFrom<i16> for AllowEvents {
    type Error = AllowEventsError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AllowEvents::AllowedAccount),
            1 => Ok(AllowEvents::AllowedForDecryption),
            _ => Err(AllowEventsError::InvalidValue(value)),
        }
    }
}

pub type BlockchainProvider = FillProvider<
    JoinFill<
        alloy::providers::Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;
