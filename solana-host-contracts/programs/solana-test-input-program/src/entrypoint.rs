#![allow(unexpected_cfgs)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[cfg(all(not(feature = "no-entrypoint"), target_os = "solana"))]
use solana_program::entrypoint::{BumpAllocator, HEAP_START_ADDRESS};

#[cfg(all(not(feature = "no-entrypoint"), target_os = "solana"))]
const CUSTOM_HEAP_BYTES: usize = 256 * 1024;

#[cfg(all(not(feature = "no-entrypoint"), target_os = "solana"))]
#[global_allocator]
static A: BumpAllocator = unsafe {
    BumpAllocator::with_fixed_address_range(HEAP_START_ADDRESS as usize, CUSTOM_HEAP_BYTES)
};

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::onchain::process_instruction(program_id, accounts, instruction_data)
}
