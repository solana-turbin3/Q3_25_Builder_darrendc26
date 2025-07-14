#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

use instructions::*;
pub mod instructions;
pub mod state;

declare_id!("FPFbvtRpThHtjNG2fJXvk1byyXbCc8F5eMEy6uaCeVn2");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>,seed:u64,recieve:u64,deposit_amt:u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recieve, &ctx.bumps)?;
        ctx.accounts.deposit(deposit_amt)?;
        Ok(())
    }    
    
    pub fn take(ctx:Context<Take>,amount:u64)->Result<()>{
        ctx.accounts.transfer_to_taker(amount)?;
        // ctx.accounts.take_and_close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>)->Result<()>{
        ctx.accounts.refund_and_close_vault()?;
        Ok(())
    }

}