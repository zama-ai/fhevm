//! Concrete Yellowstone source validation for sealed Solana blocks.
//!
//! `SubscribeUpdateBlock` is emitted only after the server has observed the
//! block's declared transaction count. The block is the only stream: each
//! `fhe_execute` carries its own derivation context in its `FheExecutedEvent`.

use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};
use yellowstone_grpc_proto::prelude::{
    SubscribeRequest, SubscribeRequestFilterBlocks, SubscribeUpdateBlock,
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
    pub transactions: Vec<SubscribeUpdateTransactionInfo>,
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
    Process(SealedBlock),
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
            transactions: block.transactions,
        };
        self.last_observed = Some((sealed.checkpoint(), identity));
        Ok(SealDecision::Process(sealed))
    }
}

pub(super) fn build_subscribe_request(
    program_id: &str,
    start: &StartPosition,
) -> SubscribeRequest {
    let mut blocks = HashMap::new();
    blocks.insert(
        "zama_host".to_owned(),
        SubscribeRequestFilterBlocks {
            account_include: vec![program_id.to_owned()],
            include_transactions: Some(true),
            include_accounts: Some(false),
            include_entries: Some(false),
            // A cuckoo filter is the plugin's compact stand-in for a very long account list: the
            // client ships a probabilistic membership set instead of the addresses themselves and
            // accepts false positives in exchange. We subscribe to exactly one program, so the
            // explicit list above is both smaller and exact.
            cuckoo_account_include: None,
        },
    );
    SubscribeRequest {
        accounts: HashMap::new(),
        slots: HashMap::new(),
        transactions: HashMap::new(),
        transactions_status: HashMap::new(),
        blocks,
        blocks_meta: HashMap::new(),
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
        let SealDecision::Process(block) = decision else {
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
        let SealDecision::Process(block) = decision else {
            panic!()
        };
        assert_eq!(
            block
                .transactions
                .iter()
                .map(|tx| tx.index)
                .collect::<Vec<_>>(),
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
        let SealDecision::Process(_) =
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

        assert!(matches!(decision, SealDecision::Process(_)));
    }

    #[test]
    fn request_subscribes_to_sealed_blocks_only() {
        let checkpoint = BlockCheckpoint {
            slot: 9,
            block_hash: hash(9),
        };
        let request = build_subscribe_request(
            "ZamaHost11111111111111111111111111111111",
            &StartPosition::Resume(checkpoint),
        );
        assert!(request.transactions.is_empty());
        assert!(request.blocks_meta.is_empty());
        assert_eq!(request.blocks.len(), 1);
        assert!(request.accounts.is_empty());
        assert_eq!(request.from_slot, Some(9));
    }
}
