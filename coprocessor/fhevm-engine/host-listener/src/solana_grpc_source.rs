//! Concrete Yellowstone source validation for sealed Solana blocks.
//!
//! The listener subscribes to the successful transactions naming the host program, one per
//! message, and to every slot's block meta. At `confirmed`, Yellowstone sends a slot's
//! transactions before its block meta, live and on `from_slot` replay, so [`SlotAssembler`] seals
//! a slot when its block meta arrives. One transaction is bounded by Solana's limits, where a
//! whole block is bounded by compute units only: a block of transactions that merely list the host
//! program can exceed any message limit. Each `fhe_execute` carries its own derivation context in
//! its `FheExecutedEvent`, so the transactions are all the listener needs.

use std::collections::HashMap;

use anchor_lang::prelude::Pubkey;
use anyhow::{anyhow, bail, Context, Result};
use yellowstone_grpc_proto::prelude::{
    SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions, SubscribeUpdateBlock,
    SubscribeUpdateBlockMeta, SubscribeUpdateTransaction,
    SubscribeUpdateTransactionInfo,
};

use crate::solana_grpc_listener::{BlockCheckpoint, StartPosition};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, PartialEq)]
struct BlockIdentity(SubscribeUpdateBlock);

#[derive(Debug)]
pub(super) enum SealDecision {
    Replay,
    /// The block, and its matching transactions sorted by index.
    Process(SealedBlock, Vec<SubscribeUpdateTransactionInfo>),
}

#[derive(Debug)]
pub(super) struct BlockValidator {
    start: StartPosition,
    checkpoint_observed: bool,
    last_observed: Option<(BlockCheckpoint, BlockIdentity)>,
}

impl BlockValidator {
    pub fn new(start: StartPosition) -> Self {
        Self {
            checkpoint_observed: matches!(start, StartPosition::Tip),
            start,
            last_observed: None,
        }
    }

    pub fn seal(
        &mut self,
        mut block: SubscribeUpdateBlock,
    ) -> Result<SealDecision> {
        let block_hash = decode_hash("blockhash", &block.blockhash)?;
        let parent_block_hash =
            decode_hash("parent blockhash", &block.parent_blockhash)?;
        block
            .transactions
            .sort_by_key(|transaction| transaction.index);
        validate_transactions(
            &block.transactions,
            block.executed_transaction_count,
        )?;
        let identity = block_identity(&block);

        if !self.checkpoint_observed {
            let (StartPosition::Resume(checkpoint)
            | StartPosition::ReplayFrom(checkpoint)) = &self.start
            else {
                unreachable!("tip starts with checkpoint observed")
            };
            if block.slot != checkpoint.slot {
                bail!(
                    "inclusive replay did not begin at checkpoint slot {}; observed {}",
                    checkpoint.slot,
                    block.slot
                );
            }
            if block_hash != checkpoint.block_hash {
                bail!("checkpoint block hash changed at slot {}", block.slot);
            }
            self.checkpoint_observed = true;
            if matches!(self.start, StartPosition::Resume(_)) {
                self.last_observed = Some((checkpoint.clone(), identity));
                return Ok(SealDecision::Replay);
            }
        }

        if let Some((checkpoint, previous_identity)) = &self.last_observed {
            if block.slot == checkpoint.slot {
                if &identity == previous_identity {
                    return Ok(SealDecision::Replay);
                }
                bail!("conflicting sealed block replay at slot {}", block.slot);
            }
            if block.slot < checkpoint.slot {
                bail!("out-of-order sealed block at slot {}", block.slot);
            }
            if block.parent_slot != checkpoint.slot
                || parent_block_hash != checkpoint.block_hash
            {
                bail!(
                    "sealed block ancestry mismatch at slot {} (parent slot {})",
                    block.slot,
                    block.parent_slot
                );
            }
        }

        let sealed = SealedBlock {
            slot: block.slot,
            block_hash,
            parent_slot: block.parent_slot,
            parent_block_hash,
            block_time: block.block_time.map(|time| time.timestamp),
            block_height: block.block_height.map(|height| height.block_height),
            executed_transaction_count: block.executed_transaction_count,
        };
        self.last_observed = Some((sealed.checkpoint(), identity));
        Ok(SealDecision::Process(sealed, block.transactions))
    }
}

