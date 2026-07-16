//! Confirmed Yellowstone completed blocks for the Solana MMR proof service.
//!
//! This adapter owns only provider transport and protobuf normalization. The
//! next stack persists each [`CompletedBlock`] and its checkpoint in one SQL
//! transaction. The caller pulls the next block only after that commit; this
//! module has no queue, retry policy, or notion of a committed checkpoint.

use futures::{Stream, StreamExt};
use std::collections::HashMap;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Channel;
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, Message, SubscribeRequest, SubscribeRequestFilterBlocks,
    SubscribeUpdate, SubscribeUpdateBlock, SubscribeUpdateTransactionInfo, TransactionStatusMeta,
};
use zama_solana_transaction::{
    CompiledInstruction as CanonicalCompiledInstruction,
    InnerInstructionGroup as CanonicalInnerInstructionGroup,
};

use super::decode::RawInstruction;

const MAX_DECODING_MESSAGE_SIZE: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BlockCheckpoint {
    pub slot: u64,
    pub block_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CanonicalTransaction {
    pub signature: [u8; 64],
    pub index: u64,
    pub succeeded: bool,
    pub is_vote: bool,
    /// Successful non-vote instructions in on-chain execution order. Failed
    /// and vote transactions remain present with an empty instruction list so
    /// the store can retain their position without producing MMR leaves.
    pub instructions: Vec<RawInstruction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CompletedBlock {
    pub slot: u64,
    pub block_hash: [u8; 32],
    pub parent_slot: u64,
    pub parent_hash: [u8; 32],
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub executed_transaction_count: u64,
    /// Sparse, provider-filtered transactions sorted by their block index.
    pub transactions: Vec<CanonicalTransaction>,
}

impl CompletedBlock {
    pub(crate) fn checkpoint(&self) -> BlockCheckpoint {
        BlockCheckpoint {
            slot: self.slot,
            block_hash: self.block_hash,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct YellowstoneSourceConfig {
    pub grpc_url: String,
    pub x_token: Option<String>,
    pub program_id: String,
}

#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub(crate) enum YellowstoneSourceError {
    #[error("retryable Yellowstone transport failure: {0}")]
    Retryable(String),
    #[error("inclusive Yellowstone replay is unavailable: {0}")]
    ReplayUnavailable(String),
    #[error("invalid Yellowstone data or configuration: {0}")]
    Invalid(String),
    #[error(
        "inclusive replay changed checkpoint: expected slot {expected_slot}, observed slot {observed_slot}"
    )]
    CheckpointChanged {
        expected_slot: u64,
        expected_hash: [u8; 32],
        observed_slot: u64,
        observed_hash: [u8; 32],
    },
    #[error(
        "block {slot} ancestry does not extend slot {expected_parent_slot}: observed parent slot {parent_slot}"
    )]
    Ancestry {
        slot: u64,
        parent_slot: u64,
        parent_hash: [u8; 32],
        expected_parent_slot: u64,
        expected_parent_hash: [u8; 32],
    },
    #[error("conflicting normalized block replay at slot {slot}")]
    ConflictingReplay { slot: u64 },
}

#[derive(Clone, Debug)]
pub(crate) struct YellowstoneBlockSource {
    config: YellowstoneSourceConfig,
}

impl YellowstoneBlockSource {
    pub(crate) fn new(config: YellowstoneSourceConfig) -> Result<Self, YellowstoneSourceError> {
        decode_hash("program id", &config.program_id)?;
        Ok(Self { config })
    }

