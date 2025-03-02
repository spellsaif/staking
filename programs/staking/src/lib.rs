use anchor_lang::prelude::*;

declare_id!("B9aq4RVtp5uzXAgHnkwwZJUXUe1BgLEwFfd4RRxBrvHr");

pub mod contexts;
pub mod state;
pub mod error;
pub mod constant;

pub use constant::*;
pub use contexts::*;
pub use state::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.process(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
}