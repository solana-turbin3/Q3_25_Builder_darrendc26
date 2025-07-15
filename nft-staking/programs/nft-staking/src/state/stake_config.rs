use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_peR_stake: u8,
    pub max_stake: u8,
    pub rewards_bump: u8,
    pub freezE_period: u32,
    pub bump: u8,
}