//! Carbon `Processor` for the four decoded EV-ACL instructions.
//!
//! For each instruction it reads the prior shadow state, applies the state
//! transition (capturing old_handle / subjects_before_rotation / handle from the
//! PRE-instruction state), then in ONE Postgres transaction: appends the event
//! row (if any), upserts the new shadow state, and advances the resume cursor —
//! so a crash leaves no gap. `UNIQUE(pda, event_index)` makes a duplicate
//! re-delivery a no-op. All other instructions are ignored by the decoder.

use carbon_core::error::{CarbonResult, Error};
use carbon_core::instruction::{InstructionMetadata, InstructionProcessorInputType};
use carbon_core::processor::Processor;
use std::sync::Arc;
use tracing::{debug, warn};

use crate::decoder::{DecodedEvAcl, EvAclInstruction};
use crate::lineage::state::{apply, LineageShadow};
use crate::metrics::Metrics;
use crate::store::repositories::lineage_repo::{EventInsert, LineageRepo, LineageState};

pub struct EvAclProcessor {
    repo: LineageRepo,
    metrics: Arc<Metrics>,
}

impl EvAclProcessor {
    pub fn new(repo: LineageRepo, metrics: Arc<Metrics>) -> Self {
        Self { repo, metrics }
    }

    /// Loads the prior shadow state for `pda` from the DB shadow.
    ///
    /// On a miss (a rotate/allow/mark whose `initialize` predates the cursor — a
    /// partial backfill) it returns `None`. We deliberately do NOT synthesize the
    /// prior from the live on-chain account: that account reflects the chain's
    /// CURRENT state, which may be many rotations AHEAD of the instruction being
    /// processed. Feeding a future-state `current_handle` into a `Rotation` event's
    /// `old_handle` would persist a wrong leaf commitment that reconstructs to a
    /// hash the KMS rejects, silently. A single instruction carries no way to prove
    /// the account is at the exact pre-instruction state, so the only safe move is
    /// to skip and require a full backfill from genesis (the caller's skip path).
    async fn prior_shadow(&self, pda: &[u8; 32]) -> anyhow::Result<Option<LineageShadow>> {
        Ok(self.repo.get_state(pda).await?.map(|state| LineageShadow {
            value_key: state.value_key,
            current_handle: state.current_handle,
            current_subjects: state.current_subjects,
            leaf_count: state.leaf_count as u64,
        }))
    }

    async fn handle(
        &self,
        decoded: &DecodedEvAcl,
        meta: &InstructionMetadata,
    ) -> anyhow::Result<()> {
        let pda = decoded.pda;
        let signature = meta.transaction_metadata.signature.to_string();
        let slot = meta.transaction_metadata.slot as i64;

        let prior = match &decoded.instruction {
            EvAclInstruction::Initialize(_) => None,
            _ => self.prior_shadow(&pda).await?,
        };

        let applied = match apply(prior, &decoded.instruction) {
            Ok(a) => a,
            Err(e) => {
                // No DB shadow for this lineage: either a genuine uninitialized
                // lineage, or a partial backfill whose `initialize` predates the
                // cursor. We refuse to synthesize a prior from the live account
                // (it may be ahead of this instruction; see `prior_shadow`), so we
                // skip — a full backfill from genesis is required to index it. The
                // cursor advances only on the next strictly-newer slot.
                warn!(
                    pda = hex::encode(pda),
                    error = %e,
                    "skipping instruction with no DB shadow (uninitialized or partial-backfill miss); re-run a full backfill from genesis to index this lineage"
                );
                let mut tx = self.repo.pool().begin().await?;
                self.repo.advance_cursor(&mut tx, &signature, slot).await?;
                tx.commit().await?;
                return Ok(());
            }
        };

        let mut tx = self.repo.pool().begin().await?;

        if let Some(event) = &applied.event {
            let event_index = self.repo.event_count(&mut tx, &pda).await?;
            let locator = EventInsert {
                pda: &pda,
                event_index,
                signature: &signature,
                slot,
            };
            self.repo.insert_event(&mut tx, &locator, event).await?;
            self.metrics.events_appended.inc();
        }

        let next = &applied.next;
        self.repo
            .upsert_state(
                &mut tx,
                &LineageState {
                    pda,
                    value_key: next.value_key,
                    current_handle: next.current_handle,
                    current_subjects: next.current_subjects.clone(),
                    leaf_count: next.leaf_count as i64,
                },
            )
            .await?;

        self.repo.advance_cursor(&mut tx, &signature, slot).await?;
        tx.commit().await?;

        self.metrics.instructions_decoded.inc();
        self.metrics.cursor_slot.set(slot);
        debug!(
            pda = hex::encode(pda),
            slot,
            signature = %signature,
            "applied EV-ACL instruction"
        );
        Ok(())
    }
}

impl Processor<InstructionProcessorInputType<'_, DecodedEvAcl>> for EvAclProcessor {
    async fn process(
        &mut self,
        data: &InstructionProcessorInputType<'_, DecodedEvAcl>,
    ) -> CarbonResult<()> {
        self.handle(data.decoded_instruction, data.metadata)
            .await
            .map_err(|e| Error::Custom(e.to_string()))
    }
}
