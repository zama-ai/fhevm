use crate::http::utils::redact::{redact_bytes_len, redact_count};
use alloy::primitives::Bytes;
use derivative::Derivative;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_0x_hexs")
    )]
    pub ciphertext_handles: Vec<String>,
    #[schema(value_type = String, example = "0x00")]
    #[validate(custom(function = "crate::http::validate_extra_data_field"))]
    pub extra_data: String,
}

#[derive(Serialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
pub struct PublicDecryptResponseJson {
    #[derivative(Debug(format_with = "redact_count"))]
    pub response: Vec<PublicDecryptResponsePayloadJson>,
}

#[derive(Clone, ToSchema, Derivative)]
#[derivative(Debug)]
pub struct PublicDecryptResponsePayloadJson {
    #[schema(value_type = String)]
    #[derivative(Debug(format_with = "redact_bytes_len"))]
    pub decrypted_value: Bytes,
    #[schema(value_type = Vec<String>)]
    #[derivative(Debug(format_with = "redact_count"))]
    pub signatures: Vec<Bytes>,
    #[schema(value_type = String)]
    pub extra_data: String,
}

impl Serialize for PublicDecryptResponsePayloadJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PublicDecryptResponsePayloadJson", 2)?;
        state.serialize_field(
            "decrypted_value",
            &serialize_vec_as_hex(&self.decrypted_value.to_vec()),
        )?;
        let signatures_hex: Vec<String> = self
            .signatures
            .iter()
            .map(|bytes| serialize_vec_as_hex(&bytes.to_vec()))
            .collect();
        state.serialize_field("signatures", &signatures_hex)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Clone, serde::Deserialize, ToSchema)]
pub struct PublicDecryptErrorResponseJson {
    pub message: String,
}

fn serialize_vec_as_hex(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}
