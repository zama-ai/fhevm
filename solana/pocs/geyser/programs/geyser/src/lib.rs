pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8");

#[program]
pub mod geyser {
    use super::*;

    /// Create-or-update a PDA owned by the signer and write `value` and
    /// `message` into it. Returns the written value and PDA bump as return data.
    pub fn write_data(
        ctx: Context<WriteData>,
        value: u64,
        message: String,
    ) -> Result<WriteResult> {
        write_data::handler(ctx, value, message)
    }
}
