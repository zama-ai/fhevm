use crate::types::{FheOperationType, FhevmError, SupportedFheCiphertexts, SupportedFheOperations};
use tfhe::{
    prelude::{
        CastInto, FheEq, FheMax, FheMin, FheOrd, FheTryTrivialEncrypt, IfThenElse, RotateLeft,
        RotateRight,
    },
    FheBool, FheUint16, FheUint32, FheUint64, FheUint8,
};

pub fn deserialize_fhe_ciphertext(
    input_type: i16,
    input_bytes: &[u8],
) -> Result<SupportedFheCiphertexts, FhevmError> {
    match input_type {
        1 => {
            let v: tfhe::FheBool = bincode::deserialize(input_bytes)
                .map_err(|e| FhevmError::DeserializationError(e))?;
            Ok(SupportedFheCiphertexts::FheBool(v))
        }
        2 => {
            let v: tfhe::FheUint8 = bincode::deserialize(input_bytes)
                .map_err(|e| FhevmError::DeserializationError(e))?;
            Ok(SupportedFheCiphertexts::FheUint8(v))
        }
        3 => {
            let v: tfhe::FheUint16 = bincode::deserialize(input_bytes)
                .map_err(|e| FhevmError::DeserializationError(e))?;
            Ok(SupportedFheCiphertexts::FheUint16(v))
        }
        4 => {
            let v: tfhe::FheUint32 = bincode::deserialize(input_bytes)
                .map_err(|e| FhevmError::DeserializationError(e))?;
            Ok(SupportedFheCiphertexts::FheUint32(v))
        }
        5 => {
            let v: tfhe::FheUint64 = bincode::deserialize(input_bytes)
                .map_err(|e| FhevmError::DeserializationError(e))?;
            Ok(SupportedFheCiphertexts::FheUint64(v))
        }
        _ => {
            return Err(FhevmError::UnknownFheType(input_type as i32));
        }
    }
}

/// Function assumes encryption key already set
pub fn debug_trivial_encrypt_be_bytes(
    output_type: i16,
    input_bytes: &[u8],
) -> SupportedFheCiphertexts {
    match output_type {
        1 => SupportedFheCiphertexts::FheBool(
            FheBool::try_encrypt_trivial(input_bytes[0] > 0).unwrap(),
        ),
        2 => SupportedFheCiphertexts::FheUint8(
            FheUint8::try_encrypt_trivial(input_bytes[0]).unwrap(),
        ),
        3 => {
            let mut padded: [u8; 2] = [0; 2];
            let padded_len = padded.len();
            let copy_from = padded_len - input_bytes.len();
            let len = padded.len().min(input_bytes.len());
            padded[copy_from..padded_len].copy_from_slice(&input_bytes[0..len]);
            let res = u16::from_be_bytes(padded);
            SupportedFheCiphertexts::FheUint16(FheUint16::try_encrypt_trivial(res).unwrap())
        }
        4 => {
            let mut padded: [u8; 4] = [0; 4];
            let padded_len = padded.len();
            let copy_from = padded_len - input_bytes.len();
            let len = padded.len().min(input_bytes.len());
            padded[copy_from..padded_len].copy_from_slice(&input_bytes[0..len]);
            let res: u32 = u32::from_be_bytes(padded);
            SupportedFheCiphertexts::FheUint32(FheUint32::try_encrypt_trivial(res).unwrap())
        }
        5 => {
            let mut padded: [u8; 8] = [0; 8];
            let padded_len = padded.len();
            let copy_from = padded_len - input_bytes.len();
            let len = padded.len().min(input_bytes.len());
            padded[copy_from..padded_len].copy_from_slice(&input_bytes[0..len]);
            let res: u64 = u64::from_be_bytes(padded);
            SupportedFheCiphertexts::FheUint64(FheUint64::try_encrypt_trivial(res).unwrap())
        }
        other => {
            panic!("Unknown input type for trivial encryption: {other}")
        }
    }
}

pub fn current_ciphertext_version() -> i16 {
    0
}

