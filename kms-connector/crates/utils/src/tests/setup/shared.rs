use crate::tests::setup::{TestInstance, TestInstanceBuilder};
use std::sync::OnceLock;
use tokio::{
    runtime::Runtime,
    sync::{Mutex as TokioMutex, MutexGuard, OnceCell},
};

// We use these static variables to have a shared tokio runtime and test instance for some
// integration tests.
//
// The shared test instance is to avoid spawning too many containers in parallel, which can be
// resource intensive.
// The shared tokio runtime is required because the `alloy::Provider` of the test instance uses a
// `reqwest::Client` under the hood, which is tied to one `tokio::Runtime`.
//
// Note that containers spawned by the `testcontainers` crate are stopped when they are dropped.
// However, static variables are not dropped. Thus, we wrap the static `TestInstance` in an
// `Option`, so we can manually `take` it and drop it, using `#[dtor]` macro.
//
// Note on `#[dtor]`:
// - if the tests fails, it might not be run, so running containers will be left in that case
//   - it should not impact the next tests that are run
// - there is warning regarding its use https://crates.io/crates/ctor
// - the behavior might be plateform specific
// - if we face too many issues with this in the future, we should try to find a workaround
pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();
pub static TEST_INSTANCE: OnceCell<TokioMutex<Option<TestInstance>>> = OnceCell::const_new();

/// Runner for DB only tests.
pub fn run_with_shared_db_setup<F, R>(test_fn: F) -> R
where
    F: AsyncFnOnce(&mut TestInstance) -> R,
{
    get_shared_runtime().block_on(async {
        let mut test_instance_guard =
            lock_test_instance(TestInstanceBuilder::static_db_setup).await;
        let test_instance = test_instance_guard.as_mut().unwrap();
        test_fn(test_instance).await
    })
}

/// Runner for DB + Gateway tests.
pub fn run_with_shared_db_gw_setup<F, R>(test_fn: F) -> R
where
    F: AsyncFnOnce(&mut TestInstance) -> R,
{
    get_shared_runtime().block_on(async {
        let mut test_instance_guard =
            lock_test_instance(TestInstanceBuilder::static_db_gw_setup).await;
        let test_instance = test_instance_guard.as_mut().unwrap();
        test_fn(test_instance).await
    })
}

fn get_shared_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    })
}

/// Locks the shared test instance.
///
/// Uses the provided `init_fn` if the test instance has not been started yet.
async fn lock_test_instance<F, Fut>(init_fn: F) -> MutexGuard<'static, Option<TestInstance>>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = TokioMutex<Option<TestInstance>>>,
{
    TEST_INSTANCE.get_or_init(init_fn).await.lock().await
}

pub fn clean_test_instance() {
    get_shared_runtime().block_on(async {
        if let Some(test_instance) = TEST_INSTANCE.get() {
            let test_instance = test_instance.lock().await.take();
            drop(test_instance); // Dropping the instance will stop the containers.
        }
    })
}