/// Rebuilds sealed blocks from the per-transaction stream. It holds one slot at a time: a slot's
/// transactions must all arrive before its block meta and before any other slot's messages.
/// Anything else stops the listener fail-closed rather than seal an incomplete slot. An identical
/// re-delivery of the last sealed slot is left to [`BlockValidator`]'s replay check.
#[derive(Debug, Default)]
pub(super) struct SlotAssembler {
    open: Option<(u64, Vec<SubscribeUpdateTransactionInfo>)>,
    last_sealed: Option<u64>,
}

impl SlotAssembler {
    pub fn transaction(
        &mut self,
        update: SubscribeUpdateTransaction,
    ) -> Result<()> {
        let slot = update.slot;
        let info = update.transaction.ok_or_else(|| {
            anyhow!("transaction update in slot {slot} has no transaction")
        })?;
        if let Some(sealed) = self.last_sealed.filter(|sealed| slot < *sealed) {
            bail!(
                "transaction for slot {slot} arrived after slot {sealed} was sealed"
            );
        }
        match &mut self.open {
            Some((open, transactions)) if *open == slot => {
                transactions.push(info)
            }
            Some((open, _)) => bail!(
                "transaction for slot {slot} arrived before the block meta of slot {open}"
            ),
            None => self.open = Some((slot, vec![info])),
        }
        Ok(())
    }