    /// Opens one confirmed completed-block subscription. The caller owns
    /// cancellation by selecting over this future and later `next_block`
    /// futures, and owns reconnect from its last durable checkpoint.
    pub(crate) async fn subscribe(
        &self,
        cursor: Option<BlockCheckpoint>,
    ) -> Result<YellowstoneSubscription, YellowstoneSourceError> {
        let endpoint = Channel::from_shared(self.config.grpc_url.clone()).map_err(|error| {
            YellowstoneSourceError::Invalid(format!("invalid gRPC URL: {error}"))
        })?;
        let channel = endpoint.connect().await.map_err(|error| {
            YellowstoneSourceError::Retryable(format!("connect gRPC endpoint: {error}"))
        })?;
        let token: Option<MetadataValue<Ascii>> = self
            .config
            .x_token
            .as_ref()
            .map(|token| token.parse())
            .transpose()
            .map_err(|error| {
                YellowstoneSourceError::Invalid(format!("invalid Yellowstone x-token: {error}"))
            })?;
        let mut client =
            GeyserClient::with_interceptor(channel, move |mut request: tonic::Request<()>| {
                if let Some(token) = &token {
                    request.metadata_mut().insert("x-token", token.clone());
                }
                Ok(request)
            })
            .max_decoding_message_size(MAX_DECODING_MESSAGE_SIZE);
        let request = build_subscribe_request(&self.config.program_id, cursor.as_ref());
        let outbound = futures::stream::once(async move { request })
            .chain(futures::stream::pending::<SubscribeRequest>());
        let response = client
            .subscribe(outbound)
            .await
            .map_err(|status| classify_status(status, cursor.is_some()))?;
        Ok(YellowstoneSubscription {
            stream: response.into_inner(),
            replay_observed: cursor.is_none(),
            expected_replay: cursor,
            last_observed: None,
        })
    }
}

pub(crate) struct YellowstoneSubscription {
    stream: tonic::Streaming<SubscribeUpdate>,
    expected_replay: Option<BlockCheckpoint>,
    replay_observed: bool,
    last_observed: Option<CompletedBlock>,
}

impl YellowstoneSubscription {
    /// Pulls exactly one normalized completed block. No later provider update
    /// is read until the caller awaits this method again.
    pub(crate) async fn next_block(&mut self) -> Result<CompletedBlock, YellowstoneSourceError> {
        next_block(
            &mut self.stream,
            self.expected_replay.as_ref(),
            &mut self.replay_observed,
            &mut self.last_observed,
        )
        .await
    }
}

async fn next_block<S>(
    stream: &mut S,
    expected_replay: Option<&BlockCheckpoint>,
    replay_observed: &mut bool,
    last_observed: &mut Option<CompletedBlock>,
) -> Result<CompletedBlock, YellowstoneSourceError>
where
    S: Stream<Item = Result<SubscribeUpdate, tonic::Status>> + Unpin,
{
    loop {
        let update = stream.next().await;
        let Some(update) = update else {
            return Err(YellowstoneSourceError::Retryable(
                "gRPC stream closed by provider".to_owned(),
            ));
        };
        let update = update.map_err(|status| classify_status(status, expected_replay.is_some()))?;
        let Some(UpdateOneof::Block(block)) = update.update_oneof else {
            continue;
        };
        let block = normalize_block(block)?;
        if !*replay_observed {
            let expected = expected_replay.expect("cursor exists when replay is unobserved");
            if block.slot != expected.slot || block.block_hash != expected.block_hash {
                return Err(YellowstoneSourceError::CheckpointChanged {
                    expected_slot: expected.slot,
                    expected_hash: expected.block_hash,
                    observed_slot: block.slot,
                    observed_hash: block.block_hash,
                });
            }
            *replay_observed = true;
        }
        if let Some(previous) = last_observed.as_ref() {
            if block.slot == previous.slot {
                if &block != previous {
                    return Err(YellowstoneSourceError::ConflictingReplay { slot: block.slot });
                }
                return Ok(block);
            }
            if block.parent_slot != previous.slot || block.parent_hash != previous.block_hash {
                return Err(YellowstoneSourceError::Ancestry {
                    slot: block.slot,
                    parent_slot: block.parent_slot,
                    parent_hash: block.parent_hash,
                    expected_parent_slot: previous.slot,
                    expected_parent_hash: previous.block_hash,
                });
            }
        }
        *last_observed = Some(block.clone());
        return Ok(block);
    }
}

