pub use anchor_lang::{
    prelude::*,
    InstructionData,
    ToAccountMetas,
};

pub use anchor_lang::solana_program::instruction::Instruction;

pub use dice_game::{
    accounts,
    instruction,
    RollProof,
    ID,
};

pub use litesvm::LiteSVM;
pub use solana_keypair::Keypair;
pub use solana_message::Message;
pub use solana_signer::Signer;
pub use solana_transaction::Transaction;

pub fn derive_vault_pda(
    house: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"vault", house.as_ref()],
        &ID,
    )
}
#[allow(dead_code)]
pub fn derive_bet_pda(
    vault: &Pubkey,
    player: &Pubkey,
    seed: u128,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"bet",
            vault.as_ref(),
            player.as_ref(),
            &seed.to_le_bytes(),
        ],
        &ID,
    )
}