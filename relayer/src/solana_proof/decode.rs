//! Decodes zama-host `EncryptedValue` instructions from raw compiled instruction
//! data (Anchor discriminator + borsh args), independent of transaction/RPC
//! shape so it can be unit-tested against synthetic data.

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

/// A single compiled instruction, already resolved to full account pubkeys
/// (32-byte, no base58). Produced from either a transaction's top-level
/// `message.instructions` or a `meta.innerInstructions` entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawInstruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

/// One subject grant as carried in `create_encrypted_value`/`allow_subjects` args.
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubjectGrant {
    pub subject: [u8; 32],
    pub role_flags: u8,
}

/// `ACL_ROLE_USE` from zama-host's `constants.rs` — the role that causes a
/// subject to receive a historical-access leaf on supersession.
pub const ACL_ROLE_USE: u8 = 0x01;

/// The zama-host `EncryptedValue` instruction, decoded from one compiled instruction.
///
/// The `encrypted_value` account is index 2 (`accounts[2]`) in all four
/// instructions' account lists (payer, authority-like signer, encrypted_value,
/// ...), independent of whether a trailing `Option` account is present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodedInstruction {
    CreateEncryptedValue {
        encrypted_value: [u8; 32],
        handle: [u8; 32],
        subjects: Vec<SubjectGrant>,
    },
    AllowSubjects {
        encrypted_value: [u8; 32],
        subjects: Vec<SubjectGrant>,
    },
    UpdateEncryptedValue {
        encrypted_value: [u8; 32],
        new_handle: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
    },
    MakeHandlePublic {
        encrypted_value: [u8; 32],
    },
}

impl DecodedInstruction {
    pub fn encrypted_value(&self) -> [u8; 32] {
        match self {
            DecodedInstruction::CreateEncryptedValue {
                encrypted_value, ..
            }
            | DecodedInstruction::AllowSubjects {
                encrypted_value, ..
            }
            | DecodedInstruction::UpdateEncryptedValue {
                encrypted_value, ..
            }
            | DecodedInstruction::MakeHandlePublic { encrypted_value } => *encrypted_value,
        }
    }
}

/// Anchor-style 8-byte global instruction discriminator: `sha256("global:<name>")[..8]`.
fn discriminator(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("global:{name}").as_bytes());
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

const ENCRYPTED_VALUE_ACCOUNT_INDEX: usize = 2;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    #[error("instruction data shorter than the 8-byte discriminator")]
    DataTooShort,
    #[error("unrecognized discriminator (not a zama-host EncryptedValue instruction)")]
    UnknownDiscriminator,
    #[error("missing account at index {0} (encrypted_value)")]
    MissingAccount(usize),
    #[error("borsh decode failed: {0}")]
    Borsh(String),
}

/// Decodes a `RawInstruction` known to target the zama-host program id.
/// Returns `Ok(None)` for zama-host instructions this module does not care
/// about (e.g. `fhe_eval`), and `Err` only for malformed data.
pub fn decode_instruction(ix: &RawInstruction) -> Result<Option<DecodedInstruction>, DecodeError> {
    if ix.data.len() < 8 {
        return Err(DecodeError::DataTooShort);
    }
    let (disc, mut body) = ix.data.split_at(8);

    let encrypted_value = || {
        ix.accounts
            .get(ENCRYPTED_VALUE_ACCOUNT_INDEX)
            .copied()
            .ok_or(DecodeError::MissingAccount(ENCRYPTED_VALUE_ACCOUNT_INDEX))
    };

    if disc == discriminator("create_encrypted_value") {
        let acl_domain_key = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let app_account = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let _ = (acl_domain_key, app_account);
        let _encrypted_value_label = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let subjects = <Vec<SubjectGrant>>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(Some(DecodedInstruction::CreateEncryptedValue {
            encrypted_value: encrypted_value()?,
            handle,
            subjects,
        }))
    } else if disc == discriminator("allow_subjects") {
        let subjects = <Vec<SubjectGrant>>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(Some(DecodedInstruction::AllowSubjects {
            encrypted_value: encrypted_value()?,
            subjects,
        }))
    } else if disc == discriminator("update_encrypted_value") {
        let new_handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let previous_handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let previous_subjects = <Vec<[u8; 32]>>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(Some(DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: encrypted_value()?,
            new_handle,
            previous_handle,
            previous_subjects,
        }))
    } else if disc == discriminator("make_handle_public") {
        Ok(Some(DecodedInstruction::MakeHandlePublic {
            encrypted_value: encrypted_value()?,
        }))
    } else {
        // Not one of the four EncryptedValue instructions we track (e.g. fhe_eval).
        Ok(None)
    }
}