pub fn try_expand_ciphertext_list(
    input_ciphertext: &[u8],
    server_key: &tfhe::ServerKey,
) -> Result<Vec<SupportedFheCiphertexts>, FhevmError> {
    let mut res = Vec::new();

    let the_list: tfhe::CompactCiphertextList =
        bincode::deserialize(input_ciphertext).map_err(|e| {
            let err: Box<(dyn std::error::Error + Send + Sync)> = e;
            FhevmError::DeserializationError(err)
        })?;

    let expanded = the_list
        .expand_with_key(server_key)
        .map_err(|e| FhevmError::CiphertextExpansionError(e))?;

    for idx in 0..expanded.len() {
        let Some(data_kind) = expanded.get_kind_of(idx) else {
            panic!("we're itering over what ciphertext told us how many ciphertexts are there, it must exist")
        };

        match data_kind {
            tfhe::FheTypes::Bool => {
                let ct: tfhe::FheBool = expanded
                    .get(idx)
                    .expect("Index must exist")
                    .expect("Must succeed, we just checked this is the type");

                res.push(SupportedFheCiphertexts::FheBool(ct));
            }
            tfhe::FheTypes::Uint8 => {
                let ct: tfhe::FheUint8 = expanded
                    .get(idx)
                    .expect("Index must exist")
                    .expect("Must succeed, we just checked this is the type");

                res.push(SupportedFheCiphertexts::FheUint8(ct));
            }
            tfhe::FheTypes::Uint16 => {
                let ct: tfhe::FheUint16 = expanded
                    .get(idx)
                    .expect("Index must exist")
                    .expect("Must succeed, we just checked this is the type");

                res.push(SupportedFheCiphertexts::FheUint16(ct));
            }
            tfhe::FheTypes::Uint32 => {
                let ct: tfhe::FheUint32 = expanded
                    .get(idx)
                    .expect("Index must exist")
                    .expect("Must succeed, we just checked this is the type");

                res.push(SupportedFheCiphertexts::FheUint32(ct));
            }
            tfhe::FheTypes::Uint64 => {
                let ct: tfhe::FheUint64 = expanded
                    .get(idx)
                    .expect("Index must exist")
                    .expect("Must succeed, we just checked this is the type");

                res.push(SupportedFheCiphertexts::FheUint64(ct));
            }
            other => {
                return Err(FhevmError::CiphertextExpansionUnsupportedCiphertextKind(
                    other,
                ));
            }
        }
    }

    Ok(res)
}

