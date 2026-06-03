# Dice Game

A decentralized on-chain dice betting game built with Anchor on Solana.

This project demonstrates:

* PDA-based escrow vaults
* Instruction introspection
* Custom proof verification
* Account constraints and validation
* LiteSVM testing
* Secure payout and refund mechanisms

---

## Overview

Players place bets predicting that a randomly generated dice roll will be lower than their chosen number.

The game stores bets on-chain and resolves them through a separate proof instruction that is inspected during execution using Solana's instruction introspection mechanism.

Unlike traditional implementations that rely on Ed25519 signature verification, this implementation demonstrates instruction introspection using a custom `RollProof` structure.

---

## Features

### Initialize House

Creates and funds a vault PDA controlled by the house.

```text
House
  └── Vault PDA
```

The vault acts as the escrow account used to pay winners and process refunds.

---

### Place Bet

A player can create a bet by specifying:

* Seed
* Bet amount
* Guess roll

Example:

```text
Bet Amount: 1 SOL
Guess Roll: 50
```

The player wins if:

```text
Actual Roll < 50
```

A Bet PDA is created and the wager is transferred into the vault.

---

### Submit Roll

A custom proof instruction:

```rust
pub struct RollProof {
    pub roll: u8,
    pub nonce: u64,
}
```

is submitted before the bet is resolved.

The program later inspects this instruction using instruction introspection.

---

### Resolve Bet

The house resolves a bet.

The resolver:

1. Reads the previous instruction.
2. Extracts the serialized `RollProof`.
3. Obtains the roll value.
4. Calculates winnings.
5. Pays the player from the vault.

Flow:

```text
SubmitRoll
      ↓
ResolveBet
      ↓
Inspect Previous Instruction
      ↓
Deserialize RollProof
      ↓
Calculate Payout
      ↓
Transfer Rewards
```

---

### Refund Bet

Players can reclaim their wager if the bet remains unresolved beyond the timeout period.

Flow:

```text
PlaceBet
      ↓
Wait
      ↓
RefundBet
```

This prevents funds from becoming permanently locked.

---

## Program Architecture

### PDAs

#### Vault PDA

```text
["vault", house]
```

Stores house liquidity and player wagers.

#### Bet PDA

```text
["bet", vault, player, seed]
```

Stores bet information.

---

## Bet Account

```rust
pub struct Bet {
    pub player: Pubkey,
    pub seed: u128,
    pub slot: u64,
    pub amount: u64,
    pub guess_roll: u8,
    pub bump: u8,
}
```

---

## Instructions

### Initialize

Creates and funds the vault PDA.

### PlaceBet

Creates a bet account and transfers wager funds.

### SubmitRoll

Stores a serialized `RollProof`.

### ResolveBet

Uses instruction introspection to obtain the roll and settle the bet.

### RefundBet

Returns funds to the player after timeout.

---

## Testing

Tests are written using LiteSVM.

Current coverage includes:

* Initialize
* Place Bet
* Invalid Bet Validation
* Invalid Roll Validation
* Resolve Bet
* Instruction Introspection Flow

Run tests:

```bash
cargo test
```

List tests:

```bash
cargo test -- --list
```

---

## Tech Stack

* Rust
* Solana
* Anchor
* LiteSVM

---

## Learning Objectives

This project demonstrates:

* PDA design
* Escrow patterns
* Account validation
* Instruction introspection
* Anchor account constraints
* Solana testing with LiteSVM

---

## Author

Kunal

Built for learning advanced Solana program development and instruction introspection patterns.
