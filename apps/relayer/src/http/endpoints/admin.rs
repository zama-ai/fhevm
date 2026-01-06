use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Serialize)]
pub struct UpdateConfigResponse {
    pub name: String,
    pub value: u32,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Admin endpoint to update configuration values dynamically
///
/// Currently supported configurations:
/// - `tx_throttler_per_secs`: Transaction throttler rate limit (TPS)
///   - Must be > 0 and <= 1000
pub async fn update_config(
    Extension(throttler_control_tx): Extension<Option<mpsc::Sender<u32>>>,
    Json(payload): Json<UpdateConfigRequest>,
) -> Response {
    let Some(control_tx) = throttler_control_tx else {
        warn!("Admin endpoint called but admin endpoints are disabled");
        return (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Admin endpoints are not enabled".to_string(),
            }),
        )
            .into_response();
    };

    match payload.name.as_str() {
        "tx_throttler_per_secs" => {
            // Validate TPS bounds
            if payload.value == 0 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "tx_throttler_per_secs must be greater than 0".to_string(),
                    }),
                )
                    .into_response();
            }

            if payload.value > 1000 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "tx_throttler_per_secs must be less than or equal to 1000"
                            .to_string(),
                    }),
                )
                    .into_response();
            }

            match control_tx.try_send(payload.value) {
                Ok(_) => {
                    info!(
                        config_name = %payload.name,
                        new_value = %payload.value,
                        "ADMIN_CONFIG_UPDATE: Configuration updated via admin endpoint"
                    );
                    (
                        StatusCode::OK,
                        Json(UpdateConfigResponse {
                            name: payload.name,
                            value: payload.value,
                            message: "Configuration updated successfully".to_string(),
                        }),
                    )
                        .into_response()
                }
                Err(mpsc::error::TrySendError::Full(_)) => {
                    warn!("Throttler control channel full, config update rejected");
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(ErrorResponse {
                            error: "Throttler is busy, please retry".to_string(),
                        }),
                    )
                        .into_response()
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    warn!("Throttler worker unavailable, config update failed");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Throttler service unavailable".to_string(),
                        }),
                    )
                        .into_response()
                }
            }
        }
        _ => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Unknown configuration parameter: {}", payload.name),
            }),
        )
            .into_response(),
    }
}
