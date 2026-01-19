//! Centralized admin configuration registry.
//!
//! This module provides the `AdminConfigRegistry` which serves as a single source of truth
//! for all admin-configurable runtime values. It handles:
//! - Thread-safe storage of configuration values
//! - Validation of updates against parameter constraints
//! - Notification of TPS changes to throttler workers via channels

use super::config_param::{ConfigError, ConfigParam, ConfigValue};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info, warn};

/// Centralized runtime configuration - single source of truth.
///
/// All values are admin-updatable via `Arc<RwLock<>>` for thread-safe access.
/// TPS parameters additionally notify their respective throttler workers via channels.
#[derive(Debug)]
pub struct AdminConfigRegistry {
    /// Storage for all configuration values
    values: Arc<RwLock<HashMap<ConfigParam, ConfigValue>>>,

    /// Channels for TPS updates (these need to notify throttler workers)
    tps_channels: HashMap<ConfigParam, mpsc::Sender<u32>>,
}

impl AdminConfigRegistry {
    /// Create a new AdminConfigRegistry with initial configuration.
    ///
    /// # Arguments
    ///
    /// * `initial_values` - Initial configuration values to populate the registry
    /// * `input_proof_tps_tx` - Optional channel to notify input proof throttler of TPS changes
    /// * `user_decrypt_tps_tx` - Optional channel to notify user decrypt throttler of TPS changes
    /// * `public_decrypt_tps_tx` - Optional channel to notify public decrypt throttler of TPS changes
    pub fn new(
        initial_values: HashMap<ConfigParam, ConfigValue>,
        input_proof_tps_tx: Option<mpsc::Sender<u32>>,
        user_decrypt_tps_tx: Option<mpsc::Sender<u32>>,
        public_decrypt_tps_tx: Option<mpsc::Sender<u32>>,
    ) -> Self {
        // Build TPS channels map
        let mut tps_channels = HashMap::new();
        if let Some(tx) = input_proof_tps_tx {
            tps_channels.insert(ConfigParam::InputProofThrottlerTps, tx);
        }
        if let Some(tx) = user_decrypt_tps_tx {
            tps_channels.insert(ConfigParam::UserDecryptThrottlerTps, tx);
        }
        if let Some(tx) = public_decrypt_tps_tx {
            tps_channels.insert(ConfigParam::PublicDecryptThrottlerTps, tx);
        }

        Self {
            values: Arc::new(RwLock::new(initial_values)),
            tps_channels,
        }
    }

    /// Create a registry with default values and TPS channels.
    ///
    /// This is useful when you don't have a full RetryAfterConfig but want to
    /// set up the TPS channels for backward compatibility.
    pub fn with_tps_channels(
        input_proof_tps_tx: Option<mpsc::Sender<u32>>,
        user_decrypt_tps_tx: Option<mpsc::Sender<u32>>,
        public_decrypt_tps_tx: Option<mpsc::Sender<u32>>,
    ) -> Self {
        Self::new(
            HashMap::new(),
            input_proof_tps_tx,
            user_decrypt_tps_tx,
            public_decrypt_tps_tx,
        )
    }

    /// Update a configuration parameter with validation.
    ///
    /// This method:
    /// 1. Validates the value against the parameter's constraints
    /// 2. For TPS parameters, notifies the throttler worker via channel
    /// 3. Updates the stored value
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The value fails validation (out of range or type mismatch)
    /// - The TPS channel is full or closed (for TPS parameters)
    pub async fn update(&self, param: ConfigParam, value: ConfigValue) -> Result<(), ConfigError> {
        // Validate against constraints
        param.constraints().validate(&value)?;

        // Special handling for TPS params (need to notify workers)
        if let Some(tx) = self.tps_channels.get(&param) {
            if let ConfigValue::U32(v) = &value {
                match tx.try_send(*v) {
                    Ok(_) => {
                        info!(
                            param = %param,
                            value = %v,
                            "TPS channel notification sent"
                        );
                    }
                    Err(mpsc::error::TrySendError::Full(_)) => {
                        warn!(param = %param, "TPS channel full, rejecting update");
                        return Err(ConfigError::ChannelError(
                            "Throttler is busy, please retry".to_string(),
                        ));
                    }
                    Err(mpsc::error::TrySendError::Closed(_)) => {
                        warn!(param = %param, "TPS channel closed");
                        return Err(ConfigError::ChannelError(
                            "Throttler service unavailable".to_string(),
                        ));
                    }
                }
            }
        }

        // Update stored value
        let mut values = self.values.write().await;
        values.insert(param, value.clone());

        info!(
            param = %param,
            value = %value,
            "ADMIN_CONFIG_UPDATE: Configuration updated"
        );

        Ok(())
    }

    /// Get a u32 configuration value. Returns `None` if not set.
    pub async fn get_u32(&self, param: ConfigParam) -> Option<u32> {
        let values = self.values.read().await;
        values.get(&param).and_then(|v| v.as_u32())
    }

