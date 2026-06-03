pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GKLuZsYeUbjeg8NjydkYA8upgWMbvmMiTPhZbNozdPhh");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        seed: u128,
        amount: u64,
        guess_roll: u8,
    ) -> Result<()> {
        ctx.accounts
            .create_bet(seed, &ctx.bumps, guess_roll, amount)?;
        ctx.accounts.deposit(amount)
    }

   pub fn resolve_bet(
    ctx: Context<ResolveBet>,
) -> Result<()> {
    let roll = ctx.accounts.verify_roll_proof()?;

    ctx.accounts.resolve_bet(
        &ctx.bumps,
        roll,
    )
}
    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }

    pub fn submit_roll(
    ctx: Context<SubmitRoll>,
    proof: RollProof,
) -> Result<()> {
    instructions::submit_roll::submit_roll(ctx, proof)
}
}