// return output ciphertext type
pub fn check_fhe_operand_types(
    fhe_operation: i32,
    input_types: &[i16],
    input_handles: &[Vec<u8>],
    is_input_handle_scalar: &[bool],
) -> Result<i16, FhevmError> {
    assert_eq!(input_handles.len(), is_input_handle_scalar.len());

    let fhe_op: SupportedFheOperations = fhe_operation.try_into()?;
    let fhe_bool_type = 1;

    let scalar_operands = is_input_handle_scalar
        .iter()
        .enumerate()
        .filter(|(_, is_scalar)| **is_scalar)
        .collect::<Vec<_>>();

    let is_scalar = scalar_operands.len() > 0;

    if scalar_operands.len() > 1 {
        return Err(FhevmError::FheOperationOnlyOneOperandCanBeScalar {
            fhe_operation,
            fhe_operation_name: format!("{:?}", fhe_op),
            scalar_operand_count: scalar_operands.len(),
            max_scalar_operands: 1,
        });
    }

    if is_scalar {
        assert_eq!(
            scalar_operands.len(),
            1,
            "We checked already that not more than 1 scalar operand can be present"
        );

        if !does_fhe_operation_support_scalar(&fhe_op) {
            return Err(FhevmError::FheOperationDoesntSupportScalar {
                fhe_operation,
                fhe_operation_name: format!("{:?}", fhe_op),
                scalar_requested: is_scalar,
                scalar_supported: false,
            });
        }

        let scalar_input_index = scalar_operands[0].0;
        if scalar_input_index != 1 {
            return Err(FhevmError::FheOperationOnlySecondOperandCanBeScalar {
                scalar_input_index,
                only_allowed_scalar_input_index: 1,
            });
        }
    }

    match fhe_op.op_type() {
        FheOperationType::Binary => {
            let expected_operands = 2;
            if input_types.len() != expected_operands {
                return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    expected_operands,
                    got_operands: input_types.len(),
                });
            }

            if !is_scalar && input_types[0] != input_types[1] {
                return Err(FhevmError::FheOperationDoesntHaveUniformTypesAsInput {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    operand_types: input_types.to_vec(),
                });
            }

            if input_types[0] == fhe_bool_type && !fhe_op.supports_bool_inputs() {
                return Err(FhevmError::OperationDoesntSupportBooleanInputs {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    operand_type: fhe_bool_type,
                });
            }

            // special case for div operation, rhs for scalar must be zero
            if is_scalar && fhe_op == SupportedFheOperations::FheDiv {
                let all_zeroes = input_handles[1].iter().all(|i| *i == 0u8);
                if all_zeroes {
                    return Err(FhevmError::FheOperationScalarDivisionByZero {
                        lhs_handle: format!("0x{}", hex::encode(&input_handles[0])),
                        rhs_value: format!("0x{}", hex::encode(&input_handles[1])),
                        fhe_operation,
                        fhe_operation_name: format!("{:?}", SupportedFheOperations::FheDiv),
                    });
                }
            }

            if fhe_op.is_comparison() {
                return Ok(fhe_bool_type); // fhe bool type
            }

            return Ok(input_types[0]);
        }
        FheOperationType::Unary => {
            let expected_operands = 1;
            if input_types.len() != expected_operands {
                return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    expected_operands,
                    got_operands: input_types.len(),
                });
            }

            let fhe_bool_type = 1;
            if input_types[0] == fhe_bool_type && !fhe_op.supports_bool_inputs() {
                return Err(FhevmError::OperationDoesntSupportBooleanInputs {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    operand_type: fhe_bool_type,
                });
            }

            return Ok(input_types[0]);
        }
        FheOperationType::Other => {
            match &fhe_op {
                // two ops + uniform types branch
                // what about scalar compute?
                SupportedFheOperations::FheIfThenElse => {
                    let expected_operands = 3;
                    if input_types.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
                        });
                    }

                    // TODO: figure out typing system with constants
                    let fhe_bool_type = 1;
                    if input_types[0] != fhe_bool_type {
                        return Err(FhevmError::FheIfThenElseUnexpectedOperandTypes {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            first_expected_operand_type: fhe_bool_type,
                            first_expected_operand_type_name: "FheBool".to_string(),
                            first_operand_type: input_types[0],
                        });
                    }

                    if input_types[1] != input_types[2] {
                        return Err(
                            FhevmError::FheIfThenElseMismatchingSecondAndThirdOperatorTypes {
                                fhe_operation,
                                fhe_operation_name: format!("{:?}", fhe_op),
                                second_operand_type: input_types[1],
                                third_operand_type: input_types[2],
                            },
                        );
                    }

                    Ok(input_types[1])
                }
                SupportedFheOperations::FheCast => {
                    let expected_operands = 2;
                    if input_types.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
                        });
                    }

                    match (is_input_handle_scalar[0], is_input_handle_scalar[1]) {
                        (false, true) => {
                            let op = &input_handles[1];
                            if op.len() != 1 {
                                return Err(
                                    FhevmError::UnexpectedCastOperandSizeForScalarOperand {
                                        fhe_operation,
                                        fhe_operation_name: format!("{:?}", fhe_op),
                                        expected_scalar_operand_bytes: 1,
                                        got_bytes: op.len(),
                                    },
                                );
                            }

                            let output_type = op[0] as i32;
                            validate_fhe_type(output_type)?;
                            Ok(output_type as i16)
                        }
                        (other_left, other_right) => {
                            let bool_to_op =
                                |inp| (if inp { "scalar" } else { "handle" }).to_string();

                            return Err(FhevmError::UnexpectedCastOperandTypes {
                                fhe_operation,
                                fhe_operation_name: format!("{:?}", fhe_op),
                                expected_operator_combination: vec![
                                    "handle".to_string(),
                                    "scalar".to_string(),
                                ],
                                got_operand_combination: vec![
                                    bool_to_op(other_left),
                                    bool_to_op(other_right),
                                ],
                            });
                        }
                    }
                }
                other => {
                    panic!("Unexpected branch: {:?}", other)
                }
            }
        }
    }
}

