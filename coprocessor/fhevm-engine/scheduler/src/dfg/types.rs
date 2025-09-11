use anyhow::Result;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};

pub type DFGTaskResult = Option<Result<(SupportedFheCiphertexts, Option<(i16, Vec<u8>)>)>>;

pub struct DFGTxResult {
    pub handle: Handle,
    pub transaction_id: Handle,
    pub compressed_ct: Result<(i16, Vec<u8>)>,
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
    Value(SupportedFheCiphertexts),
    Compressed((i16, Vec<u8>)),
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
    Compressed((i16, Vec<u8>)),
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
    UnsatisfiedDependence,
    CyclicDependence,
    DataflowGraphError,
    UnknownOperation(i32),
    InvalidInputs,
    ReRandomisationError,
    MissingInput,
    SchedulerError,
}

impl std::error::Error for SchedulerError {}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnsatisfiedDependence => {
                write!(
                    f,
                    "Unsatisfied depence in dataflow graph at scheduling time"
                )
            }
            Self::CyclicDependence => {
                write!(f, "Depence cycle in dataflow graph")
            }
            Self::DataflowGraphError => {
                write!(f, "Inconsistent dataflow graph error")
            }
            Self::UnknownOperation(op) => {
                write!(f, "Unknown operation with code: {op}")
            }
            Self::InvalidInputs => {
                write!(f, "Invalid inputs to FHE operation")
            }
            Self::SchedulerError => {
                write!(f, "Generic scheduler error")
            }
            Self::MissingInput => {
                write!(f, "Missing input to transaction")
            }
            Self::ReRandomisationError => {
                write!(f, "Re-randomisation error")
            }
        }
    }
}
