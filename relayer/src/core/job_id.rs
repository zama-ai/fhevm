use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

/// Job identifier - a 32-byte hash used for event routing and request deduplication.
///
/// For user requests (input-proof, user-decrypt, public-decrypt): content hash from request payload
/// For internal events (keyurl, gateway listener): INTERNAL_EVENT_JOB_ID constant
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct JobId([u8; 32]);

impl Debug for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl JobId {
    /// Zero-byte JobId constant.
    pub const ZERO: JobId = JobId([0u8; 32]);

    /// Returns a reference to the underlying byte array.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl From<[u8; 32]> for JobId {
    fn from(bytes: [u8; 32]) -> Self {
        JobId(bytes)
    }
}

impl From<JobId> for [u8; 32] {
    fn from(job_id: JobId) -> Self {
        job_id.0
    }
}

impl AsRef<[u8]> for JobId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for JobId {
    type Target = [u8; 32];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Vec<u8>> for JobId {
    type Error = std::array::TryFromSliceError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let arr: [u8; 32] = bytes.as_slice().try_into()?;
        Ok(JobId(arr))
    }
}

impl TryFrom<&[u8]> for JobId {
    type Error = std::array::TryFromSliceError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let arr: [u8; 32] = bytes.try_into()?;
        Ok(JobId(arr))
    }
}

/// Zero-byte JobId for internal events that don't need deduplication tracking.
/// These events are ephemeral and processed immediately without persistence.
pub const INTERNAL_EVENT_JOB_ID: JobId = JobId::ZERO;
