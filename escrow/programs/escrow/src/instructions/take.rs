#![allow(unused_imports)]
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked },
};
    
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

 #[account(
        mut,
        close = maker, // as maker created while closing rent is sent to the maker
        has_one = maker, // it should has maker
        has_one = mint_a,// it should contain mint_a
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // this is the account which holds the tokens for the escrow
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // these are the program accounts
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


impl <'info> Take<'info> {
    pub fn transfer_to_taker(&mut self, amount: u64) -> Result<()> {

        let signer_seeds: [&[&[u8]]; 1] = [
            &[
                b"escrow",
                self.maker.to_account_info().key.as_ref(),
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.bump],
            ],
        ];

        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);

        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;
        
        Ok(())
    }

    // pub fn take_and_close(&mut self) -> Result<()> {
    //     // Logic to close the escrow account and return funds to the maker
    //     let signer_seeds: [&[&[u8]]; 1] = [
    //         &[
    //             b"escrow",
    //             self.maker.to_account_info().key.as_ref(),
    //             &self.escrow.seed.to_le_bytes()[..],
    //             &[self.escrow.bump],
    //         ],
    //     ];
    //     let transfer_accounts = TransferChecked {
    //         from: self.vault.to_account_info(),
    //         mint: self.mint_a.to_account_info(),
    //         to: self.maker_ata_a.to_account_info(),
    //         authority: self.escrow.to_account_info(),
    //     };

    //     let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);

    //     transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

    //     Ok(())
    // }
    
}