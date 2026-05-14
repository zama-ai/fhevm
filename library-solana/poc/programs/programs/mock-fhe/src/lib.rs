use anchor_lang::prelude::*;

declare_id!("ASRovdJLXRnaZ387RMY57QHYcqGVjvwrzHijQLEoYBat");

#[program]
pub mod mock_fhe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
