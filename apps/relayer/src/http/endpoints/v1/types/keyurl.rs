use crate::config::settings::KeyUrl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct KeyUrlResponseJson {
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Response {
    pub fhe_key_info: Vec<FheKeyInfo>,
    pub crs: HashMap<String, KeyData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct FheKeyInfo {
    pub fhe_public_key: KeyData,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct KeyData {
    pub data_id: String,
    pub urls: Vec<String>,
}

impl From<KeyUrl> for KeyUrlResponseJson {
    fn from(value: KeyUrl) -> Self {
        KeyUrlResponseJson {
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
