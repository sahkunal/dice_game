use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct SubmitRoll {}

pub fn submit_roll(
    _ctx: Context<SubmitRoll>,
    _proof: RollProof,
) -> Result<()> {
    Ok(())
}

//This instruction stores nothing.

//Its only purpose is to exist inside the transaction for introspection.