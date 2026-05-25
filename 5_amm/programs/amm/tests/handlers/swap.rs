use {
    amm::accounts::Swap,
    amm::instruction::Swap as SwapIx,
    anchor_lang::{prelude::Pubkey, solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    anchor_spl::{associated_token, token},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_signer::Signer,
};

pub fn create_swap(
    _svm: &mut LiteSVM,
    payer: &Keypair,
    mint_x: Pubkey,
    mint_y: Pubkey,
    config: Pubkey,
    mint_lp: Pubkey,
    vault_x: Pubkey,
    vault_y: Pubkey,
    is_x: bool,
) -> Instruction {
    let user = payer.pubkey();
    let user_x = associated_token::get_associated_token_address(&user, &mint_x);
    let user_y = associated_token::get_associated_token_address(&user, &mint_y);

    Instruction::new_with_bytes(
        amm::id(),
        &SwapIx {
            is_x,
            amount_in: 10_000_000,
            min_amount_out: 1,
        }
        .data(),
        Swap {
            user,
            mint_x,
            mint_y,
            config,
            mint_lp,
            vault_x,
            vault_y,
            user_x,
            user_y,
            token_program: token::ID,
        }
        .to_account_metas(None),
    )
}
