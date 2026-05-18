use anchor_lang::{self, declare_program};
use anchor_litesvm::{AnchorContext, AnchorLiteSVM, AssertionHelpers, Signer, TestHelpers};
use spl_associated_token_account::get_associated_token_address;

declare_program!(escrow);

const DEPOSIT_AMOUNT: u64 = 1_000_000;
const RECEIVE_AMOUNT: u64 = 2_000_000;
const MINT_SUPPLY: u64 = 100_000_000;
const SEED: u64 = 12345;

const TOKEN_PROGRAM_ID: anchor_litesvm::Pubkey =
    anchor_litesvm::Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ATA_PROGRAM_ID: anchor_litesvm::Pubkey =
    anchor_litesvm::Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const SYSTEM_PROGRAM_ID: anchor_litesvm::Pubkey =
    anchor_litesvm::Pubkey::from_str_const("11111111111111111111111111111111");

struct Fixture {
    ctx: AnchorContext,
    maker: anchor_litesvm::Keypair,
    taker: anchor_litesvm::Keypair,
    mint_a: anchor_litesvm::Pubkey,
    mint_b: anchor_litesvm::Pubkey,
    maker_ata_a: anchor_litesvm::Pubkey,
    maker_ata_b: anchor_litesvm::Pubkey,
    taker_ata_a: anchor_litesvm::Pubkey,
    taker_ata_b: anchor_litesvm::Pubkey,
    escrow_pda: anchor_litesvm::Pubkey,
    vault: anchor_litesvm::Pubkey,
}

impl Fixture {
    fn new() -> Self {
        let mut ctx = AnchorLiteSVM::build_with_program(
            escrow::ID,
            include_bytes!("../../../target/deploy/escrow.so"),
        );

        let maker = ctx.svm.create_funded_account(10_000_000_000).unwrap();
        let taker = ctx.svm.create_funded_account(10_000_000_000).unwrap();

        let mint_a_kp = ctx.svm.create_token_mint(&maker, 6).unwrap();
        let mint_b_kp = ctx.svm.create_token_mint(&taker, 6).unwrap();
        let mint_a = mint_a_kp.pubkey();
        let mint_b = mint_b_kp.pubkey();

        let maker_ata_a = ctx
            .svm
            .create_associated_token_account(&mint_a, &maker)
            .unwrap();
        ctx.svm
            .mint_to(&mint_a, &maker_ata_a, &maker, MINT_SUPPLY)
            .unwrap();

        let maker_ata_b = ctx
            .svm
            .create_associated_token_account(&mint_b, &maker)
            .unwrap();
        let taker_ata_a = ctx
            .svm
            .create_associated_token_account(&mint_a, &taker)
            .unwrap();
        let taker_ata_b = ctx
            .svm
            .create_associated_token_account(&mint_b, &taker)
            .unwrap();
        ctx.svm
            .mint_to(&mint_b, &taker_ata_b, &taker, MINT_SUPPLY)
            .unwrap();

        let escrow_pda = ctx.svm.get_pda(
            &[b"escrow", maker.pubkey().as_ref(), &SEED.to_le_bytes()],
            &escrow::ID,
        );
        let vault = get_associated_token_address(&escrow_pda, &mint_a);

        Fixture {
            ctx,
            maker,
            taker,
            mint_a,
            mint_b,
            maker_ata_a,
            maker_ata_b,
            taker_ata_a,
            taker_ata_b,
            escrow_pda,
            vault,
        }
    }

    fn do_make(&mut self) {
        let ix = self
            .ctx
            .program()
            .accounts(escrow::client::accounts::Make {
                maker: self.maker.pubkey(),
                escrow: self.escrow_pda,
                mint_a: self.mint_a,
                mint_b: self.mint_b,
                maker_ata_a: self.maker_ata_a,
                vault: self.vault,
                associated_token_program: ATA_PROGRAM_ID,
                token_program: TOKEN_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(escrow::client::args::Make {
                seed: SEED,
                receive: RECEIVE_AMOUNT,
                amount: DEPOSIT_AMOUNT,
            })
            .instruction()
            .unwrap();

        self.ctx
            .execute_instruction(ix, &[&self.maker])
            .unwrap()
            .assert_success();
    }

    fn do_refund(&mut self) {
        let ix = self
            .ctx
            .program()
            .accounts(escrow::client::accounts::Refund {
                maker: self.maker.pubkey(),
                escrow: self.escrow_pda,
                mint_a: self.mint_a,
                vault: self.vault,
                maker_ata_a: self.maker_ata_a,
                associated_token_program: ATA_PROGRAM_ID,
                token_program: TOKEN_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(escrow::client::args::Refund {})
            .instruction()
            .unwrap();

        self.ctx
            .execute_instruction(ix, &[&self.maker])
            .unwrap()
            .assert_success();
    }

    fn do_take(&mut self) {
        let ix = self
            .ctx
            .program()
            .accounts(escrow::client::accounts::Take {
                taker: self.taker.pubkey(),
                maker: self.maker.pubkey(),
                escrow: self.escrow_pda,
                mint_a: self.mint_a,
                mint_b: self.mint_b,
                vault: self.vault,
                taker_ata_a: self.taker_ata_a,
                taker_ata_b: self.taker_ata_b,
                maker_ata_b: self.maker_ata_b,
                associated_token_program: ATA_PROGRAM_ID,
                token_program: TOKEN_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(escrow::client::args::Take {})
            .instruction()
            .unwrap();

        self.ctx
            .execute_instruction(ix, &[&self.taker])
            .unwrap()
            .assert_success();
    }
}

#[test]
fn test_make_deposits_tokens_and_initialises_escrow() {
    let mut f = Fixture::new();
    f.do_make();

    f.ctx.svm.assert_token_balance(&f.vault, DEPOSIT_AMOUNT);
    f.ctx
        .svm
        .assert_token_balance(&f.maker_ata_a, MINT_SUPPLY - DEPOSIT_AMOUNT);

    let state: escrow::accounts::Escrow = f.ctx.get_account(&f.escrow_pda).unwrap();
    assert_eq!(state.seed, SEED);
    assert_eq!(state.maker, f.maker.pubkey());
    assert_eq!(state.mint_a, f.mint_a);
    assert_eq!(state.mint_b, f.mint_b);
    assert_eq!(state.receive, RECEIVE_AMOUNT);
}

#[test]
fn test_refund_returns_tokens_and_closes_accounts() {
    let mut f = Fixture::new();
    f.do_make();
    f.do_refund();

    f.ctx.svm.assert_token_balance(&f.maker_ata_a, MINT_SUPPLY);
    f.ctx.svm.assert_account_closed(&f.vault);
    f.ctx.svm.assert_account_closed(&f.escrow_pda);
}

#[test]
fn test_take_swaps_tokens_and_closes_accounts() {
    let mut f = Fixture::new();
    f.do_make();
    f.do_take();

    f.ctx
        .svm
        .assert_token_balance(&f.maker_ata_b, RECEIVE_AMOUNT);
    f.ctx
        .svm
        .assert_token_balance(&f.taker_ata_b, MINT_SUPPLY - RECEIVE_AMOUNT);
    f.ctx
        .svm
        .assert_token_balance(&f.taker_ata_a, DEPOSIT_AMOUNT);
    f.ctx.svm.assert_account_closed(&f.vault);
    f.ctx.svm.assert_account_closed(&f.escrow_pda);
}
