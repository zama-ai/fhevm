//! Builds MMR inclusion proofs for a lineage's leaf, cross-checked against the
//! live on-chain `EncryptedValue` account, catching up the local store once on
//! divergence before giving up.

use zama_solana_acl::lineage::{reconstruct, LineageError};
use zama_solana_acl::mmr::MmrProof;

use crate::solana_proof::chain::ChainFetcher;
use crate::solana_proof::ingest::{catch_up_lineage, IngestError};
use crate::solana_proof::store::LeafStore;

#[derive(Debug, Clone, PartialEq)]
pub struct MmrProofResult {
    pub mmr_proof: Option<MmrProof>,
    pub leaf_count: u64,
    /// Backwards-compatible wire name for the lineage `leaf_count` the proof was built against.
    pub proof_slot: u64,
    pub verified: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum ProofError {
    #[error("chain error: {0}")]
    Chain(#[from] crate::solana_proof::chain::ChainError),
    #[error("store error: {0}")]
    Store(#[from] crate::solana_proof::store::StoreError),
    #[error("ingest error: {0}")]
    Ingest(#[from] IngestError),
    #[error("lineage reconstruction error: {0:?}")]
    Lineage(LineageError),
    #[error("no on-chain account found for lineage")]
    LineageNotFound,
    #[error("lineage proof data is lagging chain state (leaf_count {leaf_count})")]
    Lagging { leaf_count: u64 },
    #[error("lineage proof cache diverged from chain after catch-up (leaf_count {leaf_count}); cache rebuild required")]
    CorruptCache { leaf_count: u64 },
}

/// Builds a proof for `(lineage, leaf_index)`. On `PeaksDiverged` (the local
/// event log is behind the live chain account) triggers one targeted catch-up
/// ingestion for that lineage and retries once; budget exhaustion is returned
/// as retryable lag, while a second divergence means the local cache cannot be
/// reconciled by polling the remaining signatures.
pub async fn build_proof<C: ChainFetcher, S: LeafStore>(
    fetcher: &C,
    store: &S,
    program_id: [u8; 32],
    lineage: [u8; 32],
    leaf_index: u64,
    catch_up_signature_budget: usize,
) -> Result<MmrProofResult, ProofError> {
    let on_chain = fetcher
        .get_lineage_state(lineage)
        .await?
        .ok_or(ProofError::LineageNotFound)?;
    let lagging = || ProofError::Lagging {
        leaf_count: on_chain.leaf_count,
    };

    match try_build(
        store,
        lineage,
        leaf_index,
        &on_chain.peaks,
        on_chain.leaf_count,
    )
    .await?
    {
        Ok(proof) => Ok(MmrProofResult {
            mmr_proof: Some(proof),
            leaf_count: on_chain.leaf_count,
            proof_slot: on_chain.leaf_count,
            verified: true,
        }),
        Err(LineageError::PeaksDiverged) => {
            let outcome = catch_up_lineage(
                fetcher,
                store,
                program_id,
                lineage,
                catch_up_signature_budget,
            )
            .await?;
            if outcome.budget_exhausted {
                return Err(lagging());
            }
            match try_build(
                store,
                lineage,
                leaf_index,
                &on_chain.peaks,
                on_chain.leaf_count,
            )
            .await?
            {
                Ok(proof) => Ok(MmrProofResult {
                    mmr_proof: Some(proof),
                    leaf_count: on_chain.leaf_count,
                    proof_slot: on_chain.leaf_count,
                    verified: true,
                }),
                Err(LineageError::PeaksDiverged) => Err(ProofError::CorruptCache {
                    leaf_count: on_chain.leaf_count,
                }),
                Err(other) => Err(ProofError::Lineage(other)),
            }
        }
        Err(other) => Err(ProofError::Lineage(other)),
    }
}

async fn try_build<S: LeafStore>(
    store: &S,
    lineage: [u8; 32],
    leaf_index: u64,
    on_chain_peaks: &[[u8; 32]],
    on_chain_leaf_count: u64,
) -> Result<Result<MmrProof, LineageError>, ProofError> {
    let events = store.get_events(lineage).await?;
    let reconstructed = reconstruct(lineage, &events).map_err(ProofError::Lineage)?;
    Ok(reconstructed.build_verified_proof(on_chain_peaks, on_chain_leaf_count, leaf_index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_proof::chain::{ChainError, ChainTransaction, OnChainLineageState};
    use crate::solana_proof::decode::{RawInstruction, SubjectGrant};
    use crate::solana_proof::replay::LineageReplayState;
    use crate::solana_proof::store::FileLeafStore;
    use anchor_lang::prelude::Pubkey;
    use async_trait::async_trait;
    use borsh::BorshSerialize;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use zama_host::state::{AclSubjectEntry, FheEvalArgs, FheEvalOutput, FheEvalStep};
    use zama_solana_acl::lineage::LineageEvent;
    use zama_solana_acl::mmr::mmr_append;
    use zama_solana_acl::{authorize_public, public_decrypt_leaf_commitment, EncryptedValue};

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn pubkey(tag: u8) -> Pubkey {
        Pubkey::new_from_array(pk(tag))
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

    /// Anchor `emit_cpi!` self-invocation sentinel (little-endian of
    /// `0x1d9acb512ea545e4`), matching the on-chain event transport.
    const ANCHOR_EVENT_IX_TAG_LE: [u8; 8] = 0x1d9acb512ea545e4_u64.to_le_bytes();

    fn event_discriminator(name: &str) -> [u8; 8] {
        let digest = Sha256::digest(format!("event:{name}").as_bytes());
        let mut out = [0u8; 8];
        out.copy_from_slice(&digest[..8]);
        out
    }

    /// Builds the inner instruction the host emits for a `TrivialEncrypt` op via
    /// `emit_cpi!`: a self-CPI whose data is the Anchor event sentinel + event
    /// discriminator + borsh event, carrying the verified output `result` handle.
    fn trivial_encrypt_event_ix(program_id: [u8; 32], result: [u8; 32]) -> RawInstruction {
        use anchor_lang::AnchorSerialize;
        use zama_host::events::TrivialEncryptEvent;
        let event = TrivialEncryptEvent {
            version: 1,
            subject: pk(0x30),
            plaintext: pk(0x02),
            fhe_type: 5,
            result,
        };
        let mut data = ANCHOR_EVENT_IX_TAG_LE.to_vec();
        data.extend_from_slice(&event_discriminator("TrivialEncryptEvent"));
        event.serialize(&mut data).unwrap();
        RawInstruction {
            program_id,
            accounts: vec![pk(0xEE), program_id],
            data,
        }
    }

    fn fhe_eval_accounts(program_id: [u8; 32], remaining: &[[u8; 32]]) -> Vec<[u8; 32]> {
        let mut accounts = vec![
            pk(0xA0),
            pk(0xA1),
            pk(0xA2),
            pk(0xA3),
            pk(0xA4),
            pk(0xA5),
            program_id,
        ];
        accounts.extend_from_slice(remaining);
        accounts
    }

    fn eval_create_output(subject: [u8; 32]) -> FheEvalArgs {
        FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: pk(0x02),
                fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 0,
                    output_app_account_authority_index: None,
                    output_acl_domain_key: pubkey(0x40),
                    output_app_account: pubkey(0x41),
                    output_encrypted_value_label: pk(0x42),
                    output_subjects: vec![AclSubjectEntry {
                        pubkey: Pubkey::new_from_array(subject),
                    }],
                    previous_handle: None,
                    previous_subjects: None,
                    // Publicized via a standalone `make_handle_public` instruction
                    // in the test below, not inline (see the module note on why
                    // inline `make_public` is not reconstructable from replay).
                    make_public: false,
                },
            }],
        }
    }

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

        fn set_lineage_state(&self, lineage: [u8; 32], state: OnChainLineageState) {
            self.lineage_state.lock().unwrap().insert(lineage, state);
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
    async fn builds_verified_proof_when_store_matches_chain() {
        let lineage = pk(0x01);
        let owner = pk(0x30);
        let leaf = zama_solana_acl::historical_access_leaf_commitment(lineage, 0, pk(0x10), owner);
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let program_id = pk(0x99);
        let chain = FakeChain::new(program_id);
        chain.set_lineage_state(lineage, OnChainLineageState { peaks, leaf_count });

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();
        store
            .append_events(
                lineage,
                &[LineageEvent::handle_superseded(pk(0x10), &[owner])],
            )
            .await
            .unwrap();

        let result = build_proof(&chain, &store, program_id, lineage, 0, 1000)
            .await
            .unwrap();
        assert!(result.verified);
        assert_eq!(result.leaf_count, 1);
        assert_eq!(result.proof_slot, 1);
        assert!(result.mmr_proof.is_some());
    }

    #[tokio::test]
    async fn catches_up_and_retries_when_store_is_behind_chain() {
        let program_id = pk(0x99);
        let lineage = pk(0x02);
        let owner = pk(0x30);
        let chain = FakeChain::new(program_id);

        // Chain already has one leaf; the store starts empty (never ingested).
        let leaf = zama_solana_acl::historical_access_leaf_commitment(lineage, 0, pk(0x10), owner);
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();
        chain.set_lineage_state(lineage, OnChainLineageState { peaks, leaf_count });

        let update_ix = {
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
                    new_handle: pk(0x11),
                    previous_handle: pk(0x10),
                    previous_subjects: vec![owner],
                },
            )
        };
        chain.push_tx("sig1", 1, &[lineage], vec![update_ix]);

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();
        // Seed replay state as though `create_encrypted_value` already happened.
        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x10)),
                    subjects: vec![owner],
                },
            )
            .await
            .unwrap();

        // Store is empty (PeaksDiverged expected) until catch-up ingests sig1.
        let result = build_proof(&chain, &store, program_id, lineage, 0, 1000)
            .await
            .unwrap();
        assert!(
            result.verified,
            "catch-up should have ingested the missing event and verified"
        );
        assert_eq!(result.proof_slot, 1);
        assert!(result.mmr_proof.is_some());
    }

    #[tokio::test]
    async fn eval_created_lineage_make_public_builds_verified_public_proof() {
        let program_id = pk(0x99);
        let lineage = pk(0x05);
        let owner = pk(0x30);
        let handle = pk(0x44);
        let chain = FakeChain::new(program_id);

        let leaf = public_decrypt_leaf_commitment(lineage, 0, handle);
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();
        chain.set_lineage_state(
            lineage,
            OnChainLineageState {
                peaks: peaks.clone(),
                leaf_count,
            },
        );

        let eval_ix = make_ix(
            program_id,
            fhe_eval_accounts(program_id, &[lineage]),
            "fhe_eval",
            eval_create_output(owner),
        );
        chain.push_tx("sig1", 1, &[lineage], vec![eval_ix]);

        let make_public_ix = make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "make_handle_public",
            handle,
        );
        chain.push_tx("sig2", 2, &[lineage], vec![make_public_ix]);

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        let result = build_proof(&chain, &store, program_id, lineage, 0, 1000)
            .await
            .unwrap();
        let proof = result.mmr_proof.as_ref().unwrap();
        let acl = EncryptedValue {
            acl_domain_key: pk(0x40),
            app_account: pk(0x41),
            encrypted_value_label: pk(0x42),
            current_handle: handle,
            subjects: vec![owner],
            leaf_count,
            peaks,
            bump: 0,
        };
        let mut proof_bytes = vec![0x02];
        proof.serialize(&mut proof_bytes).unwrap();

        assert!(result.verified);
        assert_eq!(result.leaf_count, 1);
        assert_eq!(result.proof_slot, 1);
        assert_eq!(proof_bytes[0], 0x02);
        authorize_public(lineage, &acl, handle, proof).unwrap();
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![LineageEvent::MarkedPublic { handle }]
        );
    }

    #[tokio::test]
    async fn returns_lagging_when_catch_up_budget_is_exhausted() {
        let program_id = pk(0x99);
        let lineage = pk(0x04);
        let chain = FakeChain::new(program_id);

        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        for index in 0..2 {
            let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, index, pk(0x10));
            mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();
        }
        chain.set_lineage_state(lineage, OnChainLineageState { peaks, leaf_count });

        for (sig, slot) in [("sig1", 1), ("sig2", 2)] {
            let make_public_ix = make_ix(
                program_id,
                vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
                "make_handle_public",
                pk(0x10),
            );
            chain.push_tx(sig, slot, &[lineage], vec![make_public_ix]);
        }

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();
        store
            .set_replay_state(
                lineage,
                LineageReplayState {
                    current_handle: Some(pk(0x10)),
                    subjects: vec![],
                },
            )
            .await
            .unwrap();

        let error = build_proof(&chain, &store, program_id, lineage, 0, 1)
            .await
            .unwrap_err();
        assert!(matches!(error, ProofError::Lagging { leaf_count: 2 }));
        assert!(store.get_events(lineage).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn reports_corrupt_cache_when_divergence_persists_after_catch_up() {
        let program_id = pk(0x99);
        let lineage = pk(0x03);
        let chain = FakeChain::new(program_id);
        // Chain has a leaf the local record can never explain (no matching tx at all).
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0xAA));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();
        chain.set_lineage_state(lineage, OnChainLineageState { peaks, leaf_count });

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        let error = build_proof(&chain, &store, program_id, lineage, 0, 1000)
            .await
            .unwrap_err();
        assert!(matches!(error, ProofError::CorruptCache { leaf_count: 1 }));
    }

    /// A confidential-burn-style born-public `fhe_eval` supersede: on-chain the
    /// bind appends `hist(old, owner)`, `hist(old, compute)`, then
    /// `public(new_burn_handle)`. The output handle behind the public leaf is
    /// derived on-chain from slot entropy and carried in NO instruction arg, so
    /// the relayer cannot recompute it from thin JSON-RPC. It is instead resolved
    /// from the `TrivialEncryptEvent` the burn emits via `emit_cpi!` (an inner
    /// instruction of the `fhe_eval`, already flattened into the tx by
    /// `chain::flatten_execution_order`). With that handle the relayer now
    /// reconstructs all three leaves and serves a verified public-decrypt proof.
    #[tokio::test]
    async fn inline_born_public_fhe_eval_supersede_reconstructs_from_event_handle() {
        let program_id = pk(0x99);
        let lineage = pk(0x07);
        let owner = pk(0x30);
        let compute = pk(0x31);
        let old_handle = pk(0x20);
        // Derived on-chain from previous_bank_hash; recovered from the op event.
        let burn_handle = pk(0x21);
        let chain = FakeChain::new(program_id);

        // On-chain leaves for the born-public supersede, in append order.
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(
            &mut peaks,
            &mut leaf_count,
            zama_solana_acl::historical_access_leaf_commitment(lineage, 0, old_handle, owner),
        )
        .unwrap();
        mmr_append(
            &mut peaks,
            &mut leaf_count,
            zama_solana_acl::historical_access_leaf_commitment(lineage, 1, old_handle, compute),
        )
        .unwrap();
        mmr_append(
            &mut peaks,
            &mut leaf_count,
            public_decrypt_leaf_commitment(lineage, 2, burn_handle),
        )
        .unwrap();
        chain.set_lineage_state(
            lineage,
            OnChainLineageState {
                peaks: peaks.clone(),
                leaf_count,
            },
        );

        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![born_public_create_ix(program_id, lineage, old_handle, owner, compute)],
        );

        // The burn: an fhe_eval durable supersede output, born public inline,
        // followed by its `emit_cpi!` op event carrying the output handle.
        chain.push_tx(
            "sig2",
            2,
            &[lineage],
            vec![
                born_public_burn_ix(program_id, lineage, old_handle, owner, compute),
                trivial_encrypt_event_ix(program_id, burn_handle),
            ],
        );

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        // The public leaf (index 2) now reconstructs and verifies.
        let result = build_proof(&chain, &store, program_id, lineage, 2, 1000)
            .await
            .unwrap();
        let proof = result.mmr_proof.as_ref().unwrap();
        assert!(result.verified);
        assert_eq!(result.leaf_count, 3);
        assert_eq!(result.proof_slot, 3);

        let acl = EncryptedValue {
            acl_domain_key: pk(0x40),
            app_account: pk(0x41),
            encrypted_value_label: pk(0x42),
            current_handle: burn_handle,
            subjects: vec![owner, compute],
            leaf_count,
            peaks,
            bump: 0,
        };
        authorize_public(lineage, &acl, burn_handle, proof).unwrap();

        // The two historical leaves plus the event-resolved born-public leaf.
        assert_eq!(
            store.get_events(lineage).await.unwrap(),
            vec![
                LineageEvent::handle_superseded(old_handle, &[owner, compute]),
                LineageEvent::MarkedPublic {
                    handle: burn_handle
                },
            ]
        );
    }

    /// The event-resolved output handle is UNTRUSTED. If the emitted event
    /// carries the wrong `result`, the reconstructed public leaf differs from
    /// chain, the peaks diverge, and the proof fails closed (DD-035) rather than
    /// mis-serving a proof for a handle that was never made public.
    #[tokio::test]
    async fn inline_born_public_fhe_eval_supersede_with_wrong_event_handle_fails_closed() {
        let program_id = pk(0x99);
        let lineage = pk(0x08);
        let owner = pk(0x30);
        let compute = pk(0x31);
        let old_handle = pk(0x20);
        let real_burn_handle = pk(0x21);
        let forged_handle = pk(0xBB);
        let chain = FakeChain::new(program_id);

        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(
            &mut peaks,
            &mut leaf_count,
            zama_solana_acl::historical_access_leaf_commitment(lineage, 0, old_handle, owner),
        )
        .unwrap();
        mmr_append(
            &mut peaks,
            &mut leaf_count,
            zama_solana_acl::historical_access_leaf_commitment(lineage, 1, old_handle, compute),
        )
        .unwrap();
        mmr_append(
            &mut peaks,
            &mut leaf_count,
            public_decrypt_leaf_commitment(lineage, 2, real_burn_handle),
        )
        .unwrap();
        chain.set_lineage_state(lineage, OnChainLineageState { peaks, leaf_count });

        chain.push_tx(
            "sig1",
            1,
            &[lineage],
            vec![born_public_create_ix(program_id, lineage, old_handle, owner, compute)],
        );
        // The event carries a handle that does not match the on-chain leaf.
        chain.push_tx(
            "sig2",
            2,
            &[lineage],
            vec![
                born_public_burn_ix(program_id, lineage, old_handle, owner, compute),
                trivial_encrypt_event_ix(program_id, forged_handle),
            ],
        );

        let dir = tempfile::tempdir().unwrap();
        let store = FileLeafStore::open(dir.path().join("leaves.json"))
            .await
            .unwrap();

        let error = build_proof(&chain, &store, program_id, lineage, 2, 1000)
            .await
            .unwrap_err();
        assert!(
            matches!(error, ProofError::CorruptCache { leaf_count: 3 }),
            "a wrong event-resolved handle must fail closed, got {error:?}"
        );
    }

    fn born_public_create_ix(
        program_id: [u8; 32],
        lineage: [u8; 32],
        handle: [u8; 32],
        owner: [u8; 32],
        compute: [u8; 32],
    ) -> RawInstruction {
        #[derive(BorshSerialize)]
        struct CreateArgs {
            acl_domain_key: [u8; 32],
            app_account: [u8; 32],
            label: [u8; 32],
            handle: [u8; 32],
            subjects: Vec<SubjectGrant>,
        }
        make_ix(
            program_id,
            vec![pk(0xA), pk(0xB), lineage, pk(0xC), pk(0xD)],
            "create_encrypted_value",
            CreateArgs {
                acl_domain_key: pk(0x40),
                app_account: pk(0x41),
                label: pk(0x42),
                handle,
                subjects: vec![
                    SubjectGrant { subject: owner },
                    SubjectGrant { subject: compute },
                ],
            },
        )
    }

    fn born_public_burn_ix(
        program_id: [u8; 32],
        lineage: [u8; 32],
        previous_handle: [u8; 32],
        owner: [u8; 32],
        compute: [u8; 32],
    ) -> RawInstruction {
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: pk(0x02),
                fhe_type: 5,
                output: FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index: 0,
                    output_app_account_authority_index: None,
                    output_acl_domain_key: pubkey(0x40),
                    output_app_account: pubkey(0x41),
                    output_encrypted_value_label: pk(0x42),
                    output_subjects: vec![
                        AclSubjectEntry {
                            pubkey: Pubkey::new_from_array(owner),
                        },
                        AclSubjectEntry {
                            pubkey: Pubkey::new_from_array(compute),
                        },
                    ],
                    previous_handle: Some(previous_handle),
                    previous_subjects: Some(vec![
                        Pubkey::new_from_array(owner),
                        Pubkey::new_from_array(compute),
                    ]),
                    make_public: true,
                },
            }],
        };
        make_ix(
            program_id,
            fhe_eval_accounts(program_id, &[lineage]),
            "fhe_eval",
            plan,
        )
    }
}
