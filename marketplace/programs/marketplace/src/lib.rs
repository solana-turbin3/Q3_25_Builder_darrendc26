use anchor_lang::prelude::*;

declare_id!("268TcUH2N1qNEDFyQAGB53NxyMWwc4Fx8WJHHCnDvFuN");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
