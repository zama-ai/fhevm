use alloy::primitives::U256;
use dashmap::DashMap;
use std::future::Future;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct Deduplicator<T> {
    executions: DashMap<U256, ExecutionResult<T>>,
}

#[derive(Clone)]
enum ExecutionResult<T> {
    InProgress(Arc<Semaphore>),
    Completed(T),
    Failed(String),
}

impl<T: Clone + Send + Sync + 'static> Deduplicator<T> {
    pub fn new() -> Self {
        Self {
            executions: DashMap::new(),
        }
    }

    pub async fn execute_or_wait<F, Fut>(&self, indexer_id: U256, operation: F) -> Result<T, String>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, String>> + Send + 'static,
    {
        // Use a semaphore to coordinate execution
        let semaphore = Arc::new(Semaphore::new(0));

        let inserted_result = self.executions.entry(indexer_id);
        match inserted_result {
            dashmap::mapref::entry::Entry::Occupied(entry) => {
                // Entry exists - check what state it's in
                match entry.get() {
                    ExecutionResult::Completed(result) => Ok(result.clone()),
                    ExecutionResult::Failed(error) => Err(error.clone()),
                    ExecutionResult::InProgress(sem) => {
                        let sem_clone = sem.clone();
                        drop(entry); // Release the entry lock
                                     // Wait for the semaphore to be released
                        let _permit = sem_clone.acquire().await.unwrap();
                        // Re-check the result after waiting
                        if let Some(entry) = self.executions.get(&indexer_id) {
                            match entry.value() {
                                ExecutionResult::Completed(result) => Ok(result.clone()),
                                ExecutionResult::Failed(error) => Err(error.clone()),
                                ExecutionResult::InProgress(_) => {
                                    Err("Unexpected state after waiting".to_string())
                                }
                            }
                        } else {
                            Err("Entry disappeared while waiting".to_string())
                        }
                    }
                }
            }
            dashmap::mapref::entry::Entry::Vacant(entry) => {
                // We are the first - insert InProgress and execute
                entry.insert(ExecutionResult::InProgress(semaphore.clone()));

                // Execute the operation
                let result = operation().await;

                // Update the entry with the result
                match result {
                    Ok(value) => {
                        self.executions
                            .insert(indexer_id, ExecutionResult::Completed(value.clone()));
                        // Release all waiting tasks
                        semaphore.add_permits(1000); // Arbitrary large number
                        Ok(value)
                    }
                    Err(error) => {
                        self.executions
                            .insert(indexer_id, ExecutionResult::Failed(error.clone()));
                        // Release all waiting tasks
                        semaphore.add_permits(1000); // Arbitrary large number
                        Err(error)
                    }
                }
            }
        }
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.executions.is_empty()
    }

    #[cfg(test)]
    pub fn clear(&self) {
        self.executions.clear();
    }
}

impl<T: Clone + Send + Sync + 'static> Default for Deduplicator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32 as TestCounter;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn single_execution_test() {
        let dedup: Deduplicator<String> = Deduplicator::new();
        let indexer_id = U256::from(1);

        let result = dedup
            .execute_or_wait(indexer_id, || async { Ok("test".to_string()) })
            .await;

        assert_eq!(result.unwrap(), "test");
        dedup.clear(); // Manual cleanup for test
    }

    #[tokio::test]
    async fn concurrent_requests_share_execution() {
        let dedup: Arc<Deduplicator<String>> = Arc::new(Deduplicator::new());
        let execution_count = Arc::new(TestCounter::new(0));
        let indexer_id = U256::from(2);

        let dedup1 = dedup.clone();
        let counter1 = execution_count.clone();
        let handle1 = tokio::spawn(async move {
            dedup1
                .execute_or_wait(indexer_id, move || async move {
                    counter1.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Ok("shared".to_string())
                })
                .await
        });

        // Give first task a small head start
        sleep(Duration::from_millis(10)).await;

        let dedup2 = dedup.clone();
        let handle2 = tokio::spawn(async move {
            dedup2
                .execute_or_wait(indexer_id, || async {
                    // This should never execute but won't panic for testing
                    Ok("should_not_execute".to_string())
                })
                .await
        });

        let result1 = handle1.await.unwrap().unwrap();
        let result2 = handle2.await.unwrap().unwrap();

        assert_eq!(result1, "shared");
        assert_eq!(result2, "shared");
        assert_eq!(execution_count.load(Ordering::SeqCst), 1);
        dedup.clear();
    }

    #[tokio::test]
    async fn different_indexers_execute_separately() {
        let dedup: Arc<Deduplicator<String>> = Arc::new(Deduplicator::new());
        let execution_count = Arc::new(TestCounter::new(0));

        let indexer_id_1 = U256::from(10);
        let indexer_id_2 = U256::from(20);

        let dedup1 = dedup.clone();
        let counter1 = execution_count.clone();
        let handle1 = tokio::spawn(async move {
            dedup1
                .execute_or_wait(indexer_id_1, move || async move {
                    counter1.fetch_add(1, Ordering::SeqCst);
                    Ok("result1".to_string())
                })
                .await
        });

        let dedup2 = dedup.clone();
        let counter2 = execution_count.clone();
        let handle2 = tokio::spawn(async move {
            dedup2
                .execute_or_wait(indexer_id_2, move || async move {
                    counter2.fetch_add(1, Ordering::SeqCst);
                    Ok("result2".to_string())
                })
                .await
        });

        let result1 = handle1.await.unwrap().unwrap();
        let result2 = handle2.await.unwrap().unwrap();

        assert_eq!(result1, "result1");
        assert_eq!(result2, "result2");
        assert_eq!(execution_count.load(Ordering::SeqCst), 2);
        dedup.clear();
    }

    #[tokio::test]
    async fn failure_shared_by_all() {
        let dedup: Arc<Deduplicator<String>> = Arc::new(Deduplicator::new());
        let execution_count = Arc::new(TestCounter::new(0));
        let indexer_id = U256::from(3);

        let dedup1 = dedup.clone();
        let counter1 = execution_count.clone();
        let handle1 = tokio::spawn(async move {
            dedup1
                .execute_or_wait(indexer_id, move || async move {
                    counter1.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Err("operation failed".to_string())
                })
                .await
        });

        // Give first task a small head start
        sleep(Duration::from_millis(10)).await;

        let dedup2 = dedup.clone();
        let handle2 = tokio::spawn(async move {
            dedup2
                .execute_or_wait(indexer_id, || async {
                    // This should never execute but won't panic for testing
                    Ok("should_not_execute".to_string())
                })
                .await
        });

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        assert!(result1.is_err());
        assert!(result2.is_err());
        assert_eq!(result1.unwrap_err(), "operation failed");
        assert_eq!(result2.unwrap_err(), "operation failed");
        assert_eq!(execution_count.load(Ordering::SeqCst), 1);
        dedup.clear();
    }
}
