use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmmState {
    pub seed: u64,
    pub is_locked: bool,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub lp_bump: u8,
}

impl AmmState {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 32 + 2 + 1 + 1;
}
