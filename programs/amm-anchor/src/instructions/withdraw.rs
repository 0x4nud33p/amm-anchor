use anchor_lang::prelude::*;
use crate::constants::{AMMSEED, AMM_LP_MINT};
use crate::AmmState;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Burn, Mint, Token, TokenAccount, Transfer, burn}
};

use constant_product_curve::ConstantProduct;


#[derive(Accounts)]
pub struct Withdraw <'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds = [AMMSEED, state.seed.to_le_bytes().as_ref()],
        bump = state.bump,
        has_one = mint_x,
        has_one = mint_y
    )]
    pub state: Account<'info, AmmState>,

    #[account(
        mut,
        seeds = [AMM_LP_MINT, state.key().as_ref()],
        bump = state.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut,
        associated_token:: mint = mint_x,
        associated_token:: authority = state
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_y,
        associated_token:: authority = state
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_x,
        associated_token:: authority = user
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_y,
        associated_token:: authority = user
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token:: mint = mint_lp,
        associated_token:: authority = user
    )]
    pub user_ata_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> Withdraw <'info> {

    pub fn withdraw (&mut self, 
        amount: u64, 
        min_x: u64, 
        min_y: u64 
    ) -> Result<()> {

        require!(self.state.is_locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount, 
            self.vault_y.amount, 
            self.mint_lp.supply, 
            amount, 
            6
        ).map_err(AmmError::from)?;

        require!(amounts.x <= min_x  && amounts.y <= min_y , AmmError::SlippageExceded);

        self.withdraw_tokens(true, amount)?;
        self.withdraw_tokens(false, amount)?;

        self.burn_lp_tokens(amount)?;

        Ok(())
    }

    pub fn withdraw_tokens (&mut self, is_x:bool, amount: u64 ) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.vault_x.to_account_info(), self.user_ata_x.to_account_info()),
            false => (self.vault_y.to_account_info(), self.user_ata_y.to_account_info())
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.state.to_account_info()
        };

        let seeds = &[
            &b"state"[..],
            &self.state.seed.to_le_bytes(),
            &[self.state.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(ctx, amount)
    }

    pub fn burn_lp_tokens (&mut self, amount: u64) -> Result<()> {

        let cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_ata_lp.to_account_info(),
            authority: self.user.to_account_info()
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        burn(ctx, amount)
    }
}
