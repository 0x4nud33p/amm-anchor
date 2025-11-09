use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use errors::*;
pub use events::*;
pub use state::*;

declare_id!("Cp3bxBLgcJjGZSvjKreVvuzhVmSvGRwshSYnH2rxHtij");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16) -> Result<()> {
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

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;

        emit!(DepositEvent {
            user: ctx.accounts.user.key(),
            mint_x: ctx.accounts.state.mint_x,
            mint_y: ctx.accounts.state.mint_y,
            mint_lp: ctx.accounts.state.mint_lp,
            state: ctx.accounts.state.key(),
        });

        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)?;

        Ok(())
    }

    pub fn lock(ctx: Context<Lock>) -> Result<()> {
        ctx.accounts.lock()?;

        emit!(LockEvent {
            state: ctx.accounts.state.key(),
            is_locked: ctx.accounts.state.is_locked,
        });

        Ok(())
    }

    pub fn unlock(ctx: Context<Lock>) -> Result<()> {
        ctx.accounts.unlock()?;

        emit!(LockEvent {
            state: ctx.accounts.state.key(),
            is_locked: ctx.accounts.state.is_locked,
        });

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)?;

        emit!(WithdrawEvent {
            user: ctx.accounts.user.key(),
            mint_x: ctx.accounts.state.mint_x,
            mint_y: ctx.accounts.state.mint_y,
            mint_lp: ctx.accounts.state.mint_lp,
            state: ctx.accounts.state.key(),
        });

        Ok(())
    }
    
}
