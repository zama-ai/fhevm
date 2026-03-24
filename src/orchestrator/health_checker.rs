use std::collections::HashMap;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    async fn check(&self) -> anyhow::Result<()>;
}

/// Health checker to manage all dependency checks
#[derive(Clone)]
pub struct HealthChecker {
    pub(crate) checks: HashMap<String, Arc<dyn HealthCheck>>,
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
        }
    }

    pub fn add_health_check(&mut self, name: String, check: Arc<dyn HealthCheck>) {
        self.checks.insert(name, check);
    }

    pub async fn check_all(&self) -> (bool, HashMap<String, String>) {
        let mut results = HashMap::new();
        let mut all_healthy = true;

        for (name, check) in &self.checks {
            match check.check().await {
                Ok(_) => {
                    results.insert(name.clone(), "ok".to_string());
                }
                Err(_) => {
                    results.insert(name.clone(), "fail".to_string());
                    all_healthy = false;
                }
            }
        }

        (all_healthy, results)
    }
}
