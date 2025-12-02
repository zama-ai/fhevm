use crate::http::de_string_or_number;
use crate::http::types::ChainId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofRequestJson {
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contract_chain_id: String,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub contract_address: String,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub user_address: String,
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    pub ciphertext_with_input_verification: String,
    #[validate(custom(function = "crate::http::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct InputProofResponseJson {
    pub response: InputProofResponsePayloadJson,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct InputProofResponsePayloadJson {
    pub handles: Vec<String>,
    pub signatures: Vec<String>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct InputProofErrorResponseJson {
    pub message: String,
}
