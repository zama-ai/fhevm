//! A thin proxy program whose only job is to invoke the `geyser` program's
//! `write_data` instruction via CPI. Used to exercise the Geyser tracker
//! plugin's cross-program-invocation detection: the plugin tracks only the
//! `geyser` program, yet still sees this call because `geyser` appears as an
//! inner instruction of the transaction.

use anchor_lang::prelude::*;

use geyser::cpi::accounts::WriteData;
use geyser::program::Geyser;

declare_id!("4RsnoEwKPWbZg4Z6NUGaqP355SvtGWjjUFqEdmEGiFAB");

#[program]
pub mod caller {
    use super::*;

    /// Forward `value` and `message` to `geyser::write_data` via CPI.
    pub fn proxy_write(ctx: Context<ProxyWrite>, value: u64, message: String) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.geyser_program.key(),
            WriteData {
                authority: ctx.accounts.authority.to_account_info(),
                data_account: ctx.accounts.data_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );
        geyser::cpi::write_data(cpi_ctx, value, message)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProxyWrite<'info> {
    /// Payer + authority; forwarded to geyser as its `authority` signer.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The geyser PDA `[b"data", authority]`. Validated inside geyser; left
    /// unchecked here so we don't duplicate the seed constraints.
    /// CHECK: constraints enforced by the geyser program during the CPI.
    #[account(mut)]
    pub data_account: UncheckedAccount<'info>,

    /// The geyser program we invoke.
    pub geyser_program: Program<'info, Geyser>,

    pub system_program: Program<'info, System>,
}
