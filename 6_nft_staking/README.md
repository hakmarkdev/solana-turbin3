# NFT Staking v1.0.0

A Solana Anchor program for staking [Metaplex Core](https://developers.metaplex.com/core) NFTs to earn rewards. Staked assets are frozen in place via the Core `FreezeDelegate` plugin, and users accrue points (a per-stake bonus plus time-based daily rewards) that can be claimed as SPL reward tokens.

## Instructions

| Instruction | Description |
|---|---|
| `initialize_config` | Create the singleton config (points per stake, max stake, freeze period) and the reward mint. |
| `initialize_user` | Create a per-wallet account that tracks points and staked count. |
| `create_collection` | Create a Core collection managed by the program. |
| `mint_nft` | Mint a Core asset into a managed collection. |
| `stake` | Freeze an asset and start earning rewards. |
| `unstake` | Thaw the asset and stop earning (respects the freeze period). |
| `claim` | Mint accrued points to the user as reward tokens. |
| `claim_stake_rewards` | Claim time-based rewards while the asset stays staked. |

## State

- **StakeConfig** (`config`) — global parameters and bumps.
- **UserAccount** (`user`) — points and active stake count per wallet.
- **CollectionInfo** (`collection_info`) — metadata and staked count per collection.
- **StakeAccount** (`stake`) — owner, mint, and timestamps per staked asset.

All accounts are PDAs; rewards accrue per whole day (`SECONDS_PER_DAY = 86,400`).

## Build & Test

```bash
anchor build
cargo test -p nft-staking
```

## Requirements

- Rust + Solana CLI
- [Anchor](https://www.anchor-lang.com/) 1.0.2
- Yarn (for the Anchor toolchain)
