use uuid::Uuid;

/// Generates unique, time-ordered request IDs that are safe to use concurrently.
pub fn new_request_id() -> Uuid {
    Uuid::now_v7()
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    use std::collections::HashSet;
    use tokio::time::{sleep, Duration};

    #[test]
    fn test_sequential_uniqueness() {
        let request_ids: Vec<Uuid> = (0..100).map(|_| new_request_id()).collect();
        assert_eq!(
            request_ids.iter().collect::<HashSet<_>>().len(),
            request_ids.len()
        );
    }

    #[tokio::test]
    async fn test_concurrent_uniqueness() {
        let tasks: Vec<_> = (0..100)
            .map(|_| {
                tokio::spawn(async move { (0..100).map(|_| new_request_id()).collect::<Vec<_>>() })
            })
            .collect();

        let all_request_ids: Vec<Uuid> = future::join_all(tasks)
            .await
            .into_iter()
            .flat_map(|result| result.unwrap())
            .collect();

        // All 10,000 request IDs should be unique
        assert_eq!(all_request_ids.len(), 10_000);
        assert_eq!(all_request_ids.iter().collect::<HashSet<_>>().len(), 10_000);
    }

    #[test]
    fn test_sequential_ids_sort_correctly() {
        let request_ids: Vec<Uuid> = (0..100).map(|_| new_request_id()).collect();
        let id_strings: Vec<String> = request_ids.iter().map(|id| id.to_string()).collect();
        let mut sorted = id_strings.clone();
        sorted.sort();
        assert_eq!(id_strings, sorted);
    }

    #[tokio::test]
    async fn test_delayed_ids_sort_correctly() {
        let mut request_ids = Vec::new();
        for _ in 0..10 {
            request_ids.push(new_request_id());
            sleep(Duration::from_millis(2)).await;
        }

        let id_strings: Vec<String> = request_ids.iter().map(|id| id.to_string()).collect();
        let mut sorted = id_strings.clone();
        sorted.sort();
        assert_eq!(id_strings, sorted);
    }
}
