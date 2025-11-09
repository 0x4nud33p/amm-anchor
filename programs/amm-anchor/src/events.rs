use anchor_lang::prelude::*;

#[event]
pub struct InitializeAMMEvent {
    pub seed: u64,
    pub is_locked: bool,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp: Pubkey,
    pub fee: u16,
}

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp: Pubkey,
    pub state: Pubkey,
}

#[event]
pub struct LockEvent {
    pub state: Pubkey,
    pub is_locked: bool,
}

#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp: Pubkey,
    pub state: Pubkey,
}