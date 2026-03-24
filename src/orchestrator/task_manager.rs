use anyhow::Result;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};

/// Internal task manager for orchestrator - handles background task lifecycle
/// Uses tokio::sync::Mutex for async-friendly access to the shared JoinSet
pub(crate) struct TaskManager {
    tasks: Mutex<JoinSet<()>>,
    is_shutting_down: AtomicBool,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Mutex::new(JoinSet::new()),
            is_shutting_down: AtomicBool::new(false),
        }
    }

    /// Spawn a task and wait for it to be ready before continuing
    #[instrument(skip_all, fields(task_name = %name))]
    pub(crate) async fn spawn_task_and_wait_ready<F, R>(
        &self,
        name: &str,
        task_future: F,
        ready_future: R,
    ) -> Result<()>
    where
        F: Future<Output = ()> + Send + 'static,
        R: Future<Output = Result<()>>,
    {
        info!("Spawning task: {}", name);

        // Check shutdown flag while holding the mutex to ensure proper ordering
        let mut tasks = self.tasks.lock().await;

        if self.is_shutting_down.load(Ordering::Acquire) {
            return Err(anyhow::anyhow!(
                "TaskManager is shutting down, cannot spawn new task: {}",
                name
            ));
        }

        tasks.spawn(task_future);
        drop(tasks); // Release mutex before waiting for readiness

        // Wait for it to be ready
        info!("Waiting for task to be ready: {}", name);
        ready_future.await?;
        info!("Task ready: {}", name);

        Ok(())
    }

    /// Wait for shutdown signal and gracefully shutdown all tasks
    #[instrument(skip_all)]
    pub(crate) async fn run_until_shutdown(&self, shutdown_token: CancellationToken) -> Result<()> {
        info!("Waiting for shutdown signal...");
        shutdown_token.cancelled().await;

        info!("Shutdown signal received, stopping all tasks...");

        // Take ownership of all tasks while holding the mutex
        // We set the shutdown flag first so any new spawn attempts will fail
        // Then we take the tasks out so we can shut them down without holding the lock
        // (holding the lock during shutdown could cause deadlocks if tasks try to interact with TaskManager)
        let mut tasks = {
            let mut task_guard = self.tasks.lock().await;
            self.is_shutting_down.store(true, Ordering::Release);
            std::mem::take(&mut *task_guard) // Take tasks, leave empty JoinSet behind
        };

        // Now shut down all the tasks without holding the mutex
        tasks.shutdown().await;

        info!("All tasks stopped successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_normal_task_spawning_and_completion() {
        let task_manager = TaskManager::new();
        let completed = Arc::new(AtomicBool::new(false));
        let completed_clone = completed.clone();

        // Spawn a task that sets a flag when it completes
        let result = task_manager
            .spawn_task_and_wait_ready(
                "test_task",
                async move {
                    sleep(Duration::from_millis(10)).await;
                    completed_clone.store(true, Ordering::Release);
                },
                async { Ok(()) }, // Ready immediately
            )
            .await;

        assert!(result.is_ok(), "Task spawning should succeed");

        // Give the task time to complete
        sleep(Duration::from_millis(20)).await;
        assert!(
            completed.load(Ordering::Acquire),
            "Task should have completed"
        );
    }

    #[tokio::test]
    async fn test_shutdown_prevents_new_spawns() {
        let task_manager = TaskManager::new();
        let shutdown_token = CancellationToken::new();

        // Start shutdown in background
        let shutdown_token_clone = shutdown_token.clone();
        let task_manager_clone = Arc::new(task_manager);
        let shutdown_handle = tokio::spawn({
            let task_manager = task_manager_clone.clone();
            async move { task_manager.run_until_shutdown(shutdown_token_clone).await }
        });

        // Cancel immediately to trigger shutdown
        shutdown_token.cancel();

        // Wait for shutdown to start
        sleep(Duration::from_millis(10)).await;

        // Try to spawn a task after shutdown has started
        let result = task_manager_clone
            .spawn_task_and_wait_ready("should_fail", async {}, async { Ok(()) })
            .await;

        assert!(result.is_err(), "Spawning after shutdown should fail");
        assert!(
            result.unwrap_err().to_string().contains("shutting down"),
            "Error should mention shutdown"
        );

        // Cleanup
        shutdown_handle.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn test_shutdown_completes_successfully() {
        let task_manager = Arc::new(TaskManager::new());
        let shutdown_token = CancellationToken::new();

        // Spawn a simple task
        let spawn_result = task_manager
            .spawn_task_and_wait_ready(
                "simple_task",
                async move {
                    sleep(Duration::from_millis(5)).await;
                    // Task completes
                },
                async { Ok(()) },
            )
            .await;

        assert!(spawn_result.is_ok(), "Task should spawn successfully");

        // Trigger shutdown - should complete without hanging
        shutdown_token.cancel();
        let shutdown_result = task_manager.run_until_shutdown(shutdown_token).await;

        assert!(
            shutdown_result.is_ok(),
            "Shutdown should complete successfully"
        );
    }

    #[tokio::test]
    async fn test_shutdown_flag_prevents_spawning_after_set() {
        let task_manager = Arc::new(TaskManager::new());

        // Manually set the shutdown flag to test the specific behavior
        task_manager.is_shutting_down.store(true, Ordering::Release);

        // Try to spawn a task - this should always fail
        let result = task_manager
            .spawn_task_and_wait_ready("should_always_fail", async {}, async { Ok(()) })
            .await;

        assert!(
            result.is_err(),
            "Spawning should fail when shutdown flag is set"
        );
        assert!(
            result.unwrap_err().to_string().contains("shutting down"),
            "Error should mention shutdown"
        );
    }
}
