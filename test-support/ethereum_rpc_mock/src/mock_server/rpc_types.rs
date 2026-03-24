//! JSON-RPC request and response types
//!
//! This module contains the core types used for JSON-RPC communication:
//! request parameters and response structures for Ethereum mock server.

use alloy::primitives::{Address, Bytes, Log as InnerLog, B256, U256};
use anyhow::{Context, Result as AnyhowResult};
use serde_json::Value;

/// Parameters extracted from transaction requests
/// Note: from field removed for behavioral mock - focus on transaction content only
#[derive(Debug, Clone)]
pub struct TxParams {
    /// Recipient address (None for contract creation)
    pub to: Option<Address>,
    /// Value to transfer in wei
    pub value: U256,
    /// Transaction data/input
    pub data: Bytes,
    /// Gas limit
    pub gas: u64,
    /// Gas price in wei
    pub gas_price: U256,
    /// Transaction nonce
    pub nonce: u64,
}

impl TxParams {
    /// Create new TxParams with required fields (no from address needed)
    pub fn new(to: Option<Address>, value: U256, data: Bytes) -> Self {
        Self {
            to,
            value,
            data,
            gas: 21_000,
            gas_price: U256::from(20_000_000_000u64), // 20 gwei
            nonce: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallParams {
    pub from: Option<Address>,
    pub to: Address,
    pub input: Bytes,
    pub gas: Option<u64>,
    pub gas_price: Option<U256>,
}

impl CallParams {
    pub fn new(to: Address, input: Bytes) -> Self {
        Self {
            from: None,
            to,
            input,
            gas: None,
            gas_price: None,
        }
    }

    /// Get the target address
    pub fn target(&self) -> Option<Address> {
        Some(self.to)
    }
}

/// Unified response for blockchain requests (transactions and calls)
/// This consolidation reduces type proliferation while maintaining functionality
#[derive(Debug, Clone)]
pub enum Response {
    /// Successful execution
    Success {
        /// Transaction hash (for transactions only)
        hash: Option<B256>,
        /// Return data (for calls) or logs (for transactions)
        data: ResponseData,
        /// Optional scheduled transactions for delayed responses
        scheduled_transactions: Vec<crate::blockchain::ScheduledTransaction>,
    },
    /// Reverted execution
    Revert {
        /// Transaction hash (for transactions only)
        hash: Option<B256>,
        /// Optional revert reason
        reason: Option<String>,
    },
    /// Error in processing
    Error(String),
}

/// Data payload for successful responses
#[derive(Debug, Clone)]
pub enum ResponseData {
    /// Call return data
    Bytes(Bytes),
    /// Transaction logs
    Logs(Vec<InnerLog>),
}

impl Response {
    /// Create a successful transaction response
    pub fn transaction_success() -> Self {
        Self::Success {
            hash: Some(crate::blockchain::BlockchainState::generate_random_hash()),
            data: ResponseData::Logs(Vec::new()),
            scheduled_transactions: Vec::new(),
        }
    }

    /// Create a successful call response
    pub fn call_success(data: Bytes) -> Self {
        Self::Success {
            hash: None,
            data: ResponseData::Bytes(data),
            scheduled_transactions: Vec::new(),
        }
    }

    /// Create a revert response
    pub fn revert(reason: String) -> Self {
        Self::Revert {
            hash: Some(crate::blockchain::BlockchainState::generate_random_hash()),
            reason: Some(reason),
        }
    }

    /// Create an error response
    pub fn error(message: String) -> Self {
        Self::Error(message)
    }

    /// Add scheduled transactions to this response
    pub fn with_scheduled_transactions(
        mut self,
        scheduled: Vec<crate::blockchain::ScheduledTransaction>,
    ) -> Self {
        if let Self::Success {
            scheduled_transactions,
            ..
        } = &mut self
        {
            *scheduled_transactions = scheduled;
        }
        // Cannot add to revert or error responses - they don't have scheduled_transactions field
        self
    }

    /// Get the transaction hash if available
    pub fn hash(&self) -> Option<B256> {
        match self {
            Self::Success { hash, .. } | Self::Revert { hash, .. } => *hash,
            Self::Error(_) => None,
        }
    }

    /// Get logs from response
    pub fn logs(&self) -> &[InnerLog] {
        match self {
            Self::Success {
                data: ResponseData::Logs(logs),
                ..
            } => logs,
            _ => &[],
        }
    }

    /// Add log to response
    pub fn with_log(mut self, log: InnerLog) -> Self {
        if let Self::Success {
            data: ResponseData::Logs(logs),
            ..
        } = &mut self
        {
            logs.push(log);
        }
        self
    }

    /// Get scheduled transactions
    pub fn scheduled_transactions(&self) -> &[crate::blockchain::ScheduledTransaction] {
        match self {
            Self::Success {
                scheduled_transactions,
                ..
            } => scheduled_transactions,
            Self::Revert { .. } | Self::Error(_) => &[],
        }
    }
}

// Conversion Traits

impl TryFrom<&Value> for CallParams {
    type Error = anyhow::Error;

    fn try_from(tx: &Value) -> AnyhowResult<Self> {
        let tx_obj = tx
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Request parameter must be an object"))?;

        // Parse required 'to' field with validation
        let to_str = tx_obj
            .get("to")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required 'to' field in call params"))?;
        let to = to_str
            .parse::<Address>()
            .with_context(|| format!("Invalid 'to' address format: {}", to_str))?;

        // Parse 'input' field (alloy sends this field for eth_call)
        let input = tx_obj
            .get("input")
            .and_then(|v| v.as_str())
            .map(|s| {
                let hex_str = s.strip_prefix("0x").unwrap_or(s);
                // Validate hex string contains only valid characters
                if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(anyhow::anyhow!(
                        "Input field contains invalid hex characters: {}",
                        s
                    ));
                }
                hex::decode(hex_str)
                    .map(Bytes::from)
                    .with_context(|| format!("Failed to decode hex input: {}", s))
            })
            .transpose()?
            .unwrap_or_default();

        // Parse optional 'from' field with validation
        let from = tx_obj
            .get("from")
            .and_then(|v| v.as_str())
            .map(|s| {
                s.parse::<Address>()
                    .with_context(|| format!("Invalid 'from' address format: {}", s))
            })
            .transpose()?;

        Ok(Self {
            from,
            to,
            input,
            gas: None,
            gas_price: None,
        })
    }
}

impl TryFrom<&str> for TxParams {
    type Error = anyhow::Error;

    fn try_from(tx_hex: &str) -> AnyhowResult<Self> {
        // Validate input is not empty
        if tx_hex.is_empty() {
            return Err(anyhow::anyhow!("Transaction hex string cannot be empty"));
        }

        let hex_str = tx_hex.strip_prefix("0x").unwrap_or(tx_hex);

        // Validate hex string contains only valid characters
        if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "Transaction hex contains invalid characters: {}",
                tx_hex
            ));
        }

