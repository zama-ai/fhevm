//! Runtime state for the optimistic user-decryption wait window: how many extra
//! shares to wait for past the threshold, and for how long. Seeded from config,
//! adjustable at runtime via the admin API.

use tokio::sync::RwLock;

use crate::config::settings::ContractConfig;

/// The two wait-window knobs, read and written as a unit.
#[derive(Debug, Clone, Copy)]
pub struct WaitWindow {
    pub additional_shares: u32,
    pub timeout_secs: u32,
}

#[derive(Debug)]
pub struct UserDecryptWaitState {
    window: RwLock<WaitWindow>,
}

impl UserDecryptWaitState {
    pub fn new(config: &ContractConfig) -> Self {
        Self {
            window: RwLock::new(WaitWindow {
                additional_shares: config.user_decrypt_additional_shares,
                timeout_secs: config.user_decrypt_additional_shares_timeout_secs,
            }),
        }
    }

    /// Both knobs under one lock acquisition, so a concurrent admin update
    /// cannot be observed half-applied.
    pub async fn window(&self) -> WaitWindow {
        *self.window.read().await
    }

    pub async fn set_additional_shares(&self, val: u32) {
        self.window.write().await.additional_shares = val;
    }

    pub async fn set_additional_shares_timeout_secs(&self, val: u32) {
        self.window.write().await.timeout_secs = val;
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