fn build_subscribe_request(program_id: &str, cursor: Option<&BlockCheckpoint>) -> SubscribeRequest {
    let blocks = HashMap::from([(
        "zama_host".to_owned(),
        SubscribeRequestFilterBlocks {
            account_include: vec![program_id.to_owned()],
            include_transactions: Some(true),
            include_accounts: Some(false),
            include_entries: Some(false),
        },
    )]);
    SubscribeRequest {
        accounts: HashMap::new(),
        slots: HashMap::new(),
        transactions: HashMap::new(),
        transactions_status: HashMap::new(),
        blocks,
        blocks_meta: HashMap::new(),
        entry: HashMap::new(),
        commitment: Some(yellowstone_grpc_proto::prelude::CommitmentLevel::Confirmed as i32),
        accounts_data_slice: vec![],
        ping: None,
        from_slot: cursor.map(|checkpoint| checkpoint.slot),
    }
}

fn normalize_block(
    mut block: SubscribeUpdateBlock,
) -> Result<CompletedBlock, YellowstoneSourceError> {
    let block_hash = decode_hash("blockhash", &block.blockhash)?;
    let parent_hash = decode_hash("parent blockhash", &block.parent_blockhash)?;
    block
        .transactions
        .sort_by_key(|transaction| transaction.index);
    for pair in block.transactions.windows(2) {
        if pair[0].index == pair[1].index {
            return Err(YellowstoneSourceError::Invalid(format!(
                "duplicate transaction index {}",
                pair[0].index
            )));
        }
    }
    let transactions = block
        .transactions
        .into_iter()
        .map(|transaction| normalize_transaction(transaction, block.executed_transaction_count))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CompletedBlock {
        slot: block.slot,
        block_hash,
        parent_slot: block.parent_slot,
        parent_hash,
        block_time: block.block_time.map(|time| time.timestamp),
        block_height: block.block_height.map(|height| height.block_height),
        executed_transaction_count: block.executed_transaction_count,
        transactions,
    })
}

fn normalize_transaction(
    transaction: SubscribeUpdateTransactionInfo,
    executed_transaction_count: u64,
) -> Result<CanonicalTransaction, YellowstoneSourceError> {
    if transaction.index >= executed_transaction_count {
        return Err(YellowstoneSourceError::Invalid(format!(
            "transaction index {} is outside executed transaction count {}",
            transaction.index, executed_transaction_count
        )));
    }
    let signature = <[u8; 64]>::try_from(transaction.signature.as_slice()).map_err(|_| {
        YellowstoneSourceError::Invalid(format!(
            "transaction {} signature has invalid length {}, expected 64 bytes",
            transaction.index,
            transaction.signature.len()
        ))
    })?;
    let meta = transaction.meta.as_ref().ok_or_else(|| {
        YellowstoneSourceError::Invalid(format!(
            "transaction {} has no status metadata",
            transaction.index
        ))
    })?;
    let succeeded = meta.err.is_none();
    let instructions = if !succeeded || transaction.is_vote {
        Vec::new()
    } else {
        let message = transaction
            .transaction
            .as_ref()
            .and_then(|transaction| transaction.message.as_ref())
            .ok_or_else(|| {
                YellowstoneSourceError::Invalid(format!(
                    "successful transaction {} has no message",
                    transaction.index
                ))
            })?;
        resolve_instructions(message, meta)?
    };
    Ok(CanonicalTransaction {
        signature,
        index: transaction.index,
        succeeded,
        is_vote: transaction.is_vote,
        instructions,
    })
}

