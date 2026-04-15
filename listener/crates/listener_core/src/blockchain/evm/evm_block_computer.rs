use alloy::{
    consensus::{Header, ReceiptWithBloom, TxReceipt, Typed2718},
    eips::eip2718::Encodable2718,
    network::{
        AnyReceiptEnvelope, AnyRpcBlock, AnyTransactionReceipt, AnyTxEnvelope, UnknownTxEnvelope,
    },
    primitives::{Address, B256, Bytes, Log, U256},
    rlp::Encodable,
    rpc::types::BlockTransactions,
    serde::OtherFields,
    trie::root::ordered_trie_root,
};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockVerificationError {
    #[error("Receipt Root Mismatch: expected {expected}, calculated {calculated}")]
    ReceiptRootMismatch { expected: B256, calculated: B256 },

    #[error("Transaction Root Mismatch: expected {expected}, calculated {calculated}")]
    TransactionRootMismatch { expected: B256, calculated: B256 },

    #[error("Block Hash Mismatch: expected {expected}, calculated {calculated}")]
    BlockHashMismatch { expected: B256, calculated: B256 },

    #[error("Missing full transactions in block")]
    MissingFullTransactions,

    #[error("Receipt count mismatch: expected {expected}, actual {actual}")]
    ReceiptCountMismatch { expected: usize, actual: usize },

    #[error("Unsupported transaction type {tx_type} at index {index}")]
    UnsupportedTransactionType { tx_type: u8, index: usize },

    #[error("Transaction encoding failed at index {index}: {reason}")]
    TransactionEncodingFailed { index: usize, reason: String },

    #[error("Transaction encoding failed at index {index}: missing field {field}")]
    TransactionFieldMissing { index: usize, field: String },

    #[error("Receipt encoding failed at index {index}: {reason}")]
    ReceiptEncodingFailed { index: usize, reason: String },

    #[error("Block hash verification exhausted all strategies")]
    BlockHashVerificationExhausted {
        expected: B256,
        standard_calculated: B256,
        avalanche_calculated: Option<B256>,
    },
}

/// A helper to pass pre-encoded RLP bytes to the trie root
/// without the compiler adding extra length prefixes.
struct RawBytes(Vec<u8>);

impl alloy::rlp::Encodable for RawBytes {
    fn encode(&self, out: &mut dyn alloy::rlp::BufMut) {
        out.put_slice(&self.0);
    }
    fn length(&self) -> usize {
        self.0.len()
    }
}

/// Extract a deserializable field from OtherFields.
fn extract_field<T: DeserializeOwned>(
    fields: &OtherFields,
    key: &str,
    index: usize,
) -> Result<T, BlockVerificationError> {
    fields
        .get_deserialized(key)
        .ok_or_else(|| BlockVerificationError::TransactionFieldMissing {
            index,
            field: key.to_string(),
        })?
        .map_err(|e| BlockVerificationError::TransactionEncodingFailed {
            index,
            reason: format!("Failed to deserialize field '{}': {}", key, e),
        })
}

/// Extract a u64 field, handling both hex strings and numbers via OtherFields.
fn extract_u64(
    fields: &OtherFields,
    key: &str,
    index: usize,
) -> Result<u64, BlockVerificationError> {
    // Use get_with to access the raw JSON value and handle both hex string and number formats
    fields
        .get_with(key, |v| {
            // Try as hex string first
            if let Some(s) = v.as_str() {
                u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
            } else {
                // Try as number
                v.as_u64()
            }
        })
        .flatten()
        .ok_or_else(|| BlockVerificationError::TransactionFieldMissing {
            index,
            field: key.to_string(),
        })
}

/// Extract an optional address field.
fn extract_optional_address(fields: &OtherFields, key: &str) -> Option<Address> {
    fields.get_deserialized::<Address>(key).and_then(|r| r.ok())
}

