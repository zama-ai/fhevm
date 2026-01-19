//! Retry-after state management and ETA computation.

use tokio::sync::RwLock;

use crate::config::retry_after::{BackoffInterval, RetryAfterConfig};
use crate::http::retry_after::queue_info::{
    DecryptQueueInfo, ReadinessQueueInfo, RequestQueueInfo, TxQueueInfo,
};
use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Request state info for GET request ETA computation.
#[derive(Debug, Clone, Copy)]
pub struct RequestStateInfo {
    pub status: ReqStatus,
    pub elapsed_since_created_secs: u32,
    pub elapsed_in_current_state_secs: u32,
}

impl RequestStateInfo {
    pub fn new(
        status: ReqStatus,
        elapsed_since_created_secs: u32,
        elapsed_in_current_state_secs: u32,
    ) -> Self {
        Self {
            status,
            elapsed_since_created_secs,
            elapsed_in_current_state_secs,
        }
    }
}

/// Retry-after state holding all config values directly.
/// Initialized from config, updatable via admin API.
#[derive(Debug)]
pub struct RetryAfterState {
    min_seconds: RwLock<u32>,
    max_seconds: RwLock<u32>,
    safety_margin: RwLock<f32>,
    nominal_readiness_ms: RwLock<u32>,
    nominal_input_proof_ms: RwLock<u32>,
    nominal_user_decrypt_ms: RwLock<u32>,
    nominal_public_decrypt_ms: RwLock<u32>,
    nominal_tx_ms: RwLock<u32>,
    copro_kms_backoff_intervals: RwLock<Vec<BackoffInterval>>,
}

impl RetryAfterState {
    /// Create from config. All values required.
    pub fn new(config: &RetryAfterConfig) -> Self {
        Self {
            min_seconds: RwLock::new(config.min_seconds),
            max_seconds: RwLock::new(config.max_seconds),
            safety_margin: RwLock::new(config.safety_margin),
            nominal_readiness_ms: RwLock::new(config.nominal_times.readiness_check_seconds * 1000),
            nominal_input_proof_ms: RwLock::new(
                config.nominal_times.input_proof_processing_seconds * 1000,
            ),
            nominal_user_decrypt_ms: RwLock::new(
                config.nominal_times.user_decrypt_processing_seconds * 1000,
            ),
            nominal_public_decrypt_ms: RwLock::new(
                config.nominal_times.public_decrypt_processing_seconds * 1000,
            ),
            nominal_tx_ms: RwLock::new(config.nominal_times.tx_confirmation_ms),
            copro_kms_backoff_intervals: RwLock::new(config.copro_kms_backoff_intervals.clone()),
        }
    }

    // ========== Getters (direct access, no Option) ==========

    pub async fn min_seconds(&self) -> u32 {
        *self.min_seconds.read().await
    }
    pub async fn max_seconds(&self) -> u32 {
        *self.max_seconds.read().await
    }
    pub async fn safety_margin(&self) -> f32 {
        *self.safety_margin.read().await
    }
    pub async fn nominal_readiness_ms(&self) -> u32 {
        *self.nominal_readiness_ms.read().await
    }
    pub async fn nominal_input_proof_ms(&self) -> u32 {
        *self.nominal_input_proof_ms.read().await
    }
    pub async fn nominal_user_decrypt_ms(&self) -> u32 {
        *self.nominal_user_decrypt_ms.read().await
    }
    pub async fn nominal_public_decrypt_ms(&self) -> u32 {
        *self.nominal_public_decrypt_ms.read().await
    }
    pub async fn nominal_tx_ms(&self) -> u32 {
        *self.nominal_tx_ms.read().await
    }
    pub async fn get_backoff_intervals(&self) -> Vec<BackoffInterval> {
        self.copro_kms_backoff_intervals.read().await.clone()
    }

    // ========== Setters (for admin API) ==========

    pub async fn set_min_seconds(&self, val: u32) {
        *self.min_seconds.write().await = val;
    }
    pub async fn set_max_seconds(&self, val: u32) {
        *self.max_seconds.write().await = val;
    }
    pub async fn set_safety_margin(&self, val: f32) {
        *self.safety_margin.write().await = val;
    }
    pub async fn set_nominal_readiness_seconds(&self, val: u32) {
        *self.nominal_readiness_ms.write().await = val * 1000;
    }
    pub async fn set_nominal_input_proof_seconds(&self, val: u32) {
        *self.nominal_input_proof_ms.write().await = val * 1000;
    }
    pub async fn set_nominal_user_decrypt_seconds(&self, val: u32) {
        *self.nominal_user_decrypt_ms.write().await = val * 1000;
    }
    pub async fn set_nominal_public_decrypt_seconds(&self, val: u32) {
        *self.nominal_public_decrypt_ms.write().await = val * 1000;
    }
    pub async fn set_nominal_tx_ms(&self, val: u32) {
        *self.nominal_tx_ms.write().await = val;
    }
    pub async fn set_backoff_intervals(&self, intervals: Vec<BackoffInterval>) {
        *self.copro_kms_backoff_intervals.write().await = intervals;
    }

