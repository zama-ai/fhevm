//! Limits the bytes of the response bodies in memory: for the pod, and for each connector.

use actix_web::{
    body::{BodySize, MessageBody},
    web::Bytes,
};
use alloy::primitives::Address;
use std::{
    collections::HashMap,
    convert::Infallible,
    num::NonZeroU32,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore},
    time::timeout,
};

/// Size of each body chunk. Actix asks for the next chunk only when its write buffer has room, so
/// a slow client keeps its permit until the body is almost sent.
const CHUNK_BYTES: usize = 16 * 1024;

pub struct BodyBudget {
    pod: Arc<Semaphore>,
    pod_bytes: u32,
    max_per_connector: u32,
    held: Mutex<HashMap<Address, u32>>,
    wait: Duration,
}

/// Why a body cannot be read now.
#[derive(Debug, PartialEq, Eq)]
pub enum BodyRefusal {
    /// The connector holds its share of the budget.
    Connector,
    /// The pod budget stayed full for the wait time.
    Pod,
}

impl BodyBudget {
    /// A body counts with its true size, at most `pod_bytes`: a larger body is served alone.
    pub fn new(pod_bytes: NonZeroU32, max_per_connector: NonZeroU32, wait: Duration) -> Arc<Self> {
        Arc::new(Self {
            pod: Arc::new(Semaphore::new(pod_bytes.get() as usize)),
            pod_bytes: pod_bytes.get(),
            max_per_connector: max_per_connector.get(),
            held: Mutex::default(),
            wait,
        })
    }

    /// Reserves `bytes` for a body of `connector`. Refuses at once above the share of the connector,
    /// except for one body when it holds none. Waits up to `wait` for room in the pod.
    pub async fn reserve(
        self: &Arc<Self>,
        connector: Address,
        bytes: u64,
    ) -> Result<BodyPermit, BodyRefusal> {
        let bytes = u32::try_from(bytes)
            .unwrap_or(u32::MAX)
            .clamp(1, self.pod_bytes);
        {
            let mut held = self.held.lock().expect("held lock");
            let connector_bytes = held.entry(connector).or_default();
            match connector_bytes.checked_add(bytes) {
                Some(total) if total <= self.max_per_connector || *connector_bytes == 0 => {
                    *connector_bytes = total
                }
                _ => return Err(BodyRefusal::Connector),
            }
        }
        // From here, a timeout or a dropped request gives the share back.
        let share = ConnectorShare {
            budget: self.clone(),
            connector,
            bytes,
        };
        match timeout(self.wait, self.pod.clone().acquire_many_owned(bytes)).await {
            Ok(Ok(pod)) => Ok(BodyPermit {
                _share: share,
                _pod: pod,
            }),
            // The semaphore is never closed: this is a timeout.
            Ok(Err(_)) | Err(_) => Err(BodyRefusal::Pod),
        }
    }
}

/// Bytes of a connector in `held`. On drop, they go back to the connector.
struct ConnectorShare {
    budget: Arc<BodyBudget>,
    connector: Address,
    bytes: u32,
}

impl Drop for ConnectorShare {
    fn drop(&mut self) {
        let mut held = self.budget.held.lock().expect("held lock");
        if let Some(connector_bytes) = held.get_mut(&self.connector) {
            *connector_bytes -= self.bytes;
            if *connector_bytes == 0 {
                held.remove(&self.connector);
            }
        }
    }
}

/// Bytes reserved for one body. On drop, they go back to the pod and to the connector.
pub struct BodyPermit {
    _share: ConnectorShare,
    _pod: OwnedSemaphorePermit,
}

/// A response body. It keeps its permit until it is sent or its connection closes.
pub struct BudgetedBody {
    body: Bytes,
    _permit: BodyPermit,
}

impl BudgetedBody {
    pub fn new(body: Vec<u8>, permit: BodyPermit) -> Self {
        Self {
            body: Bytes::from(body),
            _permit: permit,
        }
    }
}

impl MessageBody for BudgetedBody {
    type Error = Infallible;

    fn size(&self) -> BodySize {
        BodySize::Sized(self.body.len() as u64)
    }

