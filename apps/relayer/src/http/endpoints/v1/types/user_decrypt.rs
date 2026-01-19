use crate::http::endpoints::common::types::{ChainId, HandleContractPairJson, RequestValidityJson};
use crate::http::utils::redact::{redact_count, redact_len};
use crate::http::{de_string_or_number, serialize_vec_as_hex};
use alloy::primitives::Bytes;
use derivative::Derivative;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_handle_contract_pairs")
    )]
    pub handle_contract_pairs: Vec<HandleContractPairJson>,
    #[validate(custom(function = "crate::http::validate_request_validity"))]
    pub request_validity: RequestValidityJson,
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contracts_chain_id: String,
    #[validate(length(min = 1, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_blockchain_addresses"))]
    pub contract_addresses: Vec<String>,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub user_address: String,
    #[validate(
        length(equal = 130, message = "Must be 130 characters long"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    #[derivative(Debug(format_with = "redact_len"))]
    pub signature: String,
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    #[derivative(Debug(format_with = "redact_len"))]
    pub public_key: String,
    #[validate(custom(function = "crate::http::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
pub struct UserDecryptResponseJson {
    #[derivative(Debug(format_with = "redact_count"))]
    pub response: Vec<UserDecryptResponsePayloadJson>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UserDecryptResponsePayloadJson {
    #[schema(value_type = String)]
    pub payload: Bytes,
    #[schema(value_type = String)]
    pub signature: Bytes,
    #[schema(value_type = String)]
    pub extra_data: String,
}

impl Serialize for UserDecryptResponsePayloadJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UserDecryptResponsePayloadJson", 2)?;
        state.serialize_field("payload", &serialize_vec_as_hex(&self.payload.to_vec()))?;
        state.serialize_field("signature", &serialize_vec_as_hex(&self.signature.to_vec()))?;
        state.end()
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct UserDecryptErrorResponseJson {
    pub message: String,
}
