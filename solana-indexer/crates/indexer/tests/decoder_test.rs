//! Hand-assembles instruction bytes for all four EV-ACL instructions and asserts
//! the decoder returns the right variant, preserves subject order, pulls the PDA
//! from accounts[2], and ignores a foreign discriminator (the silent-drop footgun).

use borsh::BorshSerialize;
use carbon_core::instruction::InstructionDecoder;
use indexer::decoder::{
    EvAclDecoder, EvAclInstruction, ALLOW_SUBJECTS_DISCRIMINATOR, INITIALIZE_DISCRIMINATOR,
    MARK_PUBLIC_DISCRIMINATOR, ROTATE_DISCRIMINATOR,
};
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

fn pk(tag: u8) -> Pubkey {
    Pubkey::new_from_array([tag; 32])
}

/// Standard 4-account layout: payer, authority, PDA (accounts[2]), system_program.
fn accounts(pda: Pubkey) -> Vec<AccountMeta> {
    vec![
        AccountMeta::new(pk(0xA0), true),
        AccountMeta::new_readonly(pk(0xA1), true),
        AccountMeta::new(pda, false),
        AccountMeta::new_readonly(pk(0xA3), false),
    ]
}

fn instruction(program_id: Pubkey, pda: Pubkey, disc: [u8; 8], args: Vec<u8>) -> Instruction {
    let mut data = disc.to_vec();
    data.extend_from_slice(&args);
    Instruction {
        program_id,
        accounts: accounts(pda),
        data,
    }
}

#[test]
fn decodes_initialize_with_subjects_in_order() {
    let program = pk(0xEE);
    let pda = pk(0x99);
    let decoder = EvAclDecoder::new(program);

    let value_key = [1u8; 32];
    let domain = pk(0x10);
    let label = [2u8; 32];
    let handle = [3u8; 32];
    let subjects = vec![pk(0x21), pk(0x22), pk(0x23)];

    // initialize(value_key, acl_domain_key, encrypted_value_label, handle, subjects)
    let mut args = Vec::new();
    value_key.serialize(&mut args).unwrap();
    domain.serialize(&mut args).unwrap();
    label.serialize(&mut args).unwrap();
    handle.serialize(&mut args).unwrap();
    subjects.serialize(&mut args).unwrap();

    let ix = instruction(program, pda, INITIALIZE_DISCRIMINATOR, args);
    let decoded = decoder.decode_instruction(&ix).expect("should decode");

    assert_eq!(
        decoded.pda,
        pda.to_bytes(),
        "PDA must come from accounts[2]"
    );
    match decoded.instruction {
        EvAclInstruction::Initialize(a) => {
            assert_eq!(a.value_key, value_key);
            assert_eq!(a.acl_domain_key, domain);
            assert_eq!(a.encrypted_value_label, label);
            assert_eq!(a.handle, handle);
            assert_eq!(a.subjects, subjects, "subject order must be preserved");
        }
        other => panic!("expected Initialize, got {other:?}"),
    }
}

#[test]
fn decodes_rotate_with_new_subjects_in_order() {
    let program = pk(0xEE);
    let pda = pk(0x99);
    let decoder = EvAclDecoder::new(program);

    let new_handle = [7u8; 32];
    let new_subjects = vec![pk(0x31), pk(0x32)];
    let mut args = Vec::new();
    new_handle.serialize(&mut args).unwrap();
    new_subjects.serialize(&mut args).unwrap();

    let ix = instruction(program, pda, ROTATE_DISCRIMINATOR, args);
    let decoded = decoder.decode_instruction(&ix).expect("should decode");
    assert_eq!(decoded.pda, pda.to_bytes());
    match decoded.instruction {
        EvAclInstruction::Rotate(a) => {
            assert_eq!(a.new_handle, new_handle);
            assert_eq!(a.new_subjects, new_subjects);
        }
        other => panic!("expected Rotate, got {other:?}"),
    }
}

#[test]
fn decodes_allow_subjects() {
    let program = pk(0xEE);
    let pda = pk(0x99);
    let decoder = EvAclDecoder::new(program);

    let subjects = vec![pk(0x41)];
    let mut args = Vec::new();
    subjects.serialize(&mut args).unwrap();

    let ix = instruction(program, pda, ALLOW_SUBJECTS_DISCRIMINATOR, args);
    let decoded = decoder.decode_instruction(&ix).expect("should decode");
    match decoded.instruction {
        EvAclInstruction::AllowSubjects(a) => assert_eq!(a.subjects, subjects),
        other => panic!("expected AllowSubjects, got {other:?}"),
    }
}

#[test]
fn decodes_mark_public_no_args() {
    let program = pk(0xEE);
    let pda = pk(0x99);
    let decoder = EvAclDecoder::new(program);

    let ix = instruction(program, pda, MARK_PUBLIC_DISCRIMINATOR, vec![]);
    let decoded = decoder.decode_instruction(&ix).expect("should decode");
    assert_eq!(decoded.pda, pda.to_bytes());
    assert_eq!(decoded.instruction, EvAclInstruction::MarkPublic);
}

#[test]
fn foreign_discriminator_is_ignored() {
    let program = pk(0xEE);
    let pda = pk(0x99);
    let decoder = EvAclDecoder::new(program);

    // A non-EV-ACL discriminator (e.g. fhe_eval's) must decode to None, not a
    // wrong variant — guards the silent-drop / wrong-decode footgun.
    let foreign = [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33];
    let ix = instruction(program, pda, foreign, vec![1, 2, 3]);
    assert!(decoder.decode_instruction(&ix).is_none());
}

#[test]
fn discriminators_match_anchor_derivation() {
    use sha2::{Digest, Sha256};

    // Anchor instruction discriminator = sha256("global:<snake_name>")[..8]. Re-derive
    // from the canonical names so a program-side rename breaks THIS test rather than
    // silently decoding the wrong variant (or nothing) at runtime.
    fn discriminator(name: &str) -> [u8; 8] {
        let digest = Sha256::digest(format!("global:{name}").as_bytes());
        digest[..8].try_into().unwrap()
    }

    assert_eq!(
        discriminator("initialize_encrypted_value_acl"),
        INITIALIZE_DISCRIMINATOR
    );
    assert_eq!(
        discriminator("rotate_encrypted_value"),
        ROTATE_DISCRIMINATOR
    );
    assert_eq!(
        discriminator("allow_encrypted_value_subjects"),
        ALLOW_SUBJECTS_DISCRIMINATOR
    );
    assert_eq!(
        discriminator("mark_encrypted_value_public"),
        MARK_PUBLIC_DISCRIMINATOR
    );
}

#[test]
fn foreign_program_is_ignored() {
    let program = pk(0xEE);
    let other_program = pk(0xCC);
    let pda = pk(0x99);
    let decoder = EvAclDecoder::new(program);

    // Right discriminator but wrong program id: not ours.
    let ix = instruction(other_program, pda, MARK_PUBLIC_DISCRIMINATOR, vec![]);
    assert!(decoder.decode_instruction(&ix).is_none());
}