fn resolve_instructions(
    message: &Message,
    meta: &TransactionStatusMeta,
) -> Result<Vec<RawInstruction>, YellowstoneSourceError> {
    let static_keys = validated_keys(&message.account_keys)?;
    let loaded_writable = validated_keys(&meta.loaded_writable_addresses)?;
    let loaded_readonly = validated_keys(&meta.loaded_readonly_addresses)?;
    let top_level = message
        .instructions
        .iter()
        .map(|instruction| CanonicalCompiledInstruction {
            program_id_index: instruction.program_id_index as usize,
            account_indices: instruction
                .accounts
                .iter()
                .map(|index| *index as usize)
                .collect(),
            data: instruction.data.clone(),
            stack_height: None,
        })
        .collect();
    let inner = meta
        .inner_instructions
        .iter()
        .map(|group| CanonicalInnerInstructionGroup {
            top_level_index: group.index as usize,
            instructions: group
                .instructions
                .iter()
                .map(|instruction| CanonicalCompiledInstruction {
                    program_id_index: instruction.program_id_index as usize,
                    account_indices: instruction
                        .accounts
                        .iter()
                        .map(|index| *index as usize)
                        .collect(),
                    data: instruction.data.clone(),
                    stack_height: instruction.stack_height,
                })
                .collect(),
        })
        .collect();
    zama_solana_transaction::resolve_transaction(
        &static_keys,
        &loaded_writable,
        &loaded_readonly,
        top_level,
        inner,
    )
    .map_err(|error| {
        YellowstoneSourceError::Invalid(format!("invalid transaction instructions: {error}"))
    })
    .map(|instructions| {
        instructions
            .into_iter()
            .map(|instruction| RawInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: instruction.data,
                top_level_index: instruction.top_level_index,
                stack_height: Some(instruction.stack_height),
            })
            .collect()
    })
}

fn validated_keys(keys: &[Vec<u8>]) -> Result<Vec<[u8; 32]>, YellowstoneSourceError> {
    keys.iter()
        .enumerate()
        .map(|(index, key)| {
            <[u8; 32]>::try_from(key.as_slice()).map_err(|_| {
                YellowstoneSourceError::Invalid(format!(
                    "account key {index} has invalid length {}, expected 32 bytes",
                    key.len()
                ))
            })
        })
        .collect()
}

fn decode_hash(name: &str, encoded: &str) -> Result<[u8; 32], YellowstoneSourceError> {
    let bytes = bs58::decode(encoded)
        .into_vec()
        .map_err(|error| YellowstoneSourceError::Invalid(format!("invalid {name}: {error}")))?;
    <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| {
        YellowstoneSourceError::Invalid(format!(
            "{name} has invalid length {}, expected 32 bytes",
            bytes.len()
        ))
    })
}