    fn poll_next(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.body.is_empty() {
            return Poll::Ready(None);
        }
        let len = self.body.len().min(CHUNK_BYTES);
        Poll::Ready(Some(Ok(self.body.split_to(len))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONNECTOR: Address = Address::repeat_byte(0x11);
    const OTHER_CONNECTOR: Address = Address::repeat_byte(0x22);
    const THIRD_CONNECTOR: Address = Address::repeat_byte(0x33);
    const WAIT: Duration = Duration::from_millis(50);

    fn nz(bytes: u32) -> NonZeroU32 {
        NonZeroU32::new(bytes).unwrap()
    }

    #[tokio::test]
    async fn refuses_a_connector_above_its_share_but_not_another_connector() {
        let budget = BodyBudget::new(nz(100), nz(60), WAIT);
        let first = budget.reserve(CONNECTOR, 40).await.unwrap();
        assert_eq!(
            budget.reserve(CONNECTOR, 30).await.err(),
            Some(BodyRefusal::Connector)
        );
        let _other = budget.reserve(OTHER_CONNECTOR, 30).await.unwrap();
        drop(first);
        assert!(budget.reserve(CONNECTOR, 30).await.is_ok());
    }

    #[tokio::test]
    async fn waits_for_room_in_the_pod_then_refuses() {
        let budget = BodyBudget::new(nz(100), nz(60), WAIT);
        let first = budget.reserve(CONNECTOR, 60).await.unwrap();
        let _other = budget.reserve(OTHER_CONNECTOR, 40).await.unwrap();
        assert_eq!(
            budget.reserve(THIRD_CONNECTOR, 10).await.err(),
            Some(BodyRefusal::Pod)
        );
        // The refused request gave back its share.
        assert!(!budget.held.lock().unwrap().contains_key(&THIRD_CONNECTOR));

        let waiting = tokio::spawn({
            let budget = budget.clone();
            async move { budget.reserve(THIRD_CONNECTOR, 10).await.is_ok() }
        });
        drop(first);
        assert!(waiting.await.unwrap());
    }

    #[tokio::test]
    async fn a_request_dropped_while_it_waits_gives_its_share_back() {
        let budget = BodyBudget::new(nz(100), nz(60), Duration::from_secs(60));
        let _full = budget.reserve(CONNECTOR, 60).await.unwrap();
        let _other = budget.reserve(OTHER_CONNECTOR, 40).await.unwrap();
        let waiting = tokio::spawn({
            let budget = budget.clone();
            async move { budget.reserve(THIRD_CONNECTOR, 10).await.is_ok() }
        });
        while !budget.held.lock().unwrap().contains_key(&THIRD_CONNECTOR) {
            tokio::task::yield_now().await;
        }

        // A client disconnect drops the request while it waits for the pod.
        waiting.abort();
        let _ = waiting.await;

        assert!(!budget.held.lock().unwrap().contains_key(&THIRD_CONNECTOR));
    }

    #[tokio::test]
    async fn a_body_larger_than_the_share_counts_its_true_size_and_is_served_alone() {
        let budget = BodyBudget::new(nz(100), nz(60), WAIT);
        let permit = budget.reserve(CONNECTOR, 80).await.unwrap();
        assert_eq!(permit._share.bytes, 80);
        assert_eq!(budget.pod.available_permits(), 20);
        assert_eq!(
            budget.reserve(CONNECTOR, 1).await.err(),
            Some(BodyRefusal::Connector)
        );
        drop(permit);
        assert!(budget.held.lock().unwrap().is_empty());
        assert_eq!(budget.pod.available_permits(), 100);
    }

    #[tokio::test]
    async fn a_body_larger_than_the_pod_budget_counts_as_the_budget() {
        let budget = BodyBudget::new(nz(100), nz(60), WAIT);
        let permit = budget.reserve(CONNECTOR, 1_000).await.unwrap();
        assert_eq!(permit._share.bytes, 100);
        assert_eq!(budget.pod.available_permits(), 0);
    }

    #[tokio::test]
    async fn the_body_holds_its_permit_until_it_is_dropped() {
        let budget = BodyBudget::new(nz(1 << 20), nz(1 << 20), WAIT);
        let permit = budget
            .reserve(CONNECTOR, 3 * CHUNK_BYTES as u64)
            .await
            .unwrap();
        let mut body = BudgetedBody::new(vec![7; 3 * CHUNK_BYTES], permit);
        for _ in 0..3 {
            let chunk = std::future::poll_fn(|cx| Pin::new(&mut body).poll_next(cx)).await;
            assert_eq!(chunk.unwrap().unwrap().len(), CHUNK_BYTES);
            assert!(!budget.held.lock().unwrap().is_empty());
        }
        let end = std::future::poll_fn(|cx| Pin::new(&mut body).poll_next(cx)).await;
        assert!(end.is_none());
        drop(body);
        assert!(budget.held.lock().unwrap().is_empty());
    }
}
