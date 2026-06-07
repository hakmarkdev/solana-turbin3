# Dice Bet v1.0.0

A Solana Program demonstrating **Instruction Introspection**. A player
bets on a coin-flip (`0` or `1`). The house reveals the roll in a sibling
instruction, and `resolve_bet` uses the instructions sysvar
to verify that the preceding instruction in the same transaction is a genuine,
house-signed `reveal` before paying out.

## Instruction introspection

`resolve_bet` never trusts the roll on its own. Using the instructions sysvar it:

1. Reads the current instruction index and loads the instruction at `index - 1`
2. Checks that instruction belongs to this program (`program_id == crate::ID`)
3. Matches its Anchor `Reveal` discriminator and decodes the `RevealArgs`
4. Requires `args.bet` to equal the bet being resolved
5. Requires the first account of the `reveal` (the `house_authority`) to be a signer **and** equal to `house.authority`


## Instructions

| Instruction        | Description                                                                 |
| ------------------ | --------------------------------------------------------------------------- |
| `initialize_house` | Create the `house` PDA and fund the `vault` with the starting bankroll      |
| `place_bet`        | Escrow the wager in the vault and record the player's `choice` (0 or 1)     |
| `reveal`           | House-signed instruction carrying the bet pubkey and the roll               |
| `resolve_bet`      | Introspect the preceding `reveal`; pay 2× on a win, else the house keeps it |

## State

- **House** (`house`) — authority and PDA bumps for the house and its vault
- **Bet** (`bet`) — player, seed, amount, slot, and choice per wager
- **RevealArgs** — instruction payload (`bet` pubkey + `roll`) read during introspection

## Build & Test

```bash
anchor build
cargo test -p dice-bet
```
