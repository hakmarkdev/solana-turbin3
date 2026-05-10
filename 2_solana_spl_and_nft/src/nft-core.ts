import "dotenv/config";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createGenericFile, createSignerFromKeypair, signerIdentity, generateSigner } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { create, mplCore } from "@metaplex-foundation/mpl-core";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { readFile } from "fs/promises";
import path from "path";
import wallet from "../devnet-wallet.json";

const umi = createUmi(process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com");

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));
umi.use(
    irysUploader({
        address: process.env.IRYS_URL ?? "https://devnet.irys.xyz/",
    })
);
umi.use(mplCore());

function requireArg(index: number, name: string): string {
    const v = process.argv[index];
    if (!v) {
        console.error(`Error: missing required argument <${name}>`);
        console.error(`Usage:`);
        console.error(`  tsx src/nft/nft_core.ts image <imagePath>`);
        console.error(`  tsx src/nft/nft_core.ts metadata <imageUri> <name> <description>`);
        console.error(`  tsx src/nft/nft_core.ts mint <name> <metadataUri>`);
        process.exit(1);
    }
    return v;
}

async function cmdUploadImage(imagePath: string, imageName: string, contentType: string): Promise<void> {
    const image = await readFile(imagePath);

    const file = createGenericFile(image, imageName, {
        contentType,
    });

    const [myUri] = await umi.uploader.upload([file]);
    console.log(`Image URI: ${myUri}`);
}

async function cmdUploadMetadata(imageUri: string, name: string, description: string): Promise<void> {
    const metadata = {
        name,
        description,
        image: imageUri,
        attributes: [{ trait_type: "Rarity", value: "Legendary" }],
        properties: {
            files: [
                {
                    type: "image/jpeg",
                    uri: imageUri,
                },
            ],
            category: "image",
        },
    };

    const myUri = await umi.uploader.uploadJson(metadata);
    console.log(`Metadata URI: ${myUri}`);
}

async function cmdMint(name: string, metadataUri: string): Promise<void> {
    const asset = generateSigner(umi);

    const tx = await create(umi, {
        asset,
        name,
        uri: metadataUri,
    }).sendAndConfirm(umi);

    const signature = base58.deserialize(tx.signature)[0];

    console.log(`Signature: ${signature}, Asset: ${asset.publicKey}`);
}

(async () => {
    try {
        const command = process.argv[2];

        switch (command) {
            case "image": {
                const imagePath = requireArg(3, "imagePath");
                const imageName = path.basename(imagePath);

                const extension = path.extname(imagePath).toLowerCase();
                let contentType = "image/jpeg";
                if (extension === ".png") contentType = "image/png";
                else if (extension === ".gif") contentType = "image/gif";
                else if (extension === ".webp") contentType = "image/webp";
                else if (extension === ".svg") contentType = "image/svg+xml";

                await cmdUploadImage(imagePath, imageName, contentType);
                break;
            }
            case "metadata": {
                const imageUri = requireArg(3, "imageUri");
                const name = requireArg(4, "name");
                const description = requireArg(5, "description");
                await cmdUploadMetadata(imageUri, name, description);
                break;
            }
            case "mint": {
                const name = requireArg(3, "name");
                const metadataUri = requireArg(4, "metadataUri");
                await cmdMint(name, metadataUri);
                break;
            }
            default: {
                console.error("Unknown command or missing command.");
                console.error("Usage:");
                console.error("  tsx src/nft/nft_core.ts image <imagePath>");
                console.error("  tsx src/nft/nft_core.ts metadata <imageUri> <name> <description>");
                console.error("  tsx src/nft/nft_core.ts mint <name> <metadataUri>");
                process.exit(1);
            }
        }
    } catch (error) {
        console.error(error);
    }
})();
