//! The per-lineage state transition: turns a decoded EV-ACL instruction plus the
//! current shadow state into the LineageEvent(s) to append and the next state.
//!
//! Pure function over `(current_state, instruction)` — no I/O. The processor owns
//! reading the prior state and persisting the result atomically with the cursor.
//!
//! The on-chain append order this mirrors (verified against the host program):
//! - `initialize`: set (handle, subjects); appends NO leaf.
//! - `allow_subjects`: extend subjects (distinct, capacity-bounded); appends NO leaf.
//! - `rotate`: emit `Rotation { old_handle = current_handle,
//!   subjects_before_rotation = current_subjects }` (one leaf per subject for the
//!   OLD handle), THEN set (new_handle, new_subjects).
//! - `mark_public`: emit `MarkedPublic { handle = current_handle }` (one leaf),
//!   leaving handle/subjects unchanged.

use zama_solana_acl::lineage::LineageEvent;
use zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS;

use crate::decoder::EvAclInstruction;

/// The mutable membership of one lineage, as the next rotate/mark will snapshot it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineageShadow {
    pub value_key: Option<[u8; 32]>,
    pub current_handle: [u8; 32],
    pub current_subjects: Vec<[u8; 32]>,
    /// Number of MMR leaves appended so far (== events' total leaf count).
    pub leaf_count: u64,
}

/// The result of applying one instruction: zero or one event to append, and the
/// shadow state to persist afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applied {
    pub event: Option<LineageEvent>,
    pub next: LineageShadow,
}

/// Applies `instruction` to `prior` (the shadow state before this instruction).
///
/// `prior` is `None` only for `initialize` on a never-seen lineage; for the other
/// three it must be `Some` (the processor synthesizes it from the chain on a
/// partial-backfill miss before calling this). Returns the event to log (if any)
/// and the post-instruction shadow.
pub fn apply(
    prior: Option<LineageShadow>,
    instruction: &EvAclInstruction,
) -> anyhow::Result<Applied> {
    match instruction {
        EvAclInstruction::Initialize(args) => {
            let subjects: Vec<[u8; 32]> = args.subjects.iter().map(|p| p.to_bytes()).collect();
            Ok(Applied {
                event: None,
                next: LineageShadow {
                    value_key: Some(args.value_key),
                    current_handle: args.handle,
                    current_subjects: subjects,
                    leaf_count: 0,
                },
            })
        }
        EvAclInstruction::AllowSubjects(args) => {
            let mut state = require_prior(prior, "allow_encrypted_value_subjects")?;
            // Mirror the host: append distinct subjects, ignore duplicates,
            // bounded by capacity. Order is insertion order.
            for subject in &args.subjects {
                let bytes = subject.to_bytes();
                if state.current_subjects.contains(&bytes) {
                    continue;
                }
                if state.current_subjects.len() >= MAX_ENCRYPTED_VALUE_SUBJECTS {
                    break;
                }
                state.current_subjects.push(bytes);
            }
            Ok(Applied {
                event: None,
                next: state,
            })
        }
        EvAclInstruction::Rotate(args) => {
            let mut state = require_prior(prior, "rotate_encrypted_value")?;
            // Snapshot BEFORE applying: old handle + the post-allow subject set.
            let event = LineageEvent::Rotation {
                old_handle: state.current_handle,
                subjects_before_rotation: state.current_subjects.clone(),
            };
            // The rotation appends one leaf per snapshotted subject.
            state.leaf_count += state.current_subjects.len() as u64;
            state.current_handle = args.new_handle;
            state.current_subjects = args.new_subjects.iter().map(|p| p.to_bytes()).collect();
            Ok(Applied {
                event: Some(event),
                next: state,
            })
        }
        EvAclInstruction::MarkPublic => {
            let mut state = require_prior(prior, "mark_encrypted_value_public")?;
            let event = LineageEvent::MarkedPublic {
                handle: state.current_handle,
            };
            state.leaf_count += 1;
            Ok(Applied {
                event: Some(event),
                next: state,
            })
        }
    }
}

fn require_prior(prior: Option<LineageShadow>, ix: &str) -> anyhow::Result<LineageShadow> {
    prior.ok_or_else(|| anyhow::anyhow!("{ix} arrived for an uninitialized lineage"))
}
