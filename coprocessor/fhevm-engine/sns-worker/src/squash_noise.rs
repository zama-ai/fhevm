use crate::ExecutionError;
use crate::SAFE_SER_LIMIT;
use serde::Serialize;

use tfhe::named::Named;
use tfhe::prelude::SquashNoise;
use tfhe::CompressedSquashedNoiseCiphertextListBuilder;
use tfhe::SquashedNoiseFheUint;
use tfhe::Versionize;

use fhevm_engine_common::types::SupportedFheCiphertexts;

macro_rules! squash_and_serialize_with_error {
    ($value:expr, $target_ty:ty, $enable_compression:expr, $ct_type:expr) => {{
        let ct_type = $ct_type;

        let squashed: $target_ty = {
            let span = tracing::info_span!(
                "squash_noise_fhe",
                ct_type = %ct_type,
                operation = "squash_noise_fhe"
            );
            let _enter = span.enter();
            $value
                .squash_noise()
                .map_err(ExecutionError::SquashedNoiseError)?
        };

        if !$enable_compression {
            let span = tracing::info_span!(
                "serialize",
                ct_type = %ct_type,
                operation = "serialize"
            );
            let _enter = span.enter();
            return safe_serialize(&squashed);
        }

        let list = {
            let span = tracing::info_span!(
                "compress",
                ct_type = %ct_type,
                operation = "compress"
            );
            let _enter = span.enter();
            let mut builder = CompressedSquashedNoiseCiphertextListBuilder::new();
            builder.push(squashed);
            builder.build()?
        };

        let span = tracing::info_span!(
            "serialize",
            ct_type = %ct_type,
            operation = "serialize"
        );
        let _enter = span.enter();
        Ok(safe_serialize(&list)?)
    }};
}

pub(crate) trait SquashNoiseCiphertext {
    /// Squashes the noise of the ciphertext and serializes it.
    /// Returns the compressed big ciphertext serialized if `enable_compression` is true,
    /// otherwise returns the squashed ciphertext serialized.
    fn squash_noise_and_serialize(
        &self,
        enable_compression: bool,
    ) -> Result<Vec<u8>, ExecutionError>;

    /// Tries to decrypt a squashed noise ciphertext and returns the cleartext.
    #[cfg(feature = "test_decrypt_128")]
    fn decrypt_squash_noise(
        &self,
        key: &tfhe::ClientKey,
        data: &[u8],
    ) -> Result<u128, ExecutionError>;

    /// Tries to decrypt a compressed squashed noise ciphertext and returns the cleartext.
    #[cfg(feature = "test_decrypt_128")]
    fn decrypt_squash_noise_compressed(
        &self,
        key: &tfhe::ClientKey,
        data: &[u8],
    ) -> Result<u128, ExecutionError>;
}

impl SquashNoiseCiphertext for SupportedFheCiphertexts {
    fn squash_noise_and_serialize(
        &self,
        enable_compression: bool,
    ) -> Result<Vec<u8>, ExecutionError> {
        let ct_type = self.type_name();
        match self {
            SupportedFheCiphertexts::FheBool(v) => {
                squash_and_serialize_with_error!(
                    v,
                    tfhe::SquashedNoiseFheBool,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint4(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }

            SupportedFheCiphertexts::FheUint8(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint16(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint32(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint64(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint128(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint160(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheUint256(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheBytes64(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheBytes128(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
            }
            SupportedFheCiphertexts::FheBytes256(v) => {
                squash_and_serialize_with_error!(
                    v,
                    SquashedNoiseFheUint,
                    enable_compression,
                    ct_type
                )
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
                let v: SquashedNoiseFheUint = safe_deserialize(data)?;
                let clear: u128 = v.decrypt(key);
                clear
            }
        };
        Ok(res)
    }

    #[cfg(feature = "test_decrypt_128")]
    fn decrypt_squash_noise_compressed(
        &self,
        key: &tfhe::ClientKey,
        list: &[u8],
    ) -> Result<u128, ExecutionError> {
        use tfhe::CompressedSquashedNoiseCiphertextList;
        use tfhe::{prelude::FheDecrypt, SquashedNoiseFheUint};
        let list: CompressedSquashedNoiseCiphertextList = safe_deserialize(list)?;

        let res = match self {
            SupportedFheCiphertexts::FheBool(_) => {
                let v: tfhe::SquashedNoiseFheBool = list.get(0)?.ok_or_else(|| {
                    anyhow::anyhow!("Failed to get the first element from the list")
                })?;
                let clear: bool = v.decrypt(key);
                clear as u128
            }
            _ => {
                let v: SquashedNoiseFheUint = list.get(0)?.ok_or_else(|| {
                    anyhow::anyhow!("Failed to get the first element from the list")
                })?;
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
    tfhe::safe_serialization::safe_serialize(object, &mut out, SAFE_SER_LIMIT)
        .map_err(|e| ExecutionError::SerializationError(e.to_string()))?;
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
