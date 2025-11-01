use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;
pub mod constants;

pub use instructions::initialize::*;
pub use state::*;
pub use events::*;
pub use constants::*;

declare_id!("Cp3bxBLgcJjGZSvjKreVvuzhVmSvGRwshSYnH2rxHtij");

#[program]
pub mod amm_anchor {
    use super::*;
    
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fee, &ctx.bumps)?;

        emit!(InitializeAMMEvent {
            seed,
            fee,
            is_locked: ctx.accounts.state.is_locked,
            mint_x: ctx.accounts.mint_x.key(),
            mint_y: ctx.accounts.mint_y.key(),
            mint_lp: ctx.accounts.mint_lp.key(),
        });

        Ok(())
    }
}