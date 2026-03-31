use borsh::{BorshDeserialize, BorshSerialize};
use sha3::{Digest, Keccak256};
use solana_host_contracts_core::{EvmAddress, Handle, Pubkey};
use solana_program::pubkey::Pubkey as SolanaPubkey;

pub const ENCRYPTED_ERC20_STATE_PDA_SEED: &[u8] = b"encrypted-erc20-state";
pub const DEFAULT_MAX_BALANCE_ENTRIES: u16 = 16;
pub const DEFAULT_MAX_ALLOWANCE_ENTRIES: u16 = 16;

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum EncryptedErc20Instruction {
    InitializePda {
        owner: Pubkey,
        host_program: Pubkey,
        name: String,
        symbol: String,
        max_balance_entries: u16,
        max_allowance_entries: u16,
    },
    Mint {
        minted_amount: u64,
    },
    Transfer {
        to: Pubkey,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    Approve {
        spender: Pubkey,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    TransferFrom {
        from: Pubkey,
        to: Pubkey,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    BalanceOf {
        wallet: Pubkey,
    },
    Allowance {
        owner: Pubkey,
        spender: Pubkey,
    },
    TotalSupply,
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
pub struct EncryptedErc20State {
    pub owner: Pubkey,
    pub host_program: Pubkey,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub max_balance_entries: u16,
    pub max_allowance_entries: u16,
    pub balances: Vec<BalanceEntry>,
    pub allowances: Vec<AllowanceEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EncryptedErc20ExecutionResult {
    pub returned_handles: Vec<Handle>,
    pub total_supply: Option<u64>,
}

pub fn find_state_pda(program_id: &SolanaPubkey) -> (SolanaPubkey, u8) {
    SolanaPubkey::find_program_address(&[ENCRYPTED_ERC20_STATE_PDA_SEED], program_id)
}

pub fn evm_address_from_solana_pubkey(pubkey: &SolanaPubkey) -> EvmAddress {
    let digest = Keccak256::digest(pubkey.as_ref());
    let mut bytes = [0_u8; 20];
    bytes.copy_from_slice(&digest[12..]);
    EvmAddress::new(bytes)
}
