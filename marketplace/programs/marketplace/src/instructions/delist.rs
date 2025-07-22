
use crate::{error::MarketplaceErrors, Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        close = seller, 
        seeds = [marketplace.key().as_ref(), seller_mint.key().as_ref()],
        has_one = seller,
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    pub seller_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = seller_mint,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = seller
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw(&mut self) -> Result<()> {

        require!(
            self.seller.key() == self.listing.seller.key(),
            MarketplaceErrors::Unauthorized
        );

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.seller_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.seller_mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

         let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_context, 1, 0)?;
        Ok(())
    }

    pub fn close_account(&mut self) -> Result<()> {
        require!(
            self.seller.key() == self.listing.seller.key(),
            MarketplaceErrors::Unauthorized
        );

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination = self.seller.to_account_info(),
            authority = self.listing.to_account_info(),
        }

        let cpi_program = self.token_program.to_account_info();

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_context)?;
        Ok(())
    }
}
