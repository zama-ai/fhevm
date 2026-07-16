//! Polls zama-host program transactions, decodes `EncryptedValue` instructions,
//! replays per-lineage state, and appends the resulting `LineageEvent`s to the
//! `LeafStore`. Also provides a targeted single-lineage catch-up path used when
//! a proof build finds the store behind the live chain account.

use std::collections::HashSet;

use zama_solana_acl::lineage::LineageEvent;

use crate::solana_proof::chain::ChainFetcher;
use crate::solana_proof::decode::{decode_program_instructions, DecodedInstruction};
use crate::solana_proof::replay::{apply_instruction, LineageReplayState, ReplayError};
use crate::solana_proof::store::{Cursor, LeafStore, LineageSignatureUpdate, SignatureUpdate};

#[derive(thiserror::Error, Debug)]
pub enum IngestError {
    #[error("chain error: {0}")]
    Chain(#[from] crate::solana_proof::chain::ChainError),
    #[error("store error: {0}")]
    Store(#[from] crate::solana_proof::store::StoreError),
    #[error("decode error: {0}")]
    Decode(#[from] crate::solana_proof::decode::DecodeError),
    #[error("replay error: {0}")]
    Replay(#[from] crate::solana_proof::replay::ReplayError),
    #[error("transaction {signature} is temporarily unavailable")]
    TransactionUnavailable { signature: String },
    #[error("poll backlog exceeds the per-cycle ceiling ({max}); refusing to advance to avoid silently skipping signatures — run an explicit backfill")]
    BacklogExceeded { max: usize },
}

/// Generous per-poll-cycle backlog ceiling. The poller ingests the whole
/// contiguous range newer than the cursor each cycle (oldest-first, gap-free);
/// this is only a fail-closed backstop against an unbounded backfill/OOM on a
/// pathological history. Exceeding it errors rather than skipping.
pub const MAX_POLL_BACKLOG_PER_CYCLE: usize = 10_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatchUpOutcome {
    pub processed: usize,
    pub budget_exhausted: bool,
}

struct SignatureWindow {
    signatures: Vec<String>,
    budget_exhausted: bool,
}

/// How to bound the matching signatures collected in one `fetch_signature_window`
/// call. Both variants stop at the same count; they differ only in what happens
/// when the backlog exceeds it.
enum Overflow {
    /// Poller: fail closed (`BacklogExceeded`) rather than silently skip. The
    /// whole range up to `max` is ingested oldest-first with no gap.
    Ceiling(usize),
    /// On-demand catch-up: signal `budget_exhausted` (a retryable lag) when more
    /// than `budget` signatures remain, so the caller reports lag, not an error.
    Budget(usize),
}

impl Overflow {
    fn max(&self) -> usize {
        match self {
            Overflow::Ceiling(m) | Overflow::Budget(m) => *m,
        }
    }

    /// The window to return once the bound is hit (or `page_limit`/`max` is zero).
    fn hit(&self) -> Result<SignatureWindow, IngestError> {
        match self {
            Overflow::Budget(_) => Ok(SignatureWindow {
                signatures: Vec::new(),
                budget_exhausted: true,
            }),
            Overflow::Ceiling(max) => Err(IngestError::BacklogExceeded { max: *max }),
        }
    }
}

/// Fetches signatures for `address` newer than `until`, paging newest-first, and
/// returns them oldest-first so events apply in chain order. Probes one signature
/// past `overflow.max()` to distinguish "exactly at the bound" from "more remain".
async fn fetch_signature_window<C, F>(
    fetcher: &C,
    address: [u8; 32],
    until: Option<&str>,
    page_limit: usize,
    overflow: Overflow,
    mut stop_at: F,
) -> Result<SignatureWindow, IngestError>
where
    C: ChainFetcher,
    F: FnMut(&str) -> bool,
{
    let max = overflow.max();
    if page_limit == 0 || max == 0 {
        return overflow.hit();
    }

    let mut before: Option<String> = None;
    let mut newest_first = Vec::new();
    loop {
        // Ask for one extra beyond what remains of `max`, so a page can reveal
        // whether the backlog overflows the bound.
        let limit = page_limit.min(max.saturating_sub(newest_first.len()).saturating_add(1));

        let page = fetcher
            .get_signatures_for_address(address, before.as_deref(), until, limit)
            .await?;
        if page.is_empty() {
            break;
        }

        let next_before = page.last().cloned();
        let mut stopped = false;
        for signature in &page {
            if stop_at(signature) {
                stopped = true;
                break;
            }
            if newest_first.len() == max {
                // A signature beyond `max` exists before reaching `until`.
                return overflow.hit();
            }
            newest_first.push(signature.clone());
        }

        if stopped || page.len() < limit {
            break;
        }
        before = next_before;
    }

    newest_first.reverse();
    Ok(SignatureWindow {
        signatures: newest_first,
        budget_exhausted: false,
    })
}

fn unique_lineages(instructions: &[DecodedInstruction]) -> Vec<[u8; 32]> {
    let mut lineages = Vec::new();
    for instruction in instructions {
        let lineage = instruction.encrypted_value();
        if !lineages.contains(&lineage) {
            lineages.push(lineage);
        }
    }
    lineages
}

fn recovered_already_applied_event(
    state: &Option<LineageReplayState>,
    instruction: &DecodedInstruction,
    error: &ReplayError,
) -> Option<Vec<LineageEvent>> {
    let state = state.as_ref()?;
    match (instruction, error) {
        (
            DecodedInstruction::UpdateEncryptedValue {
                encrypted_value,
                new_handle,
                previous_handle,
                previous_subjects,
            },
            ReplayError::PreviousStateMismatch(error_lineage),
        ) if error_lineage == encrypted_value
            && state.current_handle == Some(*new_handle)
            && state.subjects.as_slice() == previous_subjects.as_slice() =>
        {
            Some(vec![LineageEvent::handle_superseded(
                *previous_handle,
                previous_subjects,
            )])
        }
        (
            DecodedInstruction::FheEvalUpdateEncryptedValue {
                encrypted_value,
                previous_handle,
                previous_subjects,
                output_subjects,
                make_public_handle,
            },
            ReplayError::PreviousStateMismatch(error_lineage),
        ) if error_lineage == encrypted_value
            && state.current_handle == *make_public_handle
            && state.subjects.as_slice() == output_subjects.as_slice() =>
        {
            // Already applied: the lineage is at this supersede's post-state (a rotation moved
            // subjects to `output_subjects`, so a re-apply trips PreviousStateMismatch). Replay
            // the same leaves the fresh apply would have — the outgoing audience sealed first,
            // then the born-public leaf if any — mirroring the raw UpdateEncryptedValue arm.
            let mut events = vec![LineageEvent::handle_superseded(
                *previous_handle,
                previous_subjects,
            )];
            if let Some(handle) = make_public_handle {
                events.push(LineageEvent::MarkedPublic { handle: *handle });
            }
            Some(events)
        }
        (
            DecodedInstruction::RemoveSubject {
                encrypted_value,
                subject,
            },
            ReplayError::SubjectNotFound(error_lineage),
        ) if error_lineage == encrypted_value && !state.subjects.contains(subject) => {
            Some(Vec::new())
        }
        _ => None,
    }
}

fn apply_or_recover_instruction(
    state: &mut Option<LineageReplayState>,
    instruction: &DecodedInstruction,
) -> Result<Vec<LineageEvent>, ReplayError> {
    let original = state.clone();
    match apply_instruction(state, instruction) {
        Ok(event) => Ok(event),
        Err(error) => {
            if let Some(event) = recovered_already_applied_event(&original, instruction, &error) {
                *state = original;
                Ok(event)
            } else {
                Err(error)
            }
        }
    }
}

async fn build_lineage_signature_update<S: LeafStore>(
    store: &S,
    lineage: [u8; 32],
    signature: &str,
    instructions: &[DecodedInstruction],
    mark_even_without_instruction: bool,
) -> Result<Option<LineageSignatureUpdate>, IngestError> {
    if store
        .get_seen_signatures(lineage)
        .await?
        .iter()
        .any(|seen| seen == signature)
    {
        return Ok(None);
    }

    let mut state = store.get_replay_state(lineage).await?;
    let mut events = Vec::new();
    let mut touched = false;

    for instruction in instructions
        .iter()
        .filter(|instruction| instruction.encrypted_value() == lineage)
    {
        touched = true;
        events.extend(apply_or_recover_instruction(&mut state, instruction)?);
    }

    if !touched && !mark_even_without_instruction {
        return Ok(None);
    }

    Ok(Some(LineageSignatureUpdate {
        lineage,
        replay_state: state,
        events,
    }))
}

/// One program-wide poll cycle: fetches signatures newer than the stored
/// cursor, decodes+replays each transaction's instructions oldest-to-newest,
/// and persists the new cursor. `program_id` and `signature_source_address`
/// are the same account for zama-host today (program-wide polling uses the
/// program id itself as the address argument to `getSignaturesForAddress`).
pub async fn poll_once<C: ChainFetcher, S: LeafStore>(
    fetcher: &C,
    store: &S,
    program_id: [u8; 32],
    page_limit: usize,
    max_backlog: usize,
) -> Result<usize, IngestError> {
    let cursor = store.get_cursor().await?;
    let until = cursor.as_ref().and_then(|c| c.last_signature.as_deref());
    // Ingest the whole contiguous range newer than the cursor, oldest-first, so
    // no signature is skipped; a backlog past the ceiling fails closed instead.
    let signatures = fetch_signature_window(
        fetcher,
        program_id,
        until,
        page_limit,
        Overflow::Ceiling(max_backlog),
        |sig| until == Some(sig),
    )
    .await?
    .signatures;

    let mut processed = 0usize;

    for signature in &signatures {
        let tx = fetcher.get_transaction(signature).await?.ok_or_else(|| {
            IngestError::TransactionUnavailable {
                signature: signature.clone(),
            }
        })?;
        let decoded = decode_program_instructions(program_id, &tx.instructions)?;
        let mut lineages = Vec::new();
        for lineage in unique_lineages(&decoded) {
            if let Some(update) =
                build_lineage_signature_update(store, lineage, signature, &decoded, false).await?
            {
                lineages.push(update);
            }
        }
        store
            .apply_signature_update(SignatureUpdate {
                signature: signature.clone(),
                lineages,
                cursor: Some(Cursor {
                    last_signature: Some(signature.clone()),
                    last_slot: tx.slot,
                }),
            })
            .await?;
        processed += 1;
    }

    Ok(processed)
}

/// Targeted catch-up for one lineage: polls signatures for the lineage's own
/// PDA address (Solana indexes signatures per touched account, not just the
/// invoked program), decodes+replays any not already recorded for it, and
/// appends the resulting events. Used when a proof build finds the store's
/// reconstructed peaks behind the live on-chain account.
pub async fn catch_up_lineage<C: ChainFetcher, S: LeafStore>(
    fetcher: &C,
    store: &S,
    program_id: [u8; 32],
    lineage: [u8; 32],
    signature_budget: usize,
) -> Result<CatchUpOutcome, IngestError> {
    let seen: HashSet<String> = store
        .get_seen_signatures(lineage)
        .await?
        .into_iter()
        .collect();
    let window = fetch_signature_window(
        fetcher,
        lineage,
        None,
        1000,
        Overflow::Budget(signature_budget),
        |sig| seen.contains(sig),
    )
    .await?;
    if window.budget_exhausted {
        return Ok(CatchUpOutcome {
            processed: 0,
            budget_exhausted: true,
        });
    }

    let mut processed = 0usize;

    for signature in window.signatures {
        if seen.contains(&signature) {
            continue;
        }
        let tx = fetcher.get_transaction(&signature).await?.ok_or_else(|| {
            IngestError::TransactionUnavailable {
                signature: signature.clone(),
            }
        })?;
        let decoded = decode_program_instructions(program_id, &tx.instructions)?;
        let Some(lineage_update) =
            build_lineage_signature_update(store, lineage, &signature, &decoded, true).await?
        else {
            continue;
        };
        let result = store
            .apply_signature_update(SignatureUpdate {
                signature: signature.clone(),
                lineages: vec![lineage_update],
                cursor: None,
            })
            .await?;
        if result.applied_lineages > 0 {
            processed += 1;
        }
    }
    Ok(CatchUpOutcome {
        processed,
        budget_exhausted: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_proof::chain::{ChainError, ChainTransaction, OnChainLineageState};
    use crate::solana_proof::decode::{RawInstruction, SubjectGrant};
    use crate::solana_proof::store::FileLeafStore;
    use async_trait::async_trait;
    use borsh::BorshSerialize;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::sync::Mutex;

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn discriminator(name: &str) -> [u8; 8] {
        let digest = Sha256::digest(format!("global:{name}").as_bytes());
        let mut out = [0u8; 8];
        out.copy_from_slice(&digest[..8]);
        out
    }

    fn make_ix(
        program_id: [u8; 32],
        accounts: Vec<[u8; 32]>,
        name: &str,
        args: impl BorshSerialize,
    ) -> RawInstruction {
        let mut data = discriminator(name).to_vec();
        args.serialize(&mut data).unwrap();
        RawInstruction {
            program_id,
            accounts,
            data,
            top_level_index: 0,
            stack_height: Some(1),
        }
    }

    fn update_ix(
        program_id: [u8; 32],
        lineage: [u8; 32],
        new_handle: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
    ) -> RawInstruction {
        #[derive(BorshSerialize)]
        struct Args {
            new_handle: [u8; 32],
            previous_handle: [u8; 32],
            previous_subjects: Vec<[u8; 32]>,
        }
        make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "update_encrypted_value",
            Args {
                new_handle,
                previous_handle,
                previous_subjects,
            },
        )
    }

    #[test]
    fn recovers_already_applied_fhe_eval_rotation_idempotently() {
        let ev = pk(0x40);
        let owner = pk(0x30);
        let old_recipient = pk(0x31);
        let new_recipient = pk(0x32);
        // State already advanced to the post-rotation audience by a prior application.
        let mut state = Some(LineageReplayState {
            current_handle: None,
            subjects: vec![owner, new_recipient],
        });
        let instruction = DecodedInstruction::FheEvalUpdateEncryptedValue {
            encrypted_value: ev,
            previous_handle: pk(0x10),
            previous_subjects: vec![owner, old_recipient],
            output_subjects: vec![owner, new_recipient],
            make_public_handle: None,
        };

        let events = apply_or_recover_instruction(&mut state, &instruction).unwrap();

        // Replays the outgoing-audience leaves without double-advancing the state.
        assert_eq!(
            events,
            vec![LineageEvent::handle_superseded(
                pk(0x10),
                &[owner, old_recipient]
            )]
        );
        assert_eq!(state.unwrap().subjects, vec![owner, new_recipient]);
    }

    fn make_public_ix(program_id: [u8; 32], lineage: [u8; 32], handle: [u8; 32]) -> RawInstruction {
        make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "make_handle_public",
            handle,
        )
    }

    /// In-memory `ChainFetcher` fake: signatures indexed by address (superset
    /// per-lineage + all-under-program), transactions by signature.
    struct FakeChain {
        program_id: [u8; 32],
        signatures_by_address: Mutex<HashMap<[u8; 32], Vec<String>>>,
        transactions: Mutex<HashMap<String, ChainTransaction>>,
        lineage_state: Mutex<HashMap<[u8; 32], OnChainLineageState>>,
    }

    impl FakeChain {
        fn new(program_id: [u8; 32]) -> Self {
            Self {
                program_id,
                signatures_by_address: Mutex::new(HashMap::new()),
                transactions: Mutex::new(HashMap::new()),
                lineage_state: Mutex::new(HashMap::new()),
            }
        }

        fn push_tx(
            &self,
            signature: &str,
            slot: u64,
            touched: &[[u8; 32]],
            instructions: Vec<RawInstruction>,
        ) {
            self.transactions.lock().unwrap().insert(
                signature.to_string(),
                ChainTransaction {
                    signature: signature.to_string(),
                    slot,
                    instructions,
                },
            );
            let mut sigs = self.signatures_by_address.lock().unwrap();
            sigs.entry(self.program_id)
                .or_default()
                .insert(0, signature.to_string());
            for addr in touched {
                sigs.entry(*addr)
                    .or_default()
                    .insert(0, signature.to_string());
            }
        }
    }

    #[async_trait]
    impl ChainFetcher for FakeChain {
        async fn get_signatures_for_address(
            &self,
            address: [u8; 32],
            before: Option<&str>,
            until: Option<&str>,
            limit: usize,
        ) -> Result<Vec<String>, ChainError> {
            let sigs = self.signatures_by_address.lock().unwrap();
            let all = sigs.get(&address).cloned().unwrap_or_default();
            let start = before
                .and_then(|b| all.iter().position(|s| s == b).map(|idx| idx + 1))
                .unwrap_or(0);
            let bounded: Vec<String> = match until {
                Some(u) => all.into_iter().skip(start).take_while(|s| s != u).collect(),
                None => all.into_iter().skip(start).collect(),
            };
            Ok(bounded.into_iter().take(limit).collect())
        }

        async fn get_transaction(
            &self,
            signature: &str,
        ) -> Result<Option<ChainTransaction>, ChainError> {
            Ok(self.transactions.lock().unwrap().get(signature).cloned())
        }

        async fn get_lineage_state(
            &self,
            address: [u8; 32],
        ) -> Result<Option<OnChainLineageState>, ChainError> {
            Ok(self.lineage_state.lock().unwrap().get(&address).cloned())
        }
    }

    #[tokio::test]
    async fn poll_once_ingests_create_update_and_make_public_in_order() {
        let program_id = pk(0x99);
        let lineage = pk(0x01);
        let owner = pk(0x30);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        #[derive(BorshSerialize)]
        struct CreateArgs {
            acl_domain_key: [u8; 32],
            app_account: [u8; 32],
            label: [u8; 32],
            handle: [u8; 32],
            subjects: Vec<SubjectGrant>,
        }
        let create_ix = make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "create_encrypted_value",
            CreateArgs {
                acl_domain_key: pk(0x10),
                app_account: pk(0x11),
                label: pk(0x12),
                handle: pk(0x20),
                subjects: vec![SubjectGrant { subject: owner }],
            },
        );
        chain.push_tx("sig1", 1, &[lineage], vec![create_ix]);

        #[derive(BorshSerialize)]
        struct UpdateArgs {
            new_handle: [u8; 32],
            previous_handle: [u8; 32],
            previous_subjects: Vec<[u8; 32]>,
        }
        let update_ix = make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "update_encrypted_value",
            UpdateArgs {
                new_handle: pk(0x21),
                previous_handle: pk(0x20),
                previous_subjects: vec![owner],
            },
        );
        chain.push_tx("sig2", 2, &[lineage], vec![update_ix]);

        let make_public_ix = make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "make_handle_public",
            pk(0x21),
        );
        chain.push_tx("sig3", 3, &[lineage], vec![make_public_ix]);

        let processed = poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
            .await
            .unwrap();
        assert_eq!(processed, 3);

        let events = store.get_events(lineage).await.unwrap();
        assert_eq!(
            events,
            vec![
                LineageEvent::handle_superseded(pk(0x20), &[owner]),
                LineageEvent::MarkedPublic { handle: pk(0x21) },
            ]
        );

        let cursor = store.get_cursor().await.unwrap().unwrap();
        assert_eq!(cursor.last_signature, Some("sig3".to_string()));
        assert_eq!(cursor.last_slot, 3);
    }

    #[tokio::test]
    async fn decode_failure_does_not_advance_cursor_or_store_state() {
        let program_id = pk(0x99);
        let chain = FakeChain::new(program_id);
        chain.push_tx(
            "malformed",
            1,
            &[],
            vec![RawInstruction {
                program_id,
                accounts: vec![],
                data: vec![1, 2, 3],
                top_level_index: 0,
                stack_height: Some(1),
            }],
        );
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        assert!(matches!(
            poll_once(&chain, &store, program_id, 100, 100).await,
            Err(IngestError::Decode(_))
        ));
        assert_eq!(store.get_cursor().await.unwrap(), None);
    }

    #[tokio::test]
    async fn poll_once_resumes_from_cursor_without_reprocessing() {
        let program_id = pk(0x99);
        let lineage = pk(0x02);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        let make_public_ix = make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "make_handle_public",
            pk(0x20),
        );
        // Seed replay state as though `create` already happened before the cursor.
        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![],
                },
            )
            .await
            .unwrap();
        store
            .set_cursor(Cursor {
                last_signature: Some("sig0".to_string()),
                last_slot: 0,
            })
            .await
            .unwrap();

        chain.push_tx("sig0", 0, &[lineage], vec![]); // already processed, must not resurface
        chain.push_tx("sig1", 1, &[lineage], vec![make_public_ix]);

        let processed = poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
            .await
            .unwrap();
        assert_eq!(processed, 1, "only sig1 is newer than the cursor");
        let events = store.get_events(lineage).await.unwrap();
        assert_eq!(
            events,
            vec![LineageEvent::MarkedPublic { handle: pk(0x20) }]
        );
    }

    /// A backlog larger than the RPC page size is ingested in full, oldest-first,
    /// with no gap: paging across page boundaries must never skip a signature.
    #[tokio::test]
    async fn poll_once_ingests_full_backlog_without_gap() {
        let program_id = pk(0x99);
        let lineage = pk(0x04);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![],
                },
            )
            .await
            .unwrap();
        store
            .set_cursor(Cursor {
                last_signature: Some("sig0".to_string()),
                last_slot: 0,
            })
            .await
            .unwrap();

        chain.push_tx("sig0", 0, &[lineage], vec![]);
        for (sig, slot) in [("sig1", 1), ("sig2", 2), ("sig3", 3)] {
            let make_public_ix = make_public_ix(program_id, lineage, pk(0x20));
            chain.push_tx(sig, slot, &[lineage], vec![make_public_ix]);
        }

        // page size 2 < backlog 3 -> must page, not truncate.
        let processed = poll_once(&chain, &store, program_id, 2, MAX_POLL_BACKLOG_PER_CYCLE)
            .await
            .unwrap();
        assert_eq!(processed, 3);
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![
                LineageEvent::MarkedPublic { handle: pk(0x20) },
                LineageEvent::MarkedPublic { handle: pk(0x20) },
                LineageEvent::MarkedPublic { handle: pk(0x20) },
            ]
        );
        let cursor = store.get_cursor().await.unwrap().unwrap();
        assert_eq!(cursor.last_signature, Some("sig3".to_string()));
        assert_eq!(cursor.last_slot, 3);
    }

    /// A backlog past the per-cycle ceiling fails closed: it must error and leave
    /// the cursor untouched rather than silently skip the older signatures.
    #[tokio::test]
    async fn poll_once_fails_closed_when_backlog_exceeds_ceiling() {
        let program_id = pk(0x99);
        let lineage = pk(0x04);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![],
                },
            )
            .await
            .unwrap();
        store
            .set_cursor(Cursor {
                last_signature: Some("sig0".to_string()),
                last_slot: 0,
            })
            .await
            .unwrap();

        chain.push_tx("sig0", 0, &[lineage], vec![]);
        for (sig, slot) in [("sig1", 1), ("sig2", 2), ("sig3", 3)] {
            let make_public_ix = make_public_ix(program_id, lineage, pk(0x20));
            chain.push_tx(sig, slot, &[lineage], vec![make_public_ix]);
        }

        // ceiling 2 < backlog 3 -> fail closed, no partial apply.
        let error = poll_once(&chain, &store, program_id, 100, 2)
            .await
            .unwrap_err();
        assert!(matches!(error, IngestError::BacklogExceeded { max: 2 }));
        assert!(store.get_events(lineage).await.unwrap().is_empty());
        let cursor = store.get_cursor().await.unwrap().unwrap();
        assert_eq!(cursor.last_signature, Some("sig0".to_string()));
    }

    #[tokio::test]
    async fn poll_once_stops_at_missing_transaction_then_recovers_oldest_first() {
        let program_id = pk(0x99);
        let lineage = pk(0x07);
        let owner = pk(0x30);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![owner],
                },
            )
            .await
            .unwrap();
        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![update_ix(
                program_id,
                lineage,
                pk(0x21),
                pk(0x20),
                vec![owner],
            )],
        );
        chain.push_tx(
            "sig2",
            2,
            &[lineage],
            vec![make_public_ix(program_id, lineage, pk(0x21))],
        );
        let missing = chain.transactions.lock().unwrap().remove("sig1").unwrap();

        let error = poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            IngestError::TransactionUnavailable { signature } if signature == "sig1"
        ));
        assert!(store.get_cursor().await.unwrap().is_none());
        assert!(store.get_events(lineage).await.unwrap().is_empty());

        chain
            .transactions
            .lock()
            .unwrap()
            .insert("sig1".to_string(), missing);
        assert_eq!(
            poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
                .await
                .unwrap(),
            2
        );
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![
                LineageEvent::handle_superseded(pk(0x20), &[owner]),
                LineageEvent::MarkedPublic { handle: pk(0x21) },
            ]
        );
        let cursor = store.get_cursor().await.unwrap().unwrap();
        assert_eq!(cursor.last_signature.as_deref(), Some("sig2"));
        assert_eq!(cursor.last_slot, 2);
    }

    #[tokio::test]
    async fn poll_once_commits_valid_transaction_before_later_gap() {
        let program_id = pk(0x99);
        let lineage = pk(0x08);
        let owner = pk(0x30);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![owner],
                },
            )
            .await
            .unwrap();
        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![update_ix(
                program_id,
                lineage,
                pk(0x21),
                pk(0x20),
                vec![owner],
            )],
        );
        chain.push_tx(
            "sig2",
            2,
            &[lineage],
            vec![make_public_ix(program_id, lineage, pk(0x21))],
        );
        chain.push_tx(
            "sig3",
            3,
            &[lineage],
            vec![update_ix(
                program_id,
                lineage,
                pk(0x22),
                pk(0x21),
                vec![owner],
            )],
        );
        let missing = chain.transactions.lock().unwrap().remove("sig2").unwrap();

        let error = poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            IngestError::TransactionUnavailable { signature } if signature == "sig2"
        ));
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![LineageEvent::handle_superseded(pk(0x20), &[owner])]
        );
        assert_eq!(
            store.get_seen_signatures(lineage).await.unwrap(),
            vec!["sig1".to_string()]
        );
        let cursor = store.get_cursor().await.unwrap().unwrap();
        assert_eq!(cursor.last_signature.as_deref(), Some("sig1"));
        assert_eq!(cursor.last_slot, 1);

        chain
            .transactions
            .lock()
            .unwrap()
            .insert("sig2".to_string(), missing);
        assert_eq!(
            poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
                .await
                .unwrap(),
            2
        );
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![
                LineageEvent::handle_superseded(pk(0x20), &[owner]),
                LineageEvent::MarkedPublic { handle: pk(0x21) },
                LineageEvent::handle_superseded(pk(0x21), &[owner]),
            ]
        );
        let cursor = store.get_cursor().await.unwrap().unwrap();
        assert_eq!(cursor.last_signature.as_deref(), Some("sig3"));
        assert_eq!(cursor.last_slot, 3);
    }

    #[tokio::test]
    async fn catch_up_lineage_ingests_only_new_signatures_for_that_lineage() {
        let program_id = pk(0x99);
        let lineage = pk(0x03);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![],
                },
            )
            .await
            .unwrap();
        store.mark_signature_seen(lineage, "sig_old").await.unwrap();
        chain.push_tx("sig_old", 1, &[lineage], vec![]);

        let make_public_ix = make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "make_handle_public",
            pk(0x20),
        );
        chain.push_tx("sig_new", 2, &[lineage], vec![make_public_ix]);

        let outcome = catch_up_lineage(&chain, &store, program_id, lineage, 1000)
            .await
            .unwrap();
        assert_eq!(
            outcome,
            CatchUpOutcome {
                processed: 1,
                budget_exhausted: false
            }
        );
        let events = store.get_events(lineage).await.unwrap();
        assert_eq!(
            events,
            vec![LineageEvent::MarkedPublic { handle: pk(0x20) }]
        );
    }

    #[tokio::test]
    async fn catch_up_lineage_retries_unavailable_transaction_without_marking_it_seen() {
        let program_id = pk(0x99);
        let lineage = pk(0x09);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![],
                },
            )
            .await
            .unwrap();
        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![make_public_ix(program_id, lineage, pk(0x20))],
        );
        let missing = chain.transactions.lock().unwrap().remove("sig1").unwrap();

        let error = catch_up_lineage(&chain, &store, program_id, lineage, 1000)
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            IngestError::TransactionUnavailable { signature } if signature == "sig1"
        ));
        assert!(store.get_seen_signatures(lineage).await.unwrap().is_empty());
        assert!(store.get_events(lineage).await.unwrap().is_empty());

        chain
            .transactions
            .lock()
            .unwrap()
            .insert("sig1".to_string(), missing);
        assert_eq!(
            catch_up_lineage(&chain, &store, program_id, lineage, 1000)
                .await
                .unwrap(),
            CatchUpOutcome {
                processed: 1,
                budget_exhausted: false,
            }
        );
        assert_eq!(
            store.get_seen_signatures(lineage).await.unwrap(),
            vec!["sig1".to_string()]
        );
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![LineageEvent::MarkedPublic { handle: pk(0x20) }]
        );
    }

    #[tokio::test]
    async fn catch_up_after_poll_does_not_reapply_same_signature() {
        let program_id = pk(0x99);
        let lineage = pk(0x05);
        let owner = pk(0x30);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x20)),
                    subjects: vec![owner],
                },
            )
            .await
            .unwrap();
        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![update_ix(
                program_id,
                lineage,
                pk(0x21),
                pk(0x20),
                vec![owner],
            )],
        );

        assert_eq!(
            poll_once(&chain, &store, program_id, 100, MAX_POLL_BACKLOG_PER_CYCLE)
                .await
                .unwrap(),
            1
        );
        let outcome = catch_up_lineage(&chain, &store, program_id, lineage, 1000)
            .await
            .unwrap();

        assert_eq!(
            outcome,
            CatchUpOutcome {
                processed: 0,
                budget_exhausted: false
            }
        );
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![LineageEvent::handle_superseded(pk(0x20), &[owner])]
        );
        assert_eq!(
            store.get_seen_signatures(lineage).await.unwrap(),
            vec!["sig1".to_string()]
        );
    }

    #[tokio::test]
    async fn half_applied_update_restarts_without_previous_state_mismatch() {
        let program_id = pk(0x99);
        let lineage = pk(0x06);
        let owner = pk(0x30);
        let chain = FakeChain::new(program_id);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("leaves.json");

        {
            let store = FileLeafStore::open(&path).await.unwrap();
            store
                .set_replay_state(
                    lineage,
                    LineageReplayState {
                        current_handle: Some(pk(0x21)),
                        subjects: vec![owner],
                    },
                )
                .await
                .unwrap();
        }

        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![update_ix(
                program_id,
                lineage,
                pk(0x21),
                pk(0x20),
                vec![owner],
            )],
        );
        let reopened = FileLeafStore::open(&path).await.unwrap();

        assert_eq!(
            poll_once(
                &chain,
                &reopened,
                program_id,
                100,
                MAX_POLL_BACKLOG_PER_CYCLE
            )
            .await
            .unwrap(),
            1
        );
        assert_eq!(
            reopened.get_events(lineage).await.unwrap(),
            vec![LineageEvent::handle_superseded(pk(0x20), &[owner])]
        );
        assert_eq!(
            reopened.get_seen_signatures(lineage).await.unwrap(),
            vec!["sig1".to_string()]
        );
        assert_eq!(
            reopened.get_cursor().await.unwrap().unwrap().last_signature,
            Some("sig1".to_string())
        );
    }
}
