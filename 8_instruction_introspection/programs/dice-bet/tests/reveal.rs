mod common;
use common::*;

#[test]
fn reveal_with_valid_roll_succeeds() {
    let mut w = setup();
    let reveal = reveal_ix(
        &w.house_authority.pubkey(),
        bet_pda(&w.player.pubkey(), 1),
        1,
    );
    send(&mut w.svm, &[reveal], &[&w.house_authority]).unwrap();
}

#[test]
fn reveal_rejects_invalid_roll() {
    let mut w = setup();
    let reveal = reveal_ix(
        &w.house_authority.pubkey(),
        bet_pda(&w.player.pubkey(), 1),
        2,
    );
    let err = send(&mut w.svm, &[reveal], &[&w.house_authority]).unwrap_err();
    assert_err_code(&err, DiceError::InvalidRoll);
}
