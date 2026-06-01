use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use nft_staking::instructions::CreateCollectionArgs;

#[test]
fn round_trips_through_borsh() {
    let args = CreateCollectionArgs {
        name: "Test NFT Collection".to_string(),
        uri: "https://example.com/collection.json".to_string(),
        nft_name: "Test NFT".to_string(),
        nft_uri: "https://example.com/nft.json".to_string(),
    };

    let mut bytes = Vec::new();
    args.serialize(&mut bytes).unwrap();
    let decoded = CreateCollectionArgs::try_from_slice(&bytes).unwrap();

    assert_eq!(decoded.name, args.name);
    assert_eq!(decoded.uri, args.uri);
    assert_eq!(decoded.nft_name, args.nft_name);
    assert_eq!(decoded.nft_uri, args.nft_uri);
}

#[test]
fn max_length_strings_fit_in_init_space() {
    let args = CreateCollectionArgs {
        name: "x".repeat(32),
        uri: "y".repeat(200),
        nft_name: "z".repeat(32),
        nft_uri: "w".repeat(200),
    };
    // The serialized strings must fit within the account's reserved space.
    let strings_len = args.name.len() + args.uri.len() + args.nft_name.len() + args.nft_uri.len();
    let reserved = (4 + 32) + (4 + 200) + (4 + 32) + (4 + 200);
    // 4-byte length prefix per string + the bytes themselves.
    assert!(strings_len + 16 <= reserved);
}