    /// The block `meta` seals, with the transactions buffered for its slot.
    pub fn block_meta(
        &mut self,
        meta: SubscribeUpdateBlockMeta,
    ) -> Result<SubscribeUpdateBlock> {
        let transactions = match self.open.take() {
            Some((open, transactions)) if open == meta.slot => transactions,
            Some((open, _)) => bail!(
                "block meta of slot {} arrived while slot {open} was open",
                meta.slot
            ),
            None => Vec::new(),
        };
        self.last_sealed = Some(meta.slot);
        Ok(SubscribeUpdateBlock {
            slot: meta.slot,
            blockhash: meta.blockhash,
            rewards: meta.rewards,
            block_time: meta.block_time,
            block_height: meta.block_height,
            parent_slot: meta.parent_slot,
            parent_blockhash: meta.parent_blockhash,
            executed_transaction_count: meta.executed_transaction_count,
            transactions,
            updated_account_count: 0,
            accounts: Vec::new(),
            entries_count: meta.entries_count,
            entries: Vec::new(),
        })
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

fn validate_transactions(
    transactions: &[SubscribeUpdateTransactionInfo],
    executed_transaction_count: u64,
) -> Result<()> {
    for pair in transactions.windows(2) {
        if pair[0].index == pair[1].index {
            bail!("duplicate Yellowstone transaction index {}", pair[0].index);
        }
    }
    for transaction in transactions {
        if transaction.signature.len() != 64 {
            bail!(
                "transaction {} signature has invalid length {}, expected 64 bytes",
                transaction.index,
                transaction.signature.len()
            );
        }
        if transaction.index >= executed_transaction_count {
            bail!(
                "transaction index {} is outside executed transaction count {}",
                transaction.index,
                executed_transaction_count
            );
        }
        let Some(meta) = &transaction.meta else {
            bail!("transaction {} has no status meta", transaction.index);
        };
        if meta.err.is_none() {
            let tx = transaction.transaction.as_ref().ok_or_else(|| {
                anyhow!(
                    "successful transaction {} has no transaction",
                    transaction.index
                )
            })?;
            if tx.message.is_none() {
                bail!(
                    "successful transaction {} has no message",
                    transaction.index
                );
            }
        }
    }
    Ok(())
}

fn block_identity(block: &SubscribeUpdateBlock) -> BlockIdentity {
    BlockIdentity(block.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use yellowstone_grpc_proto::prelude::{
        Message, Transaction, TransactionError, TransactionStatusMeta,
    };

    fn hash(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn successful(index: u64) -> SubscribeUpdateTransactionInfo {
        SubscribeUpdateTransactionInfo {
            signature: vec![index as u8; 64],
            is_vote: false,
            transaction: Some(Transaction {
                message: Some(Message::default()),
                ..Default::default()
            }),
            meta: Some(TransactionStatusMeta::default()),
            index,
        }
    }

    fn failed(index: u64) -> SubscribeUpdateTransactionInfo {
        SubscribeUpdateTransactionInfo {
            signature: vec![index as u8; 64],
            meta: Some(TransactionStatusMeta {
                err: Some(TransactionError { err: vec![1] }),
                ..Default::default()
            }),
            index,
            ..Default::default()
        }
    }

    fn block(
        slot: u64,
        block_hash: [u8; 32],
        parent_slot: u64,
        parent_hash: [u8; 32],
        transactions: Vec<SubscribeUpdateTransactionInfo>,
    ) -> SubscribeUpdateBlock {
        let executed_transaction_count = transactions
            .iter()
            .map(|transaction| transaction.index + 1)
            .max()
            .unwrap_or(0);
        SubscribeUpdateBlock {
            slot,
            blockhash: bs58::encode(block_hash).into_string(),
            parent_slot,
            parent_blockhash: bs58::encode(parent_hash).into_string(),
            transactions,
            executed_transaction_count,
            ..Default::default()
        }
    }

    #[test]
    fn empty_block_is_processed() {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        let decision = validator
            .seal(block(2, hash(2), 1, hash(1), vec![]))
            .unwrap();
        let SealDecision::Process(block, _) = decision else {
            panic!()
        };
        assert_eq!(block.checkpoint().slot, 2);
    }

    #[test]
    fn transactions_are_sorted_and_failed_transactions_are_retained_for_ignore()
    {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        let decision = validator
            .seal(block(2, hash(2), 1, hash(1), vec![failed(3), failed(1)]))
            .unwrap();
        let SealDecision::Process(_, transactions) = decision else {
            panic!()
        };
        assert_eq!(
            transactions.iter().map(|tx| tx.index).collect::<Vec<_>>(),
            vec![1, 3]
        );
    }

    #[test]
    fn malformed_successful_transaction_halts() {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        let mut transaction = successful(0);
        transaction.meta = None;
        assert!(validator
            .seal(block(2, hash(2), 1, hash(1), vec![transaction]))
            .is_err());
    }

    #[test]
    fn inclusive_replay_is_idempotent_but_conflicts_halt() {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        let mut original = block(2, hash(2), 1, hash(1), vec![failed(1)]);
        original.block_time = Some(Default::default());
        original.block_time.as_mut().unwrap().timestamp = 100;
        let SealDecision::Process(..) =
            validator.seal(original.clone()).unwrap()
        else {
            panic!()
        };
        assert!(matches!(
            validator.seal(original.clone()).unwrap(),
            SealDecision::Replay
        ));
        let mut changed_time = original.clone();
        changed_time.block_time.as_mut().unwrap().timestamp = 101;
        assert!(validator.seal(changed_time).is_err());
        let mut changed_payload = original.clone();
        changed_payload.transactions[0]
            .meta
            .as_mut()
            .unwrap()
            .err
            .as_mut()
            .unwrap()
            .err = vec![2];
        assert!(validator.seal(changed_payload).is_err());

        let mut changed_count = original.clone();
        changed_count.executed_transaction_count += 1;
        assert!(validator.seal(changed_count).is_err());
        assert!(validator
            .seal(block(2, hash(9), 1, hash(1), vec![failed(1)]))
            .is_err());
    }

    #[test]
    fn malformed_signature_and_out_of_range_index_halt() {
        for length in [0, 63] {
            let mut transaction = failed(0);
            transaction.signature = vec![1; length];
            let mut invalid = block(2, hash(2), 1, hash(1), vec![transaction]);
            invalid.executed_transaction_count = 1;
            assert!(BlockValidator::new(StartPosition::Tip)
                .seal(invalid)
                .is_err());
        }

        let mut invalid = block(2, hash(2), 1, hash(1), vec![failed(1)]);
        invalid.executed_transaction_count = 1;
        assert!(BlockValidator::new(StartPosition::Tip)
            .seal(invalid)
            .is_err());
    }

    #[test]
    fn resume_must_observe_checkpoint_hash_before_descendant() {
        let checkpoint = BlockCheckpoint {
            slot: 5,
            block_hash: hash(5),
        };
        let mut validator =
            BlockValidator::new(StartPosition::Resume(checkpoint));
        assert!(validator
            .seal(block(6, hash(6), 5, hash(5), vec![]))
            .is_err());

        let checkpoint = BlockCheckpoint {
            slot: 5,
            block_hash: hash(5),
        };
        let mut validator =
            BlockValidator::new(StartPosition::Resume(checkpoint));
        assert!(matches!(
            validator
                .seal(block(5, hash(5), 4, hash(4), vec![]))
                .unwrap(),
            SealDecision::Replay
        ));
        assert!(validator
            .seal(block(6, hash(6), 4, hash(4), vec![]))
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

        let decision = validator
            .seal(block(5, hash(5), 4, hash(4), vec![]))
            .unwrap();

        assert!(matches!(decision, SealDecision::Process(..)));
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

    fn update(slot: u64, index: u64) -> SubscribeUpdateTransaction {
        SubscribeUpdateTransaction {
            transaction: Some(successful(index)),
            slot,
        }
    }

    /// The block meta of `block(slot, ...)` with `executed_transaction_count` transactions.
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

    /// A slot's transactions followed by its block meta seal the block the block subscription
    /// delivered, and a slot without host transactions seals empty.
    #[test]
    fn a_slot_seals_on_its_block_meta() {
        let mut assembler = SlotAssembler::default();
        assembler.transaction(update(2, 3)).unwrap();
        assembler.transaction(update(2, 1)).unwrap();
        let sealed = assembler.block_meta(meta(2, 4)).unwrap();
        assert_eq!(
            sealed,
            SubscribeUpdateBlock {
                executed_transaction_count: 4,
                ..block(
                    2,
                    hash(2),
                    1,
                    hash(1),
                    vec![successful(3), successful(1)]
                )
            }
        );
        let SealDecision::Process(_, transactions) =
            BlockValidator::new(StartPosition::Tip)
                .seal(sealed)
                .unwrap()
        else {
            panic!()
        };
        assert_eq!(
            transactions.iter().map(|tx| tx.index).collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert!(assembler
            .block_meta(meta(3, 0))
            .unwrap()
            .transactions
            .is_empty());
    }

    /// Every way a provider can break the transactions-before-meta order stops the listener.
    #[test]
    fn a_slot_out_of_order_halts() {
        // A transaction of the next slot before this slot's block meta.
        let mut assembler = SlotAssembler::default();
        assembler.transaction(update(2, 0)).unwrap();
        assert!(assembler.transaction(update(3, 0)).is_err());

        // Another slot's block meta while this slot is open.
        let mut assembler = SlotAssembler::default();
        assembler.transaction(update(2, 0)).unwrap();
        assert!(assembler.block_meta(meta(3, 1)).is_err());

        // A transaction for a slot before the last sealed one.
        let mut assembler = SlotAssembler::default();
        assembler.block_meta(meta(3, 0)).unwrap();
        assert!(assembler.transaction(update(2, 0)).is_err());

        // A late transaction for the slot just sealed, then the next slot.
        let mut assembler = SlotAssembler::default();
        assembler.block_meta(meta(2, 2)).unwrap();
        assembler.transaction(update(2, 1)).unwrap();
        assert!(assembler.transaction(update(3, 0)).is_err());

        // An update without its transaction.
        assert!(SlotAssembler::default()
            .transaction(SubscribeUpdateTransaction {
                transaction: None,
                slot: 2,
            })
            .is_err());
    }

    /// A transaction for the slot just sealed opens it again, so the validator compares the
    /// whole slot: an identical re-delivery replays, a late transaction conflicts.
    #[test]
    fn a_late_transaction_for_the_sealed_slot_conflicts() {
        let mut validator = BlockValidator::new(StartPosition::Tip);
        let mut assembler = SlotAssembler::default();
        assembler.transaction(update(2, 0)).unwrap();
        let sealed = assembler.block_meta(meta(2, 2)).unwrap();
        assert!(matches!(
            validator.seal(sealed).unwrap(),
            SealDecision::Process(..)
        ));

        assembler.transaction(update(2, 0)).unwrap();
        let redelivered = assembler.block_meta(meta(2, 2)).unwrap();
        assert!(matches!(
            validator.seal(redelivered).unwrap(),
            SealDecision::Replay
        ));

        assembler.transaction(update(2, 0)).unwrap();
        assembler.transaction(update(2, 1)).unwrap();
        let late = assembler.block_meta(meta(2, 2)).unwrap();
        assert!(validator.seal(late).is_err());
    }
}
