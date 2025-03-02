use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig{
  pub rewards_bump: u8,
  pub bump: u8,
  pub freeze_period: u32,
  pub max_stake: u8,
  pub points_per_stake: u8,
}
