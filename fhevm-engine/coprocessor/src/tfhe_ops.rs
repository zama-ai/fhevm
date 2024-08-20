use tfhe::{
    prelude::{
        CastInto, FheEq, FheMax, FheMin, FheOrd, FheTryTrivialEncrypt, IfThenElse, RotateLeft, RotateRight
    },
    FheBool, FheUint16, FheUint32, FheUint64, FheUint8,
};

use crate::{
    server::coprocessor::{async_computation_input::Input, AsyncComputationInput},
    types::{CoprocessorError, FheOperationType, SupportedFheCiphertexts, SupportedFheOperations},
};

pub fn current_ciphertext_version() -> i16 {
    1
}

pub fn perform_fhe_operation(
    fhe_operation: i16,
    input_operands: &[SupportedFheCiphertexts],
) -> Result<SupportedFheCiphertexts, Box<dyn std::error::Error + Send + Sync>> {
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
        }
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
        },
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

pub fn validate_fhe_type(input_type: i16) -> Result<(), CoprocessorError> {
    match input_type {
        1 | 2 | 3 | 4 | 5 => Ok(()),
        _ => Err(CoprocessorError::UnknownCiphertextType(input_type)),
    }
}

pub fn deserialize_fhe_ciphertext(
    input_type: i16,
    input_bytes: &[u8],
) -> Result<SupportedFheCiphertexts, Box<dyn std::error::Error + Send + Sync>> {
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
            return Err(Box::new(CoprocessorError::UnknownCiphertextType(
                input_type,
            )));
        }
    }
}

fn encode_comp_input_to_handle(input: &AsyncComputationInput) -> String {
    match &input.input {
        Some(Input::Scalar(sc)) => {
            format!("0x{}", hex::encode(sc))
        }
        Some(Input::InputHandle(handle)) => {
            format!("0x{}", hex::encode(handle))
        }
        None => panic!("we assume we get something here"),
    }
}

// return output ciphertext type
pub fn check_fhe_operand_types(
    fhe_operation: i32,
    input_types: &[i16],
    is_scalar: bool,
    input_handles: &[AsyncComputationInput],
) -> Result<i16, CoprocessorError> {
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
                return Err(
                    CoprocessorError::FheOperationDoesntHaveUniformTypesAsInput {
                        fhe_operation,
                        fhe_operation_name: format!("{:?}", fhe_op),
                        operand_types: input_types.to_vec(),
                    },
                );
            }

            // special case for div operation, rhs for scalar must be zero
            if is_scalar && fhe_op == SupportedFheOperations::FheDiv {
                if let Some(Input::Scalar(sc)) = &input_handles[1].input {
                    let all_zeroes = sc.iter().all(|i| *i == 0u8);
                    if all_zeroes {
                        return Err(CoprocessorError::FheOperationScalarDivisionByZero {
                            lhs_handle: encode_comp_input_to_handle(&input_handles[0]),
                            rhs_value: encode_comp_input_to_handle(&input_handles[1]),
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", SupportedFheOperations::FheDiv),
                        });
                    }
                } else {
                    panic!("rhs operand must be scalar here")
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

                    // TODO: figure out typing system with constants
                    let fhe_bool_type = 1;
                    if input_types[0] != fhe_bool_type {
                        return Err(CoprocessorError::FheIfThenElseUnexpectedOperandTypes {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            first_expected_operand_type: fhe_bool_type,
                            first_expected_operand_type_name: "FheBool".to_string(),
                            first_operand_type: input_types[0],
                        });
                    }

                    if input_types[1] != input_types[2] {
                        return Err(CoprocessorError::FheIfThenElseMismatchingSecondAndThirdOperatorTypes {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            second_operand_type: input_types[1],
                            third_operand_type: input_types[2],
                        });
                    }

                    Ok(input_types[1])
                }
                SupportedFheOperations::FheCast => {
                    let expected_operands = 2;
                    if input_types.len() != expected_operands {
                        return Err(CoprocessorError::UnexpectedOperandCountForFheOperation {
                            fhe_operation,
                            fhe_operation_name: format!("{:?}", fhe_op),
                            expected_operands,
                            got_operands: input_types.len(),
                        });
                    }

                    match (&input_handles[0].input, &input_handles[1].input) {
                        (Some(a), Some(b)) => match (a, b) {
                            (Input::InputHandle(_ih), Input::Scalar(op)) => {
                                if op.len() != 1 {
                                    return Err(CoprocessorError::UnexpectedCastOperandSizeForScalarOperand {
                                            fhe_operation,
                                            fhe_operation_name: format!("{:?}", fhe_op),
                                            expected_scalar_operand_bytes: 1,
                                            got_bytes: op.len(),
                                        });
                                }

                                let output_type = op[0] as i16;
                                validate_fhe_type(output_type)?;
                                Ok(output_type)
                            }
                            _ => {
                                return Err(CoprocessorError::UnexpectedCastOperandTypes {
                                    fhe_operation,
                                    fhe_operation_name: format!("{:?}", fhe_op),
                                    expected_operator_combination: vec![
                                        "handle".to_string(),
                                        "scalar".to_string(),
                                    ],
                                });
                            }
                        },
                        _ => panic!("operands should always be some here, we checked earlier"),
                    }
                }
                other => {
                    panic!("Unexpected branch: {:?}", other)
                }
            }
        }
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
#[cfg(test)]
pub fn does_fhe_operation_support_both_encrypted_operands(op: &SupportedFheOperations) -> bool {
    match op {
        SupportedFheOperations::FheDiv => false,
        _ => true,
    }
}