    // ========== ETA Computation ==========

    /// Compute ETA for a queued request based on queue info.
    pub async fn compute_queued_eta(&self, queue_info: &RequestQueueInfo) -> u32 {
        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let safety_margin = self.safety_margin().await;
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_readiness_ms = self.nominal_readiness_ms().await;

        let raw_eta_ms = match queue_info {
            RequestQueueInfo::InputProof(tx_info) => {
                let nominal_processing_ms = self.nominal_input_proof_ms().await;
                compute_tx_queue_wait_ms(tx_info) + nominal_processing_ms + nominal_tx_ms
            }
            RequestQueueInfo::UserDecrypt(info) => {
                let nominal_processing_ms = self.nominal_user_decrypt_ms().await;
                compute_decrypt_eta_ms(
                    info,
                    nominal_processing_ms,
                    nominal_readiness_ms,
                    nominal_tx_ms,
                )
            }
            RequestQueueInfo::PublicDecrypt(info) => {
                let nominal_processing_ms = self.nominal_public_decrypt_ms().await;
                compute_decrypt_eta_ms(
                    info,
                    nominal_processing_ms,
                    nominal_readiness_ms,
                    nominal_tx_ms,
                )
            }
        };

        let with_margin_ms = apply_safety_margin_ms(raw_eta_ms, safety_margin);
        with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
    }

    /// Compute retry-after for GET (polling existing request).
    pub async fn compute_for_get(
        &self,
        state_info: &RequestStateInfo,
        queue_info: Option<&RequestQueueInfo>,
    ) -> u32 {
        use ReqStatus::*;

        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let safety_margin = self.safety_margin().await;
        let nominal_tx_ms = self.nominal_tx_ms().await;

        let elapsed_ms = state_info.elapsed_in_current_state_secs * 1000;

        match state_info.status {
            Queued => {
                if let Some(q_info) = queue_info {
                    self.compute_queued_eta(q_info).await
                } else {
                    let raw_ms = self.get_default_processing_ms(queue_info).await + nominal_tx_ms;
                    let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                    with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
                }
            }
            Processing => {
                let processing_ms = self.get_default_processing_ms(queue_info).await;
                let raw_ms = (processing_ms + nominal_tx_ms).saturating_sub(elapsed_ms);
                let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
            }
            TxInFlight => {
                let raw_ms = nominal_tx_ms.saturating_sub(elapsed_ms);
                let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
            }
            ReceiptReceived => {
                self.compute_copro_kms_backoff(state_info.elapsed_in_current_state_secs)
                    .await
            }
            Completed | TimedOut | Failure => 0,
        }
    }

    /// Compute retry-after for input proof POST.
    pub async fn compute_for_input_proof_post(&self, tx_info: &TxQueueInfo) -> u32 {
        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let safety_margin = self.safety_margin().await;
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_processing_ms = self.nominal_input_proof_ms().await;

        let raw_eta_ms = compute_tx_queue_wait_ms(tx_info) + nominal_processing_ms + nominal_tx_ms;
        let with_margin_ms = apply_safety_margin_ms(raw_eta_ms, safety_margin);
        with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
    }

    /// Compute retry-after for decrypt POST (user or public).
    pub async fn compute_for_decrypt_post(
        &self,
        info: &DecryptQueueInfo,
        is_user_decrypt: bool,
    ) -> u32 {
        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let safety_margin = self.safety_margin().await;
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_readiness_ms = self.nominal_readiness_ms().await;
        let nominal_processing_ms = if is_user_decrypt {
            self.nominal_user_decrypt_ms().await
        } else {
            self.nominal_public_decrypt_ms().await
        };

        let raw_eta_ms = compute_decrypt_eta_ms(
            info,
            nominal_processing_ms,
            nominal_readiness_ms,
            nominal_tx_ms,
        );
        let with_margin_ms = apply_safety_margin_ms(raw_eta_ms, safety_margin);
        with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
    }

