use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = 8 + Listing::INIT_SPACE,
        seeds = [marketplace.key().as_ref(), seller_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    pub seller_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = seller,
    )]
    pub seller_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    // account used to hold the nft
    #[account(
        init,
        payer = seller,
        associated_token::mint = seller_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

       // metadata account which is used to verify the nft
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            seller_mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            seller_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub edition: Account<'info, MetadataAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> List<'info> {
    pub fn list(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            seller: self.seller.key(),
            mint: self.seller_mint.key(),
            price,
            bump: bumps.listing,
        });
        Ok(())
    }

    pub fn deposit(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.seller_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.seller.to_account_info(),
            mint: self.seller_mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_context, 1, 0)?;
        Ok(())
    }
}