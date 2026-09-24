//! The account context every admin-only config setter shares.

use anchor_lang::prelude::*;

use crate::state::{HostConfig, HOST_CONFIG_SEED};

/// Shared account context for admin-only config updates.
#[derive(Accounts)]
#[event_cpi]
pub struct HostAdmin<'info> {
    /// Configured host admin.
    pub admin: Signer<'info>,
    /// Singleton config PDA.
    #[account(mut, seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
}