    /// Compute retry-after for input proof GET.
    pub async fn compute_for_input_proof_get(
        &self,
        tx_info: Option<&TxQueueInfo>,
        state_info: &RequestStateInfo,
    ) -> u32 {
        use ReqStatus::*;

        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let safety_margin = self.safety_margin().await;
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_processing_ms = self.nominal_input_proof_ms().await;
        let elapsed_ms = state_info.elapsed_in_current_state_secs * 1000;

        match state_info.status {
            Queued => {
                if let Some(info) = tx_info {
                    self.compute_for_input_proof_post(info).await
                } else {
                    let raw_ms = nominal_processing_ms + nominal_tx_ms;
                    let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                    with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
                }
            }
            Processing => {
                let raw_ms = (nominal_processing_ms + nominal_tx_ms).saturating_sub(elapsed_ms);
                let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
            }
            TxInFlight => {
                let raw_ms = nominal_tx_ms.saturating_sub(elapsed_ms);
                let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
            }
            ReceiptReceived => {
                self.compute_copro_kms_backoff(state_info.elapsed_in_current_state_secs)
                    .await
            }
            Completed | TimedOut | Failure => 0,
        }
    }

    /// Compute retry-after for decrypt GET (user or public).
    pub async fn compute_for_decrypt_get(
        &self,
        info: Option<&DecryptQueueInfo>,
        state_info: &RequestStateInfo,
        is_user_decrypt: bool,
    ) -> u32 {
        use ReqStatus::*;

        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let safety_margin = self.safety_margin().await;
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_processing_ms = if is_user_decrypt {
            self.nominal_user_decrypt_ms().await
        } else {
            self.nominal_public_decrypt_ms().await
        };
        let elapsed_ms = state_info.elapsed_in_current_state_secs * 1000;

        match state_info.status {
            Queued => {
                if let Some(decrypt_info) = info {
                    self.compute_for_decrypt_post(decrypt_info, is_user_decrypt)
                        .await
                } else {
                    let raw_ms = nominal_processing_ms + nominal_tx_ms;
                    let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                    with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
                }
            }
            Processing => {
                let raw_ms = (nominal_processing_ms + nominal_tx_ms).saturating_sub(elapsed_ms);
                let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
            }
            TxInFlight => {
                let raw_ms = nominal_tx_ms.saturating_sub(elapsed_ms);
                let with_margin_ms = apply_safety_margin_ms(raw_ms, safety_margin);
                with_margin_ms.div_ceil(1000).clamp(min_secs, max_secs)
            }
            ReceiptReceived => {
                self.compute_copro_kms_backoff(state_info.elapsed_in_current_state_secs)
                    .await
            }
            Completed | TimedOut | Failure => 0,
        }
    }

    // ========== Raw ETA Computation (for metrics) ==========

    /// Compute raw ETA in ms (before margin/clamping) for input proof POST.
    /// This is useful for histogram metrics to track actual estimated times.
    pub async fn compute_raw_eta_ms_for_input_proof(&self, tx_info: &TxQueueInfo) -> u32 {
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_processing_ms = self.nominal_input_proof_ms().await;
        compute_tx_queue_wait_ms(tx_info) + nominal_processing_ms + nominal_tx_ms
    }

    /// Compute raw ETA in ms (before margin/clamping) for decrypt POST.
    /// This is useful for histogram metrics to track actual estimated times.
    pub async fn compute_raw_eta_ms_for_decrypt(
        &self,
        info: &DecryptQueueInfo,
        is_user_decrypt: bool,
    ) -> u32 {
        let nominal_tx_ms = self.nominal_tx_ms().await;
        let nominal_readiness_ms = self.nominal_readiness_ms().await;
        let nominal_processing_ms = if is_user_decrypt {
            self.nominal_user_decrypt_ms().await
        } else {
            self.nominal_public_decrypt_ms().await
        };

        compute_decrypt_eta_ms(
            info,
            nominal_processing_ms,
            nominal_readiness_ms,
            nominal_tx_ms,
        )
    }

    // ========== Internal helpers ==========

    async fn get_default_processing_ms(&self, queue_info: Option<&RequestQueueInfo>) -> u32 {
        match queue_info {
            Some(RequestQueueInfo::InputProof(_)) => self.nominal_input_proof_ms().await,
            Some(RequestQueueInfo::UserDecrypt(_)) => self.nominal_user_decrypt_ms().await,
            Some(RequestQueueInfo::PublicDecrypt(_)) => self.nominal_public_decrypt_ms().await,
            None => self.nominal_input_proof_ms().await, // Default fallback
        }
    }

