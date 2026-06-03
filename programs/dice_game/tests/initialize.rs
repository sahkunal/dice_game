mod helpers;

use helpers::*;

#[test]
fn test_initialize_success() {
    let mut svm = LiteSVM::new();

    svm.add_program_from_file(
        ID,
        "../../target/deploy/dice_game.so",
    )
    .unwrap();

    let house = Keypair::new();

    svm.airdrop(
        &house.pubkey(),
        10_000_000_000,
    )
    .unwrap();

    let (vault, _) =
        derive_vault_pda(&house.pubkey());

    let ix = Instruction {
        program_id: ID,
        accounts: accounts::Initialize {
            house: house.pubkey(),
            vault,
            system_program:
                anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::Initialize {
            amount: 1_000_000_000,
        }
        .data(),
    };

    let tx = Transaction::new(
        &[&house],
        Message::new(
            &[ix],
            Some(&house.pubkey()),
        ),
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_ok());
}