#![allow(unexpected_cfgs)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::{BumpAllocator, ProgramResult, HEAP_START_ADDRESS},
    pubkey::Pubkey,
};

const CUSTOM_HEAP_BYTES: usize = 256 * 1024;

#[cfg(all(not(feature = "no-entrypoint"), target_os = "solana"))]
#[global_allocator]
static A: BumpAllocator = unsafe {
    BumpAllocator::with_fixed_address_range(HEAP_START_ADDRESS as usize, CUSTOM_HEAP_BYTES)
};

entrypoint!(process_instruction);

fn process_instruction<'a, 'b>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::onchain::process_instruction(program_id, accounts, instruction_data)
}
