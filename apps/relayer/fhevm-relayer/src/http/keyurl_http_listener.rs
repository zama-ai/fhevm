use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUrlResponseJson {
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub fhe_key_info: Vec<FheKeyInfo>,
    pub crs: HashMap<String, KeyData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FheKeyInfo {
    pub fhe_public_key: KeyData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyData {
    pub data_id: String,
    pub urls: Vec<String>,
}

impl KeyUrlResponseJson {
    pub fn default() -> KeyUrlResponseJson {
        KeyUrlResponseJson {
            response: Response {
                fhe_key_info: vec![FheKeyInfo {
                    fhe_public_key: KeyData {
                        data_id: "string".to_string(),
                        urls: vec!["string".to_string()],
                    },
                }],
                crs: {
                    let mut map = std::collections::HashMap::new();
                    map.insert(
                        "2048".to_string(),
                        KeyData {
                            data_id: "string".to_string(),
                            urls: vec!["string".to_string()],
                        },
                    );
                    map
                },
            },
        }
    }
}
