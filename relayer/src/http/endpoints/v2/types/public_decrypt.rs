use super::error::{ApiResponseStatus, V2ErrorResponseBody};
use crate::core::event::is_solana_host_chain_id;
use crate::host::handle_chain_id::extract_chain_id_from_handle;
use crate::http::utils::redact::{redact_count, redact_len};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};
use zama_solana_request::MAX_REQUEST_HANDLES;

#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptRequestJson {
    /// Ciphertext handles to decrypt. Each is `0x` + 64 hex chars, obtained from an on-chain FHE operation.
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_0x_hexs")
    )]
    #[schema(min_items = 1, example = json!(["0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"]))]
    pub ciphertext_handles: Vec<String>,
    #[schema(schema_with = crate::http::extra_data_decryption_schema)]
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    pub extra_data: String,
    /// Solana handles only: the encrypted store that holds each handle, in handle order.
    /// Each is `0x` + 64 hex chars. Omit for EVM handles. A Solana request carries at most 32 handles.
    #[serde(default)]
    #[validate(custom(function = "crate::http::validate_0x_hexs"))]
    #[schema(example = json!(["0x5f2a1c3b4d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708"]))]
    pub encrypted_stores: Option<Vec<String>>,
}

impl PublicDecryptRequestJson {
    /// Solana handles name one 32-byte store each, and EVM handles name none. A request that
    /// mixes the two has no Gateway entry to go to.
    pub fn validate_encrypted_stores(&self, errors: &mut ValidationErrors) {
        let handles = &self.ciphertext_handles;
        let solana_handles = handles
            .iter()
            .filter_map(|handle| parse_handle(handle))
            .filter(|handle| is_solana_host_chain_id(extract_chain_id_from_handle(handle)))
            .count();
        let message = match &self.encrypted_stores {
            None if solana_handles == 0 => return,
            None => "Required for Solana handles".to_string(),
            Some(_) if solana_handles < handles.len() => {
                "Only accepted when every handle is a Solana handle".to_string()
            }
            Some(stores) if stores.len() != handles.len() => format!(
                "Must name one store per handle: {} stores for {} handles",
                stores.len(),
                handles.len()
            ),
            Some(_) if handles.len() > MAX_REQUEST_HANDLES => format!(
                "At most {MAX_REQUEST_HANDLES} Solana handles: got {}",
                handles.len()
            ),
            Some(stores) if stores.iter().any(|store| store.len() != 66) => {
                "Each store must be 0x + 64 hex chars".to_string()
            }
            Some(_) => return,
        };
        errors.add(
            "encrypted_stores",
            ValidationError::new("validation_error").with_message(message.into()),
        );
    }
}

fn parse_handle(handle: &str) -> Option<[u8; 32]> {
    hex::decode(handle.strip_prefix("0x")?)
        .ok()?
        .try_into()
        .ok()
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptPostResponseJson {
    #[schema(value_type = String, example = "queued")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: PublicDecryptQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptQueuedResult {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub job_id: String,
}

// GET response when completed
#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptResponseJson {
    /// Decrypted plaintext value. Raw hex, no `0x` prefix.
    #[schema(value_type = String, example = "00000000000000000000000000000001")]
    #[derivative(Debug(format_with = "redact_len"))]
    pub decrypted_value: String,
    /// Gateway signatures over the decrypted value. Raw hex, no `0x` prefix.
    #[schema(value_type = Vec<String>, example = json!(["1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d"]))]
    #[derivative(Debug(format_with = "redact_count"))]
    pub signatures: Vec<String>,
    /// Extra data echoed back from the gateway contract. `0x`-prefixed hex.
    #[schema(value_type = String, example = "0x00")]
    pub extra_data: String,
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptStatusResponseJson {
    #[schema(example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PublicDecryptResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<V2ErrorResponseBody>,
}

/// GET 200 — public decryption succeeded (has result, no error).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptSucceededStatusResponse {
    #[schema(value_type = String, example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: PublicDecryptResponseJson,
}

