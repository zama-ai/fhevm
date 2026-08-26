use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::task::{AbortHandle, Id, JoinSet};
use tokio::time::timeout;
use tracing::{info, instrument, warn};

/// The `JoinSet` plus each task's name and abort handle, so a task that outlasts the drain
/// can be named in the log line that reports it. Kept behind one mutex so spawn and shutdown
/// always see a consistent view of "which tasks exist".
#[derive(Default)]
struct TrackedTasks {
    joinset: JoinSet<()>,
    tasks: HashMap<Id, (String, AbortHandle)>,
}

impl TrackedTasks {
    /// Wait up to `budget` for every tracked task to finish on its own (the caller must
    /// already have made them observe cancellation), then abort whatever is still running.
    /// Aborting is individual, so the log line names the tasks that did not stop.
    async fn drain(&mut self, budget: Duration) {
        let mut pending: HashSet<Id> = self.tasks.keys().copied().collect();
        let total = pending.len();

        if total == 0 {
            info!("No tasks to drain");
            return;
        }

        let start = Instant::now();
        // Disjoint borrows: the join loop reaps from the joinset and removes the matching
        // bookkeeping from `tasks`, so neither can be borrowed through `self`.
        let Self { joinset, tasks } = self;
        let drained = timeout(budget, async {
            while !pending.is_empty() {
                let Some(result) = joinset.join_next_with_id().await else {
                    break;
                };
                let id = match result {
                    Ok((id, ())) => id,
                    Err(join_error) => join_error.id(),
                };
                pending.remove(&id);
                tasks.remove(&id);
            }
        })
        .await
        .is_ok();

        if drained {
            info!(
                elapsed = ?start.elapsed(),
                tasks = total,
                "Tasks drained"
            );
        } else {
            let stuck: Vec<&str> = pending
                .iter()
                .filter_map(|id| tasks.get(id).map(|(name, _)| name.as_str()))
                .collect();
            warn!(
                ?budget,
                remaining = stuck.len(),
                tasks = ?stuck,
                "Drain timeout, aborting stuck tasks"
            );
            for id in &pending {
                if let Some((_, handle)) = tasks.remove(id) {
                    handle.abort();
                }
            }
        }
    }

    /// Safety net for the very end of shutdown: abort anything still tracked (the drain
    /// above leaves nothing behind) and shut the joinset down.
    async fn finish(&mut self) {
        if !self.tasks.is_empty() {
            let stuck: Vec<&str> = self.tasks.values().map(|(name, _)| name.as_str()).collect();
            warn!(
                remaining = stuck.len(),
                tasks = ?stuck,
                "Tasks left tracked after the drain, aborting"
            );
        }
        self.joinset.shutdown().await;
        self.tasks.clear();
    }
}

/// Internal task manager for orchestrator - handles background task lifecycle.
/// Uses tokio::sync::Mutex for async-friendly access to the shared JoinSet.
pub(crate) struct TaskManager {
    tasks: Mutex<TrackedTasks>,
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
            tasks: Mutex::new(TrackedTasks::default()),
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

        let abort_handle = tasks.joinset.spawn(task_future);
        tasks
            .tasks
            .insert(abort_handle.id(), (name.to_string(), abort_handle));
        drop(tasks); // Release mutex before waiting for readiness

        // Wait for it to be ready
        info!("Waiting for task to be ready: {}", name);
        ready_future.await?;
        info!("Task ready: {}", name);

        Ok(())
    }

    /// Mark the manager as shutting down so no further tasks can be spawned. Must be
    /// called once, before the drain.
    pub(crate) async fn begin_shutdown(&self) {
        // Hold the lock while flipping the flag so it can never race a concurrent
        // spawn_task_and_wait_ready call (which checks the flag under the same lock).
        let _guard = self.tasks.lock().await;
        self.is_shutting_down.store(true, Ordering::Release);
    }

    /// Wait up to `budget` for the tracked tasks to exit on their own; abort any still
    /// running once the budget elapses.
    pub(crate) async fn drain_tasks(&self, budget: Duration) {
        let mut tasks = self.tasks.lock().await;
        tasks.drain(budget).await;
    }

    /// Final safety net: abort anything left tracked and shut the joinset down. Called
    /// once, after the drain.
    pub(crate) async fn finish_drain(&self) {
        let mut tasks = self.tasks.lock().await;
        tasks.finish().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
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
        let task_manager = Arc::new(TaskManager::new());

        task_manager.begin_shutdown().await;

        let result = task_manager
            .spawn_task_and_wait_ready("should_fail", async {}, async { Ok(()) })
            .await;

        assert!(result.is_err(), "Spawning after shutdown should fail");
        assert!(
            result.unwrap_err().to_string().contains("shutting down"),
            "Error should mention shutdown"
        );
    }

    #[tokio::test]
    async fn test_drain_completes_successfully() {
        let task_manager = Arc::new(TaskManager::new());

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

        task_manager.begin_shutdown().await;
        task_manager.drain_tasks(Duration::from_secs(5)).await;
        task_manager.finish_drain().await;
    }

    #[tokio::test]
    async fn test_shutdown_flag_prevents_spawning_after_set() {
        let task_manager = Arc::new(TaskManager::new());

        // Manually set the shutdown flag to test the specific behavior
        task_manager.begin_shutdown().await;

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

    #[tokio::test]
    async fn test_drain_allows_slow_cooperative_task_to_complete() {
        let task_manager = Arc::new(TaskManager::new());
        let completed = Arc::new(AtomicBool::new(false));
        let completed_clone = completed.clone();
        let shutdown_token = tokio_util::sync::CancellationToken::new();
        let token_for_task = shutdown_token.clone();

        // This task ignores the cancellation signal for a while, simulating
        // in-flight work that should be allowed to finish rather than aborted.
        task_manager
            .spawn_task_and_wait_ready(
                "slow_cooperative_task",
                async move {
                    token_for_task.cancelled().await;
                    sleep(Duration::from_millis(50)).await;
                    completed_clone.store(true, Ordering::Release);
                },
                async { Ok(()) },
            )
            .await
            .unwrap();

        shutdown_token.cancel();
        task_manager.begin_shutdown().await;
        task_manager.drain_tasks(Duration::from_secs(5)).await;

        assert!(
            completed.load(Ordering::Acquire),
            "Task should be allowed to complete during the drain window instead of being aborted"
        );
    }

    #[tokio::test]
    async fn test_drain_aborts_task_that_ignores_cancellation() {
        let task_manager = Arc::new(TaskManager::new());
        let completed = Arc::new(AtomicBool::new(false));
        let completed_clone = completed.clone();

        // This task never observes any cancellation signal, so it must be
        // aborted once its phase's drain timeout elapses.
        task_manager
            .spawn_task_and_wait_ready(
                "stuck_task",
                async move {
                    sleep(Duration::from_secs(60)).await;
                    completed_clone.store(true, Ordering::Release);
                },
                async { Ok(()) },
            )
            .await
            .unwrap();

        task_manager.begin_shutdown().await;
        // Short drain timeout so the test stays fast.
        tokio::time::timeout(
            Duration::from_secs(5),
            task_manager.drain_tasks(Duration::from_millis(20)),
        )
        .await
        .expect("drain_tasks should not hang past its budget");

        assert!(
            !completed.load(Ordering::Acquire),
            "Stuck task should have been aborted, not allowed to complete"
        );
    }
}
