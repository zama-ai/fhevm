//! Off-chain decoding of zama-host instruction data and event self-CPIs,
//! built entirely from the program's own generated types (`instruction::*`
//! structs and `#[event]` discriminators), so indexers cannot drift from the
//! on-chain wire layout. Enabled by the `decode` feature; never part of the
//! SBF binary.

use anchor_lang::{AnchorDeserialize, Discriminator};

/// Anchor's `emit_cpi!` self-invocation sentinel that prefixes the event bytes
/// of an event self-CPI's instruction data.
pub use anchor_lang::event::EVENT_IX_TAG_LE;

/// One zama-host instruction an off-chain consumer reconstructs state from:
/// the `fhe_execute` batch plus the three `EncryptedValue` ACL mutations.
/// Payloads are decoded through the generated `crate::instruction` structs and
/// their `Discriminator` consts, so the fields are the handler arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZamaHostInstruction {
    FheExecute(crate::state::FheExecuteArgs),
    AllowSubjects {
        subjects: Vec<anchor_lang::prelude::Pubkey>,
    },
    RemoveSubject {
        subject: anchor_lang::prelude::Pubkey,
    },
    MakeHandlePublic {
        handle: [u8; 32],
    },
}

/// A discriminator matched one of the decoded instructions but its payload
/// did not deserialize. Consumers decide whether that halts ingestion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MalformedInstruction {
    /// Snake-case handler name, e.g. `"allow_subjects"`.
    pub instruction: &'static str,
    pub message: String,
}

/// Decodes raw instruction data (8-byte discriminator + borsh args) into one
/// of the reconstruction-relevant instructions.
///
/// Returns `Ok(None)` when the discriminator names none of them (any other
/// zama-host instruction, or a foreign program's data), and `Err` only when a
/// matched discriminator carries a malformed payload. Trailing bytes after
/// the args are accepted, matching Anchor's generated handler dispatch.
pub fn decode_instruction(
    data: &[u8],
) -> Result<Option<ZamaHostInstruction>, MalformedInstruction> {
    let Some((disc, payload)) = data.split_at_checked(8) else {
        return Ok(None);
    };
    macro_rules! arm {
        ($generated:ident, $name:literal, $build:expr) => {
            if disc == crate::instruction::$generated::DISCRIMINATOR {
                #[allow(clippy::redundant_closure_call)]
                return crate::instruction::$generated::deserialize(&mut &payload[..])
                    .map(|args| Some(($build)(args)))
                    .map_err(|error| MalformedInstruction {
                        instruction: $name,
                        message: error.to_string(),
                    });
            }
        };
    }
    arm!(
        FheExecute,
        "fhe_execute",
        |args: crate::instruction::FheExecute| { ZamaHostInstruction::FheExecute(args.args) }
    );
    arm!(
        AllowSubjects,
        "allow_subjects",
        |args: crate::instruction::AllowSubjects| {
            ZamaHostInstruction::AllowSubjects {
                subjects: args.subjects,
            }
        }
    );
    arm!(
        RemoveSubject,
        "remove_subject",
        |args: crate::instruction::RemoveSubject| {
            ZamaHostInstruction::RemoveSubject {
                subject: args.subject,
            }
        }
    );
    arm!(
        MakeHandlePublic,
        "make_handle_public",
        |args: crate::instruction::MakeHandlePublic| {
            ZamaHostInstruction::MakeHandlePublic {
                handle: args.handle,
            }
        }
    );
    Ok(None)
}

/// Whether `instruction_data` carries the `fhe_execute` discriminator. Cheaper
/// than [`decode_instruction`] for callers that only route on the instruction
/// kind before deciding whether to decode the full batch.
pub fn is_fhe_execute_instruction(instruction_data: &[u8]) -> bool {
    instruction_data.starts_with(crate::instruction::FheExecute::DISCRIMINATOR)
}

/// Whether `instruction_data` is the event self-CPI carrying event `T`
/// (Anchor event tag followed by `T`'s event discriminator).
pub fn is_event_cpi<T: Discriminator>(instruction_data: &[u8]) -> bool {
    instruction_data.starts_with(EVENT_IX_TAG_LE)
        && instruction_data.get(8..16) == Some(T::DISCRIMINATOR)
}