    async fn compute_copro_kms_backoff(&self, elapsed_secs: u32) -> u32 {
        let min_secs = self.min_seconds().await;
        let max_secs = self.max_seconds().await;
        let intervals = self.copro_kms_backoff_intervals.read().await;

        let mut result = intervals
            .first()
            .map(|i| i.retry_interval_secs)
            .unwrap_or(min_secs);

        for interval in intervals.iter() {
            if elapsed_secs >= interval.elapsed_threshold_secs {
                result = interval.retry_interval_secs;
            } else {
                break;
            }
        }

        result.clamp(min_secs, max_secs)
    }
}

// ========== Free functions ==========

fn apply_safety_margin_ms(eta_ms: u32, safety_margin: f32) -> u32 {
    let result = (eta_ms as f64) * (1.0 + safety_margin as f64);
    let ceiled = result.ceil();
    if ceiled > u32::MAX as f64 {
        u32::MAX
    } else {
        ceiled as u32
    }
}

/// Compute TX queue wait time in ms.
pub fn compute_tx_queue_wait_ms(tx_info: &TxQueueInfo) -> u32 {
    if tx_info.drain_rate_tps > 0 {
        let result = ((tx_info.size as f64 / tx_info.drain_rate_tps as f64) * 1000.0).ceil();
        if result > u32::MAX as f64 {
            u32::MAX
        } else {
            result as u32
        }
    } else {
        300_000
    }
}

/// Compute readiness queue wait time in ms.
pub fn compute_readiness_queue_wait_ms(
    info: &ReadinessQueueInfo,
    nominal_readiness_ms: u32,
) -> u32 {
    if info.max_concurrency > 0 {
        let batches = (info.size as f64 / info.max_concurrency as f64).ceil();
        let result = batches * nominal_readiness_ms as f64;
        if result > u32::MAX as f64 {
            u32::MAX
        } else {
            result as u32
        }
    } else {
        300_000
    }
}

