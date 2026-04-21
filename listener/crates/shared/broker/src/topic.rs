//! Topic abstraction for optional namespace + routing.
//!
//! A `Topic` represents a routing key optionally scoped by a namespace.
//! This maps to:
//! - main key `{namespace}.{routing}` when namespace exists
//! - main key `{routing}` when namespace is absent
//! - dead-letter key `{main_key}:dead`

/// A topic representing optional namespace + routing key.
///
/// The fully qualified key and dead-letter key are derived on demand.
///
/// # Examples
///
/// ```
/// use broker::{Topic};
///
/// // Using predefined routing keys
/// let blocks = Topic::new("blocks").with_namespace("ethereum");
/// let forks = Topic::new("forks").with_namespace("ethereum");
///
/// // Custom routing keys
/// let custom = Topic::new("erc20-transfers").with_namespace("ethereum");
///
/// // Generic key API
/// assert_eq!(blocks.key(), "ethereum.blocks");
/// assert_eq!(blocks.dead_key(), "ethereum.blocks:dead");
/// assert_eq!(blocks.routing_segment(), "blocks");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Topic {
    /// The namespace (chain/service), e.g., "ethereum", "polygon", "my-service".
    namespace: Option<String>,
    /// The routing key (message type), e.g., "blocks", "forks", "new-filters".
    routing: String,
}

impl Topic {
    /// Create a new topic without namespace.
    pub fn new(routing: impl Into<String>) -> Self {
        let routing = routing.into().trim().to_string();
        Self {
            namespace: None,
            routing,
        }
    }

    /// Fluent setter for namespace.
    ///
    /// Empty/blank namespace is treated as `None`.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        let ns = namespace.into();
        let ns = ns.trim();
        self.namespace = if ns.is_empty() {
            None
        } else {
            Some(ns.to_string())
        };
        self
    }

    /// Convenience constructor for namespaced topics.
    pub fn namespaced(namespace: impl Into<String>, routing: impl Into<String>) -> Self {
        Self::new(routing).with_namespace(namespace)
    }

    /// Remove namespace and return an unscoped topic.
    pub fn without_namespace(mut self) -> Self {
        self.namespace = None;
        self
    }

    /// Borrow namespace as `Option<&str>`.
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Fully qualified key:
    /// - `{namespace}.{routing}` when namespace exists
    /// - `{routing}` otherwise
    pub fn key(&self) -> String {
        match self.namespace.as_deref() {
            Some(ns) => format!("{ns}.{}", self.routing),
            None => self.routing.clone(),
        }
    }

    /// Dead-letter key: `{main_key}:dead`.
    pub fn dead_key(&self) -> String {
        format!("{}:dead", self.key())
    }

    /// Raw routing segment (without namespace).
    pub fn routing_segment(&self) -> &str {
        &self.routing
    }
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_new_without_namespace() {
        let topic = Topic::new("blocks");
        assert_eq!(topic.namespace(), None);
        assert_eq!(topic.routing, "blocks");
    }

    #[test]
    fn test_topic_with_namespace() {
        let topic = Topic::new("blocks").with_namespace("ethereum");
        assert_eq!(topic.namespace(), Some("ethereum"));
        assert_eq!(topic.routing, "blocks");
    }

    #[test]
    fn test_topic_namespaced_constructor() {
        let topic = Topic::namespaced("ethereum", "blocks");
        assert_eq!(topic.namespace(), Some("ethereum"));
        assert_eq!(topic.routing, "blocks");
    }

    #[test]
    fn test_key_namespaced() {
        let topic = Topic::new("blocks").with_namespace("ethereum");
        assert_eq!(topic.key(), "ethereum.blocks");
    }

    #[test]
    fn test_key_unscoped() {
        let topic = Topic::new("blocks");
        assert_eq!(topic.key(), "blocks");
    }

    #[test]
    fn test_dead_key_unscoped() {
        let topic = Topic::new("blocks");
        assert_eq!(topic.dead_key(), "blocks:dead");
    }

    #[test]
    fn test_routing_segment() {
        let topic = Topic::new("blocks").with_namespace("ethereum");
        assert_eq!(topic.routing_segment(), "blocks");
    }

    #[test]
    fn test_display_namespaced_and_unscoped() {
        assert_eq!(
            format!("{}", Topic::new("blocks").with_namespace("ethereum")),
            "ethereum.blocks"
        );
        assert_eq!(format!("{}", Topic::new("blocks")), "blocks");
    }
}
