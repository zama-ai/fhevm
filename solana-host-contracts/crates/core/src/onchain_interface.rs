use crate::instructions::{HostInstruction, HostProgramConfig};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey as SolanaPubkey;

pub const STATE_PDA_SEED: &[u8] = b"host-state";
pub const SESSION_PDA_SEED: &[u8] = b"host-session";

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum OnchainInstruction {
    Initialize {
        config: HostProgramConfig,
    },
    InitializePda {
        config: HostProgramConfig,
    },
    Execute {
        instruction: HostInstruction,
        session_nonce: u64,
        recent_blockhash: [u8; 32],
    },
    ExecuteBatch {
        instructions: Vec<HostInstruction>,
        session_nonce: u64,
        recent_blockhash: [u8; 32],
    },
}

pub fn find_state_pda(program_id: &SolanaPubkey) -> (SolanaPubkey, u8) {
    SolanaPubkey::find_program_address(&[STATE_PDA_SEED], program_id)
}

pub fn find_session_pda(program_id: &SolanaPubkey, authority: &SolanaPubkey) -> (SolanaPubkey, u8) {
    SolanaPubkey::find_program_address(&[SESSION_PDA_SEED, authority.as_ref()], program_id)
}
