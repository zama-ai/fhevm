//! Direct ZamaHost instruction builders for low-level runtime tests.

use anchor_lang::prelude::system_program;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use zama_host as host;
use zama_host::{FheFrameAction, FheFrameStep};

use crate::{
    acl::event_authority,
    acl::rand_counter_address,
    transaction::anchor_ix,
};

pub fn execute_frame_ix(
    program_id: Pubkey,
    payer: Pubkey,
    steps: Vec<FheFrameStep>,
    actions: Vec<FheFrameAction>,
    authorized_app_accounts: Vec<Pubkey>,
    remaining_accounts: Vec<Pubkey>,
) -> Instruction {
    let mut ix = anchor_ix(
        program_id,
        host::accounts::ExecuteFrame {
            payer,
            compute_subject: payer,
            system_program: system_program::ID,
            rand_counter: rand_counter_address(program_id),
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::ExecuteFrame {
            authorized_app_accounts,
            steps,
            actions,
        },
    );
    ix.accounts.extend(
        remaining_accounts
            .into_iter()
            .map(|pubkey| AccountMeta::new(pubkey, false)),
    );
    ix
}

pub fn allow_for_decryption_ix(
    program_id: Pubkey,
    authority: Pubkey,
    acl_record: Pubkey,
    handle: [u8; 32],
) -> Instruction {
    anchor_ix(
        program_id,
        host::accounts::AllowForDecryption {
            authority,
            acl_record,
            event_authority: event_authority(program_id),
            program: program_id,
        },
        host::instruction::AllowForDecryption { handle },
    )
}

pub fn label(name: &str) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let bytes = name.as_bytes();
    assert!(bytes.len() <= out.len());
    out[..bytes.len()].copy_from_slice(bytes);
    out
}
