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
    SchedulerError,
}

impl std::error::Error for SchedulerError {
    fn description(&self) -> &str {
        match self {
            SchedulerError::SchedulerError => "Generic scheduler error",
        }
    }
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SchedulerError")
    }
}
