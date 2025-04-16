use crate::ExecutionError;
use crate::SAFE_SER_LIMIT;
use serde::Serialize;

use tfhe::named::Named;
use tfhe::prelude::SquashNoise;

use tfhe::SquashedNoiseFheUint;
use tfhe::Versionize;

use fhevm_engine_common::types::SupportedFheCiphertexts;

macro_rules! squash_and_serialize_with_error {
    ($value:expr, $target_ty:ty) => {{
        let squashed: $target_ty = $value
            .squash_noise()
            .map_err(ExecutionError::SquashedNoiseError)?;
        Ok(safe_serialize(&squashed)?)
    }};
}

pub(crate) trait SquashNoiseCiphertext {
    /// Squashes the noise of the ciphertext and serializes it.
    /// Returns the serialized ciphertext as a byte vector.
    fn squash_noise_and_serialize(&self) -> Result<Vec<u8>, ExecutionError>;

    /// Decrypts the squashed noise ciphertext and returns the decrypted value.
    #[cfg(feature = "test_decrypt_128")]
    fn decrypt_squash_noise(
        &self,
        key: &tfhe::ClientKey,
        data: &[u8],
    ) -> Result<u128, ExecutionError>;
}

impl SquashNoiseCiphertext for SupportedFheCiphertexts {
    fn squash_noise_and_serialize(&self) -> Result<Vec<u8>, ExecutionError> {
        match self {
            SupportedFheCiphertexts::FheBool(v) => {
                squash_and_serialize_with_error!(v, tfhe::SquashedNoiseFheBool)
            }
            SupportedFheCiphertexts::FheUint4(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }

            SupportedFheCiphertexts::FheUint8(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheUint16(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheUint32(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheUint64(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheUint128(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheUint160(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheUint256(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheBytes64(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheBytes128(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::FheBytes256(v) => {
                squash_and_serialize_with_error!(v, SquashedNoiseFheUint)
            }
            SupportedFheCiphertexts::Scalar(_) => {
                panic!("we should never need to serialize scalar")
            }
        }
    }

    #[cfg(feature = "test_decrypt_128")]
    fn decrypt_squash_noise(
        &self,
        key: &tfhe::ClientKey,
        data: &[u8],
    ) -> Result<u128, ExecutionError> {
        use tfhe::{prelude::FheDecrypt, SquashedNoiseFheUint};
        let res = match self {
            SupportedFheCiphertexts::FheBool(_) => {
                let v: tfhe::SquashedNoiseFheBool = safe_deserialize(data)?;
                let clear: bool = v.decrypt(key);
                clear as u128
            }
            _ => {
                let v: SquashedNoiseFheUint = safe_deserialize(data).unwrap();
                let clear: u128 = v.decrypt(key);
                clear
            }
        };
        Ok(res)
    }
}

pub fn safe_serialize<T: Serialize + Named + Versionize>(
    object: &T,
) -> Result<Vec<u8>, ExecutionError> {
    let mut out = vec![];
    tfhe::safe_serialization::safe_serialize(object, &mut out, SAFE_SER_LIMIT)?;
    Ok(out)
}

#[cfg(feature = "test_decrypt_128")]
pub fn safe_deserialize<T: serde::de::DeserializeOwned + Named + tfhe::Unversionize>(
    input: &[u8],
) -> Result<T, ExecutionError> {
    let res = tfhe::safe_serialization::safe_deserialize(input, SAFE_SER_LIMIT)
        .map_err(ExecutionError::DeserializationError)?;
    Ok(res)
}
