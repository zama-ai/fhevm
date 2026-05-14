use anchor_lang::prelude::*;

use crate::constants::MAX_SUBJECTS;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub fhe_authority: Pubkey,
    pub external_input_authority: Pubkey,
    pub bump: u8,
}

impl Config {
    pub fn authorize(&self, user: &Pubkey) -> bool {
        &self.fhe_authority == user || &self.external_input_authority == user
    }
}

pub const DISCRIMINATOR: usize = 8;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum HandlerState {
    Reserved,
    Bound,
}

impl Default for HandlerState {
    fn default() -> Self {
        Self::Reserved
    }
}

#[account]
#[derive(Default, InitSpace)]
pub struct HandlerPermissions {
    pub handle: [u8; 32], // TODO: redefine it as a type alias?
    pub state: HandlerState,
    pub allowed_accounts: [Pubkey; MAX_SUBJECTS],
    pub subject_count: u8,
    pub bump: u8,
}
