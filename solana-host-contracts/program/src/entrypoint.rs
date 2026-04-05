#![allow(unexpected_cfgs)]

use solana_program::{account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey};

#[cfg(all(not(feature = "no-entrypoint"), target_os = "solana"))]
#[global_allocator]
static A: solana_program::entrypoint::BumpAllocator = unsafe {
    solana_program::entrypoint::BumpAllocator::with_fixed_address_range(
        solana_program::entrypoint::HEAP_START_ADDRESS as usize,
        256 * 1024,
    )
};

entrypoint!(process_instruction);

fn process_instruction<'a, 'b>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::onchain::process_instruction(program_id, accounts, instruction_data)
}
