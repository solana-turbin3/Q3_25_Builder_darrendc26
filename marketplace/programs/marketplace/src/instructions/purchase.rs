use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::transfer_checked,
    token_interface::{
        close_account, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>, // the one who buys the listing

    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>, // buyer from where he buys the listing

    // both are the seller related accounts
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    pub seller_mint: InterfaceAccount<'info, Mint>,

    // account which holds the details about the listing
    #[account(
        seeds = [marketplace.key().as_ref(), seller_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    // accounts which holds the
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = seller_mint,
        associated_token::authority = buyer
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    // where the fees goes
    #[account(
        mut,
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    // account which holds the nft
    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Purchase<'info> {
    let fees = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price)
            .unwrap()
            .checked_div(1000_u64)
            .unwrap();

    let amount = self.listing.price.checked_sub(fees).unwrap();

    let cpi_accounts = TransferChecked {
        from: self.buyer.to_account_info(),
        to: self.treasury.to_account_info(),
    };

    let cpi_program = self.token_program.to_account_info();
    let cpi_context = CpiContext::new(ctx_program, ctx_accounts);
    transfer_checked(cpi_context, fees)?;

    let cpi_accounts = TransferChecked {
        from: self.buyer.to_account_info(),
        to: self.seller.to_account_info(),
    }

    let cpi_program = self.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_context, amount)?;

    Ok(())
}