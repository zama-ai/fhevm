pub mod admin;
pub mod endpoints;
pub mod middleware;
pub mod openapi;
pub mod retry_after;
pub mod server;
pub mod utils;

// Re-export key types and functions for direct access
pub use endpoints::{
    health_handler, liveness_handler, version_handler, HealthResponse, LivenessResponse,
    VersionResponse,
};

pub use middleware::openapi_middleware;

pub use retry_after::{DecryptQueueInfo, ReadinessQueueInfo, RequestQueueInfo, TxQueueInfo};

pub use utils::{
    // Other utilities
    de_string_or_number,
    deserialize_ct_handles_from_hex,
    // Parsing utilities
    parse_and_validate,
    parse_and_validate_cross,
    // Serialization helpers
    serialize_ct_handle_as_hex,
    serialize_ct_handles_as_hex,
    serialize_vec_as_hex,
    to_camel_case,
    validate_0x_hex,
    validate_0x_hexs,
    // Validation functions (most commonly used)
    validate_blockchain_address,
    validate_blockchain_addresses,
    validate_blockchain_addresses_allow_empty,
    validate_chain_id_string,
    validate_extra_data_field_decryption,
    validate_extra_data_field_input_proof,
    validate_handle_contract_pairs,
    validate_handle_entries,
    validate_host_address,
    validate_host_addresses,
    validate_no_0x_hex,
    validate_request_validity,
    validate_request_validity_seconds,
    validate_timestamp,
    validate_u32_string,
    validate_u64_string,
    validate_user_decrypt_signature,
    // v3 unified EIP-712 validators
    validate_v3_attestation_type,
    validate_v3_payload_type,
    validate_v3_version,
    // Validation messages
    validation_messages,
    ApiError,
    // Core response types
    AppResponse,
    ErrorDetail,
    ErrorLabel,
    ErrorResponse,
    ParseError,
    ValidatedJson,
};
