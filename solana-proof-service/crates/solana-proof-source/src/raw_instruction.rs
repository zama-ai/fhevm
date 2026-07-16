//! Raw compiled instruction payload carried on a normalized completed block.
//!
//! Field-only copy of the host-decoder shape so the Yellowstone source does not
//! depend on the full zama-host instruction decoder yet.

/// One compiled instruction after account-key resolution, in execution order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawInstruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub top_level_index: usize,
    pub stack_height: Option<u32>,
}
