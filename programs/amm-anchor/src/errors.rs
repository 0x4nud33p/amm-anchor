use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("The AMM pool is currently locked.")]
    AmmLocked,
    #[msg("Invalid amount provided.")]
    InvalidAmount,
    #[msg("Slippage exceeded.")]
    SlippageExceeded,
}