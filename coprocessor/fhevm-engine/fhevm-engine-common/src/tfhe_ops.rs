use crate::{
    keys::FhevmKeys,
    types::{
        get_ct_type, FheOperationType, FhevmError, SupportedFheCiphertexts, SupportedFheOperations,
    },
    utils::{safe_deserialize, safe_deserialize_conformant},
};
use tfhe::{
    integer::{
        bigint::StaticUnsignedBigInt,
        ciphertext::IntegerProvenCompactCiphertextListConformanceParams, U256,
    },
    prelude::{
        CastInto, CiphertextList, FheEq, FheMax, FheMin, FheOrd, FheTryTrivialEncrypt,
        FusedMulScalarDiv, FusedScalarMulScalarDiv, IfThenElse, RotateLeft, RotateRight,
    },
    zk::CompactPkeCrs,
    CompactCiphertextListExpander, FheBool, FheUint1024, FheUint128, FheUint16, FheUint160,
    FheUint2048, FheUint256, FheUint32, FheUint4, FheUint512, FheUint64, FheUint8, Seed,
};

pub fn deserialize_fhe_ciphertext(
    input_type: i16,
    input_bytes: &[u8],
) -> Result<SupportedFheCiphertexts, FhevmError> {
    match input_type {
        0 => {
            let v: tfhe::FheBool = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheBool(v))
        }
        1 => {
            let v: tfhe::FheUint4 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint4(v))
        }
        2 => {
            let v: tfhe::FheUint8 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint8(v))
        }
        3 => {
            let v: tfhe::FheUint16 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint16(v))
        }
        4 => {
            let v: tfhe::FheUint32 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint32(v))
        }
        5 => {
            let v: tfhe::FheUint64 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint64(v))
        }
        6 => {
            let v: tfhe::FheUint128 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint128(v))
        }
        7 => {
            let v: tfhe::FheUint160 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint160(v))
        }
        8 => {
            let v: tfhe::FheUint256 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint256(v))
        }
        9 => {
            let v: tfhe::FheUint512 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheBytes64(v))
        }
        10 => {
            let v: tfhe::FheUint1024 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheBytes128(v))
        }
        11 => {
            let v: tfhe::FheUint2048 = safe_deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheBytes256(v))
        }
        _ => Err(FhevmError::UnknownFheType(input_type as i32)),
    }
}

/// Function assumes encryption key already set
pub fn trivial_encrypt_be_bytes(
    output_type: i16,
    input_bytes: &[u8],
) -> Result<SupportedFheCiphertexts, FhevmError> {
    let last_byte = if !input_bytes.is_empty() {
        input_bytes[input_bytes.len() - 1]
    } else {
        0
    };
    match output_type {
        0 => Ok(SupportedFheCiphertexts::FheBool(
            FheBool::try_encrypt_trivial(last_byte > 0).expect("trivial encrypt bool"),
        )),
        1 => Ok(SupportedFheCiphertexts::FheUint4(
            FheUint4::try_encrypt_trivial(last_byte).expect("trivial encrypt 4"),
        )),
        2 => Ok(SupportedFheCiphertexts::FheUint8(
            FheUint8::try_encrypt_trivial(last_byte).expect("trivial encrypt 8"),
        )),
        3 => {
            let mut padded: [u8; 2] = [0; 2];
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
            }
            let res = u16::from_be_bytes(padded);
            Ok(SupportedFheCiphertexts::FheUint16(
                FheUint16::try_encrypt_trivial(res).expect("trivial encrypt 16"),
            ))
        }
        4 => {
            let mut padded: [u8; 4] = [0; 4];
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
            }
            let res: u32 = u32::from_be_bytes(padded);
            Ok(SupportedFheCiphertexts::FheUint32(
                FheUint32::try_encrypt_trivial(res).expect("trivial encrypt 32"),
            ))
        }
        5 => {
            let mut padded: [u8; 8] = [0; 8];
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
            }
            let res: u64 = u64::from_be_bytes(padded);
            Ok(SupportedFheCiphertexts::FheUint64(
                FheUint64::try_encrypt_trivial(res).expect("trivial encrypt 64"),
            ))
        }
        6 => {
            let mut padded: [u8; 16] = [0; 16];
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
            }
            let res: u128 = u128::from_be_bytes(padded);
            let output = FheUint128::try_encrypt_trivial(res).expect("trivial encrypt 128");
            Ok(SupportedFheCiphertexts::FheUint128(output))
        }
        7 => {
            let mut padded: [u8; 32] = [0; 32];
            let mut be: U256 = U256::ZERO;
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
                be.copy_from_be_byte_slice(&padded);
            }
            let output: FheUint160 = FheUint256::try_encrypt_trivial(be)
                .expect("trivial encrypt 160")
                .cast_into();
            Ok(SupportedFheCiphertexts::FheUint160(output))
        }
        8 => {
            let mut padded: [u8; 32] = [0; 32];
            let mut be: U256 = U256::ZERO;
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
                be.copy_from_be_byte_slice(&padded);
            }
            let output = FheUint256::try_encrypt_trivial(be).expect("trivial encrypt 256");
            Ok(SupportedFheCiphertexts::FheUint256(output))
        }
        9 => {
            let mut padded: [u8; 64] = [0; 64];
            let mut be: StaticUnsignedBigInt<8> = StaticUnsignedBigInt::<8>::ZERO;
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
                be.copy_from_be_byte_slice(&padded);
            }
            let output = FheUint512::try_encrypt_trivial(be).expect("trivial encrypt 512");
            Ok(SupportedFheCiphertexts::FheBytes64(output))
        }
        10 => {
            let mut padded: [u8; 128] = [0; 128];
            let mut be: StaticUnsignedBigInt<16> = StaticUnsignedBigInt::<16>::ZERO;
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
                be.copy_from_be_byte_slice(&padded);
            }
            let output = FheUint1024::try_encrypt_trivial(be).expect("trivial encrypt 1024");
            Ok(SupportedFheCiphertexts::FheBytes128(output))
        }
        11 => {
            let mut padded: [u8; 256] = [0; 256];
            let mut be: StaticUnsignedBigInt<32> = StaticUnsignedBigInt::<32>::ZERO;
            if !input_bytes.is_empty() {
                let padded_len = padded.len();
                let copy_from = if padded_len >= input_bytes.len() {
                    padded_len - input_bytes.len()
                } else {
                    0
                };
                let len = padded.len().min(input_bytes.len());
                padded[copy_from..padded_len]
                    .copy_from_slice(&input_bytes[input_bytes.len() - len..]);
                be.copy_from_be_byte_slice(&padded);
            }
            let output = FheUint2048::try_encrypt_trivial(be).expect("trivial encrypt 2048");
            Ok(SupportedFheCiphertexts::FheBytes256(output))
        }
        other => Err(FhevmError::UnknownFheType(other as i32)),
    }
}

pub fn current_ciphertext_version() -> i16 {
    0
}

pub fn try_expand_ciphertext_list(
    input_ciphertext: &[u8],
    public_params: &CompactPkeCrs,
) -> Result<Vec<SupportedFheCiphertexts>, FhevmError> {
    let pk_params = FhevmKeys::new_config()
        .public_key_encryption_parameters()
        .map_err(|_| FhevmError::MissingTfheRsData)?;

    let the_list: tfhe::ProvenCompactCiphertextList = safe_deserialize_conformant(
        input_ciphertext,
        &IntegerProvenCompactCiphertextListConformanceParams::from_public_key_encryption_parameters_and_crs_parameters(
            pk_params, public_params,
        ),
    )?;

    let expanded = the_list
        .expand_without_verification()
        .map_err(FhevmError::CiphertextExpansionError)?;

    extract_ct_list(&expanded)
}

