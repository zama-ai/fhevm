use crate::{
    keys::TFHE_PARAMS,
    types::{
        is_ebytes_type, FheOperationType, FhevmError, SupportedFheCiphertexts,
        SupportedFheOperations,
    },
    utils::{safe_deserialize, safe_deserialize_conformant},
};
use tfhe::{
    integer::{
        bigint::StaticUnsignedBigInt,
        ciphertext::IntegerProvenCompactCiphertextListConformanceParams, U256,
    },
    prelude::{
        CastInto, CiphertextList, FheEq, FheMax, FheMin, FheOrd, FheTryTrivialEncrypt, IfThenElse,
        RotateLeft, RotateRight,
    },
    shortint::parameters::CompactPublicKeyEncryptionParameters,
    zk::CompactPkePublicParams,
    FheBool, FheUint1024, FheUint128, FheUint16, FheUint160, FheUint2, FheUint2048, FheUint256,
    FheUint32, FheUint4, FheUint512, FheUint64, FheUint8, Seed,
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
        _ => {
            return Err(FhevmError::UnknownFheType(input_type as i32));
        }
    }
}

/// Function assumes encryption key already set
pub fn trivial_encrypt_be_bytes(output_type: i16, input_bytes: &[u8]) -> SupportedFheCiphertexts {
    let last_byte = if input_bytes.len() > 0 {
        input_bytes[input_bytes.len() - 1]
    } else {
        0
    };
    match output_type {
        0 => SupportedFheCiphertexts::FheBool(
            FheBool::try_encrypt_trivial(last_byte > 0).expect("trival encrypt bool"),
        ),
        1 => SupportedFheCiphertexts::FheUint4(
            FheUint4::try_encrypt_trivial(last_byte).expect("trivial encrypt 4"),
        ),
        2 => SupportedFheCiphertexts::FheUint8(
            FheUint8::try_encrypt_trivial(last_byte).expect("trivial encrypt 8"),
        ),
        3 => {
            let mut padded: [u8; 2] = [0; 2];
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheUint16(
                FheUint16::try_encrypt_trivial(res).expect("trivial encrypt 16"),
            )
        }
        4 => {
            let mut padded: [u8; 4] = [0; 4];
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheUint32(
                FheUint32::try_encrypt_trivial(res).expect("trivial encrypt 32"),
            )
        }
        5 => {
            let mut padded: [u8; 8] = [0; 8];
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheUint64(
                FheUint64::try_encrypt_trivial(res).expect("trivial encrypt 64"),
            )
        }
        6 => {
            let mut padded: [u8; 16] = [0; 16];
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheUint128(output)
        }
        7 => {
            let mut padded: [u8; 32] = [0; 32];
            let mut be: U256 = U256::ZERO;
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheUint160(output)
        }
        8 => {
            let mut padded: [u8; 32] = [0; 32];
            let mut be: U256 = U256::ZERO;
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheUint256(output)
        }
        9 => {
            let mut padded: [u8; 64] = [0; 64];
            let mut be: StaticUnsignedBigInt<8> = StaticUnsignedBigInt::<8>::ZERO;
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheBytes64(output)
        }
        10 => {
            let mut padded: [u8; 128] = [0; 128];
            let mut be: StaticUnsignedBigInt<16> = StaticUnsignedBigInt::<16>::ZERO;
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheBytes128(output)
        }
        11 => {
            let mut padded: [u8; 256] = [0; 256];
            let mut be: StaticUnsignedBigInt<32> = StaticUnsignedBigInt::<32>::ZERO;
            if input_bytes.len() > 0 {
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
            SupportedFheCiphertexts::FheBytes256(output)
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
    public_params: &CompactPkePublicParams,
) -> Result<Vec<SupportedFheCiphertexts>, FhevmError> {
    let mut res = Vec::new();

    let pk_params = CompactPublicKeyEncryptionParameters::try_from(TFHE_PARAMS)
        .map_err(|_| FhevmError::MissingTfheRsData)?;

    let the_list: tfhe::ProvenCompactCiphertextList = safe_deserialize_conformant(
        input_ciphertext,
        &IntegerProvenCompactCiphertextListConformanceParams::from_public_key_encryption_parameters_and_crs_parameters(
            pk_params, public_params,
        ),
    )?;

    let expanded = the_list
        .expand_without_verification()
        .map_err(|e| FhevmError::CiphertextExpansionError(e))?;

    for idx in 0..expanded.len() {
        let Some(data_kind) = expanded.get_kind_of(idx) else {
            panic!("we're itering over what ciphertext told us how many ciphertexts are there, it must exist")
        };

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

// return output ciphertext type
pub fn check_fhe_operand_types(
    fhe_operation: i32,
    input_types: &[i16],
    input_handles: &[Vec<u8>],
    is_input_handle_scalar: &[bool],
) -> Result<i16, FhevmError> {
    let fhe_op: SupportedFheOperations = fhe_operation.try_into()?;

    assert_eq!(input_handles.len(), is_input_handle_scalar.len());
    // TODO: figure out typing system with constants
    let fhe_bool_type = 0;

    let scalar_operands = is_input_handle_scalar
        .iter()
        .enumerate()
        .filter(|(_, is_scalar)| **is_scalar)
        .collect::<Vec<_>>();

    let is_scalar = scalar_operands.len() > 0;

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

            // special case for div operation, rhs for scalar must not be zero
            if is_scalar {
                if fhe_op == SupportedFheOperations::FheDiv {
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

                if is_ebytes_type(input_types[0]) {
                    return Err(FhevmError::FheOperationDoesntSupportScalar {
                        fhe_operation,
                        fhe_operation_name: format!("{:?}", fhe_op),
                        scalar_requested: is_scalar,
                        scalar_supported: false,
                    });
                }
            }

            if is_ebytes_type(input_types[0]) {
                if !fhe_op.supports_ebytes_inputs() {
                    return Err(FhevmError::FheOperationDoesntSupportEbytesAsInput {
                        lhs_handle: format!("0x{}", hex::encode(&input_handles[0])),
                        rhs_handle: format!("0x{}", hex::encode(&input_handles[1])),
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

            if input_types[0] == fhe_bool_type && !fhe_op.supports_bool_inputs() {
                return Err(FhevmError::OperationDoesntSupportBooleanInputs {
                    fhe_operation,
                    fhe_operation_name: format!("{:?}", fhe_op),
                    operand_type: fhe_bool_type,
                });
            }

            if is_ebytes_type(input_types[0]) {
                if !fhe_op.supports_ebytes_inputs() {
                    return Err(FhevmError::FheOperationDoesntSupportEbytesAsInput {
                        lhs_handle: format!("0x{}", hex::encode(&input_handles[0])),
                        rhs_handle: format!("0x{}", hex::encode(&input_handles[1])),
                        fhe_operation,
                        fhe_operation_name: format!("{:?}", SupportedFheOperations::FheDiv),
                    });
                }
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
                SupportedFheOperations::FheTrivialEncrypt => {
                    let expected_operands = 2;
                    if input_types.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
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

                    let output_type = op[0] as i32;
                    validate_fhe_type(output_type)?;
                    Ok(output_type as i16)
                }
                SupportedFheOperations::FheRand => {
                    // counter and output type
                    let expected_operands = 2;
                    if input_types.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
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

                    Ok(rand_type[0] as i16)
                }
                SupportedFheOperations::FheRandBounded => {
                    // counter, bound and output type
                    let expected_operands = 3;
                    if input_types.len() != expected_operands {
                        return Err(FhevmError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
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

                    validate_fhe_type(rand_type[0] as i32)?;

                    Ok(rand_type[0] as i16)
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
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => Ok(()),
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

fn trim_to_160bit(inp: &U256) -> U256 {
    let mut res: [u8; 32] = [0; 32];
    inp.copy_to_be_byte_slice(res.as_mut_slice());
    // zero first 12 bytes
    for i in 0..12 {
        res[i] = 0;
    }
    let mut output = U256::ZERO;
    output.copy_from_be_byte_slice(&res);
    output
}

pub fn perform_fhe_operation(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    // for deterministc randomness functions
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a + b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a + b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a + b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a + (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a + (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a + (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a + (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a + (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a + l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a + b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a + *b))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a - b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a - b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a - b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a - (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a - (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a - (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a - (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a - (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a - l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a - b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a - *b))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a * b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a * b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a * b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a * (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a * (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a * (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a * (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a * (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a * l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a * b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a * *b))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a / b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a / b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a / b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a / (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a / (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a / (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a / (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a / (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a / l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a / b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a / *b))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a % b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a % b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a % b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a % (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a % (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a % (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a % (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a % (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a % l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a % b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a % *b))
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
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a & (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a & (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a & (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a & (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a & (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a & l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a & b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a & *b))
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
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a | (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a | (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a | (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a | (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a | (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a | l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a | b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a | *b))
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
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a ^ (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a ^ (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a ^ (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a ^ (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a ^ (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a ^ l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a ^ b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a ^ *b))
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
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a << (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a << (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a << (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a << (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a << (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a << l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a << b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a << *b))
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
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a >> (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a >> (l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a >> (l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a >> (l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a >> (l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a >> l))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a >> b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a >> *b))
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
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a.rotate_left(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a.rotate_left(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a.rotate_left(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a.rotate_left(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a.rotate_left(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a.rotate_left(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a.rotate_left(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a.rotate_left(*b)))
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
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a.rotate_right(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a.rotate_right(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a.rotate_right(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a.rotate_right(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a.rotate_right(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a.rotate_right(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a.rotate_right(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a.rotate_right(*b)))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a.min(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a.min(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a.min(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a.min(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a.min(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a.min(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a.min(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a.min(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a.min(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a.min(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a.min(*b)))
                }
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes64(a.max(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes128(a.max(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBytes256(a.max(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint4(a.max(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint8(a.max(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint16(a.max(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint32(a.max(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint64(a.max(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheUint128(a.max(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheUint160(a.max(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint256(a.max(*b)))
                }
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
                    let (l, h) = b.to_low_high_u128();
                    let non_zero = l > 0 || h > 0;
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(non_zero)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.eq(*b)))
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
                    let (l, h) = b.to_low_high_u128();
                    let non_zero = l > 0 || h > 0;
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(non_zero)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ne(*b)))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ge(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ge(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.ge(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.ge(*b)))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.gt(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.gt(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.gt(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.gt(*b)))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.le(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.le(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.le(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.le(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheBool(a.le(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.le(*b)))
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
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.lt(b))),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.lt(b))),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(SupportedFheCiphertexts::FheBool(a.lt(b))),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u8)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u16)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u32)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l as u64)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let (l, _) = b.to_low_high_u128();
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(l)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let b = trim_to_160bit(b);
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(SupportedFheCiphertexts::FheBool(a.lt(*b)))
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
                SupportedFheCiphertexts::FheBytes64(a) => {
                    Ok(SupportedFheCiphertexts::FheBytes64(-a))
                }
                SupportedFheCiphertexts::FheBytes128(a) => {
                    Ok(SupportedFheCiphertexts::FheBytes128(-a))
                }
                SupportedFheCiphertexts::FheBytes256(a) => {
                    Ok(SupportedFheCiphertexts::FheBytes256(-a))
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheBool(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint4(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint8(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint16(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint32(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint64(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint128(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint160(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheUint256(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheBytes64(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheBytes128(inp.clone()));
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let type_id = input_operands[0].type_num();
                if l == type_id {
                    return Ok(SupportedFheCiphertexts::FheBytes256(inp.clone()));
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
                        other => panic!("unexpected type: {other}"),
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
                let (l, _) = op.to_low_high_u128();
                let l = l as i16;
                let mut bytes: [u8; 32] = [0; 32];
                inp.copy_to_be_byte_slice(&mut bytes);
                Ok(trivial_encrypt_be_bytes(l, &bytes))
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
            let (rand_counter, _) = rand_counter.to_low_high_u128();
            let (to_type, _) = to_type.to_low_high_u128();
            Ok(generate_random_number(to_type as i16, rand_counter, None))
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
            let (rand_counter, _) = rand_counter.to_low_high_u128();
            let (to_type, _) = to_type.to_low_high_u128();
            Ok(generate_random_number(
                to_type as i16,
                rand_counter,
                Some(*upper_bound),
            ))
        }
        SupportedFheOperations::FheGetInputCiphertext => todo!("Implement FheGetInputCiphertext"),
    }
}

pub fn generate_random_number(
    the_type: i16,
    seed: u128,
    upper_bound: Option<U256>,
) -> SupportedFheCiphertexts {
    let subtract_from = 255;
    match the_type {
        0 => {
            let num = FheUint2::generate_oblivious_pseudo_random_bounded(Seed(seed), 1);
            SupportedFheCiphertexts::FheBool(num.gt(0))
        }
        1 => {
            let bit_count = 4;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint4(FheUint4::generate_oblivious_pseudo_random_bounded(
                Seed(seed),
                random_bits,
            ))
        }
        2 => {
            let bit_count = 8;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint8(FheUint8::generate_oblivious_pseudo_random_bounded(
                Seed(seed),
                random_bits,
            ))
        }
        3 => {
            let bit_count = 16;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint16(FheUint16::generate_oblivious_pseudo_random_bounded(
                Seed(seed),
                random_bits,
            ))
        }
        4 => {
            let bit_count = 32;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint32(FheUint32::generate_oblivious_pseudo_random_bounded(
                Seed(seed),
                random_bits,
            ))
        }
        5 => {
            let bit_count = 64;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint64(FheUint64::generate_oblivious_pseudo_random_bounded(
                Seed(seed),
                random_bits,
            ))
        }
        6 => {
            let bit_count = 128;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint128(
                FheUint128::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            )
        }
        7 => {
            let bit_count = 160;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint160(
                FheUint160::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            )
        }
        8 => {
            let bit_count = 256;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheUint256(
                FheUint256::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            )
        }
        9 => {
            let bit_count = 512;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheBytes64(
                FheUint512::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            )
        }
        10 => {
            let bit_count = 1024;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheBytes128(
                FheUint1024::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            )
        }
        11 => {
            let bit_count = 2048;
            let random_bits = upper_bound
                .map(|i| subtract_from - i.leading_zeros())
                .unwrap_or(bit_count)
                .min(bit_count) as u64;
            SupportedFheCiphertexts::FheBytes256(
                FheUint2048::generate_oblivious_pseudo_random_bounded(Seed(seed), random_bits),
            )
        }
        other => {
            panic!("unknown type to trim to: {other}")
        }
    }
}