/// Strips the event self-CPI envelope (tag + `T`'s discriminator) and returns
/// the raw event payload, for callers that decode it with stricter rules than
/// [`decode_event_cpi`] (e.g. rejecting trailing bytes).
pub fn strip_event_cpi_envelope<T: Discriminator>(instruction_data: &[u8]) -> Option<&[u8]> {
    if !is_event_cpi::<T>(instruction_data) {
        return None;
    }
    instruction_data.get(16..)
}

/// Decodes the event self-CPI payload for event `T`, or `None` when the data
/// is not `T`'s envelope or does not deserialize. Version fields inside the
/// event are the caller's to check.
pub fn decode_event_cpi<T: Discriminator + AnchorDeserialize>(
    instruction_data: &[u8],
) -> Option<T> {
    let mut body = strip_event_cpi_envelope::<T>(instruction_data)?;
    T::deserialize(&mut body).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::AnchorSerialize;
    use sha2::{Digest, Sha256};

    /// Independent re-derivation of Anchor's discriminator scheme, so a change
    /// in the generated consts cannot slip through unnoticed.
    fn sha256_discriminator(prefix: &str, name: &str) -> [u8; 8] {
        let digest = Sha256::digest(format!("{prefix}:{name}").as_bytes());
        let mut out = [0u8; 8];
        out.copy_from_slice(&digest[..8]);
        out
    }

    #[test]
    fn generated_discriminators_match_anchor_derivation() {
        assert_eq!(
            crate::instruction::FheExecute::DISCRIMINATOR,
            sha256_discriminator("global", "fhe_execute")
        );
        assert_eq!(
            crate::instruction::AllowSubjects::DISCRIMINATOR,
            sha256_discriminator("global", "allow_subjects")
        );
        assert_eq!(
            crate::instruction::RemoveSubject::DISCRIMINATOR,
            sha256_discriminator("global", "remove_subject")
        );
        assert_eq!(
            crate::instruction::MakeHandlePublic::DISCRIMINATOR,
            sha256_discriminator("global", "make_handle_public")
        );
        assert_eq!(
            crate::events::PublicOutputsProducedEvent::DISCRIMINATOR,
            sha256_discriminator("event", "PublicOutputsProducedEvent")
        );
        assert_eq!(
            crate::events::FheExecuteRandomSeedsEvent::DISCRIMINATOR,
            sha256_discriminator("event", "FheExecuteRandomSeedsEvent")
        );
    }

    #[test]
    fn decode_instruction_roundtrips_each_variant_and_accepts_trailing_bytes() {
        let subject = anchor_lang::prelude::Pubkey::new_from_array([9; 32]);
        let cases: Vec<(Vec<u8>, ZamaHostInstruction)> = vec![
            {
                let args = crate::instruction::MakeHandlePublic { handle: [7; 32] };
                let mut data = crate::instruction::MakeHandlePublic::DISCRIMINATOR.to_vec();
                args.serialize(&mut data).unwrap();
                (
                    data,
                    ZamaHostInstruction::MakeHandlePublic { handle: [7; 32] },
                )
            },
            {
                let args = crate::instruction::RemoveSubject { subject };
                let mut data = crate::instruction::RemoveSubject::DISCRIMINATOR.to_vec();
                args.serialize(&mut data).unwrap();
                (data, ZamaHostInstruction::RemoveSubject { subject })
            },
        ];
        for (mut data, expected) in cases {
            assert_eq!(decode_instruction(&data).unwrap(), Some(expected.clone()));
            data.push(0xFF);
            assert_eq!(decode_instruction(&data).unwrap(), Some(expected));
        }
    }

    #[test]
    fn decode_instruction_skips_unknown_and_short_data() {
        assert_eq!(decode_instruction(&[0u8; 3]).unwrap(), None);
        assert_eq!(decode_instruction(&[0xAB; 16]).unwrap(), None);
    }

    #[test]
    fn matched_discriminator_with_malformed_payload_is_an_error() {
        let data = crate::instruction::AllowSubjects::DISCRIMINATOR.to_vec();
        // No borsh body at all: Vec<Pubkey> needs at least a length prefix.
        let error = decode_instruction(&data).unwrap_err();
        assert_eq!(error.instruction, "allow_subjects");
    }
}