pub fn extract_ct_list(
    expanded: &CompactCiphertextListExpander,
) -> Result<Vec<SupportedFheCiphertexts>, FhevmError> {
    let mut res = Vec::new();
    for idx in 0..expanded.len() {
        let data_kind = expanded.get_kind_of(idx).ok_or_else(|| {
            tracing::error!(len = expanded.len(), idx, "get_kind_of returned None");
            FhevmError::MissingTfheRsData
        })?;

        match data_kind {
            tfhe::FheTypes::Bool => {
                let ct: tfhe::FheBool = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheBool(ct));
            }
            tfhe::FheTypes::Uint4 => {
                let ct: tfhe::FheUint4 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint4(ct));
            }
            tfhe::FheTypes::Uint8 => {
                let ct: tfhe::FheUint8 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint8(ct));
            }
            tfhe::FheTypes::Uint16 => {
                let ct: tfhe::FheUint16 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint16(ct));
            }
            tfhe::FheTypes::Uint32 => {
                let ct: tfhe::FheUint32 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint32(ct));
            }
            tfhe::FheTypes::Uint64 => {
                let ct: tfhe::FheUint64 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint64(ct));
            }
            tfhe::FheTypes::Uint128 => {
                let ct: tfhe::FheUint128 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint128(ct));
            }
            tfhe::FheTypes::Uint160 => {
                let ct: tfhe::FheUint160 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint160(ct));
            }
            tfhe::FheTypes::Uint256 => {
                let ct: tfhe::FheUint256 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheUint256(ct));
            }
            tfhe::FheTypes::Uint512 => {
                let ct: tfhe::FheUint512 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheBytes64(ct));
            }
            tfhe::FheTypes::Uint1024 => {
                let ct: tfhe::FheUint1024 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheBytes128(ct));
            }
            tfhe::FheTypes::Uint2048 => {
                let ct: tfhe::FheUint2048 = expanded
                    .get(idx)
                    .map_err(|e| FhevmError::DeserializationError(e.into()))?
                    .ok_or(FhevmError::DeserializationError(
                        "failed to get expected data type".into(),
                    ))?;

                res.push(SupportedFheCiphertexts::FheBytes256(ct));
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

// returns the byte width of an FHE type code as accepted by `fheMulDiv`'s lhs.
fn mul_div_lhs_width_bytes(
    lhs_type: i16,
    fhe_op: &SupportedFheOperations,
) -> Result<usize, FhevmError> {
    match lhs_type {
        2 => Ok(1), // Uint8
        3 => Ok(2), // Uint16
        4 => Ok(4), // Uint32
        5 => Ok(8), // Uint64
        _ => Err(FhevmError::UnsupportedFheTypes {
            fhe_operation: format!(
                "{:?}: type {lhs_type} is not supported for FheMulDiv",
                fhe_op
            ),
            input_types: vec![],
        }),
    }
}

// return output ciphertext type
pub fn check_fhe_operand_types(
    fhe_operation: i32,
    input_handles: &[Vec<u8>],
    is_input_handle_scalar: &[bool],
) -> Result<(), FhevmError> {
    let fhe_op: SupportedFheOperations = fhe_operation.try_into()?;

    assert_eq!(input_handles.len(), is_input_handle_scalar.len());

    let scalar_operands = is_input_handle_scalar
        .iter()
        .enumerate()
        .filter(|(_, is_scalar)| **is_scalar)
        .collect::<Vec<_>>();

    let is_scalar = !scalar_operands.is_empty();

    // do this check for only random ops because
    // all random ops inputs are scalar
    if !fhe_op.does_have_more_than_one_scalar() {
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
    }

    match fhe_op.op_type() {
        FheOperationType::Binary => {
            let expected_operands = 2;
            if input_handles.len() != expected_operands {
                return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    expected_operands,
                    got_operands: input_handles.len(),
                });
            }

            // special case for div operation, rhs for scalar must not be zero
            if is_scalar && fhe_op == SupportedFheOperations::FheDiv {
                let all_zeroes = input_handles[1].iter().all(|i| *i == 0u8);
                if all_zeroes {
                    return Err(FhevmError::FheOperationScalarDivisionByZero {
                        lhs_handle: format!("0x{}", hex::encode(&input_handles[0])),
                        rhs_value: format!("0x{}", hex::encode(&input_handles[1])),
                        fhe_operation,
                        fhe_operation_name: format!("{:?}", fhe_op),
                    });
                }
            }

            Ok(())
        }
        FheOperationType::Unary => {
            let expected_operands = 1;
            if input_handles.len() != expected_operands {
                return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    expected_operands,
                    got_operands: input_handles.len(),
                });
            }

            Ok(())
        }
        FheOperationType::Other => {
            match &fhe_op {
                // two ops + uniform types branch
                // what about scalar compute?
                SupportedFheOperations::FheIfThenElse => {
                    let expected_operands = 3;
                    if input_handles.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_handles.len(),
                        });
                    }

                    Ok(())
                }
                SupportedFheOperations::FheCast => {
                    let expected_operands = 2;
                    if input_handles.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_handles.len(),
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

                            Ok(())
                        }
                        (other_left, other_right) => {
                            let bool_to_op =
                                |inp| (if inp { "scalar" } else { "handle" }).to_string();

                            Err(FhevmError::UnexpectedCastOperandTypes {
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
                            })
                        }
                    }
                }
                SupportedFheOperations::FheTrivialEncrypt => {
                    let expected_operands = 2;
                    if input_handles.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_handles.len(),
                        });
                    }

                    if !is_input_handle_scalar[0] || !is_input_handle_scalar[1] {
                        return Err(FhevmError::AllInputsForTrivialEncryptionMustBeScalar {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                        });
                    }

                    let op = &input_handles[1];
                    if op.len() != 1 {
                        return Err(
                            FhevmError::UnexpectedTrivialEncryptionOperandSizeForScalarOperand {
                                fhe_operation,
                                fhe_operation_name: format!("{:?}", fhe_op),
                                expected_scalar_operand_bytes: 1,
                                got_bytes: op.len(),
                            },
                        );
                    }

                    Ok(())
                }
                SupportedFheOperations::FheRand => {
                    // counter and output type
                    let expected_operands = 2;
                    if input_handles.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_handles.len(),
                        });
                    }

                    let scalar_operands = is_input_handle_scalar.iter().filter(|i| **i).count();
                    if scalar_operands < expected_operands {
                        return Err(FhevmError::RandOperationInputsMustAllBeScalar {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            scalar_operand_count: scalar_operands,
                            expected_scalar_operand_count: expected_operands,
                        });
                    }

                    let rand_type = &input_handles[1];
                    if rand_type.len() != 1 {
                        return Err(FhevmError::UnexpectedRandOperandSizeForOutputType {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operand_bytes: 1,
                            got_bytes: rand_type.len(),
                        });
                    }

                    validate_fhe_type(rand_type[0] as i32)?;

                    Ok(())
                }
                SupportedFheOperations::FheRandBounded => {
                    // counter, bound and output type
                    let expected_operands = 3;
                    if input_handles.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_handles.len(),
                        });
                    }

                    let scalar_operands = is_input_handle_scalar.iter().filter(|i| **i).count();
                    if scalar_operands < expected_operands {
                        return Err(FhevmError::RandOperationInputsMustAllBeScalar {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            scalar_operand_count: scalar_operands,
                            expected_scalar_operand_count: expected_operands,
                        });
                    }

                    let upper_bound = &input_handles[1];
                    if upper_bound.is_empty() && upper_bound.iter().all(|i| *i == 0) {
                        return Err(FhevmError::RandOperationUpperBoundCannotBeZero {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            upper_bound_value: format!("0x{}", hex::encode(upper_bound)),
                        });
                    }

                    let rand_type = &input_handles[2];
                    if rand_type.len() != 1 {
                        return Err(FhevmError::UnexpectedRandOperandSizeForOutputType {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operand_bytes: 1,
                            got_bytes: rand_type.len(),
                        });
                    }

                    Ok(())
                }
                SupportedFheOperations::FheSum => {
                    const FHE_SUM_MAX_INPUTS_WIDE: usize = 60;
                    const FHE_SUM_MAX_INPUTS_NARROW: usize = 100;

                    if input_handles.is_empty() {
                        return Ok(());
                    }

                    let first_type = get_ct_type(&input_handles[0])?;

                    // FheUint160 (7) and FheUint256 (8) are not supported for FheSum.
                    let fhe_sum_max_inputs = match first_type {
                        5 | 6 => FHE_SUM_MAX_INPUTS_WIDE,   // Uint64 | Uint128
                        2..=4 => FHE_SUM_MAX_INPUTS_NARROW, // Uint8 | Uint16 | Uint32
                        _ => {
                            return Err(FhevmError::UnsupportedFheTypes {
                                fhe_operation: format!(
                                    "{:?}: type {first_type} is not supported for FheSum",
                                    fhe_op
                                ),
                                input_types: vec![],
                            })
                        }
                    };

                    if input_handles.len() > fhe_sum_max_inputs {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands: fhe_sum_max_inputs,
                            got_operands: input_handles.len(),
                        });
                    }

                    for (i, handle) in input_handles.iter().enumerate().skip(1) {
                        let handle_type = get_ct_type(handle)?;
                        if handle_type != first_type {
                            return Err(FhevmError::UnsupportedFheTypes {
                                fhe_operation: format!(
                                    "{:?}: handle at index {i} has type {handle_type}, expected {first_type}",
                                    fhe_op
                                ),
                                input_types: vec![],
                            });
                        }
                    }

                    Ok(())
                }
                SupportedFheOperations::FheIsIn => {
                    const FHE_IS_IN_MAX_SET_SIZE_WIDE: usize = 60;
                    const FHE_IS_IN_MAX_SET_SIZE_NARROW: usize = 100;

                    if input_handles.is_empty() {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands: 1,
                            got_operands: 0,
                        });
                    }

                    let first_type = get_ct_type(&input_handles[0])?;

                    let fhe_is_in_max = match first_type {
                        5..=8 => FHE_IS_IN_MAX_SET_SIZE_WIDE, // Uint64 | Uint128 | Uint160/eaddress | Uint256
                        2..=4 => FHE_IS_IN_MAX_SET_SIZE_NARROW, // Uint8 | Uint16 | Uint32
                        _ => {
                            return Err(FhevmError::UnsupportedFheTypes {
                                fhe_operation: format!(
                                    "{:?}: type {first_type} is not supported for FheIsIn",
                                    fhe_op
                                ),
                                input_types: vec![],
                            })
                        }
                    };

                    let set_size = input_handles.len() - 1;
                    if set_size > fhe_is_in_max {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands: fhe_is_in_max,
                            got_operands: set_size,
                        });
                    }

                    for (i, handle) in input_handles.iter().enumerate().skip(1) {
                        let handle_type = get_ct_type(handle)?;
                        if handle_type != first_type {
                            return Err(FhevmError::UnsupportedFheTypes {
                                fhe_operation: format!(
                                    "{:?}: handle at index {i} has type {handle_type}, expected {first_type}",
                                    fhe_op
                                ),
                                input_types: vec![],
                            });
                        }
                    }

                    Ok(())
                }
                SupportedFheOperations::FheMulDiv => {
                    let expected_operands = 3;
                    if input_handles.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_handles.len(),
                        });
                    }
                    if is_input_handle_scalar[0] {
                        return Err(FhevmError::FheOperationOnlySecondOperandCanBeScalar {
                            scalar_input_index: 0,
                            only_allowed_scalar_input_index: 1,
                        });
                    }
                    if !is_input_handle_scalar[2] {
                        return Err(FhevmError::UnsupportedFheTypes {
                            fhe_operation: format!(
                                "{:?}: divisor operand (index 2) must be a scalar",
                                fhe_op
                            ),
                            input_types: vec![],
                        });
                    }
                    let lhs_type = get_ct_type(&input_handles[0])?;
                    let lhs_width_bytes = mul_div_lhs_width_bytes(lhs_type, &fhe_op)?;
                    if !is_input_handle_scalar[1] {
                        let rhs_type = get_ct_type(&input_handles[1])?;
                        if rhs_type != lhs_type {
                            return Err(FhevmError::FheOperationDoesntHaveUniformTypesAsInput {
                                fhe_operation,
                                fhe_operation_name: format!("{:?}", fhe_op),
                                operand_types: vec![lhs_type, rhs_type],
                            });
                        }
                    }
                    let start = input_handles[2]
                        .len()
                        .checked_sub(lhs_width_bytes)
                        .ok_or_else(|| FhevmError::UnsupportedFheTypes {
                            fhe_operation: format!(
                                "{:?}: divisor operand is shorter than operand width",
                                fhe_op
                            ),
                            input_types: vec![],
                        })?;
                    let divisor_low = &input_handles[2][start..];
                    if divisor_low.iter().all(|b| *b == 0) {
                        return Err(FhevmError::FheOperationScalarDivisionByZero {
                            lhs_handle: format!("0x{}", hex::encode(&input_handles[0])),
                            rhs_value: format!("0x{}", hex::encode(&input_handles[2])),
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                        });
                    }
                    Ok(())
                }
                other => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("Unexpected op_type branch: {:?}", other),
                    input_types: vec![],
                }),
            }
        }
    }
}

