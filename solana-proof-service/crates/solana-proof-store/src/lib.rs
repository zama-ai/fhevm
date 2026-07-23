//! Atomic PostgreSQL store, decode/replay reduction, and sequential runner for
//! the standalone Solana proof service (RFC-024 / fhevm-internal #1682).

pub mod decode;
pub mod reduce;
pub mod replay;
pub mod runner;
pub mod store;

pub use decode::{
    decode_program_instructions, DecodeError, DecodedInstruction, RawInstruction, SubjectGrant,
};
pub use reduce::{
    reduce_completed_block, LeafKind, PriorLineageState, ReduceError, StagedBlockReduction,
    StagedLeaf, StagedLineage,
};
pub use replay::{apply_instruction, LineageReplayState, ReplayError};
pub use runner::{run_sequential_ingest, IngestHooks, RunnerError};
pub use store::{
    ApplyOutcome, IntegrityStatus, ProofSnapshot, ResolvedProofSnapshot, SemanticLeafKey,
    SqlProofStore, StoreError,
};