        let tx_bytes = hex::decode(hex_str)
            .with_context(|| format!("Failed to decode transaction hex data: {}", tx_hex))?;

        // Decode transaction using alloy
        use alloy::consensus::TxEnvelope;
        use alloy::eips::Decodable2718;

        let mut buf = tx_bytes.as_slice();
        let tx_envelope = TxEnvelope::decode_2718(&mut buf)
            .with_context(|| "Failed to decode transaction envelope")?;

        let (to, data, value, gas, gas_price, nonce) = match &tx_envelope {
            TxEnvelope::Legacy(tx) => {
                let gas_limit = tx.tx().gas_limit;
                let gas_price_val = tx.tx().gas_price;
                let nonce_val = tx.tx().nonce;

                // Validate reasonable gas limits
                if gas_limit > 30_000_000 {
                    return Err(anyhow::anyhow!("Gas limit too high: {} (max 30M)", gas_limit));
                }

                (
                    tx.tx().to.to().copied(),
                    tx.tx().input.clone(),
                    tx.tx().value,
                    Some(gas_limit),
                    Some(U256::from(gas_price_val)),
                    Some(nonce_val),
                )
            },
            TxEnvelope::Eip1559(tx) => {
                let gas_limit = tx.tx().gas_limit;
                let max_fee = tx.tx().max_fee_per_gas;
                let nonce_val = tx.tx().nonce;

                // Validate reasonable gas limits
                if gas_limit > 30_000_000 {
                    return Err(anyhow::anyhow!("Gas limit too high: {} (max 30M)", gas_limit));
                }

                (
                    tx.tx().to.to().copied(),
                    tx.tx().input.clone(),
                    tx.tx().value,
                    Some(gas_limit),
                    Some(U256::from(max_fee)),
                    Some(nonce_val),
                )
            },
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported transaction type. Only Legacy and EIP-1559 transactions are supported, got: {:?}",
                    tx_envelope
                ))
            }
        };

        Ok(Self {
            to,
            value,
            data,
            gas: gas.unwrap_or(21000),
            gas_price: gas_price.unwrap_or(U256::from(20_000_000_000u64)),
            nonce: nonce.unwrap_or(0),
        })
    }
}
