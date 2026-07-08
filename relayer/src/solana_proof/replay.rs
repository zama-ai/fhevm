//! Tracks per-lineage current state (`current_handle`, subjects) across a
//! chronological instruction replay, turning `DecodedInstruction`s into the
//! `zama_solana_acl::lineage::LineageEvent`s the shared crate's MMR math consumes.
//!
//! `create_encrypted_value` and `allow_subjects` mutate state but append no MMR
//! leaf (mirrors the host program). `update_encrypted_value` supersedes the
//! current handle and appends one historical-access leaf per allowed subject.
//! `make_handle_public` carries the exact public handle on-chain, so replay can
//! reconstruct public-decrypt leaves even after `fhe_eval` output handles whose
//! slot entropy is unavailable to this service. A born-public `fhe_eval` output
//! resolves that output handle from the op event `decode` correlated with it, so
//! its supersede emits `HandleSuperseded` (old handle) followed by `MarkedPublic`
//! (new output handle) — matching the on-chain leaf append order.

use zama_solana_acl::lineage::LineageEvent;

use crate::solana_proof::decode::DecodedInstruction;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    #[error("update_encrypted_value's previous_handle/previous_subjects do not match tracked state for lineage {0:x?}")]
    PreviousStateMismatch([u8; 32]),
    #[error("instruction referenced a lineage that was never created: {0:x?}")]
    UnknownLineage([u8; 32]),
    #[error("remove_subject referenced a subject that is not allowed on lineage {0:x?}")]
    SubjectNotFound([u8; 32]),
    #[error("remove_subject would remove the last subject from lineage {0:x?}")]
    LastSubjectRemoval([u8; 32]),
}

/// Per-lineage state tracked across a replay: the live handle and the full
/// allowed subject list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LineageReplayState {
    /// `None` means the lineage advanced through `fhe_eval` and this proof
    /// service did not have slot entropy to recompute the output handle. That
    /// is still enough to reconstruct later historical leaves because eval and
    /// update instructions carry the outgoing `previous_handle`.
    pub current_handle: Option<[u8; 32]>,
    /// Subject insertion order preserved — mirrors the on-chain `subjects` vector.
    pub subjects: Vec<[u8; 32]>,
}

impl LineageReplayState {
    fn upsert(&mut self, grants: &[crate::solana_proof::decode::SubjectGrant]) {
        for grant in grants {
            if !self.subjects.contains(&grant.subject) {
                self.subjects.push(grant.subject);
            }
        }
    }

    fn remove_subject(
        &mut self,
        encrypted_value: [u8; 32],
        subject: [u8; 32],
    ) -> Result<(), ReplayError> {
        if self.subjects.len() <= 1 {
            return Err(ReplayError::LastSubjectRemoval(encrypted_value));
        }
        let Some(index) = self
            .subjects
            .iter()
            .position(|candidate| *candidate == subject)
        else {
            return Err(ReplayError::SubjectNotFound(encrypted_value));
        };
        self.subjects.remove(index);
        Ok(())
    }
}

fn validate_previous_state(
    state: &LineageReplayState,
    encrypted_value: [u8; 32],
    previous_handle: [u8; 32],
    previous_subjects: &[[u8; 32]],
) -> Result<(), ReplayError> {
    if state
        .current_handle
        .is_some_and(|current_handle| current_handle != previous_handle)
        || state.subjects.as_slice() != previous_subjects
    {
        return Err(ReplayError::PreviousStateMismatch(encrypted_value));
    }
    Ok(())
}