pub fn validate_fhe_type(input_type: i32) -> Result<(), FhevmError> {
    let i16_type: i16 = input_type
        .try_into()
        .or(Err(FhevmError::UnknownFheType(input_type)))?;
    match i16_type {
        0..=11 => Ok(()),
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
                // rhs may be encrypted or a scalar; divisor is always scalar
                SupportedFheOperations::FheMulDiv => true,
                _ => false,
            }
        }
    }
}

// add operations here that don't support both encrypted operands
pub fn does_fhe_operation_support_both_encrypted_operands(op: &SupportedFheOperations) -> bool {
    !matches!(op, SupportedFheOperations::FheDiv) || !matches!(op, SupportedFheOperations::FheRem)
}

#[cfg(not(feature = "gpu"))]
pub fn perform_fhe_operation(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    _: usize,
    output_type: i16,
) -> Result<SupportedFheCiphertexts, FhevmError> {
    perform_fhe_operation_impl(fhe_operation_int, input_operands, output_type)
}

#[cfg(feature = "gpu")]
pub fn perform_fhe_operation(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    gpu_idx: usize,
    output_type: i16,
) -> Result<SupportedFheCiphertexts, FhevmError> {
    use crate::gpu_memory::{get_op_size_on_gpu, release_memory_on_gpu, reserve_memory_on_gpu};

    let mut gpu_mem_res = get_op_size_on_gpu(fhe_operation_int, input_operands)?;
    input_operands
        .iter()
        .for_each(|i| gpu_mem_res += i.get_size_on_gpu());
    reserve_memory_on_gpu(gpu_mem_res, gpu_idx);
    let res = perform_fhe_operation_impl(fhe_operation_int, input_operands, output_type);
    release_memory_on_gpu(gpu_mem_res, gpu_idx);
    res
}

fn collect_operands_as<'a, T>(
    fhe_operation: &SupportedFheOperations,
    operands: &'a [SupportedFheCiphertexts],
    extract: impl Fn(&'a SupportedFheCiphertexts) -> Option<&'a T>,
) -> Result<Vec<&'a T>, FhevmError> {
    operands
        .iter()
        .map(|op| {
            extract(op).ok_or_else(|| FhevmError::UnsupportedFheTypes {
                fhe_operation: format!("{:?}", fhe_operation),
                input_types: operands.iter().map(|i| i.type_name()).collect(),
            })
        })
        .collect()
}

