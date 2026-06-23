use fhevm_engine_common::pg_pool::is_fatal_connection_error;
use fhevm_engine_common::types::FhevmError;
use scheduler::dfg::types::SchedulerError;

#[derive(Debug)]
pub enum CoprocessorError {
    DbError(sqlx::Error),
    FatalConnection(sqlx::Error),
    SchedulerError(SchedulerError),
    FhevmError(FhevmError),
    MissingKeys { reason: String },
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl CoprocessorError {
    // True if this is a lost DB connection error.
    pub fn is_fatal_connection(&self) -> bool {
        matches!(self, Self::FatalConnection(_))
    }
}

impl std::fmt::Display for CoprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbError(dbe) => {
                write!(f, "Coprocessor db error: {:?}", dbe)
            }
            Self::FatalConnection(dbe) => {
                write!(f, "Fatal DB connection error: {:?}", dbe)
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
            Self::Other(e) => {
                write!(f, "Coprocessor error: {:?}", e)
            }
        }
    }
}

impl std::error::Error for CoprocessorError {}

impl From<sqlx::Error> for CoprocessorError {
    fn from(err: sqlx::Error) -> Self {
        // Classify once, here at the source, while the error is still typed.
        if is_fatal_connection_error(&err) {
            CoprocessorError::FatalConnection(err)
        } else {
            CoprocessorError::DbError(err)
        }
    }
}

impl From<SchedulerError> for CoprocessorError {
    fn from(err: SchedulerError) -> Self {
        CoprocessorError::SchedulerError(err)
    }
}

impl From<FhevmError> for CoprocessorError {
    fn from(err: FhevmError) -> Self {
        CoprocessorError::FhevmError(err)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for CoprocessorError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        CoprocessorError::Other(err)
    }
}