/// Applies one decoded instruction to `state`, returning the `LineageEvent`s it
/// produces, in append order. `state` must be the tracked state for the
/// instruction's `encrypted_value` lineage (created on `CreateEncryptedValue`,
/// looked up by the caller for the others).
///
/// Most instructions produce zero or one event. A born-public `fhe_eval`
/// supersede produces two: the `HandleSuperseded` for the outgoing handle, then
/// a `MarkedPublic` for the resolved new output handle — mirroring the on-chain
/// append order (historical-access leaves, then the public-decrypt leaf).
pub fn apply_instruction(
    state: &mut Option<LineageReplayState>,
    instruction: &DecodedInstruction,
) -> Result<Vec<LineageEvent>, ReplayError> {
    match instruction {
        DecodedInstruction::CreateEncryptedValue {
            handle, subjects, ..
        } => {
            let mut new_state = LineageReplayState {
                current_handle: Some(*handle),
                subjects: Vec::new(),
            };
            new_state.upsert(subjects);
            *state = Some(new_state);
            Ok(Vec::new())
        }
        DecodedInstruction::AllowSubjects {
            encrypted_value,
            subjects,
        } => {
            let state = state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            state.upsert(subjects);
            Ok(Vec::new())
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
            validate_previous_state(state, *encrypted_value, *previous_handle, previous_subjects)?;
            let event = LineageEvent::handle_superseded(*previous_handle, &state.subjects);
            state.current_handle = Some(*new_handle);
            Ok(vec![event])
        }
        DecodedInstruction::RemoveSubject {
            encrypted_value,
            subject,
        } => {
            let state = state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            state.remove_subject(*encrypted_value, *subject)?;
            Ok(Vec::new())
        }
        DecodedInstruction::FheEvalCreateEncryptedValue {
            subjects,
            make_public_handle,
            ..
        } => {
            let mut new_state = LineageReplayState {
                current_handle: None,
                subjects: Vec::new(),
            };
            new_state.upsert(subjects);
            // Born-public on create: the resolved output handle is public
            // immediately, so append its public-decrypt leaf. Recording it as
            // `current_handle` also lets a later supersede reconstruct without
            // needing the slot entropy behind the on-chain handle derivation.
            let events = match make_public_handle {
                Some(handle) => {
                    new_state.current_handle = Some(*handle);
                    vec![LineageEvent::MarkedPublic { handle: *handle }]
                }
                None => Vec::new(),
            };
            *state = Some(new_state);
            Ok(events)
        }
        DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value,
            previous_handle,
            previous_subjects,
            output_subjects,
            make_public_handle,
        } => {
            let state = state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            validate_previous_state(state, *encrypted_value, *previous_handle, previous_subjects)?;
            if state.subjects.as_slice() != output_subjects.as_slice() {
                return Err(ReplayError::PreviousStateMismatch(*encrypted_value));
            }
            let mut events = vec![LineageEvent::handle_superseded(
                *previous_handle,
                &state.subjects,
            )];
            match make_public_handle {
                Some(handle) => {
                    state.current_handle = Some(*handle);
                    events.push(LineageEvent::MarkedPublic { handle: *handle });
                }
                None => state.current_handle = None,
            }
            Ok(events)
        }
        DecodedInstruction::MakeHandlePublic {
            encrypted_value,
            handle,
        } => {
            state
                .as_mut()
                .ok_or(ReplayError::UnknownLineage(*encrypted_value))?;
            Ok(vec![LineageEvent::MarkedPublic { handle: *handle }])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_proof::decode::SubjectGrant;
    use zama_solana_acl::{
        historical_access_leaf_commitment, lineage::reconstruct, mmr::mmr_verify,
        public_decrypt_leaf_commitment,
    };

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn replay(
        instructions: &[DecodedInstruction],
    ) -> Result<(Option<LineageReplayState>, Vec<LineageEvent>), ReplayError> {
        let mut state = None;
        let mut events = Vec::new();
        for instruction in instructions {
            events.extend(apply_instruction(&mut state, instruction)?);
        }
        Ok((state, events))
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
        assert!(apply_instruction(&mut state, &create).unwrap().is_empty());
        assert_eq!(state.as_ref().unwrap().current_handle, Some(pk(0x10)));

        let update = DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: ev,
            new_handle: pk(0x11),
            previous_handle: pk(0x10),
            previous_subjects: vec![owner],
        };
        let events = apply_instruction(&mut state, &update).unwrap();
        assert_eq!(
            events,
            vec![LineageEvent::handle_superseded(pk(0x10), &[owner])]
        );
        assert_eq!(state.as_ref().unwrap().current_handle, Some(pk(0x11)));

        let make_public = DecodedInstruction::MakeHandlePublic {
            encrypted_value: ev,
            handle: pk(0x11),
        };
        let events = apply_instruction(&mut state, &make_public).unwrap();
        assert_eq!(events, vec![LineageEvent::MarkedPublic { handle: pk(0x11) }]);
    }

    #[test]
    fn allow_subjects_grows_next_update_snapshot_to_all_allowed_subjects() {
        let ev = pk(2);
        let s1 = pk(0x30);
        let s2 = pk(0x31);
        let mut state = Some(LineageReplayState::default());
        // Bootstrap directly (skip create) to isolate allow_subjects behavior.
        state.as_mut().unwrap().current_handle = Some(pk(0x10));
        state.as_mut().unwrap().subjects.push(s1);

        // s2 becomes allowed and must appear in the next update's leaf set.
        let allow = DecodedInstruction::AllowSubjects {
            encrypted_value: ev,
            subjects: vec![SubjectGrant { subject: s2 }],
        };
        assert!(apply_instruction(&mut state, &allow).unwrap().is_empty());

        let update = DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: ev,
            new_handle: pk(0x11),
            previous_handle: pk(0x10),
            previous_subjects: vec![s1, s2],
        };
        let events = apply_instruction(&mut state, &update).unwrap();
        assert_eq!(
            events,
            vec![LineageEvent::handle_superseded(pk(0x10), &[s1, s2])]
        );
    }

    #[test]
    fn fhe_eval_supersession_reconstructs_same_leaves_as_update_path() {
        let ev = pk(0x01);
        let owner = pk(0x30);
        let spender = pk(0x31);
        let create = DecodedInstruction::CreateEncryptedValue {
            encrypted_value: ev,
            handle: pk(0x10),
            subjects: vec![
                SubjectGrant { subject: owner },
                SubjectGrant { subject: spender },
            ],
        };
        let update = DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: ev,
            new_handle: pk(0x11),
            previous_handle: pk(0x10),
            previous_subjects: vec![owner, spender],
        };
        let eval_update = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x10),
            previous_subjects: vec![owner, spender],
            output_subjects: vec![owner, spender],
            make_public_handle: None,
        };

        let (_, update_events) = replay(&[create.clone(), update]).unwrap();
        let (_, eval_events) = replay(&[create, eval_update]).unwrap();

        assert_eq!(
            eval_events,
            vec![LineageEvent::handle_superseded(pk(0x10), &[owner, spender])]
        );
        let update_reconstructed = reconstruct(ev, &update_events).unwrap();
        let eval_reconstructed = reconstruct(ev, &eval_events).unwrap();
        assert_eq!(
            eval_reconstructed.leaf_count,
            update_reconstructed.leaf_count
        );
        assert_eq!(eval_reconstructed.peaks, update_reconstructed.peaks);
        assert_eq!(eval_reconstructed.leaves, update_reconstructed.leaves);
        assert_eq!(
            eval_reconstructed.leaves,
            vec![
                historical_access_leaf_commitment(ev, 0, pk(0x10), owner),
                historical_access_leaf_commitment(ev, 1, pk(0x10), spender),
            ]
        );
    }

    #[test]
    fn fhe_eval_create_initializes_subjects_for_later_eval_supersession() {
        let ev = pk(0x05);
        let owner = pk(0x30);
        let create = DecodedInstruction::FheEvalCreateEncryptedValue {
            encrypted_value: ev,
            subjects: vec![SubjectGrant { subject: owner }],
            make_public_handle: None,
        };
        let eval_update = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x10),
            previous_subjects: vec![owner],
            output_subjects: vec![owner],
            make_public_handle: None,
        };

        let (state, events) = replay(&[create, eval_update]).unwrap();

        assert_eq!(state.unwrap().current_handle, None);
        assert_eq!(
            events,
            vec![LineageEvent::handle_superseded(pk(0x10), &[owner])]
        );
    }

    #[test]
    fn make_public_after_fhe_eval_create_uses_decoded_handle() {
        let ev = pk(0x06);
        let owner = pk(0x30);
        let handle = pk(0x44);
        let create = DecodedInstruction::FheEvalCreateEncryptedValue {
            encrypted_value: ev,
            subjects: vec![SubjectGrant { subject: owner }],
            make_public_handle: None,
        };
        let make_public = DecodedInstruction::MakeHandlePublic {
            encrypted_value: ev,
            handle,
        };

        let (state, events) = replay(&[create, make_public]).unwrap();
        let reconstructed = reconstruct(ev, &events).unwrap();
        let proof = reconstructed
            .build_verified_proof(&reconstructed.peaks, reconstructed.leaf_count, 0)
            .unwrap();

        assert_eq!(state.unwrap().current_handle, None);
        assert_eq!(events, vec![LineageEvent::MarkedPublic { handle }]);
        assert_eq!(
            reconstructed.leaves,
            vec![public_decrypt_leaf_commitment(ev, 0, handle)]
        );
        assert!(mmr_verify(
            &reconstructed.peaks,
            reconstructed.leaf_count,
            reconstructed.leaves[0],
            &proof
        ));
    }

    #[test]
    fn multi_output_fhe_eval_appends_historical_leaves_in_instruction_order() {
        let ev = pk(0x02);
        let owner = pk(0x30);
        let create = DecodedInstruction::CreateEncryptedValue {
            encrypted_value: ev,
            handle: pk(0x10),
            subjects: vec![SubjectGrant { subject: owner }],
        };
        let first_eval_update = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x10),
            previous_subjects: vec![owner],
            output_subjects: vec![owner],
            make_public_handle: None,
        };
        let second_eval_update = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x11),
            previous_subjects: vec![owner],
            output_subjects: vec![owner],
            make_public_handle: None,
        };

        let (_, events) = replay(&[create, first_eval_update, second_eval_update]).unwrap();
        let reconstructed = reconstruct(ev, &events).unwrap();

        assert_eq!(
            events,
            vec![
                LineageEvent::handle_superseded(pk(0x10), &[owner]),
                LineageEvent::handle_superseded(pk(0x11), &[owner]),
            ]
        );
        assert_eq!(
            reconstructed.leaves,
            vec![
                historical_access_leaf_commitment(ev, 0, pk(0x10), owner),
                historical_access_leaf_commitment(ev, 1, pk(0x11), owner),
            ]
        );
    }

    #[test]
    fn remove_subject_before_fhe_eval_excludes_removed_subject_from_historical_leaves() {
        let ev = pk(0x03);
        let owner = pk(0x30);
        let removed = pk(0x31);
        let create = DecodedInstruction::CreateEncryptedValue {
            encrypted_value: ev,
            handle: pk(0x10),
            subjects: vec![
                SubjectGrant { subject: owner },
                SubjectGrant { subject: removed },
            ],
        };
        let remove = DecodedInstruction::RemoveSubject {
            encrypted_value: ev,
            subject: removed,
        };
        let eval_update = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x10),
            previous_subjects: vec![owner],
            output_subjects: vec![owner],
            make_public_handle: None,
        };

        let (_, events) = replay(&[create, remove, eval_update]).unwrap();
        let reconstructed = reconstruct(ev, &events).unwrap();

        assert_eq!(reconstructed.leaf_count, 1);
        assert_eq!(
            reconstructed.leaves,
            vec![historical_access_leaf_commitment(ev, 0, pk(0x10), owner)]
        );
        assert_ne!(
            reconstructed.leaves[0],
            historical_access_leaf_commitment(ev, 0, pk(0x10), removed)
        );
    }

    #[test]
    fn eval_driven_historical_leaf_builds_a_verifiable_mmr_proof() {
        let ev = pk(0x04);
        let owner = pk(0x30);
        let create = DecodedInstruction::CreateEncryptedValue {
            encrypted_value: ev,
            handle: pk(0x10),
            subjects: vec![SubjectGrant { subject: owner }],
        };
        let eval_update = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x10),
            previous_subjects: vec![owner],
            output_subjects: vec![owner],
            make_public_handle: None,
        };

        let (_, events) = replay(&[create, eval_update]).unwrap();
        let reconstructed = reconstruct(ev, &events).unwrap();
        let proof = reconstructed
            .build_verified_proof(&reconstructed.peaks, reconstructed.leaf_count, 0)
            .unwrap();

        assert!(mmr_verify(
            &reconstructed.peaks,
            reconstructed.leaf_count,
            reconstructed.leaves[0],
            &proof
        ));
    }

    #[test]
    fn update_with_stale_previous_state_is_rejected() {
        let ev = pk(3);
        let mut state = Some(LineageReplayState {
            current_handle: Some(pk(0x10)),
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
            handle: pk(0x20),
        };
        assert_eq!(
            apply_instruction(&mut state, &make_public),
            Err(ReplayError::UnknownLineage(ev))
        );
    }
}
