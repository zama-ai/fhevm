use alloy_primitives::{Address, B256, Bytes, U256};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Event payload for the backtrack-reorg consumer.
/// Carries the block number where the reorg was detected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorgBacktrackEvent {
    pub block_number: u64,
    /// Hash of the new canonical block at `block_number`.
    /// The backtrack handler upserts this block itself (idempotent)
    pub block_hash: B256,
    /// Parent hash of the new block at `block_number`.
    /// The backtrack starts by fetching this hash from RPC.
    pub parent_hash: B256,
}

/// Validation errors for [`FilterCommand`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FilterCommandValidationError {
    #[error("consumer_id must not be empty")]
    EmptyConsumerId,
    #[error("at least one of from or to must be set")]
    MissingContractAddresses,
}

/// Event payload for the control.watch and control.unwatch consumers.
/// Chain ID is omitted because the listener injects chain scope through namespaced routing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterCommand {
    pub consumer_id: String,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub log_address: Option<Address>,
}

impl FilterCommand {
    /// Validate that the command has a non-empty consumer ID and at least one address.
    ///
    /// Normalizes `consumer_id` by trimming leading/trailing whitespace so
    /// the stored value is always canonical.
    #[must_use = "validation result must be checked"]
    pub fn validate(&mut self) -> Result<(), FilterCommandValidationError> {
        self.consumer_id = self.consumer_id.trim().to_owned();
        if self.consumer_id.is_empty() {
            return Err(FilterCommandValidationError::EmptyConsumerId);
        }
        if self.from.is_none() && self.to.is_none() && self.log_address.is_none() {
            return Err(FilterCommandValidationError::MissingContractAddresses);
        }
        Ok(())
    }
}

/// Validation errors for [`CatchupPayload`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CatchupPayloadValidationError {
    #[error("consumer_id must not be empty")]
    EmptyConsumerId,
    #[error("block_start must be <= block_end")]
    InvalidBlockRange,
}

/// Event payload for the `catchup` consumer.
///
/// Requests historical replay of blocks `[block_start, block_end]` (inclusive)
/// for a single consumer. Chain ID is omitted because the listener injects
/// chain scope through namespaced routing, like other control-plane payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatchupPayload {
    pub consumer_id: String,
    pub block_start: u64,
    pub block_end: u64,
}

impl CatchupPayload {
    /// Validate the payload. Trims `consumer_id` so the stored value is canonical.
    /// Allows `block_start == block_end` (single-block replay).
    #[must_use = "validation result must be checked"]
    pub fn validate(&mut self) -> Result<(), CatchupPayloadValidationError> {
        self.consumer_id = self.consumer_id.trim().to_owned();
        if self.consumer_id.is_empty() {
            return Err(CatchupPayloadValidationError::EmptyConsumerId);
        }
        if self.block_start > self.block_end {
            return Err(CatchupPayloadValidationError::InvalidBlockRange);
        }
        Ok(())
    }
}

/// A log with its block-global index.
///
/// Wire format: `{ log_index, address, topics, data }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedLog {
    /// Block-global log index.
    pub log_index: u64,
    /// The address which emitted this log.
    pub address: Address,
    /// The indexed topic list.
    pub topics: Vec<B256>,
    /// The plain data.
    pub data: Bytes,
}

/// A transaction with its associated logs, as published in the block payload.
///
/// Part of the RFC 006 output contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionPayload {
    /// Sender address.
    pub from: Address,
    /// Recipient address, `None` for contract creation.
    pub to: Option<Address>,
    /// Transaction hash.
    pub hash: B256,
    /// Position of this transaction within the block.
    pub transaction_index: u64,
    /// Value transferred in wei.
    pub value: U256,
    /// Calldata / input data.
    pub data: Bytes,
    /// Logs emitted during this transaction's execution.
    pub logs: Vec<IndexedLog>,
}

/// Processing flow that produced a block payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockFlow {
    /// Normal forward chain synchronization.
    Live,
    /// Block republished during a reorg backtrack.
    Reorged,
    /// Historical catch-up / replay (reserved for future use).
    Catchup,
}

/// Block payload published to the message broker after a block is validated and persisted.
///
/// Conforms to the RFC 006 output contract. Embeds block header fields plus full
/// transactions with their logs. Downstream consumers (Host, Gateway, Relayer)
/// deserialize this to learn about new canonical blocks and process their events.
///
/// Used for both live flow and catchup replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockPayload {
    /// Processing flow that produced this payload (Live, Reorged, or Catchup).
    pub flow: BlockFlow,
    /// The chain ID this block belongs to (e.g. 1 for Ethereum mainnet).
    pub chain_id: u64,
    /// Block number (height).
    pub block_number: u64,
    /// Block hash.
    pub block_hash: B256,
    /// Parent block hash.
    pub parent_hash: B256,
    /// Block timestamp (seconds since Unix epoch).
    pub timestamp: u64,
    /// Transactions with their logs, ordered by transaction index.
    pub transactions: Vec<TransactionPayload>,
}

