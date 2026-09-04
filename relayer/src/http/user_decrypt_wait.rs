//! Runtime state for the optimistic user-decryption wait window.
//!
//! Returning only `threshold` shares leaves the client no spare to fall back on
//! when one is corrupted. Once a request is reconstructable, we can wait briefly
//! for extra shares so the client has spares. The wait is bounded by two knobs,
//! seeded from config and adjustable at runtime via the admin API; whichever is
//! met first ends the wait, and either set to 0 disables it.

use tokio::sync::RwLock;

use crate::config::settings::ContractConfig;

/// Runtime state for the optimistic user-decryption wait window.
#[derive(Debug)]
pub struct UserDecryptWaitState {
    additional_shares: RwLock<u32>,
    additional_shares_timeout_secs: RwLock<u32>,
}

impl UserDecryptWaitState {
    pub fn new(config: &ContractConfig) -> Self {
        Self {
            additional_shares: RwLock::new(config.user_decrypt_additional_shares),
            additional_shares_timeout_secs: RwLock::new(
                config.user_decrypt_additional_shares_timeout_secs,
            ),
        }
    }

    pub async fn additional_shares(&self) -> u32 {
        *self.additional_shares.read().await
    }

    pub async fn additional_shares_timeout_secs(&self) -> u32 {
        *self.additional_shares_timeout_secs.read().await
    }

    pub async fn set_additional_shares(&self, val: u32) {
        *self.additional_shares.write().await = val;
    }

    pub async fn set_additional_shares_timeout_secs(&self, val: u32) {
        *self.additional_shares_timeout_secs.write().await = val;
    }
}

/// Whether a reconstructable (`collected >= threshold`) request is ready to
/// return: true once enough extra shares arrived or the wait window elapsed.
/// With `additional_shares == 0` or `timeout_secs == 0` it is always true, so
/// the wait is off (both also act as runtime kill-switches).
pub fn is_ready(
    collected: usize,
    threshold: usize,
    additional_shares: u32,
    elapsed_secs: i64,
    timeout_secs: u32,
) -> bool {
    let target = threshold.saturating_add(additional_shares as usize);
    collected >= target || elapsed_secs >= i64::from(timeout_secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_when_target_reached() {
        assert!(is_ready(11, 9, 2, 0, 10));
    }

    #[test]
    fn not_ready_below_target_within_window() {
        assert!(!is_ready(10, 9, 2, 3, 10));
    }

    #[test]
    fn ready_when_window_elapsed() {
        assert!(is_ready(10, 9, 2, 10, 10));
        assert!(is_ready(9, 9, 2, 15, 10));
    }

    #[test]
    fn disabled_when_additional_shares_zero() {
        assert!(is_ready(9, 9, 0, 0, 10));
    }

    #[test]
    fn disabled_when_timeout_zero() {
        assert!(is_ready(9, 9, 5, 0, 0));
    }
}
