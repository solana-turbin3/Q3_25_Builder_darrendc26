use anchor_lang::prelude::*;
use crate::state::marketplace::Marketplace;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump,)]
        pub marketplace: Account<'info, Marketplace>,

        #[account(
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>, // treasury to hold the fees. system account
     #[account(
        init,
        payer = admin,
        seeds = [b"reward",marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = admin
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>, 
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name:String, fee:u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner ( Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            reward_bump: bumps.reward_mint,
            name,
        });
        Ok(())
    }
}