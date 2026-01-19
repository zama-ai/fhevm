use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use uuid::Uuid;

/// Represents a job identifier that can support different ID types for different flows.
/// Supports UUIDv7 and SHA256-based variants for different flow types and deduplication strategies.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JobId {
    /// UUIDv7 variant - used for flows that benefit from time-ordered IDs
    /// Provides better database performance and natural chronological ordering
    UuidV7(Uuid),
    /// SHA256 hash variant - used for content-based deduplication
    /// Same input always produces same job ID, enabling efficient duplicate detection
    Sha256Hash([u8; 32]),
}

impl Debug for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobId::UuidV7(uuid) => f.debug_tuple("UuidV7").field(uuid).finish(),
            JobId::Sha256Hash(hash) => write!(f, "Sha256Hash({})", hex::encode(hash)),
        }
    }
}

impl JobId {
    /// Create a JobId from an existing UUID (treated as UUIDv7).
    /// Used for standard UUID-based job identification.
    pub fn from_uuid_v7(uuid: Uuid) -> Self {
        JobId::UuidV7(uuid)
    }

    /// Create a JobId from a SHA256 hash.
    /// Used for content-based deduplication where same input produces same ID.
    pub fn from_sha256_hash(hash: [u8; 32]) -> Self {
        JobId::Sha256Hash(hash)
    }

    /// Extract the underlying UUIDv7.
    /// Returns Some(uuid) if this JobId is a UuidV7 variant, None otherwise.
    pub fn as_uuid_v7(&self) -> Option<Uuid> {
        match self {
            JobId::UuidV7(uuid) => Some(*uuid),
            _ => None,
        }
    }

    /// Extract the underlying SHA256 hash.
    /// Returns Some(hash) if this JobId is a Sha256Hash variant, None otherwise.
    pub fn as_sha256_hash(&self) -> Option<[u8; 32]> {
        match self {
            JobId::Sha256Hash(hash) => Some(*hash),
            _ => None,
        }
    }

    /// Get the name of the current variant as a string.
    pub fn variant_name(&self) -> &'static str {
        match self {
            JobId::UuidV7(_) => "UuidV7",
            JobId::Sha256Hash(_) => "Sha256Hash",
        }
    }

    /// Convert to string representation suitable for database storage.
    /// UUIDs use their standard string format, SHA256 hashes use hex encoding.
    pub fn to_database_string(&self) -> String {
        match self {
            JobId::UuidV7(uuid) => uuid.to_string(),
            JobId::Sha256Hash(hash) => hex::encode(hash),
        }
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_database_string())
    }
}

/// Automatic conversion from UUID for convenience.
/// Allows direct use of UUID values where JobId is expected (treated as UuidV7).
impl From<Uuid> for JobId {
    fn from(uuid: Uuid) -> Self {
        JobId::UuidV7(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_id_from_uuid_v7() {
        let uuid = Uuid::new_v4(); // Using v4 for testing, but treated as v7
        let job_id = JobId::from_uuid_v7(uuid);

        assert_eq!(job_id.as_uuid_v7(), Some(uuid));
    }

    #[test]
    fn test_job_id_from_conversion() {
        let uuid = Uuid::new_v4();
        let job_id: JobId = uuid.into();

        assert_eq!(job_id.as_uuid_v7(), Some(uuid));
    }

    #[test]
    fn test_job_id_display() {
        let uuid = Uuid::new_v4();
        let job_id = JobId::from_uuid_v7(uuid);

        assert_eq!(job_id.to_string(), uuid.to_string());
    }

    #[test]
    fn test_job_id_database_string() {
        let uuid = Uuid::new_v4();
        let job_id = JobId::from_uuid_v7(uuid);

        assert_eq!(job_id.to_database_string(), uuid.to_string());
    }

    #[test]
    fn test_job_id_serialization() {
        let uuid = Uuid::new_v4();
        let job_id = JobId::from_uuid_v7(uuid);

        // Test JSON serialization
        let json = serde_json::to_string(&job_id).unwrap();
        let deserialized: JobId = serde_json::from_str(&json).unwrap();

        assert_eq!(job_id, deserialized);
        assert_eq!(job_id.as_uuid_v7(), Some(uuid));
    }

    #[test]
    fn test_job_id_equality() {
        let uuid = Uuid::new_v4();
        let job_id1 = JobId::from_uuid_v7(uuid);
        let job_id2 = JobId::from_uuid_v7(uuid);

        assert_eq!(job_id1, job_id2);
    }

    #[test]
    fn test_job_id_hash() {
        use std::collections::HashSet;

        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let job_id1 = JobId::from_uuid_v7(uuid1);
        let job_id2 = JobId::from_uuid_v7(uuid2);

        let mut set = HashSet::new();
        set.insert(job_id1);
        set.insert(job_id2);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&job_id1));
        assert!(set.contains(&job_id2));
    }

