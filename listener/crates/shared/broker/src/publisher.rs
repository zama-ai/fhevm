//! Publisher wrapper for optionally namespace-scoped publishing.

use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Debug;

use crate::traits::publisher::DynPublisher;

use crate::error::BrokerError;
use crate::topic::Topic;

/// Publisher optionally scoped to a namespace.
///
/// Publishes messages to routing keys:
/// - namespaced: `{namespace}.{routing}`
/// - unscoped: `{routing}`
#[derive(Clone)]
pub struct Publisher {
    inner: DynPublisher,
    namespace: Option<String>,
    /// Precomputed "{namespace}." prefix to avoid format! on every publish.
    namespace_prefix: Option<String>,
}

impl Publisher {
    /// Create a new Publisher (typically called by `Broker`).
    pub(crate) fn new(inner: DynPublisher, namespace: Option<String>) -> Self {
        let namespace_prefix = namespace.as_ref().map(|ns| format!("{ns}."));
        Self {
            inner,
            namespace,
            namespace_prefix,
        }
    }

    /// Get the namespace this publisher is scoped to.
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Publish to a routing key (e.g., "blocks", "forks").
    pub async fn publish<T: Serialize + Debug + Send + Sync>(
        &self,
        routing: &str,
        payload: &T,
    ) -> Result<(), BrokerError> {
        let topic = self.make_topic(routing);
        self.inner.publish(&topic, payload).await?;
        Ok(())
    }

    /// Publish to a `Topic` directly.
    ///
    /// Publisher namespace must match topic namespace (including both being unscoped).
    pub async fn publish_to_topic<T: Serialize + Debug + Send + Sync>(
        &self,
        topic: &Topic,
        payload: &T,
    ) -> Result<(), BrokerError> {
        if topic.namespace() != self.namespace() {
            return Err(BrokerError::NamespaceMismatch {
                publisher: self.namespace.clone(),
                topic: topic.namespace().map(str::to_owned),
            });
        }

        self.publish(topic.routing_segment(), payload).await
    }

    /// Publish multiple payloads with the same routing key.
    pub async fn publish_batch<T: Serialize + Debug + Send + Sync>(
        &self,
        routing: &str,
        payloads: &[T],
    ) -> Result<(), BrokerError> {
        for payload in payloads {
            self.publish(routing, payload).await?;
        }
        Ok(())
    }

    /// Graceful shutdown - cancels background tasks.
    pub async fn shutdown(&self) {
        self.inner.shutdown().await;
    }

    /// Build the fully qualified logical key.
    ///
    /// Returns `Cow::Borrowed` when unscoped (zero allocation),
    /// or `Cow::Owned` with a precomputed prefix when namespaced.
    fn make_topic<'a>(&'a self, routing: &'a str) -> Cow<'a, str> {
        match &self.namespace_prefix {
            Some(prefix) => {
                let mut s = String::with_capacity(prefix.len() + routing.len());
                s.push_str(prefix);
                s.push_str(routing);
                Cow::Owned(s)
            }
            None => Cow::Borrowed(routing),
        }
    }
}

impl std::fmt::Debug for Publisher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Publisher")
            .field("namespace", &self.namespace())
            .finish_non_exhaustive()
    }
}
