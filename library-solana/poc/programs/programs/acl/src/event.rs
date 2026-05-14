use anchor_lang::prelude::*;

use crate::types::Handle;

#[event]
pub struct Allowed {
    pub handle: Handle,
    pub context_key: Pubkey,
}

#[event]
pub struct NewHandle {
    pub handle: Handle,
    pub initial_key: Pubkey,
    pub output_index: u128,
    pub config_key: Pubkey,
}
