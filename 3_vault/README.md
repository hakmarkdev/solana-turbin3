# Anchor Vault v1.0.0

A Solana program built with **Anchor 1.0.0** that lets any wallet deposit SOL into a personal PDA vault and withdraw it later.

### Instructions

| Instruction | Args | Description |
|-------------|------|-------------|
| `deposit`   | `amount: u64` | Transfer `amount` lamports from the signer into their vault PDA. Fails if the vault is already funded or the amount is below the rent-exempt minimum. |
| `withdraw`  | — | Transfer all lamports from the vault back to the signer (PDA-signed CPI). Fails if the vault is empty. |

### Error codes

| Error | Meaning |
|-------|---------|
| `VaultAlreadyFunded` | A second deposit was attempted on a non-empty vault. |
| `AmountTooLow` | Deposit amount ≤ `Rent::minimum_balance(0)`. |
| `VaultEmpty` | Withdraw attempted on an empty vault. |

## Building

```bash
anchor build
```

## Testing

Build first, then run:

```bash
anchor build
cargo test
```

### Test coverage

| Test | What it verifies |
|------|-----------------|
| `test_deposit_success` | Vault balance equals the deposited amount |
| `test_withdraw_success` | Vault empties; signer receives lamports back |
| `test_deposit_twice_fails` | Second deposit rejected with `VaultAlreadyFunded` |
| `test_deposit_below_rent_minimum_fails` | Tiny deposit rejected with `AmountTooLow` |
| `test_withdraw_empty_vault_fails` | Withdraw on empty vault rejected with `VaultEmpty` |
| `test_different_users_have_independent_vaults` | Each user's PDA is distinct; one user's withdraw doesn't touch another's vault |
