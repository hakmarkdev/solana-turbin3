use {
    anchor_lang::{prelude::Pubkey, solana_program::instruction::Instruction},
    anchor_spl::associated_token,
    litesvm::{types::TransactionResult, LiteSVM},
    litesvm_token::CreateMint,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

mod handlers;
use handlers::*;

fn send(
    svm: &mut LiteSVM,
    ixs: &[Instruction],
    payer: &Keypair,
    signers: &[&Keypair],
) -> TransactionResult {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers).unwrap();
    svm.send_transaction(tx)
}

fn assert_ok(res: TransactionResult) {
    if let Err(ref err) = res {
        eprintln!("transaction error: {:?}", err.err);
        eprintln!("{}", err.meta.pretty_logs());
    }
    assert!(res.is_ok());
}

fn setup() -> (
    LiteSVM,
    Keypair,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
) {
    let program_id = amm::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(
        program_id,
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/deploy/amm.so"),
    )
    .unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let mint_x = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap();

    let mint_y = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap();

    let config = Pubkey::find_program_address(&[b"config", &123u64.to_le_bytes()], &program_id).0;
    let mint_lp = Pubkey::find_program_address(&[b"lp", config.as_ref()], &program_id).0;
    let vault_x = associated_token::get_associated_token_address(&config, &mint_x);
    let vault_y = associated_token::get_associated_token_address(&config, &mint_y);

    (
        svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    )
}

#[test]
fn test_initialize() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let ix = create_initialize(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    assert_ok(send(&mut svm, &[ix], &payer, &[&payer]));
}

#[test]
fn test_deposit() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init = create_initialize(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let deposit = create_deposit(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    assert_ok(send(&mut svm, &[init, deposit], &payer, &[&payer]));
}

#[test]
fn test_withdraw() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init = create_initialize(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let deposit = create_deposit(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let withdraw = create_withdraw(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    assert_ok(send(
        &mut svm,
        &[init, deposit, withdraw],
        &payer,
        &[&payer],
    ));
}

#[test]
fn test_swap() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init = create_initialize(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let deposit = create_deposit(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let swap = create_swap(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y, true,
    );
    assert_ok(send(&mut svm, &[init, deposit, swap], &payer, &[&payer]));
}

#[test]
fn test_swap_y() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init = create_initialize(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let deposit = create_deposit(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    );
    let swap = create_swap(
        &mut svm, &payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y, false,
    );
    assert_ok(send(&mut svm, &[init, deposit, swap], &payer, &[&payer]));
}
