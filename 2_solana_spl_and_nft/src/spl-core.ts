import "dotenv/config";
import {
    address,
    appendTransactionMessageInstructions,
    assertIsTransactionWithBlockhashLifetime,
    createKeyPairSignerFromBytes,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    createTransactionMessage,
    generateKeyPairSigner,
    getSignatureFromTransaction,
    sendAndConfirmTransactionFactory,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
    type Instruction,
    type KeyPairSigner
} from "@solana/kit";
import {
    getInitializeMintInstruction,
    getMintSize,
    TOKEN_PROGRAM_ADDRESS,
    findAssociatedTokenPda,
    getCreateAssociatedTokenInstructionAsync,
    getMintToInstruction,
    getTransferCheckedInstruction
} from "@solana-program/token";
import { getCreateAccountInstruction } from "@solana-program/system";
import { createSignerFromKeypair, publicKey, signerIdentity } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createMetadataAccountV3, type DataV2Args } from "@metaplex-foundation/mpl-token-metadata";
import bs58 from "bs58";
import wallet from "../devnet-wallet.json";

const RPC_URL = process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com";
const WS_URL = process.env.SOLANA_WS_URL ?? "wss://api.devnet.solana.com";

const rpc = createSolanaRpc(RPC_URL);
const rpcSubscriptions = createSolanaRpcSubscriptions(WS_URL);
const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

async function loadSigner(): Promise<KeyPairSigner> {
    return createKeyPairSignerFromBytes(new Uint8Array(wallet));
}

async function sendTx(payer: KeyPairSigner, ixs: Instruction[]): Promise<string> {
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const msg = createTransactionMessage({ version: 0 });
    const withPayer = setTransactionMessageFeePayerSigner(payer, msg);
    const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, withPayer);
    const withIxs = appendTransactionMessageInstructions(ixs, withLifetime);

    const signedTx = await signTransactionMessageWithSigners(withIxs);
    assertIsTransactionWithBlockhashLifetime(signedTx);

    const signature = getSignatureFromTransaction(signedTx);
    await sendAndConfirm(signedTx, { commitment: "confirmed" });

    return signature;
}

function requireArg(index: number, name: string): string {
    const v = process.argv[index];
    if (!v) {
        console.error(`Error: missing required argument <${name}>`);
        console.error(`Usage:`);
        console.error(`  tsx src/spl/spl_core.ts init [decimals]`);
        console.error(`  tsx src/spl/spl_core.ts metadata <mintAddress> <name> <symbol> <uri> [sellerFeeBps]`);
        console.error(`  tsx src/spl/spl_core.ts mint <mintAddress> <amount>`);
        console.error(`  tsx src/spl/spl_core.ts transfer <mintAddress> <decimals> <transferTo> <amount>`);
        process.exit(1);
    }
    return v;
}

async function cmdInit(decimals: number): Promise<void> {
    const signer = await loadSigner();
    const mint = await generateKeyPairSigner();
    const space = BigInt(getMintSize());
    const rent = await rpc.getMinimumBalanceForRentExemption(space).send();

    const signature = await sendTx(signer, [
        getCreateAccountInstruction({
            payer: signer,
            newAccount: mint,
            lamports: rent,
            space,
            programAddress: TOKEN_PROGRAM_ADDRESS
        }),
        getInitializeMintInstruction({
            mint: mint.address,
            decimals,
            mintAuthority: signer.address
        }),
    ]);

    console.log(`Mint address: ${mint.address}, Tx signature: ${signature}`);
}

async function cmdMetadata(
    mintAddress: string,
    name: string,
    symbol: string,
    uri: string,
    sellerFeeBps: number
): Promise<void> {
    const umi = createUmi(RPC_URL);
    const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
    umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

    const data: DataV2Args = {
        name,
        symbol,
        uri,
        sellerFeeBasisPoints: sellerFeeBps,
        creators: null,
        collection: null,
        uses: null
    };

    const result = await createMetadataAccountV3(umi, {
        mint: publicKey(mintAddress),
        mintAuthority: umi.identity,
        data,
        isMutable: true,
        collectionDetails: null
    }).sendAndConfirm(umi);

    console.log("Signature:", bs58.encode(Buffer.from(result.signature)));
}

