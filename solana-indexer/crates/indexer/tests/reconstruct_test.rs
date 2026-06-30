//! Drives the state machine over init -> allow -> rotate -> mark_public and
//! asserts the emitted LineageEvent stream, the post-allow snapshot (the stale-
//! snapshot regression test), two-consecutive-rotation leaf indices, the
//! mode-prefix selection, and that reconstruct + build_proof verify against
//! zama_solana_acl's MMR oracle.

use borsh::BorshDeserialize;
use indexer::decoder::{AllowSubjectsArgs, EvAclInstruction, InitializeArgs, RotateArgs};
use indexer::lineage::proof::{self, MODE_HISTORICAL, MODE_PUBLIC};
use indexer::lineage::state::{apply, LineageShadow};
use indexer::store::repositories::lineage_repo::EventRow;
use solana_pubkey::Pubkey;
use zama_solana_acl::lineage::{reconstruct, LineageEvent};
use zama_solana_acl::{mmr_verify, MmrProof};

fn pk(tag: u8) -> Pubkey {
    Pubkey::new_from_array([tag; 32])
}
fn h(tag: u8) -> [u8; 32] {
    [tag; 32]
}

fn init(value_key: [u8; 32], handle: [u8; 32], subjects: &[Pubkey]) -> EvAclInstruction {
    EvAclInstruction::Initialize(InitializeArgs {
        value_key,
        acl_domain_key: pk(0x10),
        encrypted_value_label: h(0x11),
        handle,
        subjects: subjects.to_vec(),
    })
}
fn allow(subjects: &[Pubkey]) -> EvAclInstruction {
    EvAclInstruction::AllowSubjects(AllowSubjectsArgs {
        subjects: subjects.to_vec(),
    })
}
fn rotate(new_handle: [u8; 32], new_subjects: &[Pubkey]) -> EvAclInstruction {
    EvAclInstruction::Rotate(RotateArgs {
        new_handle,
        new_subjects: new_subjects.to_vec(),
    })
}

/// Runs a sequence through `apply`, threading the shadow state and collecting the
/// emitted events in order — exactly what the processor persists.
fn run(seq: &[EvAclInstruction]) -> (Vec<LineageEvent>, LineageShadow) {
    let mut state: Option<LineageShadow> = None;
    let mut events = Vec::new();
    for ix in seq {
        let applied = apply(state.clone(), ix).expect("apply");
        if let Some(e) = applied.event {
            events.push(e);
        }
        state = Some(applied.next);
    }
    (events, state.unwrap())
}

#[test]
fn init_allow_rotate_mark_emits_expected_stream() {
    let s1 = pk(1);
    let s2 = pk(2);
    let h10 = h(10);
    let h11 = h(11);

    let (events, state) = run(&[
        init(h(0xAA), h10, &[s1]),
        allow(&[s2]),
        rotate(h11, &[s1]),
        EvAclInstruction::MarkPublic,
    ]);

    // The rotation snapshot is the POST-allow set [s1, s2] (stale-snapshot guard),
    // and the mark records h11 (the post-rotation current handle).
    assert_eq!(
        events,
        vec![
            LineageEvent::Rotation {
                old_handle: h10,
                subjects_before_rotation: vec![s1.to_bytes(), s2.to_bytes()],
            },
            LineageEvent::MarkedPublic { handle: h11 },
        ]
    );
    // Leaf count = 2 (rotation over 2 subjects) + 1 (mark) = 3.
    assert_eq!(state.leaf_count, 3);
    assert_eq!(state.current_handle, h11);
}

#[test]
fn two_consecutive_rotations_continue_leaf_indices() {
    let s1 = pk(1);
    let s2 = pk(2);
    let s3 = pk(3);
    let acct = h(0xAC);

    let (events, _) = run(&[
        init(acct, h(10), &[s1]),
        rotate(h(11), &[s1, s2, s3]),
        rotate(h(12), &[s1]),
    ]);

    // First rotation snapshots [s1] (1 leaf), second snapshots [s1,s2,s3] (3 leaves).
    let lineage = reconstruct(acct, &events).unwrap();
    assert_eq!(lineage.leaf_count, 4);
    for i in 0..4 {
        let proof = lineage.build_proof(i).unwrap();
        assert!(mmr_verify(
            &lineage.peaks,
            lineage.leaf_count,
            lineage.leaves[i as usize],
            &proof
        ));
    }
}

#[test]
fn mode_prefix_tracks_event_kind() {
    let s1 = pk(1);
    let acct = h(0xAC);

    // mark (leaf 0, public) -> rotate over [s1] (leaf 1, historical).
    let (events, _) = run(&[
        init(acct, h(10), &[s1]),
        EvAclInstruction::MarkPublic,
        rotate(h(11), &[s1]),
    ]);
    let rows: Vec<EventRow> = events.into_iter().map(|event| EventRow { event }).collect();

    // leaf 0 came from MarkedPublic -> 0x02 public.
    let built0 = proof::build(acct, &rows, 0, None).unwrap();
    assert_eq!(built0.bytes[0], MODE_PUBLIC);
    assert_eq!(built0.leaf_count, 2);
    assert!(!built0.verified, "no on-chain data passed");

    // leaf 1 came from Rotation -> 0x01 historical.
    let built1 = proof::build(acct, &rows, 1, None).unwrap();
    assert_eq!(built1.bytes[0], MODE_HISTORICAL);

    // The Borsh body after the prefix is a valid MmrProof that verifies.
    let lineage = reconstruct(
        acct,
        &rows.iter().map(|r| r.event.clone()).collect::<Vec<_>>(),
    )
    .unwrap();
    let proof = MmrProof::try_from_slice(&built1.bytes[1..]).unwrap();
    assert!(mmr_verify(
        &lineage.peaks,
        lineage.leaf_count,
        lineage.leaves[1],
        &proof
    ));
}

#[test]
fn verified_build_cross_checks_peaks() {
    let s1 = pk(1);
    let s2 = pk(2);
    let acct = h(0xAC);
    let (events, _) = run(&[init(acct, h(10), &[s1, s2]), rotate(h(11), &[s1])]);
    let rows: Vec<EventRow> = events
        .iter()
        .cloned()
        .map(|event| EventRow { event })
        .collect();

    let lineage = reconstruct(acct, &events).unwrap();
    // Matching on-chain peaks -> verified proof.
    let built = proof::build(
        acct,
        &rows,
        0,
        Some((lineage.peaks.clone(), lineage.leaf_count)),
    )
    .unwrap();
    assert!(built.verified);

    // Divergent on-chain peaks surface as an error, not a doomed proof.
    let mut bad = lineage.peaks.clone();
    bad[0][0] ^= 0xFF;
    assert!(proof::build(acct, &rows, 0, Some((bad, lineage.leaf_count))).is_err());
}

#[test]
fn out_of_range_leaf_index_is_error() {
    let s1 = pk(1);
    let acct = h(0xAC);
    let (events, _) = run(&[init(acct, h(10), &[s1]), rotate(h(11), &[s1])]);
    let rows: Vec<EventRow> = events.into_iter().map(|event| EventRow { event }).collect();
    // Only leaf 0 exists.
    assert!(proof::build(acct, &rows, 1, None).is_err());
}
