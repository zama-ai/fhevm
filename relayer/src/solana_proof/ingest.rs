//! Polls zama-host program transactions, decodes `EncryptedValue` instructions,
//! replays per-lineage state, and appends the resulting `LineageEvent`s to the
//! `LeafStore`. Also provides a targeted single-lineage catch-up path used when
//! a proof build finds the store behind the live chain account.

use zama_solana_acl::lineage::LineageEvent;

use crate::solana_proof::chain::ChainFetcher;
use crate::solana_proof::decode::decode_program_instructions;
use crate::solana_proof::replay::{apply_instruction, LineageReplayState};
use crate::solana_proof::store::{Cursor, LeafStore};

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
    poll_limit: usize,
) -> Result<usize, IngestError> {
    let cursor = store.get_cursor().await?;
    let until = cursor.as_ref().and_then(|c| c.last_signature.as_deref());
    let mut signatures = fetcher
        .get_signatures_for_address(program_id, until, poll_limit)
        .await?;
    // RPC returns newest-first; replay must run oldest-to-newest.
    signatures.reverse();

    let mut last_slot = cursor.as_ref().map(|c| c.last_slot).unwrap_or(0);
    let mut last_signature = cursor.and_then(|c| c.last_signature);
    let mut processed = 0usize;

    for signature in &signatures {
        let Some(tx) = fetcher.get_transaction(signature).await? else {
            continue;
        };
        let decoded = decode_program_instructions(program_id, &tx.instructions)?;
        for instruction in &decoded {
            let lineage = instruction.encrypted_value();
            let mut state = store.get_replay_state(lineage).await?;
            let event = apply_instruction(&mut state, instruction)?;
            if let Some(state) = state {
                store.set_replay_state(lineage, state).await?;
            }
            if let Some(event) = event {
                store
                    .append_events(lineage, std::slice::from_ref(&event))
                    .await?;
            }
            store.mark_signature_seen(lineage, signature).await?;
        }
        last_slot = tx.slot;
        last_signature = Some(signature.clone());
        processed += 1;
    }

    store
        .set_cursor(Cursor {
            last_signature,
            last_slot,
        })
        .await?;
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
) -> Result<usize, IngestError> {
    let seen = store.get_seen_signatures(lineage).await?;
    let mut signatures = fetcher
        .get_signatures_for_address(lineage, None, 1000)
        .await?;
    signatures.reverse(); // oldest-to-newest for replay

    let mut state: Option<LineageReplayState> = store.get_replay_state(lineage).await?;
    let mut new_events: Vec<LineageEvent> = Vec::new();
    let mut processed = 0usize;

    for signature in signatures {
        if seen.contains(&signature) {
            continue;
        }
        let Some(tx) = fetcher.get_transaction(&signature).await? else {
            continue;
        };
        let decoded = decode_program_instructions(program_id, &tx.instructions)?;
        for instruction in decoded.iter().filter(|i| i.encrypted_value() == lineage) {
            if let Some(event) = apply_instruction(&mut state, instruction)? {
                new_events.push(event);
            }
        }
        store.mark_signature_seen(lineage, &signature).await?;
        processed += 1;
    }

    if !new_events.is_empty() {
        store.append_events(lineage, &new_events).await?;
    }
    if let Some(state) = state {
        store.set_replay_state(lineage, state).await?;
    }
    Ok(processed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_proof::chain::{ChainError, ChainTransaction, OnChainLineageState};
    use crate::solana_proof::decode::{RawInstruction, SubjectGrant, ACL_ROLE_USE};
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
        }
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
            until: Option<&str>,
            limit: usize,
        ) -> Result<Vec<String>, ChainError> {
            let sigs = self.signatures_by_address.lock().unwrap();
            let all = sigs.get(&address).cloned().unwrap_or_default();
            let bounded = match until {
                Some(u) => all.into_iter().take_while(|s| s != u).collect(),
                None => all,
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
                subjects: vec![SubjectGrant {
                    subject: owner,
                    role_flags: ACL_ROLE_USE,
                }],
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
            (),
        );
        chain.push_tx("sig3", 3, &[lineage], vec![make_public_ix]);

        let processed = poll_once(&chain, &store, program_id, 100).await.unwrap();
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
            (),
        );
        // Seed replay state as though `create` already happened before the cursor.
        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: pk(0x20),
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

        let processed = poll_once(&chain, &store, program_id, 100).await.unwrap();
        assert_eq!(processed, 1, "only sig1 is newer than the cursor");
        let events = store.get_events(lineage).await.unwrap();
        assert_eq!(
            events,
            vec![LineageEvent::MarkedPublic { handle: pk(0x20) }]
        );
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
                    current_handle: pk(0x20),
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
            (),
        );
        chain.push_tx("sig_new", 2, &[lineage], vec![make_public_ix]);

        let processed = catch_up_lineage(&chain, &store, program_id, lineage)
            .await
            .unwrap();
        assert_eq!(processed, 1);
        let events = store.get_events(lineage).await.unwrap();
        assert_eq!(
            events,
            vec![LineageEvent::MarkedPublic { handle: pk(0x20) }]
        );
    }
}
