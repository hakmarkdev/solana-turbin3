mod common;
use common::*;

#[test]
fn place_bet_locks_wager() {
    let mut w = setup_with_house(10 * SOL);

    let seed = 1;
    let ix = place_bet_ix(&w, seed, 2 * SOL, 1);
    send(&mut w.svm, &[ix], &[&w.player]).unwrap();
    assert_eq!(balance(&w.svm, &w.vault), 12 * SOL, "wager locked in vault");

    let bet = w
        .svm
        .get_account(&bet_pda(&w.player.pubkey(), seed))
        .unwrap();
    assert_eq!(bet.owner, pid(), "bet account owned by program");
}

#[test]
fn place_bet_rejects_invalid_choice() {
    let mut w = setup_with_house(10 * SOL);
    let bad = place_bet_ix(&w, 7, SOL, 2);
    let err = send(&mut w.svm, &[bad], &[&w.player]).unwrap_err();
    assert_err_code(&err, DiceError::InvalidChoice);
}

#[test]
fn place_bet_rejects_zero_amount() {
    let mut w = setup_with_house(10 * SOL);
    let bad = place_bet_ix(&w, 11, 0, 1);
    let err = send(&mut w.svm, &[bad], &[&w.player]).unwrap_err();
    assert_err_code(&err, DiceError::InvalidAmount);
}