pub fn perform_fhe_operation_impl(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    output_type: i16,
) -> Result<SupportedFheCiphertexts, FhevmError> {
    let fhe_operation: SupportedFheOperations = fhe_operation_int.try_into()?;
    match fhe_operation {
        SupportedFheOperations::FheAdd => {
            assert_eq!(input_operands.len(), 2);

            // fhe add
            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a + b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a + b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a + b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a + b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a + to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a + to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a + to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a + to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a + to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a + to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a + to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a + to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheSub => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a - b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a - b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a - b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a - b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a - to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a - to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a - to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a - to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a - to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a - to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a - to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a - to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMul => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a * b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a * b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a * b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a * b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a * to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a * to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a * to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a * to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a * to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a * to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a * to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a * to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheDiv => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a / b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a / b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a / b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a / b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a / to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a / to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a / to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a / to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a / to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a / to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a / to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a / to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRem => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a % b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a % b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a % b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a % b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a % to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a % to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a % to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a % to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a % to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a % to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a % to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a % to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitAnd => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a & b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a & b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a & b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a & b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a & b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a & b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a & b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a & b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a & arr_non_zero(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a & to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a & to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a & to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a & to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a & to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a & to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a & to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a & to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(a & to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(a & to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(a & to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitOr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a | b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a | b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a | b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a | b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a | b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a | b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a | b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a | b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a | arr_non_zero(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a | to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a | to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a | to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a | to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a | to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a | to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a | to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a | to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(a | to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(a | to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(a | to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitXor => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a ^ b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a ^ b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a ^ b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a ^ b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a ^ b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a ^ b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a ^ b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a ^ b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a ^ arr_non_zero(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a ^ to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a ^ to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a ^ to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a ^ to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a ^ to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a ^ to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a ^ to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a ^ to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(a ^ to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(a ^ to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(a ^ to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheShl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a << b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a << b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a << b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a << b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a << b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a << b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a << b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a << to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a << to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a << to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a << to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a << to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a << to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a << to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a << to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(a << to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(
                        a << to_be_u1024_bit(b),
                    ))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(
                        a << to_be_u2048_bit(b),
                    ))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheShr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a >> b))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a >> b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a >> b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a >> b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a >> b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a >> b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a >> b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a >> to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a >> to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a >> to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a >> to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a >> to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint128(a >> to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint160(a >> to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a >> to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(a >> to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(
                        a >> to_be_u1024_bit(b),
                    ))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(
                        a >> to_be_u2048_bit(b),
                    ))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRotl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a.rotate_left(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a.rotate_left(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a.rotate_left(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a.rotate_left(b))),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a.rotate_left(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a.rotate_left(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a.rotate_left(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint4(a.rotate_left(to_be_u4_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint8(a.rotate_left(to_be_u8_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint16(a.rotate_left(to_be_u16_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint32(a.rotate_left(to_be_u32_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint64(a.rotate_left(to_be_u64_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint128(a.rotate_left(to_be_u128_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint160(a.rotate_left(to_be_u160_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint256(a.rotate_left(to_be_u256_bit(b))),
                ),
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheBytes64(a.rotate_left(to_be_u512_bit(b))),
                ),
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(
                        a.rotate_left(to_be_u1024_bit(b)),
                    ))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(
                        a.rotate_left(to_be_u2048_bit(b)),
                    ))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRotr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a.rotate_right(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a.rotate_right(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a.rotate_right(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a.rotate_right(b))),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a.rotate_right(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a.rotate_right(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a.rotate_right(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint4(a.rotate_right(to_be_u4_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint8(a.rotate_right(to_be_u8_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint16(a.rotate_right(to_be_u16_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint32(a.rotate_right(to_be_u32_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint64(a.rotate_right(to_be_u64_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint128(a.rotate_right(to_be_u128_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint160(a.rotate_right(to_be_u160_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint256(a.rotate_right(to_be_u256_bit(b))),
                ),
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheBytes64(a.rotate_right(to_be_u512_bit(b))),
                ),
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(
                        a.rotate_right(to_be_u1024_bit(b)),
                    ))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(
                        a.rotate_right(to_be_u2048_bit(b)),
                    ))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMin => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a.min(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a.min(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a.min(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a.min(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a.min(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a.min(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a.min(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a.min(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a.min(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint128(a.min(to_be_u128_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint160(a.min(to_be_u160_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint256(a.min(to_be_u256_bit(b))),
                ),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMax => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a.max(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheUint128(a.max(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheUint160(a.max(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheUint256(a.max(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint4(a.max(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a.max(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint16(a.max(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a.max(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint64(a.max(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint128(a.max(to_be_u128_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint160(a.max(to_be_u160_bit(b))),
                ),
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => Ok(
                    SupportedFheCiphertexts::FheUint256(a.max(to_be_u256_bit(b))),
                ),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheEq => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.eq(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.eq(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.eq(b))),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.eq(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.eq(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.eq(b))),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(arr_non_zero(b))))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u128_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u160_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u256_bit(b))))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u512_bit(b))))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u1024_bit(b))))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(to_be_u2048_bit(b))))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ne(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ne(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ne(b))),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ne(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ne(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ne(b))),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(arr_non_zero(b))))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u128_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u160_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u256_bit(b))))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u512_bit(b))))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u1024_bit(b))))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(to_be_u2048_bit(b))))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheGe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ge(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ge(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ge(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u128_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u160_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(to_be_u256_bit(b))))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheGt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.gt(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.gt(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.gt(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u128_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u160_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(to_be_u256_bit(b))))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheLe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.le(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.le(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.le(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u128_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u160_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(to_be_u256_bit(b))))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheLt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(b)))
                }
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.lt(b))),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.lt(b))),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.lt(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u4_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u8_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u16_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u32_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u64_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u128_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u160_bit(b))))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(to_be_u256_bit(b))))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNot => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheBool(a) => Ok(SupportedFheCiphertexts::FheBool(!a)),
                SupportedFheCiphertexts::FheUint4(a) => Ok(SupportedFheCiphertexts::FheUint4(!a)),
                SupportedFheCiphertexts::FheUint8(a) => Ok(SupportedFheCiphertexts::FheUint8(!a)),
                SupportedFheCiphertexts::FheUint16(a) => Ok(SupportedFheCiphertexts::FheUint16(!a)),
                SupportedFheCiphertexts::FheUint32(a) => Ok(SupportedFheCiphertexts::FheUint32(!a)),
                SupportedFheCiphertexts::FheUint64(a) => Ok(SupportedFheCiphertexts::FheUint64(!a)),
                SupportedFheCiphertexts::FheUint128(a) => {
                    Ok(SupportedFheCiphertexts::FheUint128(!a))
                }
                SupportedFheCiphertexts::FheUint160(a) => {
                    Ok(SupportedFheCiphertexts::FheUint160(!a))
                }
                SupportedFheCiphertexts::FheUint256(a) => {
                    Ok(SupportedFheCiphertexts::FheUint256(!a))
                }
                SupportedFheCiphertexts::FheBytes64(a) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(!a))
                }
                SupportedFheCiphertexts::FheBytes128(a) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(!a))
                }
                SupportedFheCiphertexts::FheBytes256(a) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(!a))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNeg => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint4(a) => Ok(SupportedFheCiphertexts::FheUint4(-a)),
                SupportedFheCiphertexts::FheUint8(a) => Ok(SupportedFheCiphertexts::FheUint8(-a)),
                SupportedFheCiphertexts::FheUint16(a) => Ok(SupportedFheCiphertexts::FheUint16(-a)),
                SupportedFheCiphertexts::FheUint32(a) => Ok(SupportedFheCiphertexts::FheUint32(-a)),
                SupportedFheCiphertexts::FheUint64(a) => Ok(SupportedFheCiphertexts::FheUint64(-a)),
                SupportedFheCiphertexts::FheUint128(a) => {
                    Ok(SupportedFheCiphertexts::FheUint128(-a))
                }
                SupportedFheCiphertexts::FheUint160(a) => {
                    Ok(SupportedFheCiphertexts::FheUint160(-a))
                }
                SupportedFheCiphertexts::FheUint256(a) => {
                    Ok(SupportedFheCiphertexts::FheUint256(-a))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheIfThenElse => {
            assert_eq!(input_operands.len(), 3);

            let SupportedFheCiphertexts::FheBool(flag) = &input_operands[0] else {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                });
            };

            match (&input_operands[1], &input_operands[2]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheBool(res))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint4(res))
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
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint128(res))
                }
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint160(res))
                }
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheUint256(res))
                }
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheBytes64(res))
                }
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheBytes128(res))
                }
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => {
                    let res = flag.select(a, b);
                    Ok(SupportedFheCiphertexts::FheBytes256(res))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheCast => match (&input_operands[0], &input_operands[1]) {
            (SupportedFheCiphertexts::FheBool(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheBool(inp.clone()))
                } else {
                    match l {
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint4(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint4(inp.clone()))
                } else {
                    match l {
                        0 => {
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint8(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint8(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint16(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint16(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint32(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint32(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint64(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint64(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint128(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint128(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint160(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint160(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheUint256(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheUint256(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheBytes64(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheBytes64(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheBytes128(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheBytes128(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        11 => {
                            let out: tfhe::FheUint2048 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes256(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            (SupportedFheCiphertexts::FheBytes256(inp), SupportedFheCiphertexts::Scalar(op)) => {
                let l = to_be_u16_bit(op) as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    Ok(SupportedFheCiphertexts::FheBytes256(inp.clone()))
                } else {
                    match l {
                        0 => {
                            let out: tfhe::FheBool = inp.gt(0);
                            Ok(SupportedFheCiphertexts::FheBool(out))
                        }
                        1 => {
                            let out: tfhe::FheUint4 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint4(out))
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
                        5 => {
                            let out: tfhe::FheUint64 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint64(out))
                        }
                        6 => {
                            let out: tfhe::FheUint128 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint128(out))
                        }
                        7 => {
                            let out: tfhe::FheUint160 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint160(out))
                        }
                        8 => {
                            let out: tfhe::FheUint256 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheUint256(out))
                        }
                        9 => {
                            let out: tfhe::FheUint512 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes64(out))
                        }
                        10 => {
                            let out: tfhe::FheUint1024 = inp.clone().cast_into();
                            Ok(SupportedFheCiphertexts::FheBytes128(out))
                        }
                        other => Err(FhevmError::UnknownCastType {
                            fhe_operation: format!("{:?}", fhe_operation),
                            type_to_cast_to: other,
                        }),
                    }
                }
            }
            _ => Err(FhevmError::UnsupportedFheTypes {
                fhe_operation: format!("{:?}", fhe_operation),
                input_types: input_operands.iter().map(|i| i.type_name()).collect(),
            }),
        },
        SupportedFheOperations::FheTrivialEncrypt => match (&input_operands[0], &input_operands[1])
        {
            (SupportedFheCiphertexts::Scalar(inp), SupportedFheCiphertexts::Scalar(op)) => {
                trivial_encrypt_be_bytes(to_be_u16_bit(op) as i16, inp)
            }
            _ => Err(FhevmError::UnsupportedFheTypes {
                fhe_operation: format!("{:?}", fhe_operation),
                input_types: input_operands.iter().map(|i| i.type_name()).collect(),
            }),
        },
        SupportedFheOperations::FheRand => {
            let SupportedFheCiphertexts::Scalar(rand_counter) = &input_operands[0] else {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                });
            };
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[1] else {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                });
            };
            let rand_seed = to_be_u128_bit(rand_counter);
            let to_type = to_be_u16_bit(to_type) as i16;
            generate_random_number(to_type as i16, rand_seed, None)
        }
        SupportedFheOperations::FheRandBounded => {
            let SupportedFheCiphertexts::Scalar(rand_counter) = &input_operands[0] else {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                });
            };
            let SupportedFheCiphertexts::Scalar(upper_bound) = &input_operands[1] else {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                });
            };
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[2] else {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                });
            };
            let rand_seed = to_be_u128_bit(rand_counter);
            let to_type = to_be_u16_bit(to_type) as i16;
            generate_random_number(to_type as i16, rand_seed, Some(upper_bound))
        }
        SupportedFheOperations::FheGetInputCiphertext => Err(FhevmError::UnsupportedFheTypes {
            fhe_operation: format!("{:?}", fhe_operation),
            input_types: input_operands.iter().map(|i| i.type_name()).collect(),
        }),
        SupportedFheOperations::FheSum => {
            if input_operands.is_empty() {
                if !matches!(output_type, 2..=6) {
                    return Err(FhevmError::UnsupportedFheTypes {
                        fhe_operation: format!(
                            "{:?}: type {output_type} is not supported for FheSum",
                            fhe_operation
                        ),
                        input_types: vec![],
                    });
                }
                return trivial_encrypt_be_bytes(output_type, &[0u8]);
            }
            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint8(_) => {
                    collect_operands_as(&fhe_operation, input_operands, |op| match op {
                        SupportedFheCiphertexts::FheUint8(v) => Some(v),
                        _ => None,
                    })
                    .map(|refs| SupportedFheCiphertexts::FheUint8(refs.into_iter().sum()))
                }
                SupportedFheCiphertexts::FheUint16(_) => {
                    collect_operands_as(&fhe_operation, input_operands, |op| match op {
                        SupportedFheCiphertexts::FheUint16(v) => Some(v),
                        _ => None,
                    })
                    .map(|refs| SupportedFheCiphertexts::FheUint16(refs.into_iter().sum()))
                }
                SupportedFheCiphertexts::FheUint32(_) => {
                    collect_operands_as(&fhe_operation, input_operands, |op| match op {
                        SupportedFheCiphertexts::FheUint32(v) => Some(v),
                        _ => None,
                    })
                    .map(|refs| SupportedFheCiphertexts::FheUint32(refs.into_iter().sum()))
                }
                SupportedFheCiphertexts::FheUint64(_) => {
                    collect_operands_as(&fhe_operation, input_operands, |op| match op {
                        SupportedFheCiphertexts::FheUint64(v) => Some(v),
                        _ => None,
                    })
                    .map(|refs| SupportedFheCiphertexts::FheUint64(refs.into_iter().sum()))
                }
                SupportedFheCiphertexts::FheUint128(_) => {
                    collect_operands_as(&fhe_operation, input_operands, |op| match op {
                        SupportedFheCiphertexts::FheUint128(v) => Some(v),
                        _ => None,
                    })
                    .map(|refs| SupportedFheCiphertexts::FheUint128(refs.into_iter().sum()))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMulDiv => {
            assert_eq!(input_operands.len(), 3);
            // operands: [lhs(encrypted), rhs(encrypted or Scalar), divisor(Scalar)]
            match (&input_operands[0], &input_operands[1], &input_operands[2]) {
                (
                    SupportedFheCiphertexts::FheUint8(a),
                    SupportedFheCiphertexts::FheUint8(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint8(
                    a.fused_mul_scalar_div(b, to_be_u8_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint16(a),
                    SupportedFheCiphertexts::FheUint16(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint16(
                    a.fused_mul_scalar_div(b, to_be_u16_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint32(a),
                    SupportedFheCiphertexts::FheUint32(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint32(
                    a.fused_mul_scalar_div(b, to_be_u32_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint64(a),
                    SupportedFheCiphertexts::FheUint64(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint64(
                    a.fused_mul_scalar_div(b, to_be_u64_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint8(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint8(
                    a.fused_scalar_mul_scalar_div(to_be_u8_bit(b), to_be_u8_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint16(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint16(
                    a.fused_scalar_mul_scalar_div(to_be_u16_bit(b), to_be_u16_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint32(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint32(
                    a.fused_scalar_mul_scalar_div(to_be_u32_bit(b), to_be_u32_bit(d)),
                )),
                (
                    SupportedFheCiphertexts::FheUint64(a),
                    SupportedFheCiphertexts::Scalar(b),
                    SupportedFheCiphertexts::Scalar(d),
                ) => Ok(SupportedFheCiphertexts::FheUint64(
                    a.fused_scalar_mul_scalar_div(to_be_u64_bit(b), to_be_u64_bit(d)),
                )),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheIsIn => {
            if input_operands.is_empty() {
                return Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!(
                        "{:?}: requires the ciphertext value operand",
                        fhe_operation
                    ),
                    input_types: vec![],
                });
            }
            // Empty set: trivially false without any PBS.
            if input_operands.len() == 1 {
                return Ok(SupportedFheCiphertexts::FheBool(
                    FheBool::try_encrypt_trivial(false).expect("trivial encrypt bool"),
                ));
            }
            let type_err = || FhevmError::UnsupportedFheTypes {
                fhe_operation: format!("{:?}: set elements must match value type", fhe_operation),
                input_types: input_operands.iter().map(|i| i.type_name()).collect(),
            };
            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint8(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint8(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint8>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint8::contains(
                        &set, value,
                    )))
                }
                SupportedFheCiphertexts::FheUint16(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint16(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint16>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint16::contains(
                        &set, value,
                    )))
                }
                SupportedFheCiphertexts::FheUint32(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint32(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint32>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint32::contains(
                        &set, value,
                    )))
                }
                SupportedFheCiphertexts::FheUint64(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint64(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint64>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint64::contains(
                        &set, value,
                    )))
                }
                SupportedFheCiphertexts::FheUint128(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint128(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint128>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint128::contains(
                        &set, value,
                    )))
                }
                SupportedFheCiphertexts::FheUint160(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint160(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint160>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint160::contains(
                        &set, value,
                    )))
                }
                SupportedFheCiphertexts::FheUint256(value) => {
                    let set = input_operands[1..]
                        .iter()
                        .map(|op| match op {
                            SupportedFheCiphertexts::FheUint256(ct) => Ok(ct.clone()),
                            _ => Err(type_err()),
                        })
                        .collect::<Result<Vec<FheUint256>, _>>()?;
                    Ok(SupportedFheCiphertexts::FheBool(FheUint256::contains(
                        &set, value,
                    )))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
    }
}

pub fn to_be_u4_bit(inp: &[u8]) -> u8 {
    inp.last().unwrap_or(&0) & 0x0f
}

pub fn to_be_u8_bit(inp: &[u8]) -> u8 {
    *inp.last().unwrap_or(&0)
}

// copies input bytes to constant size array as big endian
// while padding result with zeros from left if resulting array
// is larger than input and truncating input array from the left
// if input array is larger than resulting array
fn to_constant_size_array<const SIZE: usize>(inp: &[u8]) -> [u8; SIZE] {
    let mut res = [0u8; SIZE];

    match inp.len().cmp(&SIZE) {
        std::cmp::Ordering::Less => {
            // truncate input slice from the left
            let slice = &mut res[SIZE - inp.len()..];
            slice.copy_from_slice(inp);
        }
        std::cmp::Ordering::Equal => {
            res.copy_from_slice(inp);
        }
        std::cmp::Ordering::Greater => {
            // input slice larger than result, truncate input slice from the left
            res.copy_from_slice(&inp[inp.len() - SIZE..]);
        }
    }

    res
}

pub fn to_be_u16_bit(inp: &[u8]) -> u16 {
    u16::from_be_bytes(to_constant_size_array::<{ std::mem::size_of::<u16>() }>(
        inp,
    ))
}

pub fn to_be_u32_bit(inp: &[u8]) -> u32 {
    u32::from_be_bytes(to_constant_size_array::<{ std::mem::size_of::<u32>() }>(
        inp,
    ))
}

pub fn to_be_u64_bit(inp: &[u8]) -> u64 {
    u64::from_be_bytes(to_constant_size_array::<{ std::mem::size_of::<u64>() }>(
        inp,
    ))
}

pub fn to_be_u128_bit(inp: &[u8]) -> u128 {
    u128::from_be_bytes(to_constant_size_array::<{ std::mem::size_of::<u128>() }>(
        inp,
    ))
}

// return U256 because that's supported from tfhe-rs and will need cast later
pub fn to_be_u160_bit(inp: &[u8]) -> U256 {
    const SIZE: usize = 160 / 8;
    // truncate first
    let arr = to_constant_size_array::<SIZE>(inp);
    const FINAL_SIZE: usize = 256 / 8;
    // final value
    let final_arr = to_constant_size_array::<FINAL_SIZE>(&arr);
    let mut res = U256::ZERO;
    res.copy_from_be_byte_slice(&final_arr);
    res
}

pub fn to_be_u256_bit(inp: &[u8]) -> U256 {
    const FINAL_SIZE: usize = 256 / 8;
    // final value
    let arr = to_constant_size_array::<FINAL_SIZE>(inp);
    let mut res = U256::ZERO;
    res.copy_from_be_byte_slice(&arr);
    res
}

pub fn to_be_u512_bit(inp: &[u8]) -> StaticUnsignedBigInt<8> {
    type TheType = StaticUnsignedBigInt<8>;
    const FINAL_SIZE: usize = std::mem::size_of::<TheType>();
    // final value
    let arr = to_constant_size_array::<FINAL_SIZE>(inp);
    let mut res = TheType::ZERO;
    res.copy_from_be_byte_slice(&arr);
    res
}

pub fn to_be_u1024_bit(inp: &[u8]) -> StaticUnsignedBigInt<16> {
    type TheType = StaticUnsignedBigInt<16>;
    const FINAL_SIZE: usize = std::mem::size_of::<TheType>();
    // final value
    let arr = to_constant_size_array::<FINAL_SIZE>(inp);
    let mut res = TheType::ZERO;
    res.copy_from_be_byte_slice(&arr);
    res
}

pub fn to_be_u2048_bit(inp: &[u8]) -> StaticUnsignedBigInt<32> {
    type TheType = StaticUnsignedBigInt<32>;
    const FINAL_SIZE: usize = std::mem::size_of::<TheType>();
    // final value
    let arr = to_constant_size_array::<FINAL_SIZE>(inp);
    let mut res = TheType::ZERO;
    res.copy_from_be_byte_slice(&arr);
    res
}

fn arr_non_zero(inp: &[u8]) -> bool {
    for b in inp {
        if *b > 0 {
            return true;
        }
    }
    false
}

fn be_number_random_bits(inp: &[u8]) -> u32 {
    let mut res = 0;
    for i in inp.iter().rev() {
        let i = *i;
        match i.cmp(&0) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {
                // all bits zero, add 8
                res += 8;
            }
            std::cmp::Ordering::Greater => {
                res += 7 - i.leading_zeros();
                break;
            }
        }
    }

    res
}

#[test]
fn random_bits_from_arr() {
    assert_eq!(be_number_random_bits(&(1u32).to_be_bytes()), 0);
    assert_eq!(be_number_random_bits(&(2u32).to_be_bytes()), 1);
    assert_eq!(be_number_random_bits(&(4u32).to_be_bytes()), 2);
    assert_eq!(be_number_random_bits(&(8u32).to_be_bytes()), 3);
    assert_eq!(be_number_random_bits(&(16u32).to_be_bytes()), 4);
    assert_eq!(be_number_random_bits(&(32u32).to_be_bytes()), 5);
    assert_eq!(be_number_random_bits(&(64u32).to_be_bytes()), 6);
    assert_eq!(be_number_random_bits(&(128u32).to_be_bytes()), 7);
    assert_eq!(be_number_random_bits(&(256u32).to_be_bytes()), 8);
    assert_eq!(be_number_random_bits(&(512u32).to_be_bytes()), 9);
    assert_eq!(be_number_random_bits(&(1024u32).to_be_bytes()), 10);
    assert_eq!(be_number_random_bits(&(2048u32).to_be_bytes()), 11);
    assert_eq!(be_number_random_bits(&(4096u32).to_be_bytes()), 12);
    assert_eq!(be_number_random_bits(&(8192u32).to_be_bytes()), 13);
    assert_eq!(be_number_random_bits(&(16384u32).to_be_bytes()), 14);
    assert_eq!(be_number_random_bits(&(32768u32).to_be_bytes()), 15);
    assert_eq!(be_number_random_bits(&(65536u32).to_be_bytes()), 16);
}

pub fn generate_random_number(
    the_type: i16,
    seed: u128,
    upper_bound: Option<&[u8]>,
) -> Result<SupportedFheCiphertexts, FhevmError> {
    match the_type {
        0 => Ok(SupportedFheCiphertexts::FheBool(
            FheBool::generate_oblivious_pseudo_random(Seed(seed)),
        )),
        1 => {
            let bit_count = 4;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint4(
                FheUint4::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        2 => {
            let bit_count = 8;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint8(
                FheUint8::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        3 => {
            let bit_count = 16;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint16(
                FheUint16::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        4 => {
            let bit_count = 32;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint32(
                FheUint32::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        5 => {
            let bit_count = 64;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint64(
                FheUint64::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        6 => {
            let bit_count = 128;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint128(
                FheUint128::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        7 => {
            let bit_count = 160;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint160(
                FheUint160::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        8 => {
            let bit_count = 256;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheUint256(
                FheUint256::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        9 => {
            let bit_count = 512;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheBytes64(
                FheUint512::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        10 => {
            let bit_count = 1024;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheBytes128(
                FheUint1024::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        11 => {
            let bit_count = 2048;
            let random_bits = upper_bound
                .map(be_number_random_bits)
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            Ok(SupportedFheCiphertexts::FheBytes256(
                FheUint2048::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            ))
        }
        other => Err(FhevmError::UnknownFheType(other as i32)),
    }
}

#[cfg(test)]
mod fhe_sum_tests {
    use super::{
        check_fhe_operand_types, does_fhe_operation_support_scalar, SupportedFheOperations,
    };
    use crate::types::FheOperationType;

    const FHE_SUM_OP: i32 = SupportedFheOperations::FheSum as i32;

    fn check_sum(n: usize, type_byte: u8) -> bool {
        let handles: Vec<Vec<u8>> = (0..n)
            .map(|_| {
                let mut h = vec![0u8; 32];
                h[30] = type_byte;
                h
            })
            .collect();
        let scalars = vec![false; n];
        check_fhe_operand_types(FHE_SUM_OP, &handles, &scalars).is_ok()
    }

    #[test]
    fn fhe_sum_op_type_is_other() {
        assert!(SupportedFheOperations::FheSum.op_type() == FheOperationType::Other);
    }

    #[test]
    fn fhe_sum_try_from_i16_roundtrip() {
        let op: SupportedFheOperations = (28i16).try_into().unwrap();
        assert_eq!(op, SupportedFheOperations::FheSum);
        assert_eq!(op as i16, 28);
    }

    #[test]
    fn fhe_sum_check_operand_types_empty_is_ok() {
        assert!(check_fhe_operand_types(FHE_SUM_OP, &[], &[]).is_ok());
    }

    #[test]
    fn fhe_sum_check_operand_types_single_input() {
        assert!(check_sum(1, 2));
        assert!(check_sum(1, 5));
    }

    #[test]
    fn fhe_sum_check_operand_types_too_many_inputs() {
        // (count, type_byte): Uint8=2 max 100, Uint64=5 and Uint128=6 max 60
        for (n, ty) in [(101usize, 2u8), (61, 5), (61, 6)] {
            assert!(!check_sum(n, ty), "n={n} ty={ty} should fail");
        }
    }

    #[test]
    fn fhe_sum_check_operand_types_valid_bounds() {
        // (count, type_byte): narrow max=100, wide max=60
        for (n, ty) in [(2usize, 2u8), (100, 2), (60, 5), (60, 6)] {
            assert!(check_sum(n, ty), "n={n} ty={ty} should pass");
        }
    }

    #[test]
    fn fhe_sum_rejects_scalar_input() {
        let handles: Vec<Vec<u8>> = (0..3).map(|_| vec![0u8; 32]).collect();
        assert!(check_fhe_operand_types(FHE_SUM_OP, &handles, &[false, true, false]).is_err());
    }

    #[test]
    fn fhe_sum_scalar_not_supported() {
        assert!(!does_fhe_operation_support_scalar(
            &SupportedFheOperations::FheSum
        ));
        assert!(!SupportedFheOperations::FheSum.does_have_more_than_one_scalar());
    }

    #[test]
    fn fhe_sum_check_operand_types_mismatched_types() {
        // First handle is Uint8 (type_byte=2), second is Uint16 (type_byte=3) — should fail.
        let mut h0 = vec![0u8; 32];
        h0[30] = 2; // Uint8
        let mut h1 = vec![0u8; 32];
        h1[30] = 3; // Uint16
        let handles = vec![h0, h1];
        let scalars = vec![false, false];
        assert!(
            check_fhe_operand_types(FHE_SUM_OP, &handles, &scalars).is_err(),
            "mixed types should fail"
        );
    }
}

#[cfg(test)]
mod fhe_is_in_tests {
    use super::{
        check_fhe_operand_types, does_fhe_operation_support_scalar, SupportedFheOperations,
    };
    use crate::types::FheOperationType;

    const FHE_IS_IN_OP: i32 = SupportedFheOperations::FheIsIn as i32;

    fn handle_with_type(type_byte: u8) -> Vec<u8> {
        let mut h = vec![0u8; 32];
        h[30] = type_byte;
        h
    }

    fn scalar_bytes(v: u64) -> Vec<u8> {
        v.to_be_bytes().to_vec()
    }

    #[test]
    fn fhe_is_in_op_type_is_other() {
        assert!(SupportedFheOperations::FheIsIn.op_type() == FheOperationType::Other);
    }

    #[test]
    fn fhe_is_in_try_from_i16_roundtrip() {
        let op = SupportedFheOperations::try_from(29i16).unwrap();
        assert_eq!(op, SupportedFheOperations::FheIsIn);
    }

    #[test]
    fn fhe_is_in_does_not_have_more_than_one_scalar() {
        // FheIsIn now uses all ciphertext handles — no scalar flag.
        assert!(!SupportedFheOperations::FheIsIn.does_have_more_than_one_scalar());
    }

    #[test]
    fn fhe_is_in_scalar_not_directly_supported() {
        assert!(!does_fhe_operation_support_scalar(
            &SupportedFheOperations::FheIsIn
        ));
    }

    #[test]
    fn fhe_is_in_valid_single_element_set() {
        let value = handle_with_type(2); // Uint8
        let set_elem = handle_with_type(2); // Uint8 ciphertext
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &[value, set_elem], &[false, false]).is_ok());
    }

    #[test]
    fn fhe_is_in_valid_multi_element_set() {
        let value = handle_with_type(5); // Uint64
        let handles: Vec<Vec<u8>> = std::iter::once(value)
            .chain((0..10).map(|_| handle_with_type(5)))
            .collect();
        let scalars: Vec<bool> = std::iter::repeat_n(false, 11).collect();
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &handles, &scalars).is_ok());
    }

    #[test]
    fn fhe_is_in_rejects_empty_inputs() {
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &[], &[]).is_err());
    }

    #[test]
    fn fhe_is_in_accepts_single_ciphertext_empty_set() {
        // Empty set (only the value ciphertext) is valid; execution returns trivial false.
        let value = handle_with_type(2); // Uint8
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &[value], &[false]).is_ok());
    }

    #[test]
    fn fhe_is_in_rejects_first_operand_as_scalar() {
        let scalar = scalar_bytes(5);
        let set_elem = handle_with_type(2);
        assert!(
            check_fhe_operand_types(FHE_IS_IN_OP, &[scalar, set_elem], &[true, false]).is_err()
        );
    }

    #[test]
    fn fhe_is_in_rejects_any_operand_as_scalar() {
        let value = handle_with_type(2); // Uint8
        let scalar_elem = scalar_bytes(42);
        // set element must be ciphertext, not scalar
        assert!(
            check_fhe_operand_types(FHE_IS_IN_OP, &[value, scalar_elem], &[false, true]).is_err()
        );
    }

    #[test]
    fn fhe_is_in_rejects_unsupported_type_ebytes128() {
        let value = handle_with_type(9); // EBytes128 (not supported)
        let set_elem = handle_with_type(9);
        assert!(
            check_fhe_operand_types(FHE_IS_IN_OP, &[value, set_elem], &[false, false]).is_err()
        );
    }

    #[test]
    fn fhe_is_in_rejects_too_many_set_elements_narrow() {
        // Uint8 max is 100; 101 elements should fail.
        let value = handle_with_type(2); // Uint8
        let handles: Vec<Vec<u8>> = std::iter::once(value)
            .chain((0..101).map(|_| handle_with_type(2)))
            .collect();
        let scalars: Vec<bool> = std::iter::repeat_n(false, 102).collect();
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &handles, &scalars).is_err());
    }

    #[test]
    fn fhe_is_in_rejects_too_many_set_elements_wide() {
        // Uint64 max is 60; 61 elements should fail.
        let value = handle_with_type(5); // Uint64
        let handles: Vec<Vec<u8>> = std::iter::once(value)
            .chain((0..61).map(|_| handle_with_type(5)))
            .collect();
        let scalars: Vec<bool> = std::iter::repeat_n(false, 62).collect();
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &handles, &scalars).is_err());
    }

    #[test]
    fn fhe_is_in_accepts_max_set_size_narrow() {
        // Uint8 max is 100; exactly 100 elements should succeed.
        let value = handle_with_type(2); // Uint8
        let handles: Vec<Vec<u8>> = std::iter::once(value)
            .chain((0..100).map(|_| handle_with_type(2)))
            .collect();
        let scalars: Vec<bool> = std::iter::repeat_n(false, 101).collect();
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &handles, &scalars).is_ok());
    }

    #[test]
    fn fhe_is_in_accepts_max_set_size_wide() {
        // Uint64 max is 60; exactly 60 elements should succeed.
        let value = handle_with_type(5); // Uint64
        let handles: Vec<Vec<u8>> = std::iter::once(value)
            .chain((0..60).map(|_| handle_with_type(5)))
            .collect();
        let scalars: Vec<bool> = std::iter::repeat_n(false, 61).collect();
        assert!(check_fhe_operand_types(FHE_IS_IN_OP, &handles, &scalars).is_ok());
    }

    #[test]
    fn fhe_is_in_supported_types_uint8_through_uint256() {
        for type_byte in 2u8..=8u8 {
            let value = handle_with_type(type_byte);
            let set_elem = handle_with_type(type_byte);
            assert!(
                check_fhe_operand_types(FHE_IS_IN_OP, &[value, set_elem], &[false, false]).is_ok(),
                "type {type_byte} should be supported"
            );
        }
    }

    #[test]
    fn fhe_is_in_rejects_mixed_type_set_elements() {
        // value is Uint8 (2), set element is Uint32 (4) — must be rejected.
        let value = handle_with_type(2);
        let wrong_type_elem = handle_with_type(4);
        assert!(
            check_fhe_operand_types(FHE_IS_IN_OP, &[value, wrong_type_elem], &[false, false])
                .is_err()
        );
    }
}

#[cfg(test)]
mod fhe_mul_div_tests {
    use super::{check_fhe_operand_types, SupportedFheOperations};

    const OP: i32 = SupportedFheOperations::FheMulDiv as i32;

    fn handle_with_type(type_byte: u8) -> Vec<u8> {
        let mut h = vec![0u8; 32];
        h[30] = type_byte;
        h
    }

    fn divisor(val: u64, width: usize) -> Vec<u8> {
        let mut out = vec![0u8; 32];
        let bytes = val.to_be_bytes();
        out[32 - width..].copy_from_slice(&bytes[8 - width..]);
        out
    }

    #[test]
    fn enc_enc_uint8_through_uint64_accepted() {
        for ty in 2u8..=5 {
            let width = 1usize << (ty - 2);
            let lhs = handle_with_type(ty);
            let rhs = handle_with_type(ty);
            let d = divisor(1, width);
            let res = check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, false, true]);
            assert!(res.is_ok(), "type {ty} enc×enc should pass, got {res:?}");
        }
    }

    #[test]
    fn enc_scalar_uint8_through_uint64_accepted() {
        for ty in 2u8..=5 {
            let width = 1usize << (ty - 2);
            let lhs = handle_with_type(ty);
            let rhs = divisor(7, width);
            let d = divisor(1, width);
            let res = check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, true, true]);
            assert!(res.is_ok(), "type {ty} enc×scalar should pass, got {res:?}");
        }
    }

    #[test]
    fn rejects_unsupported_lhs_type() {
        for ty in [0u8, 1, 6, 7, 8] {
            let lhs = handle_with_type(ty);
            let rhs = handle_with_type(ty);
            let d = divisor(1, 1);
            assert!(
                check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, false, true]).is_err(),
                "type {ty} must be rejected"
            );
        }
    }

    #[test]
    fn rejects_mismatched_encrypted_types() {
        let lhs = handle_with_type(4); // Uint32
        let rhs = handle_with_type(5); // Uint64
        let d = divisor(1, 4);
        assert!(check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, false, true]).is_err());
    }

    #[test]
    fn rejects_divisor_truncating_to_zero_per_operand_width() {
        // `0x...0100` has non-zero bytes32 but zero u8 → Uint8 must reject.
        let lhs = handle_with_type(2); // Uint8
        let rhs = handle_with_type(2);
        let mut d = vec![0u8; 32];
        d[30] = 1; // byte 31 stays 0 → low u8 byte is zero
        assert!(check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, false, true]).is_err());
    }

    #[test]
    fn rejects_divisor_all_zero_bytes() {
        let lhs = handle_with_type(5); // Uint64
        let rhs = handle_with_type(5);
        let d = vec![0u8; 32];
        assert!(check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, false, true]).is_err());
    }

    #[test]
    fn rejects_lhs_marked_scalar() {
        let lhs = handle_with_type(2);
        let rhs = handle_with_type(2);
        let d = divisor(1, 1);
        assert!(check_fhe_operand_types(OP, &[lhs, rhs, d], &[true, false, true]).is_err());
    }

    #[test]
    fn rejects_non_scalar_divisor() {
        let lhs = handle_with_type(2);
        let rhs = handle_with_type(2);
        let d = handle_with_type(2);
        assert!(check_fhe_operand_types(OP, &[lhs, rhs, d], &[false, false, false]).is_err());
    }

    #[test]
    fn rejects_wrong_operand_count() {
        let lhs = handle_with_type(2);
        let rhs = handle_with_type(2);
        assert!(check_fhe_operand_types(OP, &[lhs, rhs], &[false, false]).is_err());
    }
}