// --- Debug-only Display impls (remove when no longer needed) ---

impl fmt::Display for IndexedLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_hex = format!("{}", self.data);
        write!(
            f,
            "Log(index={}, addr={}, data={}, topics={})",
            self.log_index,
            self.address,
            data_hex,
            self.topics.len(),
        )?;
        for topic in &self.topics {
            writeln!(f, "  {topic}")?;
        }
        Ok(())
    }
}

impl fmt::Display for TransactionPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_str = match &self.to {
            Some(addr) => format!("{addr}"),
            None => "contract creation".to_string(),
        };
        write!(
            f,
            "Tx(hash={}, from={}, to={}, idx={}, value={}, logs={})",
            self.hash,
            self.from,
            to_str,
            self.transaction_index,
            self.value,
            self.logs.len(),
        )?;
        for log in &self.logs {
            writeln!(f, "  {log}")?;
        }
        Ok(())
    }
}

impl fmt::Display for BlockPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Block(flow={:?}, chain={}, number={}, hash={}, ts={}, txs={})",
            self.flow,
            self.chain_id,
            self.block_number,
            self.block_hash,
            self.timestamp,
            self.transactions.len(),
        )?;
        for tx in &self.transactions {
            writeln!(f, "  {tx}")?;
        }
        Ok(())
    }
}

