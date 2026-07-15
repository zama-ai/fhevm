use anyhow::Result;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};
use std::sync::Arc;

#[derive(Clone)]
pub struct CompressedCiphertext {
    pub ct_type: i16,
    pub ct_bytes: Vec<u8>,
}

pub struct ComputationOutput {
    pub compressed: Option<CompressedCiphertext>,
    pub working: Arc<SupportedFheCiphertexts>,
}

pub struct TaskResult {
    pub compressed_ct: Option<CompressedCiphertext>,
    pub working_ct: Option<Arc<SupportedFheCiphertexts>>,
    pub is_allowed: bool,
    pub transaction_id: Handle,
}
pub struct TaskOutput {
    pub transaction_id: Handle,
    pub result: Result<TaskResult>,
}
pub struct DFGTxResult {
    pub handle: Handle,
    pub transaction_id: Handle,
    pub compressed_ct: Result<CompressedCiphertext>,
}
impl std::fmt::Debug for DFGTxResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(
            f,
            "Result: [{:?}] - tid [{:?}]",
            self.handle, self.transaction_id
        );
        if self.compressed_ct.is_err() {
            let _ = write!(f, "\t ERROR");
        } else {
            let _ = write!(f, "\t OK");
        }
        writeln!(f)
    }
}
#[derive(Clone)]
pub enum DFGTxInput {
    Value((Arc<SupportedFheCiphertexts>, bool)),
    Compressed((CompressedCiphertext, bool)),
}
impl std::fmt::Debug for DFGTxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(_) => write!(f, "DecCT"),
            Self::Compressed(_) => write!(f, "ComCT"),
        }
    }
}

#[derive(Clone)]
pub enum DFGTaskInput {
    /// Boundary or cross-partition input. GPU execution may move an
    /// exclusively-owned copy to the partition's device.
    Value(Arc<SupportedFheCiphertexts>),
    /// Same-partition output, already resident on the partition's device.
    SharedValue(Arc<SupportedFheCiphertexts>),
    Compressed(CompressedCiphertext),
    Dependence(Handle),
}
impl std::fmt::Debug for DFGTaskInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(_) => write!(f, "DecCT"),
            Self::SharedValue(_) => write!(f, "SharedDecCT"),
            Self::Compressed(_) => write!(f, "ComCT"),
            Self::Dependence(_) => write!(f, "DepHL"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SchedulerError {
    CyclicDependence,
    DataflowGraphError,
    MissingInputs,
    DecompressionError,
    ReRandomisationError,
    SchedulerError,
    ExecutionPanic(String),
}

/// Controls whether a scheduler failure may be persisted as consensus state.
/// Only failures derived deterministically from the scheduled data are
/// terminal. An unwind has no reliable type information, so it must retry the
/// complete database transaction rather than marking one coprocessor's row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorPersistence {
    Terminal,
    RetryWorkItem,
    RetryTransaction,
}

impl SchedulerError {
    pub fn persistence(&self) -> ErrorPersistence {
        match self {
            Self::DataflowGraphError | Self::MissingInputs | Self::SchedulerError => {
                ErrorPersistence::RetryWorkItem
            }
            Self::ExecutionPanic(_) => ErrorPersistence::RetryTransaction,
            Self::CyclicDependence | Self::DecompressionError | Self::ReRandomisationError => {
                ErrorPersistence::Terminal
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CyclicDependence => {
                write!(f, "Dependence cycle in dataflow graph")
            }
            Self::DataflowGraphError => {
                write!(f, "Inconsistent dataflow graph error")
            }
            Self::MissingInputs => {
                write!(f, "Missing inputs")
            }
            Self::DecompressionError => {
                write!(f, "Decompression error")
            }
            Self::ReRandomisationError => {
                write!(f, "Re-randomisation error")
            }
            Self::SchedulerError => {
                write!(f, "Generic scheduler error")
            }
            Self::ExecutionPanic(s) => {
                write!(f, "Panic during execution of operation: {}", s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorPersistence, SchedulerError};

    #[test]
    fn unclassified_execution_panics_retry_the_transaction() {
        assert_eq!(
            SchedulerError::ExecutionPanic("node-local failure".into()).persistence(),
            ErrorPersistence::RetryTransaction
        );
        assert_eq!(
            SchedulerError::MissingInputs.persistence(),
            ErrorPersistence::RetryWorkItem
        );
        assert_eq!(
            SchedulerError::CyclicDependence.persistence(),
            ErrorPersistence::Terminal
        );
    }
}
