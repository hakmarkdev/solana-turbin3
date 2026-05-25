# Anchor AMM v1.0.0

A constant-product automated market maker (AMM) built with Anchor on Solana.

## Overview

Liquidity providers and traders interact with a single pool of two SPL tokens governed by the constant-product invariant (`x * y = k`):

1. **Initialize** — Creates a new pool, minting the LP token and opening vaults for both tokens. A fee (in basis points) and an optional authority are set at creation time.
2. **Deposit** — Liquidity provider transfers token X and token Y into the vaults in the correct ratio and receives LP tokens representing their share of the pool.
3. **Withdraw** — Liquidity provider burns LP tokens and receives a proportional amount of token X and token Y from the vaults.
4. **Swap** — Trader sends one token and receives the other, with the pool fee retained in the vaults for liquidity providers.

### Config State

| Field         | Type           | Description                                   |
|---------------|----------------|-----------------------------------------------|
| `seed`        | `u64`          | Allows multiple independent pools per authority |
| `authority`   | `Option<Pubkey>` | Optional account that can lock the pool      |
| `mint_x`      | `Pubkey`       | First token mint                              |
| `mint_y`      | `Pubkey`       | Second token mint                             |
| `fee`         | `u16`          | Swap fee in basis points (0–10 000)           |
| `locked`      | `bool`         | When `true`, deposits, withdrawals and swaps are blocked |
| `config_bump` | `u8`           | PDA bump for the config account               |
| `lp_bump`     | `u8`           | PDA bump for the LP mint                      |

## Building

```bash
cargo build-sbf
```

## Testing

```bash
cargo build-sbf
cargo test
```

### Test coverage

| Test | What it verifies |
|------|-----------------|
| `test_initialize` | Pool is created, config, LP mint, and both vaults are initialised on-chain |
| `test_deposit` | LP tokens are minted to the provider, both vaults receive the correct token amounts |
| `test_withdraw` | LP tokens are burned, provider recovers proportional token X and token Y from the vaults |
| `test_swap` | Trader sends token X and receives token Y, pool balances shift according to the constant-product formula |
| `test_swap_y` | Trader sends token Y and receives token X, covers the reverse swap direction |
