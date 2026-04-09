use borsh::{BorshDeserialize, BorshSerialize};
use solana_host_contracts_core::{Handle, Pubkey};
use solana_program::pubkey::Pubkey as SolanaPubkey;

pub const TEST_INPUT_STATE_PDA_SEED: &[u8] = b"test-input-state";

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum TestInputInstruction {
    InitializePda {
        owner: Pubkey,
        host_program: Pubkey,
    },
    RequestUint64NonTrivial {
        input_handle: Handle,
        input_proof: Vec<u8>,
        user_id: Pubkey,
    },
    Add42ToInput64 {
        input_handle: Handle,
        input_proof: Vec<u8>,
        user_id: Pubkey,
    },
    CreateUserDecryptFixture {
        fixture_index: u8,
        user_id: Pubkey,
    },
    CreateUserDecryptFixtures {
        user_id: Pubkey,
    },
    CreateUserDecryptFixturesChunk {
        start_fixture_index: u8,
        fixture_count: u8,
        user_id: Pubkey,
    },
    CreatePublicEbool,
    CreatePublicMixed,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TestInputState {
    pub owner: Pubkey,
    pub host_program: Pubkey,
    pub res_uint64: Option<Handle>,
    pub next_session_nonce: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TestInputExecutionResult {
    pub returned_handles: Vec<Handle>,
}

pub fn find_state_pda(program_id: &SolanaPubkey) -> (SolanaPubkey, u8) {
    SolanaPubkey::find_program_address(&[TEST_INPUT_STATE_PDA_SEED], program_id)
}

pub fn host_identity_from_solana_pubkey(pubkey: &SolanaPubkey) -> Pubkey {
    Pubkey::from(pubkey.to_bytes())
}
