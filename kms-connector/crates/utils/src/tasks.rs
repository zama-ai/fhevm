use std::sync::{Arc, OnceLock};
use tokio::{sync::Semaphore, task::JoinHandle};
use tracing::{error, warn};

/// A semaphore used to limit the number of tasks spawned by the services.
///
/// It is intended to be used for tasks spawned in loops, where we do not know how many of them
/// could be spawned in parallel.
/// It is not intended to be used for mandatory tasks such as spawning the signal handlers, or the
/// monitoring web server.
static TASKS_LIMITER: OnceLock<Arc<Semaphore>> = OnceLock::new();

/// Sets the task number limit. Can only be called once by the program.
pub fn set_task_limit(n: usize) {
    TASKS_LIMITER
        .set(Arc::new(Semaphore::new(n)))
        .expect("The task number limit has already been configured")
}

/// Spawns a new asynchronous task.
///
/// If the task number limit has been reached, waits for one of them to finish before spawning the
/// next one.
///
/// The `set_task_limit` function should have been used before this one.
pub async fn spawn_with_limit<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // Use a default implementation so `set_task_limit` must not be used in tests.
    let tasks_limiter = TASKS_LIMITER.get_or_init(|| {
        warn!(
            "The task number limit was not configured. Using default values {}...",
            default_task_limit()
        );
        Arc::new(Semaphore::new(default_task_limit()))
    });

    let permit = Arc::clone(tasks_limiter)
        .acquire_owned()
        .await
        .inspect_err(|_| {
            error!(
                "Semaphore is closed. Tasks limit may not work properly. Spawning the task anyway"
            )
        });

    tokio::spawn(async move {
        let _permit = permit; // to drop the semaphore only when the task is completed
        future.await
    })
}

pub fn default_task_limit() -> usize {
    1000
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;
    use std::time::Duration;

    #[tokio::test]
    async fn test_spawn_with_limit() {
        let task_limit = 1;
        set_task_limit(task_limit);

        // Spawn task that never finish
        let infinite_task =
            spawn_with_limit(async { futures::future::pending::<()>().await }).await;

        // Wait a bit
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(!infinite_task.is_finished());

        // Try to spawn a new empty task.
        // But this one should never even start as the task limit has been set to 1.
        // Note that we do not `.await` this spawn.
        let try_spawn = spawn_with_limit(async {});

        // Wait a bit, and check the next task has not been started.
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(try_spawn.now_or_never().is_none());
    }
}
