use serde::{Deserialize, Serialize};

pub const INTERFACE_V0_VERSION: u8 = 1;

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProgramEventV0 {
    OpRequestedAddV1 {
        caller: SolanaPubkeyBytes,
        lhs: HandleBytes,
        rhs: HandleBytes,
        is_scalar: bool,
        result_handle: HandleBytes,
    },
    HandleAllowedV1 {
        caller: SolanaPubkeyBytes,
        handle: HandleBytes,
        account: SolanaPubkeyBytes,
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
