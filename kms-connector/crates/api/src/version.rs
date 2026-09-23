//! Interface version and route constants, shared so endpoint and relayer cannot drift on
//! paths, and so `/version` reports a value defined next to the shapes it describes.

use serde::{Deserialize, Serialize};

/// The Relayer ↔ Connector interface version.
pub const INTERFACE_VERSION: &str = "v1";

pub const PUBLIC_DECRYPTION_ROUTE: &str = "/v1/public-decrypt";
pub const USER_DECRYPTION_ROUTE: &str = "/v1/user-decrypt";
pub const VERSION_ROUTE: &str = "/version";

/// `200` body of `/version` endpoint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
    pub version: String,
}

impl Default for VersionResponse {
    fn default() -> Self {
        Self {
            version: INTERFACE_VERSION.to_string(),
        }
    }
}
