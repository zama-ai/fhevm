use tfhe::{prelude::{FheEq, FheMax, FheMin, FheOrd, FheTryTrivialEncrypt, RotateLeft, RotateRight}, FheBool, FheUint16, FheUint32, FheUint64, FheUint8};

use crate::{types::{CoprocessorError, FheOperationType, SupportedFheCiphertexts, SupportedFheOperations}, utils::check_if_handle_is_zero};

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
        },
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
        },
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
        },
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
        },
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
        },
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
        },
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
        },
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
        },
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
        },
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
        },
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
        },
        SupportedFheOperations::FheEq => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
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
        },
        SupportedFheOperations::FheNe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
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
        },
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
        },
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
        },
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
        },
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
        },
        SupportedFheOperations::FheNot => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint8(a) => {
                    Ok(SupportedFheCiphertexts::FheUint8(!a))
                }
                SupportedFheCiphertexts::FheUint16(a) => {
                    Ok(SupportedFheCiphertexts::FheUint16(!a))
                }
                SupportedFheCiphertexts::FheUint32(a) => {
                    Ok(SupportedFheCiphertexts::FheUint32(!a))
                }
                SupportedFheCiphertexts::FheUint64(a) => {
                    Ok(SupportedFheCiphertexts::FheUint64(!a))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        },
        SupportedFheOperations::FheNeg => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint8(a) => {
                    Ok(SupportedFheCiphertexts::FheUint8(-a))
                }
                SupportedFheCiphertexts::FheUint16(a) => {
                    Ok(SupportedFheCiphertexts::FheUint16(-a))
                }
                SupportedFheCiphertexts::FheUint32(a) => {
                    Ok(SupportedFheCiphertexts::FheUint32(-a))
                }
                SupportedFheCiphertexts::FheUint64(a) => {
                    Ok(SupportedFheCiphertexts::FheUint64(-a))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        },
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
        5 => {
            let mut padded: [u8; 8] = [0; 8];
            let len = padded.len().min(input_bytes.len());
            padded[0..len].copy_from_slice(&input_bytes[0..len]);
            let res: u64 = u64::from_le_bytes(padded);
            SupportedFheCiphertexts::FheUint64(FheUint64::try_encrypt_trivial(res).unwrap())
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
        5 => {
            let v: tfhe::FheUint64 = bincode::deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheUint64(v))
        }
        _ => {
            return Err(Box::new(CoprocessorError::UnknownCiphertextType(input_type)));
        }
    }
}

// return output ciphertext type
pub fn check_fhe_operand_types(fhe_operation: i32, input_types: &[i16], is_scalar: bool, input_handles: &[String]) -> Result<i16, CoprocessorError> {
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

            // special case for div operation
            if is_scalar && fhe_op == SupportedFheOperations::FheDiv {
                if check_if_handle_is_zero(input_handles[1].as_str()) {
                    return Err(CoprocessorError::FheOperationScalarDivisionByZero {
                        lhs_handle: input_handles[0].clone(),
                        rhs_value: input_handles[1].clone(),
                        fhe_operation,
                        fhe_operation_name: format!("{:?}", SupportedFheOperations::FheDiv),
                    });
                }
            }

            if fhe_op.is_comparison() {
                return Ok(1); // fhe bool type
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
        FheOperationType::Binary => true,
        FheOperationType::Unary => false,
        FheOperationType::Other => false,
    }
}