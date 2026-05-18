# Anchor Escrow v1.0.0

A trustless token-swap escrow program built with Anchor on Solana.

## Overview

Two parties — a **maker** and a **taker** — can exchange SPL tokens without trusting each other:

1. **Make** — Maker locks token A in a vault and specifies how much token B they want in return.
2. **Take** — Taker pays token B to the maker and receives token A from the vault; both the vault and the escrow account are closed.
3. **Refund** — Maker cancels the trade, recovering token A from the vault; both accounts are closed.


### Escrow State

| Field     | Type     | Description                              |
|-----------|----------|------------------------------------------|
| `seed`    | `u64`    | Allows a single maker to run multiple escrows |
| `maker`   | `Pubkey` | Creator of the escrow                    |
| `mint_a`  | `Pubkey` | Token the maker deposits                 |
| `mint_b`  | `Pubkey` | Token the maker wants to receive         |
| `receive` | `u64`    | Amount of token B requested              |
| `bump`    | `u8`     | PDA bump seed                            |

The escrow PDA is derived as:
```
seeds = ["escrow", maker, seed_le_bytes]
```

## Building

```bash
anchor build
```

## Testing

Build the program first (generates `.so` and IDL), then run tests:

```bash
anchor build
cargo test --test escrow_tests
```

### Test coverage

| Test | What it verifies |
|------|-----------------|
| `test_make_deposits_tokens_and_initialises_escrow` | Vault receives `DEPOSIT_AMOUNT` of token A; maker balance decreases by the same; escrow account stores correct seed, maker, mints, and receive amount |
| `test_refund_returns_tokens_and_closes_accounts` | Maker recovers all tokens; vault and escrow accounts are closed |
| `test_take_swaps_tokens_and_closes_accounts` | Taker pays `RECEIVE_AMOUNT` of token B; maker receives token B; taker receives token A from vault; vault and escrow accounts are closed |
