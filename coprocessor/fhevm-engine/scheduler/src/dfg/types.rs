use anyhow::Result;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};

#[derive(Clone)]
pub struct CompressedCiphertext {
    pub ct_type: i16,
    pub ct_bytes: Vec<u8>,
}

pub struct TaskResult {
    pub compressed_ct: CompressedCiphertext,
    pub is_allowed: bool,
    pub transaction: Transaction,
}
pub struct DFGTxResult {
    pub handle: Handle,
    pub transaction: Transaction,
    pub compressed_ct: Result<CompressedCiphertext>,
}
impl std::fmt::Debug for DFGTxResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(
            f,
            "Result: [{:?}] - tid [{:?}]",
            self.handle, self.transaction
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

pub type TransactionId = Vec<u8>;
pub type BlockHash = Vec<u8>;

#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct Transaction {
    pub transaction_id: TransactionId,
    pub block_hash: BlockHash,
}

impl PartialEq<Transaction> for &Transaction {
    fn eq(&self, other: &Transaction) -> bool {
        self.transaction_id == other.transaction_id && self.block_hash == other.block_hash
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction: Tx[{:?}]:Block[{:?}]",
            hex::encode(&self.transaction_id),
            hex::encode(&self.block_hash)
        )
    }
}
