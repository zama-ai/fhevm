use anyhow::Result;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};

#[derive(Clone, Debug)]
pub struct CompressedCiphertext {
    pub ct_type: i16,
    pub bytes: Vec<u8>,
}

impl From<(i16, Vec<u8>)> for CompressedCiphertext {
    fn from((ct_type, bytes): (i16, Vec<u8>)) -> Self {
        Self { ct_type, bytes }
    }
}

impl From<CompressedCiphertext> for (i16, Vec<u8>) {
    fn from(value: CompressedCiphertext) -> Self {
        (value.ct_type, value.bytes)
    }
}

#[derive(Clone)]
pub enum TaskResult {
    Allowed {
        ct: CompressedCiphertext,
        transaction_id: Handle,
    },
    Intermediate {
        ct: SupportedFheCiphertexts,
        transaction_id: Handle,
    },
}

impl TaskResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed { .. })
    }

    pub fn transaction_id(&self) -> &Handle {
        match self {
            Self::Allowed { transaction_id, .. } | Self::Intermediate { transaction_id, .. } => {
                transaction_id
            }
        }
    }
}

impl TryFrom<&TaskResult> for CompressedCiphertext {
    type Error = anyhow::Error;

    fn try_from(value: &TaskResult) -> Result<Self> {
        match value {
            TaskResult::Allowed { ct, .. } => Ok(ct.clone()),
            TaskResult::Intermediate { .. } => Err(SchedulerError::SchedulerError.into()),
        }
    }
}

impl TryFrom<&TaskResult> for DFGTxInput {
    type Error = anyhow::Error;

    fn try_from(value: &TaskResult) -> Result<Self> {
        match value {
            TaskResult::Allowed { ct, .. } => Ok(DFGTxInput::Compressed((ct.clone(), true))),
            TaskResult::Intermediate { ct, .. } => {
                // Normalize cross-transaction propagation through compressed form to avoid
                // representation-sensitive behavior when the same handle can appear in
                // multiple contexts (e.g. allowed/non-allowed paths).
                let ct_type = ct.type_num();
                let bytes = ct.compress()?;
                Ok(DFGTxInput::Compressed((
                    CompressedCiphertext::from((ct_type, bytes)),
                    false,
                )))
            }
        }
    }
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
    Value((SupportedFheCiphertexts, bool)),
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
    Value(SupportedFheCiphertexts),
    Compressed(CompressedCiphertext),
    Dependence(Handle),
}
impl std::fmt::Debug for DFGTaskInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(_) => write!(f, "DecCT"),
            Self::Compressed(_) => write!(f, "ComCT"),
            Self::Dependence(_) => write!(f, "DepHL"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SchedulerError {
    CyclicDependence,
    DataflowGraphError,
    MissingInputs,
    DecompressionError,
    ReRandomisationError,
    SchedulerError,
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
        }
    }
}