fn compute_decrypt_eta_ms(
    info: &DecryptQueueInfo,
    nominal_processing_ms: u32,
    nominal_readiness_ms: u32,
    nominal_tx_ms: u32,
) -> u32 {
    let readiness_wait = compute_readiness_queue_wait_ms(&info.readiness, nominal_readiness_ms);
    let tx_wait = compute_tx_queue_wait_ms(&info.tx);
    readiness_wait + tx_wait + nominal_readiness_ms + nominal_processing_ms + nominal_tx_ms
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::retry_after::{NominalProcessingTimes, RetryAfterConfig};

    fn test_config() -> RetryAfterConfig {
        RetryAfterConfig {
            min_seconds: 1,
            max_seconds: 300,
            safety_margin: 0.2,
            nominal_times: NominalProcessingTimes {
                readiness_check_seconds: 4,
                input_proof_processing_seconds: 2,
                user_decrypt_processing_seconds: 6,
                public_decrypt_processing_seconds: 6,
                tx_confirmation_ms: 250,
            },
            copro_kms_backoff_intervals: vec![
                BackoffInterval {
                    elapsed_threshold_secs: 0,
                    retry_interval_secs: 4,
                },
                BackoffInterval {
                    elapsed_threshold_secs: 60,
                    retry_interval_secs: 10,
                },
                BackoffInterval {
                    elapsed_threshold_secs: 120,
                    retry_interval_secs: 30,
                },
            ],
        }
    }

    #[tokio::test]
    async fn test_new_from_config() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        assert_eq!(state.min_seconds().await, 1);
        assert_eq!(state.max_seconds().await, 300);
        assert!((state.safety_margin().await - 0.2).abs() < f32::EPSILON);
        assert_eq!(state.nominal_readiness_ms().await, 4000);
        assert_eq!(state.nominal_input_proof_ms().await, 2000);
        assert_eq!(state.nominal_user_decrypt_ms().await, 6000);
        assert_eq!(state.nominal_public_decrypt_ms().await, 6000);
        assert_eq!(state.nominal_tx_ms().await, 250);
    }

    #[tokio::test]
    async fn test_setters() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        state.set_min_seconds(5).await;
        assert_eq!(state.min_seconds().await, 5);

        state.set_safety_margin(0.5).await;
        assert!((state.safety_margin().await - 0.5).abs() < f32::EPSILON);
    }

    #[tokio::test]
    async fn test_compute_for_input_proof_post() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        let tx_info = TxQueueInfo {
            size: 100,
            drain_rate_tps: 20,
        };
        // queue_wait = 100/20 * 1000 = 5000ms
        // processing = 2000ms, tx = 250ms
        // raw_eta = 7250ms, with margin (0.2) = 8700ms
        // result = ceil(8700/1000) = 9s
        let result = state.compute_for_input_proof_post(&tx_info).await;
        assert_eq!(result, 9);
    }

    #[tokio::test]
    async fn test_compute_queued_eta_input_proof() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        let tx_info = TxQueueInfo {
            size: 100,
            drain_rate_tps: 20,
        };
        let queue_info = RequestQueueInfo::InputProof(tx_info);
        let result = state.compute_queued_eta(&queue_info).await;
        assert_eq!(result, 9);
    }

    #[tokio::test]
    async fn test_compute_queued_eta_clamped_to_min() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        let tx_info = TxQueueInfo {
            size: 0,
            drain_rate_tps: 100,
        };
        let queue_info = RequestQueueInfo::InputProof(tx_info);
        let result = state.compute_queued_eta(&queue_info).await;
        assert!(result >= 1); // min_seconds
    }

    #[tokio::test]
    async fn test_compute_queued_eta_clamped_to_max() {
        let mut config = test_config();
        config.max_seconds = 10;
        let state = RetryAfterState::new(&config);

        let tx_info = TxQueueInfo {
            size: 10000,
            drain_rate_tps: 1,
        };
        let queue_info = RequestQueueInfo::InputProof(tx_info);
        let result = state.compute_queued_eta(&queue_info).await;
        assert_eq!(result, 10); // max_seconds
    }

    #[tokio::test]
    async fn test_compute_tx_queue_wait_ms() {
        let info = TxQueueInfo {
            size: 100,
            drain_rate_tps: 20,
        };
        assert_eq!(compute_tx_queue_wait_ms(&info), 5000);

        let empty = TxQueueInfo {
            size: 0,
            drain_rate_tps: 20,
        };
        assert_eq!(compute_tx_queue_wait_ms(&empty), 0);

        let zero_tps = TxQueueInfo {
            size: 100,
            drain_rate_tps: 0,
        };
        assert_eq!(compute_tx_queue_wait_ms(&zero_tps), 300_000);
    }

    #[tokio::test]
    async fn test_compute_readiness_queue_wait_ms() {
        let info = ReadinessQueueInfo {
            size: 500,
            max_concurrency: 250,
        };
        // batches = ceil(500/250) = 2, wait = 2 * 4000 = 8000ms
        assert_eq!(compute_readiness_queue_wait_ms(&info, 4000), 8000);

        let zero = ReadinessQueueInfo {
            size: 0,
            max_concurrency: 250,
        };
        assert_eq!(compute_readiness_queue_wait_ms(&zero, 4000), 0);
    }

    #[tokio::test]
    async fn test_compute_copro_kms_backoff() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        assert_eq!(state.compute_copro_kms_backoff(0).await, 4);
        assert_eq!(state.compute_copro_kms_backoff(30).await, 4);
        assert_eq!(state.compute_copro_kms_backoff(60).await, 10);
        assert_eq!(state.compute_copro_kms_backoff(90).await, 10);
        assert_eq!(state.compute_copro_kms_backoff(120).await, 30);
        assert_eq!(state.compute_copro_kms_backoff(200).await, 30);
    }

    #[tokio::test]
    async fn test_compute_for_get_completed() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        let state_info = RequestStateInfo::new(ReqStatus::Completed, 10, 5);
        let result = state.compute_for_get(&state_info, None).await;
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_compute_for_get_receipt_received() {
        let config = test_config();
        let state = RetryAfterState::new(&config);

        let state_info = RequestStateInfo::new(ReqStatus::ReceiptReceived, 100, 65);
        let result = state.compute_for_get(&state_info, None).await;
        assert_eq!(result, 10); // 65s elapsed, uses interval at 60s threshold
    }

    #[test]
    fn test_apply_safety_margin_ms() {
        assert_eq!(apply_safety_margin_ms(1000, 0.0), 1000);
        // Note: f32→f64 conversion can cause small precision errors, ceil rounds up
        assert_eq!(apply_safety_margin_ms(1000, 0.2), 1201);
        assert_eq!(apply_safety_margin_ms(1000, 0.5), 1500);
        assert_eq!(apply_safety_margin_ms(0, 0.2), 0);
    }
}
