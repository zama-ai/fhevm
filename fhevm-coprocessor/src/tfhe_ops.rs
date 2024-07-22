use crate::types::{SupportedFheCiphertexts, CoprocessorError};

pub fn current_ciphertext_version() -> i16 {
    1
}

pub fn perform_fhe_operation(operation: i16, input_operands: &[SupportedFheCiphertexts]) -> Result<SupportedFheCiphertexts, Box<dyn std::error::Error + Send + Sync>> {
    match operation {
        1 => {
            assert_eq!(input_operands.len(), 2);
            // fhe add
            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint8(a + b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(SupportedFheCiphertexts::FheUint32(a + b))
                }
                _ => {
                    panic!("Unsupported fhe types");
                }
            }
        }
        _ => panic!("Not implemented yet")
    }
}

pub fn deserialize_fhe_ciphertext(input_type: i16, input_bytes: &[u8]) -> Result<SupportedFheCiphertexts, Box<dyn std::error::Error + Send + Sync>> {
    match input_type {
        1 => {
            let v: tfhe::FheBool = bincode::deserialize(input_bytes)?;
            Ok(SupportedFheCiphertexts::FheBool(v))
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