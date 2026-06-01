# NFT Marketplace v1.0.0

A Solana NFT marketplace built with **Anchor**. Sellers escrow
verified-collection NFTs for sale; buyers pay the seller plus a marketplace fee
and receive reward tokens.

## Instructions

| Instruction          | Description                                            |
| -------------------- | ----------------------------------------------------- |
| `initialize`         | Create a marketplace, its treasury and rewards mint   |
| `list`               | Escrow a verified-collection NFT at a price           |
| `delist`             | Return an escrowed NFT to the seller                  |
| `purchase`           | Pay seller + fee, deliver NFT, mint reward tokens     |
| `update_marketplace` | Admin: update the fee                                 |
| `withdraw_fees`      | Admin: withdraw collected fees from the treasury      |

## Build & test

```bash
anchor build
cargo test
```
