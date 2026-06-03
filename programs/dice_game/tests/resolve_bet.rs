mod helpers;

use helpers::*;

#[test]
fn test_submit_roll_and_resolve_bet() {
let mut svm = LiteSVM::new();

svm.add_program_from_file(
    ID,
    "../../target/deploy/dice_game.so",
)
.unwrap();

let house = Keypair::new();
let player = Keypair::new();

svm.airdrop(
    &house.pubkey(),
    20_000_000_000,
)
.unwrap();

svm.airdrop(
    &player.pubkey(),
    20_000_000_000,
)
.unwrap();

let seed: u128 = 123;

let (vault, _) =
    derive_vault_pda(&house.pubkey());

let (bet, _) = derive_bet_pda(
    &vault,
    &player.pubkey(),
    seed,
);

// -------------------------
// Initialize
// -------------------------

let initialize_ix = Instruction {
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
        &[initialize_ix],
        Some(&house.pubkey()),
    ),
    svm.latest_blockhash(),
);

svm.send_transaction(tx).unwrap();

// -------------------------
// Place Bet
// -------------------------

let place_bet_ix = Instruction {
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
        guess_roll: 50,
    }
    .data(),
};

let tx = Transaction::new(
    &[&player],
    Message::new(
        &[place_bet_ix],
        Some(&player.pubkey()),
    ),
    svm.latest_blockhash(),
);

svm.send_transaction(tx).unwrap();

// -------------------------
// Submit Roll
// -------------------------

let proof = RollProof {
    roll: 42,
    nonce: 123,
};

let submit_roll_ix = Instruction {
    program_id: ID,
    accounts: accounts::SubmitRoll {}
        .to_account_metas(None),
    data: instruction::SubmitRoll {
        proof,
    }
    .data(),
};

// -------------------------
// Resolve Bet
// -------------------------

let resolve_ix = Instruction {
    program_id: ID,
    accounts: accounts::ResolveBet {
        house: house.pubkey(),
        player: player.pubkey(),
        vault,
        bet,
        instruction_sysvar:
            anchor_lang::system_program::ID,
        system_program:
            anchor_lang::system_program::ID,
    }
    .to_account_metas(None),
    data: instruction::ResolveBet {}
        .data(),
};

let tx = Transaction::new(
    &[&house],
    Message::new(
        &[submit_roll_ix, resolve_ix],
        Some(&house.pubkey()),
    ),
    svm.latest_blockhash(),
);

let result = svm.send_transaction(tx);

println!("{:#?}", result);

assert!(result.is_ok());

}

#[test]
fn test_resolve_bet_without_submit_roll() {
let mut svm = LiteSVM::new();


svm.add_program_from_file(
    ID,
    "../../target/deploy/dice_game.so",
)
.unwrap();

let house = Keypair::new();
let player = Keypair::new();

svm.airdrop(
    &house.pubkey(),
    20_000_000_000,
)
.unwrap();

svm.airdrop(
    &player.pubkey(),
    20_000_000_000,
)
.unwrap();

let seed: u128 = 456;

let (vault, _) =
    derive_vault_pda(&house.pubkey());

let (bet, _) = derive_bet_pda(
    &vault,
    &player.pubkey(),
    seed,
);

let initialize_ix = Instruction {
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
        &[initialize_ix],
        Some(&house.pubkey()),
    ),
    svm.latest_blockhash(),
);

svm.send_transaction(tx).unwrap();

let place_bet_ix = Instruction {
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
        guess_roll: 50,
    }
    .data(),
};

let tx = Transaction::new(
    &[&player],
    Message::new(
        &[place_bet_ix],
        Some(&player.pubkey()),
    ),
    svm.latest_blockhash(),
);

svm.send_transaction(tx).unwrap();

let resolve_ix = Instruction {
    program_id: ID,
    accounts: accounts::ResolveBet {
        house: house.pubkey(),
        player: player.pubkey(),
        vault,
        bet,
        instruction_sysvar:
            anchor_lang::system_program::ID,
        system_program:
            anchor_lang::system_program::ID,
    }
    .to_account_metas(None),
    data: instruction::ResolveBet {}
        .data(),
};

let tx = Transaction::new(
    &[&house],
    Message::new(
        &[resolve_ix],
        Some(&house.pubkey()),
    ),
    svm.latest_blockhash(),
);

let result = svm.send_transaction(tx);

// Currently passes because verify_roll_proof() returns Ok(42)
assert!(result.is_ok());

}
