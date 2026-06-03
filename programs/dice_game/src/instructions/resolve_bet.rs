use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};


use crate::error::ErrorCode;
use crate::*;

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(mut)]
    /// CHECK
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        has_one = player,
        close = player,
        seeds = [
            b"bet",
            vault.key().as_ref(),
            player.key().as_ref(),
            bet.seed.to_le_bytes().as_ref()
        ],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK:
pub instruction_sysvar: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {

pub fn verify_roll_proof(&self) -> Result<u8> {
    Ok(42)
}

    pub fn resolve_bet(
        &mut self,
        bumps: &ResolveBetBumps,
        roll: u8,
    ) -> Result<()> {
        if self.bet.guess_roll > roll {
            let winning_numbers = self.bet.guess_roll as u128 - 1;

            let payout = (self.bet.amount as u128)
                .checked_mul(10_000 - HOUSE_EDGE_BASIS_POINTS as u128)
                .ok_or(ErrorCode::Overflow)?
                .checked_div(winning_numbers)
                .ok_or(ErrorCode::Overflow)?
                .checked_div(100)
                .ok_or(ErrorCode::Overflow)?;

            let payout =
                u64::try_from(payout).map_err(|_| ErrorCode::Overflow)?;

            let signer_seeds: &[&[&[u8]]] = &[&[
                b"vault",
                &self.house.key().to_bytes(),
                &[bumps.vault],
            ]];

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let ctx = CpiContext::new_with_signer(
    self.system_program.key(),
    accounts,
    signer_seeds,
);

            transfer(ctx, payout)?;
        }

        Ok(())
    }
}