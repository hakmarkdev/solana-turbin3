# SPL and Core Scripts

A CLI toolkit for creating SPL tokens and Core NFTs.

---

## Setup

### 1. Add your wallet

```text
root/
└── devnet-wallet.json
```

### 2. Install dependencies

```bash
npm install
```

### 3. Copy the env file

```bash
cp .env.example .env
```

---

## SPL Token CLI (`spl-core.ts`)

Use the `spl-core.ts` script to manage your SPL Tokens. Run these commands in order. Each command prints the address or signature you need to pass as arguments to the next one.

| Command | Example Usage |
|---|---|
| **Init** (Creates a new mint) | `npx ts-node src/spl-core.ts init 6` |
| **Metadata** (Attaches details) | `npx ts-node src/spl-core.ts metadata <mintAddress> "MyToken" "MTK" "https://uri" 0` |
| **Mint** (Mints supply to ATA) | `npx ts-node src/spl-core.ts mint <mintAddress> 1000000` |
| **Transfer** (Sends tokens) | `npx ts-node src/spl-core.ts transfer <mintAddress> 6 <receiverAddress> 500` |

---

## NFT CLI (`nft-core.ts`)

Use the `nft-core.ts` script to manage Metaplex Core NFTs. Add your image to the project, then run the commands in order.

| Command | Example Usage |
|---|---|
| **Image** (Uploads image) | `npx ts-node src/nft-core.ts image ./my-image.png` |
| **Metadata** (Uploads JSON) | `npx ts-node src/nft-core.ts metadata <imageUri> "My NFT" "A cool NFT"` |
| **Mint** (Mints the NFT) | `npx ts-node src/nft-core.ts mint "My NFT" <metadataUri>` |