async function cmdMint(mintAddress: string, amount: bigint): Promise<void> {
    const signer = await loadSigner();
    const mint = address(mintAddress);

    const [ata] = await findAssociatedTokenPda({
        mint,
        owner: signer.address,
        tokenProgram: TOKEN_PROGRAM_ADDRESS
    });
    console.log(`ATA is: ${ata}`);

    const signature = await sendTx(signer, [
        await getCreateAssociatedTokenInstructionAsync({
            payer: signer,
            mint,
            owner: signer.address
        }),
        getMintToInstruction({
            mint,
            token: ata,
            mintAuthority: signer,
            amount
        })
    ]);

    console.log(`Mint TxID: ${signature}`);
}

async function cmdTransfer(mintAddress: string, decimals: number, transferTo: string, amount: bigint): Promise<void> {
    const signer = await loadSigner();
    const mint = address(mintAddress);
    const to = address(transferTo);

    const [fromAta] = await findAssociatedTokenPda({
        mint,
        owner: signer.address,
        tokenProgram: TOKEN_PROGRAM_ADDRESS
    });
    console.log(`From ATA: ${fromAta}`);

    const [toAta] = await findAssociatedTokenPda({
        mint,
        owner: to,
        tokenProgram: TOKEN_PROGRAM_ADDRESS
    });
    console.log(`To ATA: ${toAta}`);

    const signature = await sendTx(signer, [
        await getCreateAssociatedTokenInstructionAsync({
            payer: signer,
            mint,
            owner: to
        }),
        getTransferCheckedInstruction({
            source: fromAta,
            mint,
            destination: toAta,
            authority: signer,
            amount,
            decimals
        })
    ]);

    console.log(`Transfer TxID: ${signature}`);
}

(async () => {
    try {
        const command = process.argv[2];

        switch (command) {
            case "init": {
                const arg = process.argv[3];
                const decimals = arg ? Number(arg) : 6;
                if (isNaN(decimals)) {
                    console.error("Error: Please provide a valid number for decimals.");
                    process.exit(1);
                }
                await cmdInit(decimals);
                break;
            }
            case "metadata": {
                const mintAddress = requireArg(3, "mintAddress");
                const name = requireArg(4, "name");
                const symbol = requireArg(5, "symbol");
                const uri = requireArg(6, "uri");
                const sellerFeeBps = process.argv[7] ? Number(process.argv[7]) : 0;
                await cmdMetadata(mintAddress, name, symbol, uri, sellerFeeBps);
                break;
            }
            case "mint": {
                const mintAddress = requireArg(3, "mintAddress");
                const amountStr = requireArg(4, "amount");
                const amount = BigInt(amountStr);
                await cmdMint(mintAddress, amount);
                break;
            }
            case "transfer": {
                const mintAddress = requireArg(3, "mintAddress");
                const decimalsStr = requireArg(4, "decimals");
                const transferTo = requireArg(5, "transferTo");
                const amountStr = requireArg(6, "amount");
                const decimals = Number(decimalsStr);
                const amount = BigInt(amountStr);
                await cmdTransfer(mintAddress, decimals, transferTo, amount);
                break;
            }
            default: {
                console.error("Unknown command or missing command.");
                console.error("Usage:");
                console.error("  tsx src/spl/spl_core.ts init [decimals]");
                console.error("  tsx src/spl/spl_core.ts metadata <mintAddress> <name> <symbol> <uri> [sellerFeeBps]");
                console.error("  tsx src/spl/spl_core.ts mint <mintAddress> <amount>");
                console.error("  tsx src/spl/spl_core.ts transfer <mintAddress> <decimals> <transferTo> <amount>");
                process.exit(1);
            }
        }
    } catch (error) {
        console.error(error);
    }
})();
