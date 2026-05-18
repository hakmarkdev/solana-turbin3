use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

fn vault_pda(signer: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"vault", signer.as_ref()], &vault::id()).0
}

fn vault_accounts(signer: &Pubkey) -> Vec<AccountMeta> {
    vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(vault_pda(signer), false),
        AccountMeta::new_readonly(system_program::ID, false),
    ]
}

fn create_svm() -> LiteSVM {
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault.so");
    svm.add_program(vault::id(), bytes).unwrap();
    svm
}

fn make_deposit_ix(signer: &Pubkey, amount: u64) -> Instruction {
    Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Deposit { amount }.data(),
        vault_accounts(signer),
    )
}

fn make_withdraw_ix(signer: &Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Withdraw {}.data(),
        vault_accounts(signer),
    )
}

fn build_tx(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) -> VersionedTransaction {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap()
}

fn send_tx(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) {
    let tx = build_tx(svm, payer, ix);
    svm.send_transaction(tx).expect("transaction failed");
}

fn try_send_tx(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) -> bool {
    let tx = build_tx(svm, payer, ix);
    svm.send_transaction(tx).is_ok()
}

fn lamports(svm: &LiteSVM, pubkey: &Pubkey) -> u64 {
    svm.get_account(pubkey).map(|a| a.lamports).unwrap_or(0)
}

#[test]
fn test_deposit_success() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let deposit_amount = 1_000_000_000u64;
    send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), deposit_amount),
    );

    let vault = vault_pda(&payer.pubkey());
    assert_eq!(lamports(&svm, &vault), deposit_amount);
}

#[test]
fn test_deposit_twice_fails() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let amount = 1_000_000_000u64;
    assert!(try_send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), amount)
    ));
    assert!(!try_send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), amount)
    ));
}

#[test]
fn test_deposit_below_rent_minimum_fails() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    assert!(!try_send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), 1)
    ));
}

#[test]
fn test_deposit_at_exact_rent_minimum_fails() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let rent_min = svm.minimum_balance_for_rent_exemption(0);
    assert!(!try_send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), rent_min)
    ));

    svm.expire_blockhash();
    assert!(try_send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), rent_min + 1)
    ));
}

#[test]
fn test_withdraw_success() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let deposit_amount = 2_000_000_000u64;
    send_tx(
        &mut svm,
        &payer,
        make_deposit_ix(&payer.pubkey(), deposit_amount),
    );

    let vault = vault_pda(&payer.pubkey());
    assert_eq!(lamports(&svm, &vault), deposit_amount);

    let balance_before = lamports(&svm, &payer.pubkey());
    send_tx(&mut svm, &payer, make_withdraw_ix(&payer.pubkey()));

    assert_eq!(
        lamports(&svm, &vault),
        0,
        "vault should be empty after withdrawal"
    );
    assert!(
        lamports(&svm, &payer.pubkey()) > balance_before,
        "signer should receive deposited lamports back"
    );
}

#[test]
fn test_withdraw_empty_vault_fails() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    assert!(!try_send_tx(
        &mut svm,
        &payer,
        make_withdraw_ix(&payer.pubkey())
    ));
}

#[test]
fn test_redeposit_after_full_withdraw() {
    let mut svm = create_svm();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let amount = 1_000_000_000u64;
    let vault = vault_pda(&payer.pubkey());

    send_tx(&mut svm, &payer, make_deposit_ix(&payer.pubkey(), amount));
    assert_eq!(lamports(&svm, &vault), amount);

    send_tx(&mut svm, &payer, make_withdraw_ix(&payer.pubkey()));
    assert_eq!(lamports(&svm, &vault), 0);
    svm.expire_blockhash();

    send_tx(&mut svm, &payer, make_deposit_ix(&payer.pubkey(), amount));
    assert_eq!(lamports(&svm, &vault), amount);
}

#[test]
fn test_cannot_steal_anothers_vault() {
    let mut svm = create_svm();
    let alice = Keypair::new();
    let bob = Keypair::new();
    svm.airdrop(&alice.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&bob.pubkey(), 10_000_000_000).unwrap();

    let amount = 1_000_000_000u64;
    send_tx(&mut svm, &alice, make_deposit_ix(&alice.pubkey(), amount));

    let alice_vault = vault_pda(&alice.pubkey());
    let steal_ix = Instruction::new_with_bytes(
        vault::id(),
        &vault::instruction::Withdraw {}.data(),
        vec![
            AccountMeta::new(bob.pubkey(), true),
            AccountMeta::new(alice_vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    assert!(!try_send_tx(&mut svm, &bob, steal_ix));
    assert_eq!(lamports(&svm, &alice_vault), amount);
}

#[test]
fn test_different_users_have_independent_vaults() {
    let mut svm = create_svm();
    let alice = Keypair::new();
    let bob = Keypair::new();
    svm.airdrop(&alice.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&bob.pubkey(), 10_000_000_000).unwrap();

    let amount = 1_000_000_000u64;

    send_tx(&mut svm, &alice, make_deposit_ix(&alice.pubkey(), amount));
    send_tx(&mut svm, &bob, make_deposit_ix(&bob.pubkey(), amount));

    let alice_vault = vault_pda(&alice.pubkey());
    let bob_vault = vault_pda(&bob.pubkey());

    assert_ne!(
        alice_vault, bob_vault,
        "each user gets a distinct vault PDA"
    );
    assert_eq!(lamports(&svm, &alice_vault), amount);
    assert_eq!(lamports(&svm, &bob_vault), amount);

    send_tx(&mut svm, &alice, make_withdraw_ix(&alice.pubkey()));

    assert_eq!(
        lamports(&svm, &alice_vault),
        0,
        "Alice's vault should be empty"
    );
    assert_eq!(
        lamports(&svm, &bob_vault),
        amount,
        "Bob's vault should be untouched"
    );
}