/// RLP encode an optional address (None becomes empty bytes).
fn encode_optional_address(addr: Option<Address>, out: &mut Vec<u8>) {
    match addr {
        Some(a) => a.encode(out),
        None => {
            // Encode empty bytes for contract creation
            let empty: &[u8] = &[];
            empty.encode(out);
        }
    }
}

/// Encode an Optimism deposit transaction (type 126 / 0x7E).
///
/// RLP Field Order:
/// 0x7E + RLP([sourceHash, from, to, mint, value, gas, isSystemTx, data])
///
/// Note: The `from` address comes from the recovered signer in the RPC response,
/// not from the inner fields, since deposit transactions don't have signatures.
fn encode_deposit_transaction(
    unknown: &UnknownTxEnvelope,
    from: Address,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;

    // Extract required fields
    let source_hash: B256 = extract_field(fields, "sourceHash", index)?;
    // Note: `from` is passed as parameter since it comes from the RPC response's signer field
    let to: Option<Address> = extract_optional_address(fields, "to");
    let mint: U256 = extract_field(fields, "mint", index).unwrap_or_default();
    let value: U256 = extract_field(fields, "value", index)?;
    let gas: u64 = extract_u64(fields, "gas", index)?;
    let is_system_tx: bool = extract_field(fields, "isSystemTx", index).unwrap_or(false);
    let data: Bytes = extract_field(fields, "input", index)?; // Note: "input" not "data"

    // RLP encode fields in order
    let mut payload = Vec::new();
    source_hash.encode(&mut payload);
    from.encode(&mut payload);
    encode_optional_address(to, &mut payload);
    mint.encode(&mut payload);
    value.encode(&mut payload);
    gas.encode(&mut payload);
    is_system_tx.encode(&mut payload);
    data.encode(&mut payload);

    // Build final: type_byte + RLP list
    let mut result = vec![0x7e];
    let header = alloy::rlp::Header {
        list: true,
        payload_length: payload.len(),
    };
    header.encode(&mut result);
    result.extend_from_slice(&payload);

    Ok(result)
}

/// Encode an Arbitrum internal transaction (type 106 / 0x6A).
///
/// RLP Field Order:
/// 0x6A + RLP([chainId, data])
fn encode_arbitrum_internal_transaction(
    unknown: &UnknownTxEnvelope,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;

    let chain_id: u64 = extract_u64(fields, "chainId", index)?;
    let data: Bytes = extract_field(fields, "input", index)?;

    let mut payload = Vec::new();
    chain_id.encode(&mut payload);
    data.encode(&mut payload);

    let mut result = vec![0x6a];
    let header = alloy::rlp::Header {
        list: true,
        payload_length: payload.len(),
    };
    header.encode(&mut result);
    result.extend_from_slice(&payload);

    Ok(result)
}

/// Encode a deposit receipt with L2-specific fields (type 126).
///
/// Standard encoding: `type + RLP([status, cumulative_gas_used, bloom, logs])`
/// L2 encoding: `type + RLP([status, cumulative_gas_used, bloom, logs, depositNonce, depositReceiptVersion])`
fn encode_deposit_receipt(
    receipt: &ReceiptWithBloom<alloy::consensus::Receipt<Log>>,
    deposit_nonce: Option<u64>,
    deposit_receipt_version: Option<u8>,
) -> Result<Vec<u8>, BlockVerificationError> {
    let mut payload = Vec::new();

    // Standard receipt fields (use TxReceipt trait methods)
    receipt.status().encode(&mut payload);
    receipt.cumulative_gas_used().encode(&mut payload);
    receipt.bloom().encode(&mut payload);

    // Encode logs as a list - convert slice to Vec for Encodable trait
    let logs: Vec<_> = receipt.logs().to_vec();
    logs.encode(&mut payload);

    // L2 deposit fields (Canyon upgrade)
    if let Some(nonce) = deposit_nonce {
        nonce.encode(&mut payload);
    }
    if let Some(version) = deposit_receipt_version {
        version.encode(&mut payload);
    }

    // Build final: type_byte + RLP list
    let mut result = vec![0x7e];
    let header = alloy::rlp::Header {
        list: true,
        payload_length: payload.len(),
    };
    header.encode(&mut result);
    result.extend_from_slice(&payload);

    Ok(result)
}

