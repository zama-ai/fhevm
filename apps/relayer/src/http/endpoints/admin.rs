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

pub fn update_tx_throttler_tps(
    control_tx: mpsc::Sender<u32>,
    payload: UpdateConfigRequest,
) -> Response {
    if payload.value == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: (payload.name.to_string() + " must be greater than 0").to_string(),
            }),
        )
            .into_response();
    }

    if payload.value > 1000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: (payload.name.to_string() + " must be less than or equal to 1000")
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

/// Admin endpoint to update configuration values dynamically
///
/// Currently supported configurations:
/// - `input_proof_throttler_tps`: Input Proof Transaction throttler rate limit (TPS)
/// - `user_decrypt_throttler_tps`: User Decrypt Transaction throttler rate limit (TPS)
/// - `public_decrypt_throttler_tps`: Public Decrypt Transaction throttler rate limit (TPS)
///   - Must be > 0 and <= 1000
pub async fn update_config(
    Extension(input_proof_throttler_control_tx): Extension<Option<mpsc::Sender<u32>>>,
    Extension(user_decrypt_throttler_control_tx): Extension<Option<mpsc::Sender<u32>>>,
    Extension(public_decrypt_throttler_control_tx): Extension<Option<mpsc::Sender<u32>>>,
    Json(payload): Json<UpdateConfigRequest>,
) -> Response {
    let Some(input_proof_control_tx) = input_proof_throttler_control_tx else {
        warn!("Admin endpoint called but admin endpoints are disabled");
        return (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Admin endpoints are not enabled".to_string(),
            }),
        )
            .into_response();
    };

    let Some(user_decrypt_control_tx) = user_decrypt_throttler_control_tx else {
        warn!("Admin endpoint called but admin endpoints are disabled");
        return (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Admin endpoints are not enabled".to_string(),
            }),
        )
            .into_response();
    };

    let Some(public_decrypt_control_tx) = public_decrypt_throttler_control_tx else {
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
        "input_proof_throttler_tps" => update_tx_throttler_tps(input_proof_control_tx, payload),
        "user_decrypt_throttler_tps" => update_tx_throttler_tps(user_decrypt_control_tx, payload),
        "public_decrypt_throttler_tps" => {
            update_tx_throttler_tps(public_decrypt_control_tx, payload)
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
