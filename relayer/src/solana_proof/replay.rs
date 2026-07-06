//! Tracks per-lineage current state (`current_handle`, subjects) across a
//! chronological instruction replay, turning `DecodedInstruction`s into the
//! `zama_solana_acl::lineage::LineageEvent`s the shared crate's MMR math consumes.
//!
//! `create_encrypted_value` and `allow_subjects` mutate state but append no MMR
//! leaf (mirrors the host program). `update_encrypted_value` supersedes the
//! current handle and appends one historical-access leaf per allowed subject.
//! `make_handle_public` carries no args on-chain; its leaf's handle is this
//! replayer's tracked `current_handle` at the time it executes.

use zama_solana_acl::lineage::LineageEvent;

use crate::solana_proof::decode::DecodedInstruction;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    #[error("update_encrypted_value's previous_handle/previous_subjects do not match tracked state for lineage {0:x?}")]
    PreviousStateMismatch([u8; 32]),
    #[error("instruction referenced a lineage that was never created: {0:x?}")]
    UnknownLineage([u8; 32]),
}

/// Per-lineage state tracked across a replay: the live handle and the full
/// allowed subject list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LineageReplayState {
    pub current_handle: [u8; 32],
    /// Subject insertion order preserved — mirrors the on-chain `subjects` vector.
    pub subjects: Vec<[u8; 32]>,
}

impl LineageReplayState {
    fn subject_pubkeys(&self) -> Vec<[u8; 32]> {
        self.subjects.clone()
    }

    fn upsert(&mut self, grants: &[crate::solana_proof::decode::SubjectGrant]) {
        for grant in grants {
            if !self.subjects.contains(&grant.subject) {
                self.subjects.push(grant.subject);
            }
        }
    }
}

/// Applies one decoded instruction to `state`, returning the `LineageEvent` it
/// produces, if any. `state` must be the tracked state for the instruction's
/// `encrypted_value` lineage (created on `CreateEncryptedValue`, looked up by
/// the caller for the others).
pub fn apply_instruction(
    state: &mut Option<LineageReplayState>,
    instruction: &DecodedInstruction,
) -> Result<Option<LineageEvent>, ReplayError> {
    match instruction {
        DecodedInstruction::CreateEncryptedValue {
            handle, subjects, ..
        } => {
            let mut new_state = LineageReplayState {
                current_handle: *handle,
                subjects: Vec::new(),
            };
            new_state.upsert(subjects);
            *state = Some(new_state);
            Ok(None)
        }
        DecodedInstruction::AllowSubjects {
            encrypted_value,
            subjects,
        } => {
            let state = state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            state.upsert(subjects);
            Ok(None)
        }
        DecodedInstruction::UpdateEncryptedValue {
            encrypted_value,
            new_handle,
            previous_handle,
            previous_subjects,
        } => {
            let state = state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            if state.current_handle != *previous_handle
                || &state.subject_pubkeys() != previous_subjects
            {
                return Err(ReplayError::PreviousStateMismatch(*encrypted_value));
            }
            let event = LineageEvent::handle_superseded(*previous_handle, &state.subjects);
            state.current_handle = *new_handle;
            Ok(Some(event))
        }
        DecodedInstruction::MakeHandlePublic { encrypted_value } => {
            let state = state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            Ok(Some(LineageEvent::MarkedPublic {
                handle: state.current_handle,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_proof::decode::SubjectGrant;

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    #[test]
    fn create_then_update_then_make_public_produces_expected_events_and_handles() {
        let ev = pk(1);
        let owner = pk(0x30);
        let mut state: Option<LineageReplayState> = None;

        let create = DecodedInstruction::CreateEncryptedValue {
            encrypted_value: ev,
            handle: pk(0x10),
            subjects: vec![SubjectGrant { subject: owner }],
        };
        assert_eq!(apply_instruction(&mut state, &create).unwrap(), None);
        assert_eq!(state.as_ref().unwrap().current_handle, pk(0x10));

        let update = DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: ev,
            new_handle: pk(0x11),
            previous_handle: pk(0x10),
            previous_subjects: vec![owner],
        };
        let event = apply_instruction(&mut state, &update).unwrap().unwrap();
        assert_eq!(event, LineageEvent::handle_superseded(pk(0x10), &[owner]));
        assert_eq!(state.as_ref().unwrap().current_handle, pk(0x11));

        let make_public = DecodedInstruction::MakeHandlePublic {
            encrypted_value: ev,
        };
        let event = apply_instruction(&mut state, &make_public)
            .unwrap()
            .unwrap();
        assert_eq!(event, LineageEvent::MarkedPublic { handle: pk(0x11) });
    }

    #[test]
    fn allow_subjects_grows_next_update_snapshot_to_all_allowed_subjects() {
        let ev = pk(2);
        let s1 = pk(0x30);
        let s2 = pk(0x31);
        let mut state = Some(LineageReplayState::default());
        // Bootstrap directly (skip create) to isolate allow_subjects behavior.
        state.as_mut().unwrap().current_handle = pk(0x10);
        state.as_mut().unwrap().subjects.push(s1);

        // s2 becomes allowed and must appear in the next update's leaf set.
        let allow = DecodedInstruction::AllowSubjects {
            encrypted_value: ev,
            subjects: vec![SubjectGrant { subject: s2 }],
        };
        assert_eq!(apply_instruction(&mut state, &allow).unwrap(), None);

        let update = DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: ev,
            new_handle: pk(0x11),
            previous_handle: pk(0x10),
            previous_subjects: vec![s1, s2],
        };
        let event = apply_instruction(&mut state, &update).unwrap().unwrap();
        assert_eq!(event, LineageEvent::handle_superseded(pk(0x10), &[s1, s2]));
    }

    #[test]
    fn update_with_stale_previous_state_is_rejected() {
        let ev = pk(3);
        let mut state = Some(LineageReplayState {
            current_handle: pk(0x10),
            subjects: vec![pk(0x30)],
        });
        let update = DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: ev,
            new_handle: pk(0x11),
            previous_handle: pk(0xFF), // wrong
            previous_subjects: vec![pk(0x30)],
        };
        assert_eq!(
            apply_instruction(&mut state, &update),
            Err(ReplayError::PreviousStateMismatch(ev))
        );
    }

    #[test]
    fn instruction_on_unknown_lineage_is_rejected() {
        let ev = pk(4);
        let mut state: Option<LineageReplayState> = None;
        let make_public = DecodedInstruction::MakeHandlePublic {
            encrypted_value: ev,
        };
        assert_eq!(
            apply_instruction(&mut state, &make_public),
            Err(ReplayError::UnknownLineage(ev))
        );
    }
}
