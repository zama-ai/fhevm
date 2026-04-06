use borsh::{BorshDeserialize, BorshSerialize};
use sha3::{Digest, Keccak256};
use solana_host_contracts_core::{host_identity_from_evm_address, EvmAddress, Handle, Pubkey};
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
        user_evm_address: EvmAddress,
    },
    Add42ToInput64 {
        input_handle: Handle,
        input_proof: Vec<u8>,
        user_evm_address: EvmAddress,
    },
    CreateUserDecryptFixture {
        fixture_index: u8,
        user_evm_address: EvmAddress,
    },
    CreateUserDecryptFixtures {
        user_evm_address: EvmAddress,
    },
    CreateUserDecryptFixturesChunk {
        start_fixture_index: u8,
        fixture_count: u8,
        user_evm_address: EvmAddress,
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

pub fn evm_address_from_solana_pubkey(pubkey: &SolanaPubkey) -> EvmAddress {
    let digest = Keccak256::digest(pubkey.as_ref());
    let mut bytes = [0_u8; 20];
    bytes.copy_from_slice(&digest[12..]);
    EvmAddress::new(bytes)
}

pub fn evm_host_identity_from_solana_pubkey(pubkey: &SolanaPubkey) -> Pubkey {
    host_identity_from_evm_address(evm_address_from_solana_pubkey(pubkey))
}