/// Encode a receipt, handling L2-specific formats.
fn encode_receipt(
    receipt: &AnyTransactionReceipt,
    _index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let any_envelope = &receipt.inner.inner;
    let receipt_type = any_envelope.r#type;

    // Transform logs to consensus format
    let consensus_receipt = any_envelope.inner.clone().map_logs(|rpc_log| rpc_log.inner);

    // Check for L2 deposit receipt (type 126)
    if receipt_type == 0x7e {
        // Extract L2-specific fields from "other"
        let deposit_nonce: Option<u64> = receipt
            .other
            .get("depositNonce")
            .and_then(|v| v.as_str())
            .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok());

        let deposit_receipt_version: Option<u8> = receipt
            .other
            .get("depositReceiptVersion")
            .and_then(|v| v.as_str())
            .and_then(|s| u8::from_str_radix(s.trim_start_matches("0x"), 16).ok());

        // If L2 fields present, encode with them
        if deposit_nonce.is_some() || deposit_receipt_version.is_some() {
            return encode_deposit_receipt(
                &consensus_receipt,
                deposit_nonce,
                deposit_receipt_version,
            );
        }
    }

    // Standard encoding for all other receipts
    let consensus_envelope = AnyReceiptEnvelope {
        inner: consensus_receipt,
        r#type: receipt_type,
    };

    Ok(consensus_envelope.encoded_2718())
}

/// EVM block computer that verifies block data consistency.
/// All EVM chains (Ethereum, Optimism, Arbitrum, Base, etc.) use the same block hash algorithm.
pub struct EvmBlockComputer;

impl EvmBlockComputer {
    /// Verify the entire block: transaction root, receipt root, and block hash.
    pub fn verify_block(
        block: &AnyRpcBlock,
        receipts: &[AnyTransactionReceipt],
    ) -> Result<(), BlockVerificationError> {
        Self::verify_transaction_root(block)?;
        Self::verify_receipt_root(block, receipts)?;
        Self::verify_block_hash(block)?;
        Ok(())
    }

    /// Safely encode a transaction, handling unknown types gracefully.
    /// Implements encoding for L2-specific transaction types:
    /// - Type 126 (0x7E): Optimism/Base deposit transactions
    /// - Type 106 (0x6A): Arbitrum internal transactions
    ///
    /// The `from` address is needed for deposit transactions which don't have signatures.
    fn safe_encode_transaction(
        tx: &AnyTxEnvelope,
        from: Address,
        index: usize,
    ) -> Result<Vec<u8>, BlockVerificationError> {
        match tx {
            AnyTxEnvelope::Ethereum(eth_tx) => Ok(eth_tx.encoded_2718()),
            AnyTxEnvelope::Unknown(unknown) => {
                let tx_type = unknown.ty();

                // Try type-specific encoding based on transaction type
                match tx_type {
                    // Optimism, Base
                    126 => encode_deposit_transaction(unknown, from, index),
                    // ARB
                    106 => encode_arbitrum_internal_transaction(unknown, index),
                    _ => Err(BlockVerificationError::UnsupportedTransactionType { tx_type, index }),
                }
            }
        }
    }

