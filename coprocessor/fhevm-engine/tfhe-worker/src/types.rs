use fhevm_engine_common::types::FhevmError;
use scheduler::dfg::types::SchedulerError;

#[derive(Debug)]
pub enum CoprocessorError {
    DbError(sqlx::Error),
    SchedulerError(SchedulerError),
    FhevmError(FhevmError),
    MissingKeys { reason: String },
}

impl std::fmt::Display for CoprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbError(dbe) => {
                write!(f, "Coprocessor db error: {:?}", dbe)
            }
            Self::SchedulerError(se) => {
                write!(f, "Coprocessor scheduler error: {:?}", se)
            }
            Self::FhevmError(e) => {
                write!(f, "fhevm error: {:?}", e)
            }
            Self::MissingKeys { reason } => {
                write!(f, "Missing keys: {}", reason)
            }
        }
    }
}

impl std::error::Error for CoprocessorError {}

impl From<sqlx::Error> for CoprocessorError {
    fn from(err: sqlx::Error) -> Self {
        CoprocessorError::DbError(err)
    }
}

impl From<SchedulerError> for CoprocessorError {
    fn from(err: SchedulerError) -> Self {
        CoprocessorError::SchedulerError(err)
    }
}
