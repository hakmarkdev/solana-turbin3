mod common;
use common::*;

#[test]
fn resolve_winning_bet_pays_2x() {
    let mut w = setup_with_house(10 * SOL);

    let seed = 42;
    let choice = 1u8;
    let place = place_bet_ix(&w, seed, 2 * SOL, choice);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let player_before = balance(&w.svm, &w.player.pubkey());
    let reveal = reveal_ix(
        &w.house_authority.pubkey(),
        bet_pda(&w.player.pubkey(), seed),
        choice,
    );
    let resolve = resolve_ix(&w, seed);
    send(
        &mut w.svm,
        &[reveal, resolve],
        &[&w.payer, &w.house_authority],
    )
    .unwrap();

    let player_after = balance(&w.svm, &w.player.pubkey());
    assert!(
        player_after > player_before + 3 * SOL,
        "winner paid out: before={player_before} after={player_after}"
    );
    assert!(
        w.svm
            .get_account(&bet_pda(&w.player.pubkey(), seed))
            .is_none(),
        "bet account closed after settle"
    );
}

#[test]
fn resolve_losing_bet_keeps_wager() {
    let mut w = setup_with_house(10 * SOL);

    let seed = 99;
    let choice = 0u8;
    let place = place_bet_ix(&w, seed, 2 * SOL, choice);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();
    let vault_before = balance(&w.svm, &w.vault);

    let reveal = reveal_ix(
        &w.house_authority.pubkey(),
        bet_pda(&w.player.pubkey(), seed),
        1,
    );
    let resolve = resolve_ix(&w, seed);
    send(
        &mut w.svm,
        &[reveal, resolve],
        &[&w.payer, &w.house_authority],
    )
    .unwrap();

    assert_eq!(
        balance(&w.svm, &w.vault),
        vault_before,
        "house keeps the wager"
    );
    assert!(
        w.svm
            .get_account(&bet_pda(&w.player.pubkey(), seed))
            .is_none(),
        "bet account closed after settle"
    );
}

#[test]
fn resolve_winning_bet_with_empty_vault_fails() {
    let mut w = setup_with_house(SOL);

    let seed = 23;
    let choice = 1u8;
    let place = place_bet_ix(&w, seed, 2 * SOL, choice);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let reveal = reveal_ix(
        &w.house_authority.pubkey(),
        bet_pda(&w.player.pubkey(), seed),
        choice,
    );
    let resolve = resolve_ix(&w, seed);
    let err = send(
        &mut w.svm,
        &[reveal, resolve],
        &[&w.payer, &w.house_authority],
    )
    .unwrap_err();
    assert_err_code(&err, DiceError::InsufficientVault);
}

#[test]
fn resolve_without_reveal_fails() {
    let mut w = setup_with_house(10 * SOL);
    let seed = 3;
    let place = place_bet_ix(&w, seed, SOL, 1);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let resolve = resolve_ix(&w, seed);
    let err = send(&mut w.svm, &[resolve], &[&w.payer]).unwrap_err();
    assert!(
        !err.is_empty(),
        "resolve must fail without a preceding reveal"
    );
}

#[test]
fn resolve_rejects_non_dice_preceding_ix() {
    let mut w = setup_with_house(10 * SOL);
    let seed = 21;
    let place = place_bet_ix(&w, seed, SOL, 1);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let preceding = system_transfer_ix(&w.payer.pubkey(), &w.player.pubkey(), SOL);
    let resolve = resolve_ix(&w, seed);
    let err = send(&mut w.svm, &[preceding, resolve], &[&w.payer]).unwrap_err();
    assert_err_code(&err, DiceError::BadRevealProgram);
}

#[test]
fn resolve_rejects_wrong_dice_instruction() {
    let mut w = setup_with_house(10 * SOL);
    let seed = 22;
    let place = place_bet_ix(&w, seed, SOL, 1);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let decoy = place_bet_ix(&w, 555, SOL, 1);
    let resolve = resolve_ix(&w, seed);
    let err = send(&mut w.svm, &[decoy, resolve], &[&w.payer, &w.player]).unwrap_err();
    assert_err_code(&err, DiceError::BadRevealDiscriminator);
}

#[test]
fn resolve_with_forged_house_signer_fails() {
    let mut w = setup_with_house(10 * SOL);
    let seed = 5;
    let place = place_bet_ix(&w, seed, SOL, 1);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let attacker = Keypair::new();
    w.svm.airdrop(&attacker.pubkey(), 10 * SOL).unwrap();
    let reveal = reveal_ix(&attacker.pubkey(), bet_pda(&w.player.pubkey(), seed), 1);
    let resolve = resolve_ix(&w, seed);
    let err = send(&mut w.svm, &[reveal, resolve], &[&w.payer, &attacker]).unwrap_err();
    assert!(!err.is_empty(), "forged house signer must be rejected");
}

#[test]
fn resolve_with_mismatched_bet_fails() {
    let mut w = setup_with_house(10 * SOL);
    let seed = 8;
    let place = place_bet_ix(&w, seed, SOL, 1);
    send(&mut w.svm, &[place], &[&w.player]).unwrap();

    let wrong_bet = bet_pda(&w.player.pubkey(), 1234);
    let reveal = reveal_ix(&w.house_authority.pubkey(), wrong_bet, 1);
    let resolve = resolve_ix(&w, seed);
    let err = send(
        &mut w.svm,
        &[reveal, resolve],
        &[&w.payer, &w.house_authority],
    )
    .unwrap_err();
    assert!(!err.is_empty(), "reveal for the wrong bet must be rejected");
}
