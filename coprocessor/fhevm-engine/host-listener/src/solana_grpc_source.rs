//! Concrete Yellowstone source validation for sealed Solana blocks.
//!
//! `SubscribeUpdateBlock` is emitted only after the server has observed the
//! block's declared transaction count. Account filters remain a separate,
//! bounded source of the two sysvars needed for handle reconstruction; they do
//! not imply that arbitrary account state is complete.

use std::collections::{BTreeMap, HashMap};

use anyhow::{anyhow, bail, Context, Result};
use yellowstone_grpc_proto::prelude::{
    SubscribeRequest, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterBlocks, SubscribeUpdateAccount, SubscribeUpdateBlock,
    SubscribeUpdateTransactionInfo,
};

use crate::solana_grpc_listener::{BlockCheckpoint, StartPosition};
use crate::solana_slot_hashes::{
    clock_unix_timestamp, previous_bank_hash_from_slot_hashes, CLOCK_SYSVAR,
    SLOT_HASHES_SYSVAR,
};

const MAX_CONTEXT_SLOTS: usize = 256;

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
    pub previous_bank_hash: Option<[u8; 32]>,
    pub clock_unix_timestamp: Option<i64>,
}

impl SealedBlock {
    pub(super) fn checkpoint(&self) -> BlockCheckpoint {
        BlockCheckpoint {
            slot: self.slot,
            block_hash: self.block_hash,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AccountIdentity {
    data: Vec<u8>,
    write_version: u64,
    transaction_signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default)]
struct SlotContext {
    slot_hashes: Option<AccountIdentity>,
    clock: Option<AccountIdentity>,
}

impl SlotContext {
    fn insert(
        &mut self,
        pubkey: &[u8],
        identity: AccountIdentity,
    ) -> Result<()> {
        let target = if bs58::encode(pubkey).into_string() == SLOT_HASHES_SYSVAR
        {
            &mut self.slot_hashes
        } else if bs58::encode(pubkey).into_string() == CLOCK_SYSVAR {
            &mut self.clock
        } else {
            bail!("unexpected account in Solana sysvar stream")
        };

        match target {
            Some(current) if current != &identity => {
                bail!("conflicting Solana sysvar account identity")
            }
            Some(_) => Ok(()),
            None => {
                *target = Some(identity);
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct BlockIdentity {
    block_hash: [u8; 32],
    parent_slot: u64,
    parent_block_hash: [u8; 32],
    transactions: Vec<SubscribeUpdateTransactionInfo>,
}

#[derive(Debug)]
pub(super) enum SealDecision {
    Replay,
    Process(SealedBlock),
}

#[derive(Debug)]
pub(super) struct BlockValidator {
    start: StartPosition,
    resume_is_applied: bool,
    checkpoint_observed: bool,
    last_observed: Option<(BlockCheckpoint, BlockIdentity)>,
    last_committed: Option<BlockCheckpoint>,
    contexts: BTreeMap<u64, SlotContext>,
}

impl BlockValidator {
    pub fn new(start: StartPosition, resume_is_applied: bool) -> Self {
        Self {
            checkpoint_observed: matches!(start, StartPosition::Tip),
            start,
            resume_is_applied,
            last_observed: None,
            last_committed: None,
            contexts: BTreeMap::new(),
        }
    }

    pub fn observe_account(
        &mut self,
        update: SubscribeUpdateAccount,
    ) -> Result<()> {
        let info = update
            .account
            .ok_or_else(|| anyhow!("Solana account update has no account"))?;
        if self
            .last_committed
            .as_ref()
            .is_some_and(|checkpoint| update.slot <= checkpoint.slot)
        {
            bail!(
                "late Solana sysvar account update for slot {} after checkpoint",
                update.slot
            );
        }
        if !self.contexts.contains_key(&update.slot)
            && self.contexts.len() == MAX_CONTEXT_SLOTS
        {
            bail!(
                "Solana sysvar context exceeded {MAX_CONTEXT_SLOTS} pending slots"
            );
        }
        self.contexts.entry(update.slot).or_default().insert(
            &info.pubkey,
            AccountIdentity {
                data: info.data,
                write_version: info.write_version,
                transaction_signature: info.txn_signature,
            },
        )
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
        let identity = block_identity(&block, block_hash, parent_block_hash);

        if !self.checkpoint_observed {
            let StartPosition::Resume(checkpoint) = &self.start else {
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
            if self.resume_is_applied {
                self.last_observed = Some((checkpoint.clone(), identity));
                self.last_committed = Some(checkpoint.clone());
                self.contexts.remove(&block.slot);
                return Ok(SealDecision::Replay);
            }
        }

        if let Some((checkpoint, previous_identity)) = &self.last_observed {
            if block.slot == checkpoint.slot {
                if &identity == previous_identity {
                    if self
                        .last_committed
                        .as_ref()
                        .is_some_and(|committed| committed.slot == block.slot)
                    {
                        self.contexts.remove(&block.slot);
                    }
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

        let context = self.contexts.get(&block.slot);
        let (previous_bank_hash, clock_unix_timestamp) = match context {
            Some(context) => (
                context.slot_hashes.as_ref().and_then(|account| {
                    previous_bank_hash_from_slot_hashes(
                        &account.data,
                        block.slot,
                    )
                }),
                context
                    .clock
                    .as_ref()
                    .and_then(|account| clock_unix_timestamp(&account.data)),
            ),
            None => (None, None),
        };
        let sealed = SealedBlock {
            slot: block.slot,
            block_hash,
            parent_slot: block.parent_slot,
            parent_block_hash,
            block_time: block.block_time.map(|time| time.timestamp),
            block_height: block.block_height.map(|height| height.block_height),
            executed_transaction_count: block.executed_transaction_count,
            transactions: block.transactions,
            previous_bank_hash,
            clock_unix_timestamp,
        };
        self.last_observed = Some((sealed.checkpoint(), identity));
        Ok(SealDecision::Process(sealed))
    }

    pub fn refresh_context(&self, block: &mut SealedBlock) {
        let Some(context) = self.contexts.get(&block.slot) else {
            return;
        };
        block.previous_bank_hash =
            context.slot_hashes.as_ref().and_then(|account| {
                previous_bank_hash_from_slot_hashes(&account.data, block.slot)
            });
        block.clock_unix_timestamp = context
            .clock
            .as_ref()
            .and_then(|account| clock_unix_timestamp(&account.data));
    }

    pub fn commit(&mut self, block: &SealedBlock) {
        self.last_committed = Some(block.checkpoint());
        self.contexts.remove(&block.slot);
    }

    #[cfg(test)]
    fn current_checkpoint(&self) -> Option<&BlockCheckpoint> {
        self.last_committed.as_ref()
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
        },
    );
    let accounts = HashMap::from([(
        "sysvars".to_owned(),
        SubscribeRequestFilterAccounts {
            account: vec![
                SLOT_HASHES_SYSVAR.to_owned(),
                CLOCK_SYSVAR.to_owned(),
            ],
            owner: vec![],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    )]);
    SubscribeRequest {
        accounts,
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
            StartPosition::Resume(checkpoint) => Some(checkpoint.slot),
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

fn block_identity(
    block: &SubscribeUpdateBlock,
    block_hash: [u8; 32],
    parent_block_hash: [u8; 32],
) -> BlockIdentity {
    BlockIdentity {
        block_hash,
        parent_slot: block.parent_slot,
        parent_block_hash,
        transactions: block.transactions.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yellowstone_grpc_proto::prelude::{
        Message, SubscribeUpdateAccountInfo, Transaction, TransactionError,
        TransactionStatusMeta,
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
    fn empty_block_advances_checkpoint() {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let decision = validator
            .seal(block(2, hash(2), 1, hash(1), vec![]))
            .unwrap();
        let SealDecision::Process(block) = decision else {
            panic!()
        };
        validator.commit(&block);
        assert_eq!(validator.current_checkpoint().unwrap().slot, 2);
    }

    #[test]
    fn transactions_are_sorted_and_failed_transactions_are_retained_for_ignore()
    {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
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
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let mut transaction = successful(0);
        transaction.meta = None;
        assert!(validator
            .seal(block(2, hash(2), 1, hash(1), vec![transaction]))
            .is_err());
    }

    #[test]
    fn inclusive_replay_is_idempotent_but_conflicts_halt() {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let original = block(2, hash(2), 1, hash(1), vec![failed(1)]);
        let SealDecision::Process(processed) =
            validator.seal(original.clone()).unwrap()
        else {
            panic!()
        };
        validator.commit(&processed);
        assert!(matches!(
            validator.seal(original).unwrap(),
            SealDecision::Replay
        ));
        let mut changed_payload = failed(1);
        changed_payload
            .meta
            .as_mut()
            .unwrap()
            .err
            .as_mut()
            .unwrap()
            .err = vec![2];
        assert!(validator
            .seal(block(2, hash(2), 1, hash(1), vec![changed_payload]))
            .is_err());
        assert!(validator
            .seal(block(2, hash(2), 1, hash(1), vec![failed(2)]))
            .is_err());
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
            assert!(BlockValidator::new(StartPosition::Tip, true)
                .seal(invalid)
                .is_err());
        }

        let mut invalid = block(2, hash(2), 1, hash(1), vec![failed(1)]);
        invalid.executed_transaction_count = 1;
        assert!(BlockValidator::new(StartPosition::Tip, true)
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
            BlockValidator::new(StartPosition::Resume(checkpoint), true);
        assert!(validator
            .seal(block(6, hash(6), 5, hash(5), vec![]))
            .is_err());

        let checkpoint = BlockCheckpoint {
            slot: 5,
            block_hash: hash(5),
        };
        let mut validator =
            BlockValidator::new(StartPosition::Resume(checkpoint), true);
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
            BlockValidator::new(StartPosition::Resume(checkpoint), false);

        let decision = validator
            .seal(block(5, hash(5), 4, hash(4), vec![]))
            .unwrap();

        assert!(matches!(decision, SealDecision::Process(_)));
        assert!(validator.current_checkpoint().is_none());
    }

    #[test]
    fn missing_context_does_not_commit_the_sealed_block() {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let decision = validator
            .seal(block(2, hash(2), 1, hash(1), vec![successful(0)]))
            .unwrap();
        let SealDecision::Process(block) = decision else {
            panic!()
        };
        assert!(block.previous_bank_hash.is_none());
        assert!(block.clock_unix_timestamp.is_none());
        assert!(validator.current_checkpoint().is_none());
    }

    #[test]
    fn sealed_block_can_wait_for_later_sysvar_context() {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let SealDecision::Process(mut sealed) = validator
            .seal(block(2, hash(2), 1, hash(1), vec![successful(0)]))
            .unwrap()
        else {
            panic!()
        };

        let mut slot_hashes = 1_u64.to_le_bytes().to_vec();
        slot_hashes.extend_from_slice(&1_u64.to_le_bytes());
        slot_hashes.extend_from_slice(&hash(1));
        let mut clock = vec![0; 40];
        clock[32..40].copy_from_slice(&1_700_000_000_i64.to_le_bytes());
        for (address, data) in
            [(SLOT_HASHES_SYSVAR, slot_hashes), (CLOCK_SYSVAR, clock)]
        {
            validator
                .observe_account(SubscribeUpdateAccount {
                    slot: 2,
                    account: Some(SubscribeUpdateAccountInfo {
                        pubkey: bs58::decode(address).into_vec().unwrap(),
                        data,
                        write_version: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .unwrap();
        }

        validator.refresh_context(&mut sealed);
        assert_eq!(sealed.previous_bank_hash, Some(hash(1)));
        assert_eq!(sealed.clock_unix_timestamp, Some(1_700_000_000));
        assert!(validator.current_checkpoint().is_none());
        validator.commit(&sealed);
        assert_eq!(validator.current_checkpoint().unwrap().slot, 2);
    }

    #[test]
    fn context_can_lag_past_a_later_sealed_block() {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let SealDecision::Process(mut first) = validator
            .seal(block(2, hash(2), 1, hash(1), vec![successful(0)]))
            .unwrap()
        else {
            panic!()
        };
        assert!(matches!(
            validator
                .seal(block(3, hash(3), 2, hash(2), vec![successful(0)]))
                .unwrap(),
            SealDecision::Process(_)
        ));

        let mut slot_hashes = 1_u64.to_le_bytes().to_vec();
        slot_hashes.extend_from_slice(&1_u64.to_le_bytes());
        slot_hashes.extend_from_slice(&hash(1));
        let mut clock = vec![0; 40];
        clock[32..40].copy_from_slice(&1_700_000_000_i64.to_le_bytes());
        for (address, data) in
            [(SLOT_HASHES_SYSVAR, slot_hashes), (CLOCK_SYSVAR, clock)]
        {
            validator
                .observe_account(SubscribeUpdateAccount {
                    slot: 2,
                    account: Some(SubscribeUpdateAccountInfo {
                        pubkey: bs58::decode(address).into_vec().unwrap(),
                        data,
                        write_version: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .unwrap();
        }

        validator.refresh_context(&mut first);
        assert_eq!(first.previous_bank_hash, Some(hash(1)));
        assert_eq!(first.clock_unix_timestamp, Some(1_700_000_000));
        assert!(validator.current_checkpoint().is_none());
    }

    #[test]
    fn account_identity_conflict_and_context_overflow_halt() {
        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        let account = |slot, data: Vec<u8>| SubscribeUpdateAccount {
            slot,
            account: Some(SubscribeUpdateAccountInfo {
                pubkey: bs58::decode(CLOCK_SYSVAR).into_vec().unwrap(),
                data,
                write_version: 1,
                ..Default::default()
            }),
            ..Default::default()
        };
        validator.observe_account(account(1, vec![1])).unwrap();
        assert!(validator.observe_account(account(1, vec![2])).is_err());

        let mut validator = BlockValidator::new(StartPosition::Tip, true);
        for slot in 0..MAX_CONTEXT_SLOTS as u64 {
            validator.observe_account(account(slot, vec![1])).unwrap();
        }
        assert!(validator
            .observe_account(account(MAX_CONTEXT_SLOTS as u64, vec![1]))
            .is_err());
    }

    #[test]
    fn request_uses_sealed_blocks_and_separate_sysvars() {
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
        assert_eq!(request.accounts.len(), 1);
        let account_filter = request.accounts.get("sysvars").unwrap();
        assert_eq!(
            account_filter.account,
            vec![SLOT_HASHES_SYSVAR.to_owned(), CLOCK_SYSVAR.to_owned()]
        );
        assert_eq!(request.from_slot, Some(9));
    }

    #[test]
    fn resume_accepts_checkpoint_context_then_rejects_late_updates() {
        let account = || SubscribeUpdateAccount {
            slot: 9,
            account: Some(SubscribeUpdateAccountInfo {
                pubkey: bs58::decode(CLOCK_SYSVAR).into_vec().unwrap(),
                data: vec![1],
                write_version: 1,
                ..Default::default()
            }),
            ..Default::default()
        };
        let checkpoint = BlockCheckpoint {
            slot: 9,
            block_hash: hash(9),
        };
        let mut validator =
            BlockValidator::new(StartPosition::Resume(checkpoint), true);

        validator.observe_account(account()).unwrap();
        assert!(matches!(
            validator
                .seal(block(9, hash(9), 8, hash(8), vec![]))
                .unwrap(),
            SealDecision::Replay
        ));
        assert!(validator.observe_account(account()).is_err());
    }
}
