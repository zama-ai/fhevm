use tfhe::integer::U256;
use tfhe::prelude::FheDecrypt;
use tfhe::CompressedCiphertextListBuilder;

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
        }
    }
}

#[derive(Clone)]
pub enum SupportedFheCiphertexts {
    FheBool(tfhe::FheBool),
    FheUint8(tfhe::FheUint8),
    FheUint16(tfhe::FheUint16),
    FheUint32(tfhe::FheUint32),
    FheUint64(tfhe::FheUint64),
    Scalar(U256),
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
    FheCast = 30,
    FheIfThenElse = 31,
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
            SupportedFheCiphertexts::FheBool(v) => (type_num, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint8(v) => (type_num, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint16(v) => (type_num, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint32(v) => (type_num, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::FheUint64(v) => (type_num, bincode::serialize(v).unwrap()),
            SupportedFheCiphertexts::Scalar(_) => {
                panic!("we should never need to serialize scalar")
            }
        }
    }

    pub fn type_num(&self) -> i16 {
        match self {
            SupportedFheCiphertexts::FheBool(_) => 1,
            SupportedFheCiphertexts::FheUint8(_) => 2,
            SupportedFheCiphertexts::FheUint16(_) => 3,
            SupportedFheCiphertexts::FheUint32(_) => 4,
            SupportedFheCiphertexts::FheUint64(_) => 5,
            SupportedFheCiphertexts::Scalar(_) => {
                panic!("we should never need to serialize scalar")
            }
        }
    }

    pub fn decrypt(&self, client_key: &tfhe::ClientKey) -> String {
        match self {
            SupportedFheCiphertexts::FheBool(v) => v.decrypt(client_key).to_string(),
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
            SupportedFheCiphertexts::Scalar(v) => {
                let (l, h) = v.to_low_high_u128();
                format!("{l}{h}")
            }
        }
    }

    pub fn compress(self) -> Vec<u8> {
        let mut builder = CompressedCiphertextListBuilder::new();
        match self {
            SupportedFheCiphertexts::FheBool(c) => builder.push(c),
            SupportedFheCiphertexts::FheUint8(c) => builder.push(c),
            SupportedFheCiphertexts::FheUint16(c) => builder.push(c),
            SupportedFheCiphertexts::FheUint32(c) => builder.push(c),
            SupportedFheCiphertexts::FheUint64(c) => builder.push(c),
            SupportedFheCiphertexts::Scalar(_) => {
                // TODO: Need to fix that, scalars are not ciphertexts.
                panic!("cannot compress a scalar");
            }
        };
        let list = builder.build().expect("ciphertext compression");
        bincode::serialize(&list).expect("compressed list serialization")
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
            SupportedFheOperations::FheIfThenElse | SupportedFheOperations::FheCast => {
                FheOperationType::Other
            }
            SupportedFheOperations::FheGetInputCiphertext => FheOperationType::Other,
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            SupportedFheOperations::FheEq
            | SupportedFheOperations::FheNe
            | SupportedFheOperations::FheGe
            | SupportedFheOperations::FheGt
            | SupportedFheOperations::FheLe
            | SupportedFheOperations::FheLt => true,
            _ => false,
        }
    }

    pub fn supports_bool_inputs(&self) -> bool {
        match self {
            SupportedFheOperations::FheEq
            | SupportedFheOperations::FheNe
            | SupportedFheOperations::FheNot => true,
            _ => false,
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
            30 => Ok(SupportedFheOperations::FheCast),
            31 => Ok(SupportedFheOperations::FheIfThenElse),
            32 => Ok(SupportedFheOperations::FheGetInputCiphertext),
            _ => Err(FhevmError::UnknownFheOperation(value as i32)),
        };

        // ensure we're always having the same value serialized back and forth
        if let Ok(v) = &res {
            assert_eq!(v.clone() as i16, value);
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

pub type Handle = [u8; 32];