// Standard serialization implementations for v2 API types
impl From<crate::core::event::PublicDecryptResponse> for PublicDecryptResponseJson {
    fn from(response: crate::core::event::PublicDecryptResponse) -> Self {
        let signatures: Vec<String> = response.signatures.iter().map(hex::encode).collect();

        PublicDecryptResponseJson {
            decrypted_value: hex::encode(&response.decrypted_value),
            signatures,
            extra_data: response.extra_data, // Already a string
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::solana_host_chain_id;

    fn request(
        handles: &[[u8; 32]],
        encrypted_stores: Option<Vec<String>>,
    ) -> PublicDecryptRequestJson {
        PublicDecryptRequestJson {
            ciphertext_handles: handles
                .iter()
                .map(|handle| format!("0x{}", hex::encode(handle)))
                .collect(),
            extra_data: "0x00".to_string(),
            encrypted_stores,
        }
    }

    fn handle_on(chain_id: u64, tag: u8) -> [u8; 32] {
        let mut handle = [tag; 32];
        handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
        handle
    }

    fn store(byte: u8) -> String {
        format!("0x{}", hex::encode([byte; 32]))
    }

    fn store_error(request: &PublicDecryptRequestJson) -> Option<String> {
        let mut errors = ValidationErrors::new();
        request.validate_encrypted_stores(&mut errors);
        let errors = errors.field_errors();
        let error = errors.get("encrypted_stores")?.first()?;
        Some(error.message.as_ref()?.to_string())
    }

    #[test]
    fn solana_handles_with_one_store_each_and_evm_handles_without_stores_pass() {
        let solana = solana_host_chain_id(1);
        let solana_request = request(
            &[handle_on(solana, 0x11), handle_on(solana, 0x22)],
            Some(vec![store(0xaa), store(0xbb)]),
        );
        let evm_request = request(&[handle_on(8009, 0x11)], None);

        assert_eq!(store_error(&solana_request), None);
        assert_eq!(store_error(&evm_request), None);
    }

    #[test]
    fn a_solana_handle_without_stores_is_refused() {
        let request = request(&[handle_on(solana_host_chain_id(1), 0x11)], None);

        assert_eq!(
            store_error(&request).as_deref(),
            Some("Required for Solana handles")
        );
    }

    #[test]
    fn a_store_count_that_differs_from_the_handle_count_is_refused() {
        let solana = solana_host_chain_id(1);
        let request = request(
            &[handle_on(solana, 0x11), handle_on(solana, 0x22)],
            Some(vec![store(0xaa)]),
        );

        let error = store_error(&request).expect("refused");

        assert!(error.contains("1 stores for 2 handles"), "got: {error}");
    }

    #[test]
    fn stores_are_refused_unless_every_handle_is_on_solana() {
        let evm_only = request(&[handle_on(8009, 0x11)], Some(vec![store(0xaa)]));
        let mixed = request(
            &[
                handle_on(solana_host_chain_id(1), 0x11),
                handle_on(8009, 0x22),
            ],
            Some(vec![store(0xaa), store(0xbb)]),
        );

        for request in [evm_only, mixed] {
            let error = store_error(&request).expect("refused");
            assert!(error.contains("every handle"), "got: {error}");
        }
    }

    #[test]
    fn more_solana_handles_than_the_gateway_accepts_are_refused() {
        let solana = solana_host_chain_id(1);
        let handles: Vec<_> = (0..=MAX_REQUEST_HANDLES as u8)
            .map(|tag| handle_on(solana, tag))
            .collect();
        let stores = handles.iter().map(|_| store(0xaa)).collect();

        let error = store_error(&request(&handles, Some(stores))).expect("refused");

        assert!(error.contains("At most 32 Solana handles"), "got: {error}");
    }

    #[test]
    fn a_store_that_is_not_32_bytes_is_refused() {
        let request = request(
            &[handle_on(solana_host_chain_id(1), 0x11)],
            Some(vec![format!("0x{}", "aa".repeat(31))]),
        );

        let error = store_error(&request).expect("refused");

        assert!(error.contains("64 hex chars"), "got: {error}");
    }
}
