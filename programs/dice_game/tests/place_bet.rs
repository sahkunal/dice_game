mod helpers;

use helpers::*;

#[test]
fn test_place_bet_below_minimum() {
    let mut svm = LiteSVM::new();

    svm.add_program_from_file(
        ID,
        "../../target/deploy/dice_game.so",
    )
    .unwrap();

    let house = Keypair::new();
    let player = Keypair::new();

    svm.airdrop(&house.pubkey(), 20_000_000_000).unwrap();
    svm.airdrop(&player.pubkey(), 20_000_000_000).unwrap();

    let seed = 123u128;

    let (vault, _) = derive_vault_pda(&house.pubkey());
    let (bet, _) =
        derive_bet_pda(&vault, &player.pubkey(), seed);

    let ix = Instruction {
        program_id: ID,
        accounts: accounts::PlaceBet {
            player: player.pubkey(),
            house: house.pubkey(),
            vault,
            bet,
            system_program:
                anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::PlaceBet {
            seed,
            amount: 1,
            guess_roll: 50,
        }
        .data(),
    };

    let tx = Transaction::new(
        &[&player],
        Message::new(
            &[ix],
            Some(&player.pubkey()),
        ),
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}

#[test]
fn test_invalid_roll_low() {
    let mut svm = LiteSVM::new();

    svm.add_program_from_file(
        ID,
        "../../target/deploy/dice_game.so",
    )
    .unwrap();

    let house = Keypair::new();
    let player = Keypair::new();

    svm.airdrop(&house.pubkey(), 20_000_000_000).unwrap();
    svm.airdrop(&player.pubkey(), 20_000_000_000).unwrap();

    let seed = 123u128;

    let (vault, _) = derive_vault_pda(&house.pubkey());
    let (bet, _) =
        derive_bet_pda(&vault, &player.pubkey(), seed);

    let ix = Instruction {
        program_id: ID,
        accounts: accounts::PlaceBet {
            player: player.pubkey(),
            house: house.pubkey(),
            vault,
            bet,
            system_program:
                anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::PlaceBet {
            seed,
            amount: 100_000_000,
            guess_roll: 0,
        }
        .data(),
    };

    let tx = Transaction::new(
        &[&player],
        Message::new(
            &[ix],
            Some(&player.pubkey()),
        ),
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}

#[test]
fn test_invalid_roll_high() {
    let mut svm = LiteSVM::new();

    svm.add_program_from_file(
        ID,
        "../../target/deploy/dice_game.so",
    )
    .unwrap();

    let house = Keypair::new();
    let player = Keypair::new();

    svm.airdrop(&house.pubkey(), 20_000_000_000).unwrap();
    svm.airdrop(&player.pubkey(), 20_000_000_000).unwrap();

    let seed = 123u128;

    let (vault, _) = derive_vault_pda(&house.pubkey());
    let (bet, _) =
        derive_bet_pda(&vault, &player.pubkey(), seed);

    let ix = Instruction {
        program_id: ID,
        accounts: accounts::PlaceBet {
            player: player.pubkey(),
            house: house.pubkey(),
            vault,
            bet,
            system_program:
                anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::PlaceBet {
            seed,
            amount: 100_000_000,
            guess_roll: 100,
        }
        .data(),
    };

    let tx = Transaction::new(
        &[&player],
        Message::new(
            &[ix],
            Some(&player.pubkey()),
        ),
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_err());
}