    /// Verify the transaction trie root matches the header's transactions_root.
    pub fn verify_transaction_root(block: &AnyRpcBlock) -> Result<(), BlockVerificationError> {
        let header = &block.header;

        let BlockTransactions::Full(txs) = &block.transactions else {
            return Err(BlockVerificationError::MissingFullTransactions);
        };

        let mut encoded_txs = Vec::with_capacity(txs.len());
        for (index, tx) in txs.iter().enumerate() {
            // Get the sender (from) address from the Recovered wrapper
            // tx.inner is Transaction<AnyTxEnvelope>, tx.inner.inner is Recovered<AnyTxEnvelope>
            let recovered = &tx.inner.inner;
            let from = recovered.signer();
            let tx_envelope = recovered.inner();
            let encoded = Self::safe_encode_transaction(tx_envelope, from, index)?;
            encoded_txs.push(RawBytes(encoded));
        }

        let calculated = ordered_trie_root(&encoded_txs);

        if calculated != header.transactions_root {
            return Err(BlockVerificationError::TransactionRootMismatch {
                expected: header.transactions_root,
                calculated,
            });
        }

        Ok(())
    }

    /// Verify the receipt trie root matches the header's receipts_root.
    /// Handles L2-specific receipt formats (e.g., Optimism deposit receipts with depositNonce).
    pub fn verify_receipt_root(
        block: &AnyRpcBlock,
        receipts: &[AnyTransactionReceipt],
    ) -> Result<(), BlockVerificationError> {
        let header = &block.header;

        // Verify receipt count matches transaction count
        let tx_count = match &block.transactions {
            BlockTransactions::Full(txs) => txs.len(),
            BlockTransactions::Hashes(hashes) => hashes.len(),
            BlockTransactions::Uncle => 0,
        };

        if receipts.len() != tx_count {
            return Err(BlockVerificationError::ReceiptCountMismatch {
                expected: tx_count,
                actual: receipts.len(),
            });
        }

        let mut encoded_receipts = Vec::with_capacity(receipts.len());

        for (index, r) in receipts.iter().enumerate() {
            let encoded = encode_receipt(r, index)?;
            encoded_receipts.push(RawBytes(encoded));
        }

        let calculated = ordered_trie_root(&encoded_receipts);

        if calculated != header.receipts_root {
            return Err(BlockVerificationError::ReceiptRootMismatch {
                expected: header.receipts_root,
                calculated,
            });
        }

        Ok(())
    }

    /// Verify the block hash matches the header's hash.
    pub fn verify_block_hash(block: &AnyRpcBlock) -> Result<(), BlockVerificationError> {
        let rpc_header = &block.header;
        let expected = rpc_header.hash;

        // Build standard Ethereum consensus header
        let consensus_header = Header {
            parent_hash: rpc_header.parent_hash,
            ommers_hash: rpc_header.ommers_hash,
            beneficiary: rpc_header.beneficiary,
            state_root: rpc_header.state_root,
            transactions_root: rpc_header.transactions_root,
            receipts_root: rpc_header.receipts_root,
            logs_bloom: rpc_header.logs_bloom,
            difficulty: rpc_header.difficulty,
            number: rpc_header.number,
            gas_limit: rpc_header.gas_limit,
            gas_used: rpc_header.gas_used,
            timestamp: rpc_header.timestamp,
            extra_data: rpc_header.extra_data.clone(),
            mix_hash: rpc_header.mix_hash.unwrap_or_default(),
            nonce: rpc_header.nonce.unwrap_or_default(),
            base_fee_per_gas: rpc_header.base_fee_per_gas,
            withdrawals_root: rpc_header.withdrawals_root,
            blob_gas_used: rpc_header.blob_gas_used,
            excess_blob_gas: rpc_header.excess_blob_gas,
            parent_beacon_block_root: rpc_header.parent_beacon_block_root,
            requests_hash: rpc_header.requests_hash,
        };

        let calculated = consensus_header.hash_slow();

        if calculated == expected {
            return Ok(());
        }

        Err(BlockVerificationError::BlockHashMismatch {
            expected,
            calculated,
        })
    }
}

#[cfg(test)]
#[path = "./tests/evm_block_computer_tests.rs"]
mod tests;