fn borsh_err(e: std::io::Error) -> DecodeError {
    DecodeError::Borsh(e.to_string())
}

/// Decodes every instruction in a transaction targeting `program_id`, in the
/// order supplied (caller is responsible for top-level/inner interleaving —
/// see `chain::flatten_execution_order`). Ignores instructions for other programs.
pub fn decode_program_instructions(
    program_id: [u8; 32],
    instructions: &[RawInstruction],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let mut out = Vec::new();
    for ix in instructions {
        if ix.program_id != program_id {
            continue;
        }
        if let Some(decoded) = decode_instruction(ix)? {
            out.push(decoded);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::BorshSerialize;

    fn program_id() -> [u8; 32] {
        [7u8; 32]
    }

    fn ix_with_data(
        accounts: Vec<[u8; 32]>,
        name: &str,
        args: impl BorshSerialize,
    ) -> RawInstruction {
        let mut data = discriminator(name).to_vec();
        args.serialize(&mut data).unwrap();
        RawInstruction {
            program_id: program_id(),
            accounts,
            data,
        }
    }

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    #[test]
    fn decodes_create_encrypted_value() {
        let ev = pk(1);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        #[derive(BorshSerialize)]
        struct Args {
            acl_domain_key: [u8; 32],
            app_account: [u8; 32],
            label: [u8; 32],
            handle: [u8; 32],
            subjects: Vec<SubjectGrant>,
        }
        let args = Args {
            acl_domain_key: pk(0x10),
            app_account: pk(0x11),
            label: pk(0x12),
            handle: pk(0x20),
            subjects: vec![SubjectGrant {
                subject: pk(0x30),
                role_flags: ACL_ROLE_USE,
            }],
        };
        let ix = ix_with_data(accounts, "create_encrypted_value", args);
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::CreateEncryptedValue {
                encrypted_value: ev,
                handle: pk(0x20),
                subjects: vec![SubjectGrant {
                    subject: pk(0x30),
                    role_flags: ACL_ROLE_USE
                }],
            }
        );
    }

    #[test]
    fn decodes_update_encrypted_value() {
        let ev = pk(2);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        #[derive(BorshSerialize)]
        struct Args {
            new_handle: [u8; 32],
            previous_handle: [u8; 32],
            previous_subjects: Vec<[u8; 32]>,
        }
        let args = Args {
            new_handle: pk(0x21),
            previous_handle: pk(0x20),
            previous_subjects: vec![pk(0x30), pk(0x31)],
        };
        let ix = ix_with_data(accounts, "update_encrypted_value", args);
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::UpdateEncryptedValue {
                encrypted_value: ev,
                new_handle: pk(0x21),
                previous_handle: pk(0x20),
                previous_subjects: vec![pk(0x30), pk(0x31)],
            }
        );
    }

    #[test]
    fn decodes_make_handle_public_with_no_args() {
        let ev = pk(3);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        let ix = ix_with_data(accounts, "make_handle_public", ());
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::MakeHandlePublic {
                encrypted_value: ev
            }
        );
    }

    #[test]
    fn ignores_unknown_discriminator() {
        let mut data = [9u8; 8].to_vec();
        data.extend_from_slice(&[1, 2, 3]);
        let ix = RawInstruction {
            program_id: program_id(),
            accounts: vec![pk(0), pk(0), pk(0)],
            data,
        };
        assert_eq!(decode_instruction(&ix).unwrap(), None);
    }

    #[test]
    fn other_program_instructions_are_skipped_by_the_program_filter() {
        let ev = pk(4);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        let mut ix = ix_with_data(accounts, "make_handle_public", ());
        ix.program_id = pk(0xFF); // a different (CPI caller) program
        let decoded = decode_program_instructions(program_id(), &[ix]).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn rejects_data_shorter_than_discriminator() {
        let ix = RawInstruction {
            program_id: program_id(),
            accounts: vec![],
            data: vec![1, 2, 3],
        };
        assert_eq!(decode_instruction(&ix), Err(DecodeError::DataTooShort));
    }
}