pub fn validate_fhe_type(input_type: i32) -> Result<(), FhevmError> {
    let i16_type: i16 = input_type
        .try_into()
        .or(Err(FhevmError::UnknownFheType(input_type)))?;
    match i16_type {
        1 | 2 | 3 | 4 | 5 => Ok(()),
        _ => Err(FhevmError::UnknownFheType(input_type)),
    }
}

pub fn does_fhe_operation_support_scalar(op: &SupportedFheOperations) -> bool {
    match op.op_type() {
        FheOperationType::Binary => true,
        FheOperationType::Unary => false,
        FheOperationType::Other => {
            match op {
                // second operand determines which type to cast to
                SupportedFheOperations::FheCast => true,
                _ => false,
            }
        }
    }
}

// add operations here that don't support both encrypted operands
pub fn does_fhe_operation_support_both_encrypted_operands(op: &SupportedFheOperations) -> bool {
    match op {
        SupportedFheOperations::FheDiv => false,
        _ => true,
    }
}

pub fn perform_fhe_operation(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
) -> Result<SupportedFheCiphertexts, FhevmError> {
    let fhe_operation: SupportedFheOperations = fhe_operation_int.try_into()?;
    match fhe_operation {
        SupportedFheOperations::FheAdd => {
            assert_eq!(input_operands.len(), 2);

            // fhe add
            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a + b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a + b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a + b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a + b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a + (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a + (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a + (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a + (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheSub => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a - b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a - b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a - b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a - b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a - (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a - (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a - (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a - (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheMul => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a * b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a * b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a * b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a * b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a * (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a * (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a * (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a * (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheDiv => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a / b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a / b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a / b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a / b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a / (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a / (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a / (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a / (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheRem => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a % b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a % b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a % b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a % b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a % (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a % (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a % (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a % (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheBitAnd => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a & b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a & b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a & b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a & b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a & (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a & (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a & (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a & (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheBitOr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a | b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a | b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a | b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a | b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a | (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a | (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a | (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a | (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheBitXor => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a ^ b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a ^ b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a ^ b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a ^ b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a ^ (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a ^ (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a ^ (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a ^ (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheShl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a << b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a << b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a << b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a << b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a << (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a << (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a << (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a << (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheShr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a >> b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a >> b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a >> b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a >> b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a >> (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a >> (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a >> (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a >> (l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheRotl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a.rotate_left(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a.rotate_left(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a.rotate_left(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a.rotate_left(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a.rotate_left(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a.rotate_left(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a.rotate_left(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a.rotate_left(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheRotr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a.rotate_right(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a.rotate_right(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a.rotate_right(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a.rotate_right(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a.rotate_right(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a.rotate_right(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a.rotate_right(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a.rotate_right(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheMin => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a.min(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a.min(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a.min(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a.min(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a.min(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a.min(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a.min(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a.min(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheMax => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a.max(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a.max(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a.max(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a.max(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint8(a.max(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint16(a.max(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint32(a.max(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheUint64(a.max(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheEq => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, h) = b.to_low_high_u128();
                    let non_zero = l > 0 || h > 0;
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(non_zero)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheNe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, h) = b.to_low_high_u128();
                    let non_zero = l > 0 || h > 0;
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(non_zero)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheGe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheGt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheLe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheLt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    // TODO: figure out type to add correctly 256 bit operands from handles
                    let (l, h) = b.to_low_high_u128();
                    assert_eq!(h, 0, "Not supported yet");
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u64)))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheNot => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheBool(a) => Ok(SupportedFheCiphertexts::FheBool(!a)),
                SupportedFheCiphertexts::FheUint8(a) => Ok(SupportedFheCiphertexts::FheUint8(!a)),
                SupportedFheCiphertexts::FheUint16(a) => Ok(SupportedFheCiphertexts::FheUint16(!a)),
                SupportedFheCiphertexts::FheUint32(a) => Ok(SupportedFheCiphertexts::FheUint32(!a)),
                SupportedFheCiphertexts::FheUint64(a) => Ok(SupportedFheCiphertexts::FheUint64(!a)),
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheNeg => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint8(a) => Ok(SupportedFheCiphertexts::FheUint8(-a)),
                SupportedFheCiphertexts::FheUint16(a) => Ok(SupportedFheCiphertexts::FheUint16(-a)),
                SupportedFheCiphertexts::FheUint32(a) => Ok(SupportedFheCiphertexts::FheUint32(-a)),
                SupportedFheCiphertexts::FheUint64(a) => Ok(SupportedFheCiphertexts::FheUint64(-a)),
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        SupportedFheOperations::FheIfThenElse => {
            assert_eq!(input_operands.len(), 3);

            let SupportedFheCiphertexts::FheBool(flag) = &input_operands[0] else {
                panic!("flag for if-then-else must be boolean")
            };

            match (&input_operands[1], &input_operands[2]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheBool(res))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint8(res))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint16(res))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint32(res))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint64(res))
                }
                _ => {
                    panic!("Mismatch between cmux operand types")
                }
            }
        }
        SupportedFheOperations::FheCast => match (&input_operands[0], &input_operands[1]) {
            (SupportedFheCiphertexts::FheBool(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let (l, h) = op.to_low_high_u128();
                assert_eq!(h, 0, "Not supported yet");
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheBool(inp.clone()));
                } else {
                    match l {
                        2 => {
                            let out: tfhe::FheUint8 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint8(out))
                        }
                        3 => {
                            let out: tfhe::FheUint16 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint16(out))
                        }
                        4 => {
                            let out: tfhe::FheUint32 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint32(out))
                        }
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        other => panic!("unexpected type: {other}"),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint8(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let (l, h) = op.to_low_high_u128();
                assert_eq!(h, 0, "Not supported yet");
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint8(inp.clone()));
                } else {
                    match l {
                        1 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        3 => {
                            let out: tfhe::FheUint16 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint16(out))
                        }
                        4 => {
                            let out: tfhe::FheUint32 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint32(out))
                        }
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        other => panic!("unexpected type: {other}"),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint16(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let (l, h) = op.to_low_high_u128();
                assert_eq!(h, 0, "Not supported yet");
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint16(inp.clone()));
                } else {
                    match l {
                        1 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        2 => {
                            let out: tfhe::FheUint8 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint8(out))
                        }
                        4 => {
                            let out: tfhe::FheUint32 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint32(out))
                        }
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        other => panic!("unexpected type: {other}"),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint32(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let (l, h) = op.to_low_high_u128();
                assert_eq!(h, 0, "Not supported yet");
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint32(inp.clone()));
                } else {
                    match l {
                        1 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        2 => {
                            let out: tfhe::FheUint8 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint8(out))
                        }
                        3 => {
                            let out: tfhe::FheUint16 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint16(out))
                        }
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        other => panic!("unexpected type: {other}"),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint64(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let (l, h) = op.to_low_high_u128();
                assert_eq!(h, 0, "Not supported yet");
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint64(inp.clone()));
                } else {
                    match l {
                        1 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        2 => {
                            let out: tfhe::FheUint8 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint8(out))
                        }
                        3 => {
                            let out: tfhe::FheUint16 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint16(out))
                        }
                        4 => {
                            let out: tfhe::FheUint32 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint32(out))
                        }
                        other => panic!("unexpected type: {other}"),
                    }
                }
            }
            _ => {
                panic!("unknown cast pair")
            }
        },
        SupportedFheOperations::FheGetInputCiphertext => {
            Err(FhevmError::UnknownFheOperation(fhe_operation_int as i32))
        }
    }
}
