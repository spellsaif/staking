use anchor_lang::prelude::*;

declare_id!("B9aq4RVtp5uzXAgHnkwwZJUXUe1BgLEwFfd4RRxBrvHr");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