// --- End debug-only Display impls ---

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_payload() -> BlockPayload {
        BlockPayload {
            flow: BlockFlow::Live,
            chain_id: 1,
            block_number: 12345,
            block_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
                .parse()
                .unwrap(),
            parent_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .parse()
                .unwrap(),
            timestamp: 1700000000,
            transactions: vec![TransactionPayload {
                from: "0x0000000000000000000000000000000000000001"
                    .parse()
                    .unwrap(),
                to: Some(
                    "0x00000000000000000000000000000000deadbeef"
                        .parse()
                        .unwrap(),
                ),
                hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
                    .parse()
                    .unwrap(),
                transaction_index: 0,
                value: U256::from(1_000_000_000_000_000_000u128),
                data: Bytes::from_static(&[0xa9, 0x05, 0x9c, 0xbb]),
                logs: vec![IndexedLog {
                    log_index: 0,
                    address: "0x00000000000000000000000000000000deadbeef"
                        .parse()
                        .unwrap(),
                    topics: vec![
                        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                            .parse()
                            .unwrap(),
                    ],
                    data: Bytes::from_static(&[0x00, 0x01, 0x02]),
                }],
            }],
        }
    }

    #[test]
    fn block_payload_serializes_to_expected_json_and_round_trips() {
        let payload = sample_payload();
        let json = serde_json::to_value(&payload).unwrap();
        let expected = json!({
            "flow": "LIVE",
            "chain_id": 1,
            "block_number": 12345,
            "block_hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            "parent_hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "timestamp": 1700000000,
            "transactions": [{
                "from": "0x0000000000000000000000000000000000000001",
                "to": "0x00000000000000000000000000000000deadbeef",
                "hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "transaction_index": 0,
                "value": "0xde0b6b3a7640000",
                "data": "0xa9059cbb",
                "logs": [{
                    "log_index": 0,
                    "address": "0x00000000000000000000000000000000deadbeef",
                    "topics": [
                        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                    ],
                    "data": "0x000102"
                }]
            }]
        });

        assert_eq!(json, expected);

        // Round-trip
        let deserialized: BlockPayload = serde_json::from_value(json).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn empty_block_serializes_with_empty_transactions() {
        let payload = BlockPayload {
            flow: BlockFlow::Live,
            chain_id: 1,
            block_number: 0,
            block_hash: B256::ZERO,
            parent_hash: B256::ZERO,
            timestamp: 0,
            transactions: vec![],
        };

        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["transactions"], json!([]));
    }

    #[test]
    fn filter_command_rejects_empty_consumer_id() {
        let mut cmd = FilterCommand {
            consumer_id: "   ".into(),
            from: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
            to: None,
            log_address: None,
        };
        assert_eq!(
            cmd.validate().unwrap_err(),
            FilterCommandValidationError::EmptyConsumerId,
        );
    }

    #[test]
    fn filter_command_rejects_literal_empty_consumer_id() {
        let mut cmd = FilterCommand {
            consumer_id: "".into(),
            from: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
            to: None,
            log_address: None,
        };
        assert_eq!(
            cmd.validate().unwrap_err(),
            FilterCommandValidationError::EmptyConsumerId,
        );
    }

    #[test]
    fn filter_command_rejects_missing_addresses() {
        let mut cmd = FilterCommand {
            consumer_id: "gateway".into(),
            from: None,
            to: None,
            log_address: None,
        };
        assert_eq!(
            cmd.validate().unwrap_err(),
            FilterCommandValidationError::MissingContractAddresses,
        );
    }

    #[test]
    fn filter_command_accepts_valid() {
        let mut cmd = FilterCommand {
            consumer_id: "gateway".into(),
            from: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
            to: None,
            log_address: None,
        };
        cmd.validate().unwrap();
    }

    #[test]
    fn filter_command_trims_consumer_id() {
        let mut cmd = FilterCommand {
            consumer_id: "  gateway  ".into(),
            from: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
            to: None,
            log_address: None,
        };
        cmd.validate().unwrap();
        assert_eq!(cmd.consumer_id, "gateway");
    }

    #[test]
    fn filter_command_accepts_to_only() {
        let mut cmd = FilterCommand {
            consumer_id: "gateway".into(),
            from: None,
            to: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
            log_address: None,
        };
        cmd.validate().unwrap();
    }

    #[test]
    fn filter_command_round_trips() {
        let filter = FilterCommand {
            consumer_id: "gateway".into(),
            from: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
            to: None,
            log_address: None,
        };

        let json = serde_json::to_value(&filter).unwrap();
        let expected = json!({
            "consumer_id": "gateway",
            "from": "0x00000000000000000000000000000000deadbeef",
            "to": null,
            "log_address": null,
        });
        assert_eq!(json, expected);

        let deserialized: FilterCommand = serde_json::from_value(json).unwrap();
        assert_eq!(filter, deserialized);
    }

    #[test]
    fn filter_command_accepts_log_address_only() {
        let mut cmd = FilterCommand {
            consumer_id: "gateway".into(),
            from: None,
            to: None,
            log_address: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
        };
        cmd.validate().unwrap();
    }

    #[test]
    fn filter_command_round_trips_with_log_address() {
        let filter = FilterCommand {
            consumer_id: "gateway".into(),
            from: None,
            to: None,
            log_address: Some(
                "0x00000000000000000000000000000000deadbeef"
                    .parse()
                    .unwrap(),
            ),
        };

        let json = serde_json::to_value(&filter).unwrap();
        let expected = json!({
            "consumer_id": "gateway",
            "from": null,
            "to": null,
            "log_address": "0x00000000000000000000000000000000deadbeef",
        });
        assert_eq!(json, expected);

        let deserialized: FilterCommand = serde_json::from_value(json).unwrap();
        assert_eq!(filter, deserialized);
    }

    #[test]
    fn catchup_payload_rejects_empty_consumer_id() {
        let mut p = CatchupPayload {
            consumer_id: "".into(),
            block_start: 1,
            block_end: 10,
        };
        assert_eq!(
            p.validate().unwrap_err(),
            CatchupPayloadValidationError::EmptyConsumerId,
        );
    }

    #[test]
    fn catchup_payload_rejects_whitespace_consumer_id() {
        let mut p = CatchupPayload {
            consumer_id: "   ".into(),
            block_start: 1,
            block_end: 10,
        };
        assert_eq!(
            p.validate().unwrap_err(),
            CatchupPayloadValidationError::EmptyConsumerId,
        );
    }

    #[test]
    fn catchup_payload_trims_consumer_id() {
        let mut p = CatchupPayload {
            consumer_id: "  gw  ".into(),
            block_start: 1,
            block_end: 10,
        };
        p.validate().unwrap();
        assert_eq!(p.consumer_id, "gw");
    }

    #[test]
    fn catchup_payload_rejects_inverted_range() {
        let mut p = CatchupPayload {
            consumer_id: "gateway".into(),
            block_start: 10,
            block_end: 5,
        };
        assert_eq!(
            p.validate().unwrap_err(),
            CatchupPayloadValidationError::InvalidBlockRange,
        );
    }

    #[test]
    fn catchup_payload_accepts_single_block_range() {
        let mut p = CatchupPayload {
            consumer_id: "gateway".into(),
            block_start: 42,
            block_end: 42,
        };
        p.validate().unwrap();
    }

    #[test]
    fn catchup_payload_round_trips() {
        let payload = CatchupPayload {
            consumer_id: "gateway".into(),
            block_start: 100,
            block_end: 200,
        };

        let json = serde_json::to_value(&payload).unwrap();
        let expected = json!({
            "consumer_id": "gateway",
            "block_start": 100,
            "block_end": 200,
        });
        assert_eq!(json, expected);

        let deserialized: CatchupPayload = serde_json::from_value(json).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn contract_creation_serializes_to_null() {
        let tx = TransactionPayload {
            from: "0x0000000000000000000000000000000000000001"
                .parse()
                .unwrap(),
            to: None,
            hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
                .parse()
                .unwrap(),
            transaction_index: 3,
            value: U256::ZERO,
            data: Bytes::from_static(&[0x60, 0x60, 0x60, 0x40, 0x52]),
            logs: vec![],
        };

        let json = serde_json::to_value(&tx).unwrap();
        assert_eq!(json["to"], json!(null));

        let deserialized: TransactionPayload = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.to, None);
    }
}
