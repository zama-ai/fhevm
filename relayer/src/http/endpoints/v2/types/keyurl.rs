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
    /// Active KMS context id, from on-chain `getCurrentKmsContextAndEpoch`.
    #[schema(example = "1")]
    pub context_id: String,
    /// Active epoch id, from on-chain `getCurrentKmsContextAndEpoch`.
    #[schema(example = "1")]
    pub epoch_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FheKeyInfo {
    pub fhe_public_key: KeyData,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    /// On-chain `getActiveKeyId` / `getActiveCrsId` as a decimal string.
    #[schema(example = "3")]
    pub data_id: String,
    /// Storage URLs from `getKeyMaterials` / `getCrsMaterials`.
    pub urls: Vec<String>,
}

impl KeyUrlResponseJson {
    /// Build a chain-sourced `/v2/keyurl` response from the values the host-chain
    /// poller read on-chain.
    pub fn new(
        fhe_public_key: KeyData,
        crs: KeyData,
        context_id: String,
        epoch_id: String,
    ) -> Self {
        let mut crs_map = HashMap::new();
        crs_map.insert(CRS_PARAM_SIZE_KEY.to_string(), crs);

        KeyUrlResponseJson {
            status: "succeeded".to_string(),
            response: Response {
                fhe_key_info: vec![FheKeyInfo { fhe_public_key }],
                crs: crs_map,
                context_id,
                epoch_id,
            },
        }
    }
}
