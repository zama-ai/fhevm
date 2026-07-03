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
}

/// Builds a proof for `(lineage, leaf_index)`. On `PeaksDiverged` (the local
/// event log is behind the live chain account) triggers one targeted catch-up
/// ingestion for that lineage and retries once; a second divergence gives up
/// and returns `verified: false` rather than erroring, since a caller may
/// still want the raw leaf_count/attempted proof for diagnostics.
pub async fn build_proof<C: ChainFetcher, S: LeafStore>(
    fetcher: &C,
    store: &S,
    program_id: [u8; 32],
    lineage: [u8; 32],
    leaf_index: u64,
) -> Result<MmrProofResult, ProofError> {
    let on_chain = fetcher
        .get_lineage_state(lineage)
        .await?
        .ok_or(ProofError::LineageNotFound)?;

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
            verified: true,
        }),
        Err(LineageError::PeaksDiverged) => {
            catch_up_lineage(fetcher, store, program_id, lineage).await?;
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
                    verified: true,
                }),
                Err(_) => Ok(MmrProofResult {
                    mmr_proof: None,
                    leaf_count: on_chain.leaf_count,
                    verified: false,
                }),
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
    use crate::solana_proof::decode::{RawInstruction, ACL_ROLE_USE};
    use crate::solana_proof::replay::LineageReplayState;
    use crate::solana_proof::store::FileLeafStore;
    use async_trait::async_trait;
    use borsh::BorshSerialize;
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use zama_solana_acl::lineage::LineageEvent;
    use zama_solana_acl::mmr::mmr_append;

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

        let result = build_proof(&chain, &store, program_id, lineage, 0)
            .await
            .unwrap();
        assert!(result.verified);
        assert_eq!(result.leaf_count, 1);
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
                    current_handle: pk(0x10),
                    subjects: vec![(owner, ACL_ROLE_USE)],
                },
            )
            .await
            .unwrap();

        // Store is empty (PeaksDiverged expected) until catch-up ingests sig1.
        let result = build_proof(&chain, &store, program_id, lineage, 0)
            .await
            .unwrap();
        assert!(
            result.verified,
            "catch-up should have ingested the missing event and verified"
        );
        assert!(result.mmr_proof.is_some());
    }

    #[tokio::test]
    async fn gives_up_gracefully_when_divergence_persists_after_catch_up() {
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

        let result = build_proof(&chain, &store, program_id, lineage, 0)
            .await
            .unwrap();
        assert!(!result.verified);
        assert!(result.mmr_proof.is_none());
        assert_eq!(result.leaf_count, 1);
    }
}
