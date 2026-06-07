#![allow(dead_code)]

pub use {dice_bet::error::DiceError, solana_keypair::Keypair, solana_signer::Signer};

use {
    anchor_lang::{InstructionData, ToAccountMetas},
    dice_bet::state::RevealArgs,
    litesvm::LiteSVM,
    solana_instruction::Instruction,
    solana_instructions_sysvar::ID as INSTRUCTIONS_SYSVAR_ID,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_transaction::versioned::VersionedTransaction,
};

pub const SYSTEM_PROGRAM: Pubkey = solana_pubkey::pubkey!("11111111111111111111111111111111");
pub const SOL: u64 = 1_000_000_000;

pub struct World {
    pub svm: LiteSVM,
    pub payer: Keypair,
    pub house_authority: Keypair,
    pub player: Keypair,
    pub house: Pubkey,
    pub vault: Pubkey,
}

pub fn pid() -> Pubkey {
    dice_bet::ID
}

pub fn pda(seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, &pid()).0
}

pub fn bet_pda(player: &Pubkey, seed: u64) -> Pubkey {
    pda(&[b"bet", player.as_ref(), &seed.to_le_bytes()])
}

pub fn balance(svm: &LiteSVM, k: &Pubkey) -> u64 {
    svm.get_account(k).map(|a| a.lamports).unwrap_or(0)
}

pub fn send(svm: &mut LiteSVM, ixs: &[Instruction], signers: &[&Keypair]) -> Result<(), String> {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(&signers[0].pubkey()), &blockhash);
    let signer_refs: Vec<&Keypair> = signers.to_vec();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &signer_refs)
        .map_err(|e| e.to_string())?;
    svm.send_transaction(tx)
        .map(|_| ())
        .map_err(|e| format!("{:?}", e.err))
}

pub fn setup() -> World {
    let mut svm = LiteSVM::new();
    svm.add_program(
        pid(),
        include_bytes!("../../../../target/deploy/dice_bet.so"),
    )
    .unwrap();

    let payer = Keypair::new();
    let house_authority = Keypair::new();
    let player = Keypair::new();
    for k in [&payer, &house_authority, &player] {
        svm.airdrop(&k.pubkey(), 100 * SOL).unwrap();
    }

    World {
        svm,
        payer,
        house_authority,
        player,
        house: pda(&[b"house"]),
        vault: pda(&[b"vault"]),
    }
}

pub fn setup_with_house(bankroll: u64) -> World {
    let mut w = setup();
    let init = init_house_ix(&w, bankroll);
    send(&mut w.svm, &[init], &[&w.house_authority]).unwrap();
    w
}

pub fn assert_err_code(err: &str, expected: DiceError) {
    let code = u32::from(expected);
    assert!(
        err.contains(&code.to_string()),
        "expected error code {code}, got: {err}"
    );
}

pub fn init_house_ix(w: &World, bankroll: u64) -> Instruction {
    Instruction {
        program_id: pid(),
        data: dice_bet::instruction::InitializeHouse { bankroll }.data(),
        accounts: dice_bet::accounts::InitializeHouse {
            authority: w.house_authority.pubkey(),
            house: w.house,
            vault: w.vault,
            system_program: SYSTEM_PROGRAM,
        }
        .to_account_metas(None),
    }
}

pub fn place_bet_ix(w: &World, seed: u64, amount: u64, choice: u8) -> Instruction {
    Instruction {
        program_id: pid(),
        data: dice_bet::instruction::PlaceBet {
            seed,
            amount,
            choice,
        }
        .data(),
        accounts: dice_bet::accounts::PlaceBet {
            player: w.player.pubkey(),
            house: w.house,
            vault: w.vault,
            bet: bet_pda(&w.player.pubkey(), seed),
            system_program: SYSTEM_PROGRAM,
        }
        .to_account_metas(None),
    }
}

pub fn reveal_ix(signer: &Pubkey, bet: Pubkey, roll: u8) -> Instruction {
    Instruction {
        program_id: pid(),
        data: dice_bet::instruction::Reveal {
            args: RevealArgs { bet, roll },
        }
        .data(),
        accounts: dice_bet::accounts::Reveal {
            house_authority: *signer,
        }
        .to_account_metas(None),
    }
}

pub fn system_transfer_ix(from: &Pubkey, to: &Pubkey, lamports: u64) -> Instruction {
    solana_system_interface::instruction::transfer(from, to, lamports)
}

pub fn resolve_ix(w: &World, seed: u64) -> Instruction {
    Instruction {
        program_id: pid(),
        data: dice_bet::instruction::ResolveBet {}.data(),
        accounts: dice_bet::accounts::ResolveBet {
            player: w.player.pubkey(),
            house: w.house,
            vault: w.vault,
            bet: bet_pda(&w.player.pubkey(), seed),
            instructions: INSTRUCTIONS_SYSVAR_ID,
            system_program: SYSTEM_PROGRAM,
        }
        .to_account_metas(None),
    }
}
