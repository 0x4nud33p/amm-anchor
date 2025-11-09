use anchor_lang::prelude::*;

use crate::AmmState;
use crate::constants::AMMSEED;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [AMMSEED, state.seed.to_le_bytes().as_ref()],
        bump = state.bump,
    )]
    pub state: Account<'info, AmmState>,
}

impl<'info> ToggleLock<'info> {
    pub fn lock(&mut self) -> Result<()> {
        self.state.is_locked = true;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<()> {
        self.state.is_locked = false;
        Ok(())
    }
}