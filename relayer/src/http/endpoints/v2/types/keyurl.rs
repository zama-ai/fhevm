use crate::config::settings::deserialize_vec_from_map_or_seq;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// CRS parameter size used as the map key in the `/v2/keyurl` response.
///
/// The on-chain CRS material does not carry its parameter size, so this is fixed
/// to the single supported size. Kept as a constant to preserve the response shape
/// expected by the relayer-sdk (`crs."2048"`).
pub const CRS_PARAM_SIZE_KEY: &str = "2048";

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyUrlResponseJson {
    #[schema(example = "succeeded")]
    pub status: String,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub fhe_key_info: Vec<FheKeyInfo>,
    pub crs: HashMap<String, KeyData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FheKeyInfo {
    pub fhe_public_key: KeyData,
}

// Also the config type behind `keyurl.source: config`, so static values pass straight through
// to the response. Plain `//`: doc comments here land in openapi.yml.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    /// On-chain key/CRS identifier, as a `0x`-prefixed 32-byte big-endian hex string.
    // Config and env keys are snake_case; the response is camelCase.
    #[serde(alias = "data_id")]
    #[schema(example = "0x0400000000000000000000000000000000000000000000000000000000000003")]
    pub data_id: String,
    /// Storage URLs for the key/CRS material.
    // Env config delivers `URLS__0` as an index-keyed map, not a list.
    #[serde(deserialize_with = "deserialize_vec_from_map_or_seq")]
    pub urls: Vec<String>,
}

impl KeyUrlResponseJson {
    /// Build a `/v2/keyurl` response from a key and a CRS, whatever their source.
    pub fn new(fhe_public_key: KeyData, crs: KeyData) -> Self {
        let mut crs_map = HashMap::new();
        crs_map.insert(CRS_PARAM_SIZE_KEY.to_string(), crs);

        KeyUrlResponseJson {
            status: "succeeded".to_string(),
            response: Response {
                fhe_key_info: vec![FheKeyInfo { fhe_public_key }],
                crs: crs_map,
            },
        }
    }
}
