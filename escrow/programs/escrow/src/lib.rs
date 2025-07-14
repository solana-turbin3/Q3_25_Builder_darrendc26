use anchor_lang::prelude::*;

declare_id!("FPFbvtRpThHtjNG2fJXvk1byyXbCc8F5eMEy6uaCeVn2");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
