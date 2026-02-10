use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;

pub const INTERFACE_V0_VERSION: u8 = 1;

pub type HandleBytes = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProgramEventV0 {
    OpRequestedAddV1 {
        caller: Pubkey,
        lhs: HandleBytes,
        rhs: HandleBytes,
        is_scalar: bool,
        result_handle: HandleBytes,
    },
    OpRequestedSubV1 {
        caller: Pubkey,
        lhs: HandleBytes,
        rhs: HandleBytes,
        is_scalar: bool,
        result_handle: HandleBytes,
    },
    OpRequestedBinaryV1 {
        caller: Pubkey,
        lhs: HandleBytes,
        rhs: HandleBytes,
        is_scalar: bool,
        result_handle: HandleBytes,
        opcode: u8,
    },
    OpRequestedUnaryV1 {
        caller: Pubkey,
        input: HandleBytes,
        result_handle: HandleBytes,
        opcode: u8,
    },
    OpRequestedIfThenElseV1 {
        caller: Pubkey,
        control: HandleBytes,
        if_true: HandleBytes,
        if_false: HandleBytes,
        result_handle: HandleBytes,
    },
    OpRequestedCastV1 {
        caller: Pubkey,
        input: HandleBytes,
        to_type: u8,
        result_handle: HandleBytes,
    },
    OpRequestedTrivialEncryptV1 {
        caller: Pubkey,
        pt: HandleBytes,
        to_type: u8,
        result_handle: HandleBytes,
    },
    OpRequestedRandV1 {
        caller: Pubkey,
        rand_type: u8,
        seed: HandleBytes,
        result_handle: HandleBytes,
    },
    OpRequestedRandBoundedV1 {
        caller: Pubkey,
        upper_bound: HandleBytes,
        rand_type: u8,
        seed: HandleBytes,
        result_handle: HandleBytes,
    },
    HandleAllowedV1 {
        caller: Pubkey,
        handle: HandleBytes,
        account: Pubkey,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FinalizedEventEnvelope {
    pub version: u8,
    pub host_chain_id: i64,
    pub slot: u64,
    pub block_time_unix: i64,
    pub tx_signature: Vec<u8>,
    pub tx_index: u32,
    pub op_index: u16,
    pub event: ProgramEventV0,
}

impl FinalizedEventEnvelope {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.version != INTERFACE_V0_VERSION {
            anyhow::bail!(
                "unsupported event version: expected {}, got {}",
                INTERFACE_V0_VERSION,
                self.version
            );
        }
        if self.tx_signature.is_empty() {
            anyhow::bail!("empty tx signature in finalized envelope");
        }
        Ok(())
    }
}
