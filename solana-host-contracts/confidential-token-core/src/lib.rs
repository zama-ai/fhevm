use borsh::{BorshDeserialize, BorshSerialize};
use sha3::{Digest, Keccak256};
use solana_host_contracts_core::{host_identity_from_evm_address, EvmAddress, Handle, Pubkey};
use solana_program::pubkey::Pubkey as SolanaPubkey;

pub const CONFIDENTIAL_TOKEN_STATE_PDA_SEED: &[u8] = b"confidential-token-state";
pub const DEFAULT_MAX_BALANCE_ENTRIES: u16 = 16;
pub const DEFAULT_MAX_ALLOWANCE_ENTRIES: u16 = 16;

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum ConfidentialTokenInstruction {
    InitializePda {
        owner: Pubkey,
        host_program: Pubkey,
        name: String,
        symbol: String,
        max_balance_entries: u16,
        max_allowance_entries: u16,
    },
    ResetState,
    MintTo {
        recipient: Pubkey,
        amount: u64,
    },
    Transfer {
        recipient: Pubkey,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    ApproveDelegate {
        delegate: Pubkey,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    TransferAsDelegate {
        source: Pubkey,
        recipient: Pubkey,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    Balance {
        owner: Pubkey,
    },
    DelegateAllowance {
        owner: Pubkey,
        delegate: Pubkey,
    },
    Supply,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BalanceEntry {
    pub wallet: Pubkey,
    pub handle: Handle,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AllowanceEntry {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub handle: Handle,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfidentialTokenState {
    pub owner: Pubkey,
    pub host_program: Pubkey,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub zero_handle: Option<Handle>,
    pub max_balance_entries: u16,
    pub max_allowance_entries: u16,
    pub balances: Vec<BalanceEntry>,
    pub allowances: Vec<AllowanceEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfidentialTokenExecutionResult {
    pub returned_handles: Vec<Handle>,
    pub total_supply: Option<u64>,
}

pub fn find_state_pda(program_id: &SolanaPubkey) -> (SolanaPubkey, u8) {
    SolanaPubkey::find_program_address(&[CONFIDENTIAL_TOKEN_STATE_PDA_SEED], program_id)
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
