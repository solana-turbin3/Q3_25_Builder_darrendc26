use anchor_lang::prelude::*;

declare_id!("9iMW9jKf3oy47Tc9RY8XJixTjU7ivrH7fSYBNR39Mp2U");

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