    /// Get a required u32 value. Logs error and returns 0 if not set.
    pub async fn require_u32(&self, param: ConfigParam) -> u32 {
        match self.get_u32(param).await {
            Some(v) => v,
            None => {
                error!(
                    alert = true,
                    param = %param,
                    "Required config param not set, returning default 0"
                );
                0
            }
        }
    }

    /// Get an f32 configuration value. Returns `None` if not set.
    pub async fn get_f32(&self, param: ConfigParam) -> Option<f32> {
        let values = self.values.read().await;
        values.get(&param).and_then(|v| v.as_f32())
    }

    /// Get a required f32 value. Logs error and returns 0.0 if not set.
    pub async fn require_f32(&self, param: ConfigParam) -> f32 {
        match self.get_f32(param).await {
            Some(v) => v,
            None => {
                error!(
                    alert = true,
                    param = %param,
                    "Required config param not set, returning default 0.0"
                );
                0.0
            }
        }
    }

    /// Get a configuration value.
    ///
    /// Returns `None` if the parameter is not set.
    pub async fn get(&self, param: ConfigParam) -> Option<ConfigValue> {
        let values = self.values.read().await;
        values.get(&param).cloned()
    }

    /// Get all current configuration values.
    ///
    /// This is used for the GET /admin/config endpoint.
    pub async fn get_all(&self) -> HashMap<ConfigParam, ConfigValue> {
        self.values.read().await.clone()
    }

    /// Check if admin functionality is enabled.
    ///
    /// Returns true if at least one TPS channel is configured.
    pub fn is_enabled(&self) -> bool {
        !self.tps_channels.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_update_and_get() {
        let registry = AdminConfigRegistry::new(HashMap::new(), None, None, None);

        // Update a value
        let result = registry
            .update(ConfigParam::RetryAfterMinSeconds, ConfigValue::U32(5))
            .await;
        assert!(result.is_ok());

        // Get the value back
        let value = registry.get_u32(ConfigParam::RetryAfterMinSeconds).await;
        assert_eq!(value, Some(5));
    }

    #[tokio::test]
    async fn test_registry_validation_error() {
        let registry = AdminConfigRegistry::new(HashMap::new(), None, None, None);

        // Try to update with out-of-range value (TPS must be <= 1000)
        let result = registry
            .update(ConfigParam::InputProofThrottlerTps, ConfigValue::U32(2000))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registry_type_mismatch() {
        let registry = AdminConfigRegistry::new(HashMap::new(), None, None, None);

        // Try to update u32 param with f32 value
        let result = registry
            .update(ConfigParam::RetryAfterMinSeconds, ConfigValue::F32(5.0))
            .await;
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }

    #[tokio::test]
    async fn test_registry_f32_value() {
        let registry = AdminConfigRegistry::new(HashMap::new(), None, None, None);

        // Update safety margin (f32 type)
        let result = registry
            .update(ConfigParam::RetryAfterSafetyMargin, ConfigValue::F32(0.2))
            .await;
        assert!(result.is_ok());

        // Get the value back
        let value = registry.get_f32(ConfigParam::RetryAfterSafetyMargin).await;
        assert_eq!(value, Some(0.2));
    }

    #[tokio::test]
    async fn test_registry_get_all() {
        let mut initial = HashMap::new();
        initial.insert(ConfigParam::RetryAfterMinSeconds, ConfigValue::U32(1));
        initial.insert(ConfigParam::RetryAfterMaxSeconds, ConfigValue::U32(300));

        let registry = AdminConfigRegistry::new(initial, None, None, None);

        let all = registry.get_all().await;
        assert_eq!(all.len(), 2);
        assert_eq!(
            all.get(&ConfigParam::RetryAfterMinSeconds),
            Some(&ConfigValue::U32(1))
        );
    }

    #[tokio::test]
    async fn test_registry_tps_channel_notification() {
        let (tx, mut rx) = mpsc::channel(1);
        let registry = AdminConfigRegistry::new(HashMap::new(), Some(tx), None, None);

        // Update TPS value
        let result = registry
            .update(ConfigParam::InputProofThrottlerTps, ConfigValue::U32(50))
            .await;
        assert!(result.is_ok());

        // Check that channel received the value
        let received = rx.try_recv();
        assert_eq!(received, Ok(50));
    }

    #[tokio::test]
    async fn test_registry_is_enabled() {
        let registry_disabled = AdminConfigRegistry::new(HashMap::new(), None, None, None);
        assert!(!registry_disabled.is_enabled());

        let (tx, _rx) = mpsc::channel(1);
        let registry_enabled = AdminConfigRegistry::new(HashMap::new(), Some(tx), None, None);
        assert!(registry_enabled.is_enabled());
    }
}