fn classify_status(status: tonic::Status, is_replay: bool) -> YellowstoneSourceError {
    let message = status.message();
    if is_replay
        && (message == "from_slot is not supported"
            || (message.starts_with("broadcast from ") && message.contains(" is not available")))
    {
        YellowstoneSourceError::ReplayUnavailable(status.to_string())
    } else {
        YellowstoneSourceError::Retryable(status.to_string())
    }
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use super::*;
    use yellowstone_grpc_proto::prelude::{
        BlockHeight, CompiledInstruction, InnerInstruction, InnerInstructions, Transaction,
        TransactionError, UnixTimestamp,
    };

    fn hash(tag: u8) -> [u8; 32] {
        [tag; 32]
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

    fn successful(index: u64) -> SubscribeUpdateTransactionInfo {
        let digest = Sha256::digest(b"global:make_handle_public");
        let mut lifecycle_data = digest[..8].to_vec();
        lifecycle_data.extend_from_slice(&[9; 32]);
        SubscribeUpdateTransactionInfo {
            signature: vec![index as u8; 64],
            transaction: Some(Transaction {
                message: Some(Message {
                    account_keys: vec![
                        hash(1).to_vec(),
                        hash(2).to_vec(),
                        hash(3).to_vec(),
                        hash(4).to_vec(),
                        hash(5).to_vec(),
                    ],
                    instructions: vec![CompiledInstruction {
                        program_id_index: 0,
                        accounts: vec![1, 2, 3],
                        data: lifecycle_data,
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            meta: Some(TransactionStatusMeta {
                inner_instructions: vec![InnerInstructions {
                    index: 0,
                    instructions: vec![InnerInstruction {
                        program_id_index: 4,
                        accounts: vec![1],
                        data: vec![8],
                        stack_height: Some(2),
                    }],
                }],
                ..Default::default()
            }),
            index,
            ..Default::default()
        }
    }

    fn vote(index: u64) -> SubscribeUpdateTransactionInfo {
        SubscribeUpdateTransactionInfo {
            is_vote: true,
            ..successful(index)
        }
    }

    fn block(slot: u64, transactions: Vec<SubscribeUpdateTransactionInfo>) -> SubscribeUpdateBlock {
        SubscribeUpdateBlock {
            slot,
            blockhash: bs58::encode(hash(slot as u8)).into_string(),
            parent_slot: slot - 1,
            parent_blockhash: bs58::encode(hash((slot - 1) as u8)).into_string(),
            block_time: Some(UnixTimestamp {
                timestamp: 1_700_000_000,
            }),
            block_height: Some(BlockHeight { block_height: 99 }),
            executed_transaction_count: transactions
                .iter()
                .map(|transaction| transaction.index + 1)
                .max()
                .unwrap_or(0),
            transactions,
            ..Default::default()
        }
    }

    fn update(block: SubscribeUpdateBlock) -> SubscribeUpdate {
        SubscribeUpdate {
            update_oneof: Some(UpdateOneof::Block(block)),
            ..Default::default()
        }
    }

    #[test]
    fn request_is_one_confirmed_filtered_block_subscription() {
        let checkpoint = BlockCheckpoint {
            slot: 9,
            block_hash: hash(9),
        };
        let request = build_subscribe_request(
            "ZamaHost11111111111111111111111111111111",
            Some(&checkpoint),
        );
        assert_eq!(request.blocks.len(), 1);
        assert!(request.accounts.is_empty());
        assert!(request.slots.is_empty());
        assert!(request.transactions.is_empty());
        assert!(request.transactions_status.is_empty());
        assert!(request.blocks_meta.is_empty());
        assert!(request.entry.is_empty());
        assert_eq!(request.from_slot, Some(9));
        assert_eq!(
            request.commitment,
            Some(yellowstone_grpc_proto::prelude::CommitmentLevel::Confirmed as i32)
        );
        let filter = request.blocks.get("zama_host").unwrap();
        assert_eq!(
            filter.account_include,
            vec!["ZamaHost11111111111111111111111111111111"]
        );
        assert_eq!(filter.include_transactions, Some(true));
        assert_eq!(filter.include_accounts, Some(false));
        assert_eq!(filter.include_entries, Some(false));
    }

    #[test]
    fn source_rejects_invalid_program_id_before_connecting() {
        let result = YellowstoneBlockSource::new(YellowstoneSourceConfig {
            grpc_url: "http://127.0.0.1:10000".to_owned(),
            x_token: None,
            program_id: "not-a-program-id".to_owned(),
        });
        assert!(matches!(result, Err(YellowstoneSourceError::Invalid(_))));
    }

    #[test]
    fn normalizes_empty_block_and_full_metadata() {
        let normalized = normalize_block(block(7, vec![])).unwrap();
        assert_eq!(normalized.slot, 7);
        assert_eq!(normalized.block_hash, hash(7));
        assert_eq!(normalized.parent_slot, 6);
        assert_eq!(normalized.parent_hash, hash(6));
        assert_eq!(normalized.block_time, Some(1_700_000_000));
        assert_eq!(normalized.block_height, Some(99));
        assert_eq!(normalized.executed_transaction_count, 0);
        assert!(normalized.transactions.is_empty());
    }

    #[test]
    fn resolves_successful_instruction_payload_in_execution_order() {
        let normalized = normalize_block(block(7, vec![successful(0)])).unwrap();
        let transaction = &normalized.transactions[0];
        assert_eq!(transaction.signature, [0; 64]);
        assert!(transaction.succeeded);
        assert!(!transaction.is_vote);
        assert_eq!(transaction.instructions.len(), 2);
        assert_eq!(transaction.instructions[0].program_id, hash(1));
        assert_eq!(
            transaction.instructions[0].accounts,
            vec![hash(2), hash(3), hash(4)]
        );
        assert_eq!(transaction.instructions[0].stack_height, Some(1));
        assert_eq!(transaction.instructions[1].program_id, hash(5));
        assert_eq!(transaction.instructions[1].data, vec![8]);
        assert_eq!(transaction.instructions[1].stack_height, Some(2));
        let lifecycle =
            super::super::decode::decode_program_instructions(hash(1), &transaction.instructions)
                .unwrap();
        assert!(matches!(
            lifecycle.as_slice(),
            [super::super::decode::DecodedInstruction::MakeHandlePublic {
                encrypted_value,
                handle,
            }] if *encrypted_value == hash(4) && *handle == [9; 32]
        ));
    }

    #[test]
    fn retains_sorted_failed_and_vote_transaction_identities() {
        let normalized = normalize_block(block(7, vec![vote(3), failed(1)])).unwrap();
        assert_eq!(
            normalized
                .transactions
                .iter()
                .map(|transaction| transaction.index)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert!(!normalized.transactions[0].succeeded);
        assert!(normalized.transactions[0].instructions.is_empty());
        assert!(normalized.transactions[1].succeeded);
        assert!(normalized.transactions[1].is_vote);
        assert!(normalized.transactions[1].instructions.is_empty());
    }

    #[test]
    fn rejects_malformed_hash_signature_metadata_and_message() {
        let mut malformed = block(7, vec![]);
        malformed.blockhash = "not-base58-0".to_owned();
        assert!(matches!(
            normalize_block(malformed),
            Err(YellowstoneSourceError::Invalid(_))
        ));

        let mut transaction = failed(0);
        transaction.signature.pop();
        assert!(normalize_block(block(7, vec![transaction])).is_err());

        let mut transaction = failed(0);
        transaction.meta = None;
        assert!(normalize_block(block(7, vec![transaction])).is_err());

        let mut transaction = successful(0);
        transaction.transaction.as_mut().unwrap().message = None;
        assert!(normalize_block(block(7, vec![transaction])).is_err());
    }

    #[test]
    fn rejects_duplicate_and_out_of_range_sparse_indexes() {
        assert!(normalize_block(block(7, vec![failed(1), failed(1)])).is_err());
        let mut invalid = block(7, vec![failed(2)]);
        invalid.executed_transaction_count = 2;
        assert!(normalize_block(invalid).is_err());
    }

    #[test]
    fn rejects_bad_account_keys_and_shared_decoder_failures() {
        let mut short_key = successful(0);
        short_key
            .transaction
            .as_mut()
            .unwrap()
            .message
            .as_mut()
            .unwrap()
            .account_keys[0]
            .pop();
        assert!(normalize_block(block(7, vec![short_key])).is_err());

        let mut bad_index = successful(0);
        bad_index
            .transaction
            .as_mut()
            .unwrap()
            .message
            .as_mut()
            .unwrap()
            .instructions[0]
            .program_id_index = 9;
        assert!(normalize_block(block(7, vec![bad_index])).is_err());
    }

    #[tokio::test]
    async fn inclusive_replay_must_be_first_and_match_hash() {
        let checkpoint = BlockCheckpoint {
            slot: 7,
            block_hash: hash(7),
        };
        let mut stream = futures::stream::iter([Ok(update(block(8, vec![])))]);
        let mut replay_observed = false;
        let mut last_observed = None;
        let result = next_block(
            &mut stream,
            Some(&checkpoint),
            &mut replay_observed,
            &mut last_observed,
        )
        .await;
        assert!(matches!(
            result,
            Err(YellowstoneSourceError::CheckpointChanged {
                expected_slot: 7,
                observed_slot: 8,
                ..
            })
        ));

        let mut wrong_hash = block(7, vec![]);
        wrong_hash.blockhash = bs58::encode(hash(9)).into_string();
        let mut stream = futures::stream::iter([Ok(update(wrong_hash))]);
        let mut replay_observed = false;
        let mut last_observed = None;
        let result = next_block(
            &mut stream,
            Some(&checkpoint),
            &mut replay_observed,
            &mut last_observed,
        )
        .await;
        assert_eq!(
            result.unwrap_err(),
            YellowstoneSourceError::CheckpointChanged {
                expected_slot: 7,
                expected_hash: hash(7),
                observed_slot: 7,
                observed_hash: hash(9),
            }
        );

        let mut stream = futures::stream::iter([Ok(update(block(7, vec![])))]);
        let mut replay_observed = false;
        let mut last_observed = None;
        let replay = next_block(
            &mut stream,
            Some(&checkpoint),
            &mut replay_observed,
            &mut last_observed,
        )
        .await
        .unwrap();
        assert_eq!(replay.checkpoint(), checkpoint);
    }

    #[test]
    fn replay_unavailable_is_terminal_but_transport_is_retryable() {
        for message in [
            "from_slot is not supported",
            "broadcast from 7 is not available, last available: 12",
        ] {
            assert!(matches!(
                classify_status(tonic::Status::internal(message), true),
                YellowstoneSourceError::ReplayUnavailable(_)
            ));
        }
        for status in [
            tonic::Status::unavailable("connection reset"),
            tonic::Status::internal("failed to send replay update"),
            tonic::Status::internal("broadcast from 7 is not available"),
        ] {
            assert!(matches!(
                classify_status(status, false),
                YellowstoneSourceError::Retryable(_)
            ));
        }
    }

    #[tokio::test]
    async fn pulls_one_block_per_await_and_checks_ancestry() {
        let mut stream =
            futures::stream::iter([Ok(update(block(7, vec![]))), Ok(update(block(8, vec![])))]);
        let mut replay_observed = true;
        let mut last_observed = None;
        assert_eq!(
            next_block(&mut stream, None, &mut replay_observed, &mut last_observed,)
                .await
                .unwrap()
                .slot,
            7
        );
        assert_eq!(
            next_block(&mut stream, None, &mut replay_observed, &mut last_observed,)
                .await
                .unwrap()
                .slot,
            8
        );
    }

    #[tokio::test]
    async fn exact_block_replay_is_exposed_but_conflict_halts() {
        let original = block(7, vec![successful(0)]);
        let mut changed = original.clone();
        changed.block_time.as_mut().unwrap().timestamp += 1;
        let mut stream = futures::stream::iter([
            Ok(update(original.clone())),
            Ok(update(original)),
            Ok(update(changed)),
        ]);
        let mut replay_observed = true;
        let mut last_observed = None;
        let first = next_block(&mut stream, None, &mut replay_observed, &mut last_observed)
            .await
            .unwrap();
        let replay = next_block(&mut stream, None, &mut replay_observed, &mut last_observed)
            .await
            .unwrap();
        assert_eq!(first, replay);
        assert_eq!(
            next_block(&mut stream, None, &mut replay_observed, &mut last_observed,)
                .await
                .unwrap_err(),
            YellowstoneSourceError::ConflictingReplay { slot: 7 }
        );
    }

    #[tokio::test]
    async fn rejects_live_ancestry_mismatch() {
        let mut second = block(8, vec![]);
        second.parent_slot = 6;
        second.parent_blockhash = bs58::encode(hash(6)).into_string();
        let mut stream = futures::stream::iter([Ok(update(block(7, vec![]))), Ok(update(second))]);
        let mut replay_observed = true;
        let mut last_observed = None;
        next_block(&mut stream, None, &mut replay_observed, &mut last_observed)
            .await
            .unwrap();
        assert!(matches!(
            next_block(&mut stream, None, &mut replay_observed, &mut last_observed,).await,
            Err(YellowstoneSourceError::Ancestry { .. })
        ));
    }
}