    #[test]
    fn test_job_id_uuid_v7() {
        // Note: uuid v7 requires uuid crate with v7 feature or manual construction
        // For testing purposes, we'll use a regular UUID and treat it as v7
        let uuid = Uuid::new_v4(); // In real usage, this would be Uuid::new_v7()
        let job_id = JobId::from_uuid_v7(uuid);

        assert_eq!(job_id.as_uuid_v7(), Some(uuid));
        assert_eq!(job_id.as_uuid_v7(), Some(uuid)); // Same UUID via v7 method
        assert_eq!(job_id.to_string(), uuid.to_string());
        assert_eq!(job_id.to_database_string(), uuid.to_string());
    }

    #[test]
    fn test_job_id_sha256_hash() {
        let hash = [0u8; 32]; // Example hash
        let job_id = JobId::from_sha256_hash(hash);

        assert_eq!(job_id.as_sha256_hash(), Some(hash));
        assert_eq!(job_id.as_uuid_v7(), None);
        assert_eq!(job_id.to_string(), hex::encode(hash));
        assert_eq!(job_id.to_database_string(), hex::encode(hash));
    }

    #[test]
    fn test_job_id_variant_serialization() {
        // Test all variants can be serialized and deserialized
        let uuid = Uuid::new_v4();
        let hash = [42u8; 32];

        let job_id_uuid = JobId::from_uuid_v7(uuid);
        let job_id_hash = JobId::from_sha256_hash(hash);

        // Test serialization/deserialization for each variant
        let json_uuid = serde_json::to_string(&job_id_uuid).unwrap();
        let deserialized_uuid: JobId = serde_json::from_str(&json_uuid).unwrap();
        assert_eq!(job_id_uuid, deserialized_uuid);

        let json_hash = serde_json::to_string(&job_id_hash).unwrap();
        let deserialized_hash: JobId = serde_json::from_str(&json_hash).unwrap();
        assert_eq!(job_id_hash, deserialized_hash);
    }

    #[test]
    fn test_job_id_variant_equality_and_hashing() {
        use std::collections::HashSet;

        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let hash = [1u8; 32];

        let job_id_uuid1 = JobId::from_uuid_v7(uuid1);
        let job_id_uuid1_dup = JobId::from_uuid_v7(uuid1);
        let job_id_uuid2 = JobId::from_uuid_v7(uuid2);
        let job_id_hash = JobId::from_sha256_hash(hash);

        // Same UUID should be equal
        assert_eq!(job_id_uuid1, job_id_uuid1_dup);

        // Different variants should not be equal
        assert_ne!(job_id_uuid1, job_id_uuid2);
        assert_ne!(job_id_uuid1, job_id_hash);
        assert_ne!(job_id_uuid2, job_id_hash);

        // All should be hashable
        let mut set = HashSet::new();
        set.insert(job_id_uuid1);
        set.insert(job_id_uuid2);
        set.insert(job_id_hash);

        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_safe_option_based_extraction() {
        let uuid = Uuid::new_v4();
        let hash = [1u8; 32];

        let job_id_uuid = JobId::from_uuid_v7(uuid);
        let job_id_hash = JobId::from_sha256_hash(hash);

        // Test successful extractions
        assert_eq!(job_id_uuid.as_uuid_v7(), Some(uuid));
        assert_eq!(job_id_hash.as_sha256_hash(), Some(hash));

        // Test cross-variant extractions return None
        assert_eq!(job_id_uuid.as_sha256_hash(), None);
        assert_eq!(job_id_hash.as_uuid_v7(), None);
    }
}
