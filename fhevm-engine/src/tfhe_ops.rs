use tfhe::{prelude::FheTryTrivialEncrypt, FheBool, FheUint16, FheUint32, FheUint8};

use crate::types::{CoprocessorError, FheOperationType, SupportedFheCiphertexts, SupportedFheOperations};

pub fn current_ciphertext_version() -> i16 {
    1
}

pub fn perform_fhe_operation(fhe_operation: i16, input_operands: &[SupportedFheCiphertexts]) -> Result<SupportedFheCiphertexts, Box<dyn std::error::Error + Send + Sync>> {
    let fhe_operation: SupportedFheOperations = fhe_operation.try_into()?;
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
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        },
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
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        },
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
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        },
        SupportedFheOperations::FheNot => todo!(),
        SupportedFheOperations::FheIfThenElse => todo!(),
    }
}

/// Function assumes encryption key already set
pub fn debug_trivial_encrypt_le_bytes(output_type: i16, input_bytes: &[u8]) -> SupportedFheCiphertexts {
    match output_type {
        1 => {
            SupportedFheCiphertexts::FheBool(FheBool::try_encrypt_trivial(input_bytes[0] > 0).unwrap())
        }
        2 => {
            SupportedFheCiphertexts::FheUint8(FheUint8::try_encrypt_trivial(input_bytes[0]).unwrap())
        }
        3 => {
            let mut padded: [u8; 2] = [0; 2];
            let len = padded.len().min(input_bytes.len());
            padded[0..len].copy_from_slice(&input_bytes[0..len]);
            let res = u16::from_le_bytes(padded);
            SupportedFheCiphertexts::FheUint16(FheUint16::try_encrypt_trivial(res).unwrap())
        }
        4 => {
            let mut padded: [u8; 4] = [0; 4];
            let len = padded.len().min(input_bytes.len());
            padded[0..len].copy_from_slice(&input_bytes[0..len]);
            let res: u32 = u32::from_le_bytes(padded);
            SupportedFheCiphertexts::FheUint32(FheUint32::try_encrypt_trivial(res).unwrap())
        }
        other => {
            panic!("Unknown input type for trivial encryption: {other}")
        }
    }
}

pub fn deserialize_fhe_ciphertext(input_type: i16, input_bytes: &[u8]) -> Result<SupportedFheCiphertexts, Box<dyn std::error::Error + Send + Sync>> {
    match input_type {
        1 => {
            let v: tfhe::FheBool = bincode::deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheBool(v))
        }
        2 => {
            let v: tfhe::FheUint8 = bincode::deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint8(v))
        }
        3 => {
            let v: tfhe::FheUint16 = bincode::deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint16(v))
        }
        4 => {
            let v: tfhe::FheUint32 = bincode::deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint32(v))
        }
        _ => {
            return Err(Box::new(CoprocessorError::UnknownCiphertextType(input_type)));
        }
    }
}

// return output ciphertext type
pub fn check_fhe_operand_types(fhe_operation: i32, input_types: &[i16], is_scalar: bool) -> Result<i16, CoprocessorError> {
    let fhe_op: SupportedFheOperations = fhe_operation.try_into()?;

    if is_scalar && !does_fhe_operation_support_scalar(&fhe_op) {
        return Err(CoprocessorError::FheOperationDoesntSupportScalar {
            fhe_operation,
            fhe_operation_name: format!("{:?}", fhe_op),
            scalar_requested: is_scalar,
            scalar_supported: false,
        });
    }

    match fhe_op.op_type() {
        FheOperationType::Binary => {
            let expected_operands = 2;
            if input_types.len() != expected_operands {
                return Err(CoprocessorError::UnexpectedOperandCountForFheOperation {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    expected_operands,
                    got_operands: input_types.len(),
                });
            }

            if !is_scalar && input_types[0] != input_types[1] {
                return Err(CoprocessorError::FheOperationDoesntHaveUniformTypesAsInput {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    operand_types: input_types.to_vec(),
                });
            }

            return Ok(input_types[0]);
        }
        FheOperationType::Unary => {
            let expected_operands = 1;
            if input_types.len() != expected_operands {
                return Err(CoprocessorError::UnexpectedOperandCountForFheOperation {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    expected_operands,
                    got_operands: input_types.len(),
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
                        return Err(CoprocessorError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
                        });
                    }

                    todo!("special type checking for certain operands")
                },
                other => {
                    panic!("Unexpected branch: {:?}", other)
                }
            }
        }
    }
}

// add operations here that don't support both encrypted operands
#[cfg(test)]
pub fn does_fhe_operation_support_both_encrypted_operands(op: &SupportedFheOperations) -> bool {
    match op {
        SupportedFheOperations::FheDiv => false,
        _ => true
    }
}

pub fn does_fhe_operation_support_scalar(op: &SupportedFheOperations) -> bool {
    match op.op_type() {
        FheOperationType::Binary => {
            true
        },
        FheOperationType::Unary => false,
        FheOperationType::Other => false,
    }
}