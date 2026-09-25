//! Concrete Yellowstone source validation for sealed Solana blocks.
//!
//! The listener subscribes to the successful transactions naming the host program, one per
//! message, and to every slot's block meta. At `confirmed`, Yellowstone sends a slot's
//! transactions before its block meta, live and on `from_slot` replay, so [`BlockValidator`] seals
//! a slot when its block meta arrives. One transaction is bounded by Solana's limits, where a
//! whole block is bounded by compute units only: a block of transactions that merely list the host
//! program can exceed any message limit. Each transaction is reduced to the host program's
//! instructions when it arrives, so an open slot holds what the host executed, not what the
//! transactions carry. Each `fhe_execute` carries its own derivation context in its
//! `FheExecutedEvent`, so those instructions are all the listener needs.

use std::collections::{BTreeSet, HashMap};

use anchor_lang::prelude::Pubkey;
use anyhow::{bail, ensure, Context, Result};
use solana_sdk::signature::Signature;
use yellowstone_grpc_proto::prelude::{
    SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions, SubscribeUpdateBlockMeta,
};

use crate::solana_grpc_listener::{
    BlockCheckpoint, PreparedTransaction, StartPosition,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SealedBlock {
    pub slot: u64,
    pub block_hash: [u8; 32],
    pub parent_slot: u64,
    pub parent_block_hash: [u8; 32],
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub executed_transaction_count: u64,
}

impl SealedBlock {
    pub(super) fn checkpoint(&self) -> BlockCheckpoint {
        BlockCheckpoint {
            slot: self.slot,
            block_hash: self.block_hash,
        }
    }
}

#[derive(Debug)]
pub(super) enum SealDecision {
    /// Nothing to apply: a re-delivery of the last sealed slot, or the first slot of a tip start.
    Skip,
    /// The block, and its host transactions sorted by index.
    Process(SealedBlock, Vec<PreparedTransaction>),
}

/// The last sealed slot, which a re-delivery must match.
#[derive(Debug)]
struct LastSealed {
    block: SealedBlock,
    /// The index and signature of each transaction the slot holds. `None` for the first slot of a
    /// tip start, which is never applied.
    transactions: Option<BTreeSet<(u64, Signature)>>,
}

/// Seals slots from the per-transaction stream and checks that each extends the last. It holds
/// one slot open at a time: a slot's transactions arrive before its block meta and before any later
/// slot's message, and anything else stops the listener.
///
/// After a `from_slot` replay, Yellowstone sends the live messages it buffered meanwhile, so the
/// last sealed slot can arrive again, whole or as its block meta alone. A transaction the slot
/// already holds is ignored, and a block meta must equal the slot's. A transaction the slot does
/// not hold arrived after its block meta, when the slot was already applied without it: that stops
/// the listener, and the slot needs the repair in the listener's README.
#[derive(Debug)]
pub(super) struct BlockValidator {
    start: StartPosition,
    checkpoint_observed: bool,
    open: Option<(u64, Vec<PreparedTransaction>)>,
    last: Option<LastSealed>,
}

impl BlockValidator {
    pub fn new(start: StartPosition) -> Self {
        Self {
            checkpoint_observed: matches!(start, StartPosition::Tip),
            start,
            open: None,
            last: None,
        }
    }

    pub fn transaction(
        &mut self,
        slot: u64,
        transaction: PreparedTransaction,
    ) -> Result<()> {
        if let Some(last) = &self.last {
            let sealed = last.block.slot;
            ensure!(
                slot >= sealed,
                "transaction for slot {slot} arrived after slot {sealed} was sealed"
            );
            if slot == sealed {
                let held = last.transactions.as_ref().is_none_or(|held| {
                    held.contains(&(transaction.index, transaction.signature))
                });
                ensure!(
                    held,
                    "transaction {} (index {}) of slot {slot} arrived after the slot was applied \
                     without it; repair the record from a slot before {slot}",
                    transaction.signature,
                    transaction.index
                );
                return Ok(());
            }
        }
        match &mut self.open {
            Some((open, transactions)) if *open == slot => {
                transactions.push(transaction)
            }
            Some((open, _)) => bail!(
                "transaction for slot {slot} arrived before the block meta of slot {open}"
            ),
            None => self.open = Some((slot, vec![transaction])),
        }
        Ok(())
    }

    /// Seals the slot of `meta` with the transactions held open for it.
    pub fn block_meta(
        &mut self,
        meta: SubscribeUpdateBlockMeta,
    ) -> Result<SealDecision> {
        let block = SealedBlock {
            slot: meta.slot,
            block_hash: decode_hash("blockhash", &meta.blockhash)?,
            parent_slot: meta.parent_slot,
            parent_block_hash: decode_hash(
                "parent blockhash",
                &meta.parent_blockhash,
            )?,
            block_time: meta.block_time.map(|time| time.timestamp),
            block_height: meta.block_height.map(|height| height.block_height),
            executed_transaction_count: meta.executed_transaction_count,
        };
        let mut transactions = match self.open.take() {
            Some((open, transactions)) if open == block.slot => transactions,
            Some((open, _)) => bail!(
                "block meta of slot {} arrived while slot {open} was open",
                block.slot
            ),
            None => Vec::new(),
        };
        transactions.sort_by_key(|transaction| transaction.index);
        for pair in transactions.windows(2) {
            ensure!(
                pair[0].index != pair[1].index,
                "duplicate Yellowstone transaction index {}",
                pair[0].index
            );
        }
        if let Some(transaction) = transactions.last() {
            ensure!(
                transaction.index < block.executed_transaction_count,
                "transaction index {} is outside executed transaction count {}",
                transaction.index,
                block.executed_transaction_count
            );
        }

        if let Some(last) = &self.last {
            if block.slot == last.block.slot {
                ensure!(
                    block == last.block,
                    "conflicting sealed block replay at slot {}",
                    block.slot
                );
                return Ok(SealDecision::Skip);
            }
            ensure!(
                block.slot > last.block.slot,
                "out-of-order sealed block at slot {}",
                block.slot
            );
            ensure!(
                block.parent_slot == last.block.slot
                    && block.parent_block_hash == last.block.block_hash,
                "sealed block ancestry mismatch at slot {} (parent slot {})",
                block.slot,
                block.parent_slot
            );
        }
        let held = Some(
            transactions
                .iter()
                .map(|transaction| (transaction.index, transaction.signature))
                .collect(),
        );

        if !self.checkpoint_observed {
            let (StartPosition::Resume(checkpoint)
            | StartPosition::ReplayFrom(checkpoint)) = &self.start
            else {
                unreachable!("tip starts with checkpoint observed")
            };
            ensure!(
                block.slot == checkpoint.slot,
                "inclusive replay did not begin at checkpoint slot {}; observed {}",
                checkpoint.slot,
                block.slot
            );
            ensure!(
                block.block_hash == checkpoint.block_hash,
                "checkpoint block hash changed at slot {}",
                block.slot
            );
            self.checkpoint_observed = true;
            if matches!(self.start, StartPosition::Resume(_)) {
                self.last = Some(LastSealed {
                    block,
                    transactions: held,
                });
                return Ok(SealDecision::Skip);
            }
        } else if self.last.is_none() {
            // Yellowstone applies the subscription's filter from the first broadcast it handles
            // after the request, and a slot's transactions and its block meta are two broadcasts:
            // a tip start's first slot can arrive without its transactions.
            self.last = Some(LastSealed {
                block,
                transactions: None,
            });
            return Ok(SealDecision::Skip);
        }
        self.last = Some(LastSealed {
            block: block.clone(),
            transactions: held,
        });
        Ok(SealDecision::Process(block, transactions))
    }
}

pub(super) fn build_subscribe_request(
    program_id: &Pubkey,
    start: &StartPosition,
) -> SubscribeRequest {
    let transactions = HashMap::from([(
        "zama_host".to_owned(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            // The listener ignores failed transactions, so leaving them out changes no output.
            failed: Some(false),
            signature: None,
            // Matches static and lookup-table-loaded keys alike.
            account_include: vec![program_id.to_string()],
            account_exclude: vec![],
            account_required: vec![],
            token_accounts: None,
        },
    )]);
    let blocks_meta = HashMap::from([(
        "zama_host".to_owned(),
        SubscribeRequestFilterBlocksMeta {},
    )]);
    SubscribeRequest {
        accounts: HashMap::new(),
        slots: HashMap::new(),
        transactions,
        transactions_status: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta,
        entry: HashMap::new(),
        commitment: Some(
            yellowstone_grpc_proto::prelude::CommitmentLevel::Confirmed as i32,
        ),
        accounts_data_slice: vec![],
        ping: None,
        from_slot: match start {
            StartPosition::Tip => None,
            StartPosition::Resume(checkpoint)
            | StartPosition::ReplayFrom(checkpoint) => Some(checkpoint.slot),
        },
    }
}

fn decode_hash(name: &str, value: &str) -> Result<[u8; 32]> {
    let bytes = bs58::decode(value)
        .into_vec()
        .with_context(|| format!("invalid {name}"))?;
    <[u8; 32]>::try_from(bytes.as_slice())
        .with_context(|| format!("{name} is not 32 bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use yellowstone_grpc_proto::prelude::UnixTimestamp;

    fn hash(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn transaction(index: u64) -> PreparedTransaction {
        PreparedTransaction {
            signature: Signature::from([index as u8; 64]),
            index,
            instructions: vec![],
        }
    }

    /// The block meta of `slot`, child of `slot - 1`, with `executed_transaction_count`
    /// transactions.
    fn meta(
        slot: u64,
        executed_transaction_count: u64,
    ) -> SubscribeUpdateBlockMeta {
        SubscribeUpdateBlockMeta {
            slot,
            blockhash: bs58::encode(hash(slot as u8)).into_string(),
            parent_slot: slot - 1,
            parent_blockhash: bs58::encode(hash(slot as u8 - 1)).into_string(),
            executed_transaction_count,
            ..Default::default()
        }
    }

    /// A tip start that has skipped slot `slot - 1`, so `slot` is the first it applies.
    fn following(slot: u64) -> BlockValidator {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        assert!(matches!(
            validator.block_meta(meta(slot - 1, 0)).unwrap(),
            SealDecision::Skip
        ));
        validator
    }

    fn indexes(decision: SealDecision) -> Vec<u64> {
        let SealDecision::Process(_, transactions) = decision else {
            panic!("expected a processed slot, got {decision:?}")
        };
        transactions.iter().map(|tx| tx.index).collect()
    }

    /// The first slot of a tip start can arrive as its block meta alone, so it is skipped and
    /// its late transactions ignored; the next slot is applied.
    #[test]
    fn a_tip_start_skips_its_first_slot() {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        assert!(matches!(
            validator.block_meta(meta(2, 1)).unwrap(),
            SealDecision::Skip
        ));
        validator.transaction(2, transaction(0)).unwrap();
        validator.transaction(3, transaction(0)).unwrap();
        assert_eq!(indexes(validator.block_meta(meta(3, 1)).unwrap()), vec![0]);
    }

    /// A slot seals on its block meta with its transactions sorted by index; a slot without host
    /// transactions seals empty.
    #[test]
    fn a_slot_seals_on_its_block_meta() {
        let mut validator = following(2);
        validator.transaction(2, transaction(3)).unwrap();
        validator.transaction(2, transaction(1)).unwrap();
        let SealDecision::Process(block, transactions) =
            validator.block_meta(meta(2, 4)).unwrap()
        else {
            panic!()
        };
        assert_eq!(
            block,
            SealedBlock {
                slot: 2,
                block_hash: hash(2),
                parent_slot: 1,
                parent_block_hash: hash(1),
                block_time: None,
                block_height: None,
                executed_transaction_count: 4,
            }
        );
        assert_eq!(
            transactions.iter().map(|tx| tx.index).collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert!(indexes(validator.block_meta(meta(3, 0)).unwrap()).is_empty());
    }

    #[test]
    fn duplicate_and_out_of_range_indexes_halt() {
        let mut validator = following(2);
        validator.transaction(2, transaction(1)).unwrap();
        validator.transaction(2, transaction(1)).unwrap();
        assert!(validator.block_meta(meta(2, 2)).is_err());

        let mut validator = following(2);
        validator.transaction(2, transaction(1)).unwrap();
        assert!(validator.block_meta(meta(2, 1)).is_err());
    }

    /// After a replay the last slot can arrive again, whole or as its block meta alone: it is
    /// skipped. A changed block meta, or a transaction the applied slot did not hold, halts.
    #[test]
    fn a_redelivered_slot_is_skipped_and_a_late_transaction_halts() {
        let mut validator = following(2);
        validator.transaction(2, transaction(0)).unwrap();
        assert_eq!(indexes(validator.block_meta(meta(2, 2)).unwrap()), vec![0]);

        validator.transaction(2, transaction(0)).unwrap();
        assert!(matches!(
            validator.block_meta(meta(2, 2)).unwrap(),
            SealDecision::Skip
        ));
        assert!(matches!(
            validator.block_meta(meta(2, 2)).unwrap(),
            SealDecision::Skip
        ));
        let changed_time = SubscribeUpdateBlockMeta {
            block_time: Some(UnixTimestamp { timestamp: 101 }),
            ..meta(2, 2)
        };
        let changed_hash = SubscribeUpdateBlockMeta {
            blockhash: bs58::encode(hash(9)).into_string(),
            ..meta(2, 2)
        };
        for changed in [changed_time, changed_hash, meta(2, 3)] {
            assert!(validator.block_meta(changed).is_err());
        }

        let late = validator.transaction(2, transaction(1)).unwrap_err();
        assert!(
            format!("{late:#}").contains("applied without it"),
            "{late:#}"
        );
    }

    /// Every other break of the transactions-before-meta order halts.
    #[test]
    fn a_slot_out_of_order_halts() {
        // A transaction of the next slot before this slot's block meta.
        let mut validator = following(2);
        validator.transaction(2, transaction(0)).unwrap();
        assert!(validator.transaction(3, transaction(0)).is_err());

        // Another slot's block meta while this slot is open.
        let mut validator = following(2);
        validator.transaction(2, transaction(0)).unwrap();
        assert!(validator.block_meta(meta(3, 1)).is_err());

        // A transaction or a block meta for a slot before the last sealed one.
        let mut validator = following(3);
        assert!(validator.transaction(1, transaction(0)).is_err());
        assert!(validator.block_meta(meta(1, 0)).is_err());
    }

    #[test]
    fn a_slot_that_does_not_extend_the_last_halts() {
        let mut validator = following(2);
        assert!(validator
            .block_meta(SubscribeUpdateBlockMeta {
                parent_blockhash: bs58::encode(hash(9)).into_string(),
                ..meta(2, 0)
            })
            .is_err());
    }

    #[test]
    fn resume_must_observe_checkpoint_hash_before_descendant() {
        let checkpoint = BlockCheckpoint {
            slot: 5,
            block_hash: hash(5),
        };
        let mut validator =
            BlockValidator::new(StartPosition::Resume(checkpoint.clone()));
        assert!(validator.block_meta(meta(6, 0)).is_err());

        let mut validator =
            BlockValidator::new(StartPosition::Resume(checkpoint));
        validator.transaction(5, transaction(0)).unwrap();
        assert!(matches!(
            validator.block_meta(meta(5, 1)).unwrap(),
            SealDecision::Skip
        ));
        // The replayed checkpoint slot's transactions are what a re-delivery must match.
        validator.transaction(5, transaction(0)).unwrap();
        assert!(validator.transaction(5, transaction(1)).is_err());
        assert!(validator
            .block_meta(SubscribeUpdateBlockMeta {
                parent_slot: 4,
                ..meta(7, 0)
            })
            .is_err());
    }

    #[test]
    fn unapplied_resume_processes_the_checkpoint_block() {
        let checkpoint = BlockCheckpoint {
            slot: 5,
            block_hash: hash(5),
        };
        let mut validator =
            BlockValidator::new(StartPosition::ReplayFrom(checkpoint));
        assert!(matches!(
            validator.block_meta(meta(5, 0)).unwrap(),
            SealDecision::Process(..)
        ));
    }

    #[test]
    fn request_subscribes_to_host_transactions_and_block_meta() {
        let checkpoint = BlockCheckpoint {
            slot: 9,
            block_hash: hash(9),
        };
        let program = Pubkey::new_unique();
        let request = build_subscribe_request(
            &program,
            &StartPosition::Resume(checkpoint),
        );
        let [filter] = &request.transactions.values().collect::<Vec<_>>()[..]
        else {
            panic!("one transaction filter")
        };
        assert_eq!(filter.account_include, vec![program.to_string()]);
        assert_eq!((filter.vote, filter.failed), (Some(false), Some(false)));
        assert_eq!(request.blocks_meta.len(), 1);
        assert!(request.blocks.is_empty());
        assert!(request.accounts.is_empty());
        assert_eq!(request.from_slot, Some(9));
    }
}
