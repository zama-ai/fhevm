use fhevm_engine_common::types::SupportedFheCiphertexts;

pub type DFGTaskResult = Option<(SupportedFheCiphertexts, i16, Vec<u8>)>;

#[derive(Clone)]
pub enum DFGTaskInput {
    Value(SupportedFheCiphertexts),
    Compressed((i16, Vec<u8>)),
    Dependence(Option<usize>),
}

#[derive(Debug)]
pub enum SchedulerError {
    UnsatisfiedDependence,
    CyclicDependence,
    DataflowGraphError,
    UnknownOperation(i32),
    InvalidInputs,
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
        }
    }
}
