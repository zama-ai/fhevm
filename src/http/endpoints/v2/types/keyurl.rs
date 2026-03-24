use crate::config::settings::KeyUrl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

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

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    #[schema(example = "fhe_public_key_v1")]
    pub data_id: String,
    #[schema(example = json!(["https://keys.fhevm.io/v1/fhe_public_key_v1"]))]
    pub urls: Vec<String>,
}

impl From<KeyUrl> for KeyUrlResponseJson {
    fn from(value: KeyUrl) -> Self {
        KeyUrlResponseJson {
            status: "succeeded".to_string(),
            response: Response {
                fhe_key_info: vec![FheKeyInfo {
                    fhe_public_key: KeyData {
                        data_id: value.fhe_public_key.data_id,
                        urls: vec![value.fhe_public_key.url],
                    },
                }],
                crs: {
                    let mut map = std::collections::HashMap::new();
                    map.insert(
                        "2048".to_string(),
                        KeyData {
                            data_id: value.crs.data_id,
                            urls: vec![value.crs.url],
                        },
                    );
                    map
                },
            },
        }
    }
}

impl From<crate::core::event::KeyUrlData> for KeyUrlResponseJson {
    fn from(value: crate::core::event::KeyUrlData) -> Self {
        KeyUrlResponseJson {
            status: "succeeded".to_string(),
            response: Response {
                fhe_key_info: vec![FheKeyInfo {
                    fhe_public_key: KeyData {
                        data_id: value.fhe_public_key.data_id,
                        urls: vec![value.fhe_public_key.url],
                    },
                }],
                crs: {
                    let mut map = std::collections::HashMap::new();
                    map.insert(
                        "2048".to_string(),
                        KeyData {
                            data_id: value.crs.data_id,
                            urls: vec![value.crs.url],
                        },
                    );
                    map
                },
            },
        }
    }
}
