use anchor_lang::prelude::*;

use crate::types::Handle;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub fhe_authority: Pubkey,
    pub external_input_authority: Pubkey,
}

impl Config {
    pub fn authorize(&self, user: &Pubkey) -> bool {
        &self.fhe_authority == user || &self.external_input_authority == user
    }
}

pub const DISCRIMINATOR: usize = 8;
pub const VEC_PREFIX: usize = 4;
pub const PUBKEY_SIZE: usize = 32;
pub const HANDLE_SIZE: usize = 32;
pub const CHUNK: usize = 10;

#[account]
#[derive(Default)]
pub struct HandlerPermissions {
    pub handle: Handle,
    pub allowed_accounts: Vec<Pubkey>,
}

impl HandlerPermissions {
    pub const fn space_for_chunks(n: usize) -> usize {
        HANDLE_SIZE + VEC_PREFIX + n * CHUNK * PUBKEY_SIZE
    }

    pub const INIT_SPACE: usize = Self::space_for_chunks(1);
}
