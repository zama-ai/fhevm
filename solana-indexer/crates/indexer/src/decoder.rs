//! Hand-written Carbon `InstructionDecoder` for the four encrypted-value-ACL
//! host instructions.
//!
//! TODO(codegen): the four EV-ACL instructions are absent from the checked-in
//! Anchor IDL at `coprocessor/fhevm-engine/host-listener/idl/zama_host.json`
//! (it carries 20 unrelated instructions), so `carbon-cli parse` cannot generate
//! them today. Once they are exported into that IDL, regenerate with
//! `npx @sevenlabs-hq/carbon-cli parse -i <idl> -o <out> -s anchor` and replace
//! this file. The layouts below are owned by us and verified against
//! `solana/programs/zama-host/src/instructions/encrypted_value_acl.rs`.
//!
//! Anchor instruction discriminator = `sha256("global:<snake_name>")[..8]`; args
//! that follow are Borsh-encoded. `Pubkey` args are decoded with the Solana 3.x
//! `solana_pubkey::Pubkey` that Carbon resolves, then converted to `[u8; 32]` at
//! the `zama_solana_acl` boundary via `to_bytes()`.

use borsh::{BorshDeserialize, BorshSerialize};
use carbon_core::instruction::InstructionDecoder;
use solana_pubkey::Pubkey;

/// `sha256("global:initialize_encrypted_value_acl")[..8]`.
pub const INITIALIZE_DISCRIMINATOR: [u8; 8] = [164, 86, 4, 52, 199, 75, 55, 213];
/// `sha256("global:rotate_encrypted_value")[..8]`.
pub const ROTATE_DISCRIMINATOR: [u8; 8] = [43, 245, 176, 89, 185, 32, 50, 229];
/// `sha256("global:allow_encrypted_value_subjects")[..8]`.
pub const ALLOW_SUBJECTS_DISCRIMINATOR: [u8; 8] = [177, 184, 49, 199, 142, 199, 142, 238];
/// `sha256("global:mark_encrypted_value_public")[..8]`.
pub const MARK_PUBLIC_DISCRIMINATOR: [u8; 8] = [10, 93, 118, 108, 158, 99, 18, 102];

/// Borsh args of `initialize_encrypted_value_acl` (after the 8-byte discriminator).
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitializeArgs {
    pub value_key: [u8; 32],
    pub acl_domain_key: Pubkey,
    pub encrypted_value_label: [u8; 32],
    pub handle: [u8; 32],
    pub subjects: Vec<Pubkey>,
}

/// Borsh args of `rotate_encrypted_value`.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RotateArgs {
    pub new_handle: [u8; 32],
    pub new_subjects: Vec<Pubkey>,
}

/// Borsh args of `allow_encrypted_value_subjects`.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct AllowSubjectsArgs {
    pub subjects: Vec<Pubkey>,
}

/// A decoded EV-ACL instruction. `mark_encrypted_value_public` carries no args.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvAclInstruction {
    Initialize(InitializeArgs),
    Rotate(RotateArgs),
    AllowSubjects(AllowSubjectsArgs),
    MarkPublic,
}

/// The shared account layout of all four instructions (verified against the
/// `#[derive(Accounts)]` structs): `[0]=payer, [1]=app_account_authority,
/// [2]=encrypted_value_acl (the PDA), [3]=system_program`. The lineage PDA — the
/// durable per-lineage key — is always `accounts[2]`.
pub const PDA_ACCOUNT_INDEX: usize = 2;

/// A decoded instruction bundled with the lineage PDA pulled from `accounts[2]`.
///
/// `rotate`/`allow`/`mark` do NOT carry `value_key` in their args; only the PDA
/// is always present, so it is the key the processor persists state on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedEvAcl {
    pub instruction: EvAclInstruction,
    pub pda: [u8; 32],
}

/// Hand-written Carbon decoder scoped to the `zama-host` EV-ACL instructions.
pub struct EvAclDecoder {
    pub program_id: Pubkey,
}

impl EvAclDecoder {
    pub fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
}

impl<'a> InstructionDecoder<'a> for EvAclDecoder {
    type InstructionType = DecodedEvAcl;

    fn decode_instruction(
        &self,
        instruction: &'a solana_instruction::Instruction,
    ) -> Option<Self::InstructionType> {
        // Scope to our program. A foreign program (or a foreign discriminator
        // from our own program) returns None and is silently skipped for this pipe.
        if instruction.program_id != self.program_id {
            return None;
        }
        if instruction.data.len() < 8 {
            return None;
        }
        let (disc, mut args) = instruction.data.split_at(8);
        let disc: [u8; 8] = disc.try_into().ok()?;

        let decoded = match disc {
            INITIALIZE_DISCRIMINATOR => {
                EvAclInstruction::Initialize(InitializeArgs::deserialize(&mut args).ok()?)
            }
            ROTATE_DISCRIMINATOR => {
                EvAclInstruction::Rotate(RotateArgs::deserialize(&mut args).ok()?)
            }
            ALLOW_SUBJECTS_DISCRIMINATOR => {
                EvAclInstruction::AllowSubjects(AllowSubjectsArgs::deserialize(&mut args).ok()?)
            }
            MARK_PUBLIC_DISCRIMINATOR => EvAclInstruction::MarkPublic,
            _ => return None,
        };

        // The PDA is arranged-accounts[2] on every EV-ACL instruction.
        let pda = instruction
            .accounts
            .get(PDA_ACCOUNT_INDEX)?
            .pubkey
            .to_bytes();

        Some(DecodedEvAcl {
            instruction: decoded,
            pda,
        })
    }
}
