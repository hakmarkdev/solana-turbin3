use {
    amm::accounts::Withdraw,
    amm::instruction::Withdraw as WithdrawIx,
    anchor_lang::{prelude::Pubkey, solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    anchor_spl::{associated_token, token},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_signer::Signer,
};

pub fn create_withdraw(
    _svm: &mut LiteSVM,
    payer: &Keypair,
    mint_x: Pubkey,
    mint_y: Pubkey,
    config: Pubkey,
    mint_lp: Pubkey,
    vault_x: Pubkey,
    vault_y: Pubkey,
) -> Instruction {
    let user = payer.pubkey();
    let user_x = associated_token::get_associated_token_address(&user, &mint_x);
    let user_y = associated_token::get_associated_token_address(&user, &mint_y);
    let user_lp = associated_token::get_associated_token_address(&user, &mint_lp);

    Instruction::new_with_bytes(
        amm::id(),
        &WithdrawIx {
            amount: 50_000_000,
            min_x: 1,
            min_y: 1,
        }
        .data(),
        Withdraw {
            user,
            mint_x,
            mint_y,
            config,
            mint_lp,
            vault_x,
            vault_y,
            user_x,
            user_y,
            user_lp,
            token_program: token::ID,
        }
        .to_account_metas(None),
    )
}
